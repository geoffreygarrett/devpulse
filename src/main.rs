use git2::{Repository, Revwalk, Error};
use url::Url;

// Function to convert a URL into a directory-friendly name
fn url_to_directory_name(url: &str) -> Result<String, url::ParseError> {
    let parsed_url = Url::parse(url)?;
    let host = parsed_url.host_str().unwrap_or_default().replace(".", "_");
    let path = parsed_url.path().replace("/", "_");
    Ok(format!("{}_{}", host, path.trim_matches('_')))
}

// Function to create a Revwalk to list commits between two commits
fn create_revwalk<'a>(repo: &'a Repository, old_commit: &'a str, new_commit: &'a str) -> Result<Revwalk<'a>, Error> {
    let old_commit = repo.revparse_single(old_commit)?.peel_to_commit()?;
    let new_commit = repo.revparse_single(new_commit)?.peel_to_commit()?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push(new_commit.id())?;
    revwalk.hide(old_commit.id())?;
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;
    Ok(revwalk)
}

// Function to calculate code churn between commits
fn calculate_code_churn(repo: &Repository, revwalk: Revwalk) -> Result<(), Error> {
    println!("Commit | Additions | Deletions");
    for id in revwalk {
        let commit = repo.find_commit(id?)?;
        let tree = commit.tree()?;
        let parent_id = commit.parent(0).ok().map_or_else(|| repo.treebuilder(None).unwrap().write().unwrap(), |p| p.id());
        let parent = repo.find_commit(parent_id)?.tree()?;
        let diff = repo.diff_tree_to_tree(Some(&parent), Some(&tree), None)?;
        let deletions = diff.stats()?.deletions();
        let additions = diff.stats()?.insertions();
        println!("Commit: {} | Additions: {} | Deletions: {}", commit.id(), additions, deletions);
    }
    Ok(())
}


fn main() {
    let url = "https://github.com/tudat-team/tudatpy";
    let local_path = match url_to_directory_name(url) {
        Ok(name) => format!("./.cache/{}", name),
        Err(e) => {
            eprintln!("Failed to parse URL: {}", e);
            return;
        }
    };

    let repo = match Repository::open(&local_path) {
        Ok(repo) => repo,
        Err(_) => match Repository::clone(url, &local_path) {
            Ok(repo) => repo,
            Err(e) => {
                eprintln!("Failed to clone repository: {}", e);
                return;
            }
        },
    };

    let old_commit = "469fddb7d66f06981596c051cf7cca2681e55f55";  // Earlier commit
    let new_commit = "60719404b37c8cfb747e9d4cd7ef6f865b148acf";  // Later commit

    if let Ok(mut commits) = create_revwalk(&repo, old_commit, new_commit) {
        let commit_count = commits.by_ref().count();
        println!("Number of commits from old ({}) to new ({}): {}", old_commit, new_commit, commit_count);

        if let Ok(commits) = create_revwalk(&repo, old_commit, new_commit) {
            if let Err(e) = calculate_code_churn(&repo, commits) {
                eprintln!("Failed to calculate code churn: {}", e);
            }
        }
    } else {
        eprintln!("Failed to list commits.");
    }

    println!("Repository status at {:?}", repo.path());
}
