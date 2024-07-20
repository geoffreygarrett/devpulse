use git2::{Error, Repository, Revwalk};
use serde::{Deserialize, Serialize};
use url::Url;

// Function to convert a URL into a directory-friendly name
pub fn url_to_directory_name(url: &str) -> Result<String, url::ParseError> {
    let parsed_url = Url::parse(url)?;
    let host = parsed_url.host_str().unwrap_or_default().replace(".", "_");
    let path = parsed_url.path().replace("/", "_");
    Ok(format!("{}_{}", host, path.trim_matches('_')))
}

// Function to create a Revwalk to list commits between two commits
pub fn create_revwalk<'a>(repo: &'a Repository, old_commit: &'a str, new_commit: &'a str) -> Result<Revwalk<'a>, Error> {
    let old_commit = repo.revparse_single(old_commit)?.peel_to_commit()?;
    let new_commit = repo.revparse_single(new_commit)?.peel_to_commit()?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push(new_commit.id())?;
    revwalk.hide(old_commit.id())?;
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;
    Ok(revwalk)
}

// Function to calculate code churn between commits
#[derive(Debug, Serialize, Deserialize)]
pub struct CodeChurn {
    commit: String,
    additions: usize,
    deletions: usize,
}

pub fn calculate_code_churn(repo: &Repository, revwalk: Revwalk) -> Result<Vec<CodeChurn>, Error> {
    let mut churn_data = Vec::new();

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

        churn_data.push(CodeChurn {
            commit: commit.id().to_string(),
            additions,
            deletions,
        });
    }

    Ok(churn_data)
}


pub fn process_repository_v1(url: &str, old_commit: &str, new_commit: &str) -> Result<Vec<CodeChurn>, Box<dyn std::error::Error>> {
    let local_path = url_to_directory_name(url)
        .map(|name| format!("./.cache/{}", name))
        .map_err(|e| format!("Failed to parse URL: {}", e))?;

    let repo = Repository::open(&local_path).or_else(|_| {
        Repository::clone(url, &local_path).map_err(|e| format!("Failed to clone repository: {}", e))
    })?;

    let mut commits = create_revwalk(&repo, old_commit, new_commit)
        .map_err(|e| format!("Failed to list commits: {}", e))?;
    let commit_count = commits.by_ref().count();
    println!("Number of commits from old ({}) to new ({}): {}", old_commit, new_commit, commit_count);

    let commits = create_revwalk(&repo, old_commit, new_commit)?;
    let code_churns = calculate_code_churn(&repo, commits).map_err(|e| format!("Failed to calculate code churn: {}", e))?;

    println!("Repository status at {:?}", repo.path());

    Ok(code_churns)
}
