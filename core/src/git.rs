// Function to convert a URL into a directory-friendly name
pub(crate) fn url_to_directory_name(url: &str) -> Result<String, url::ParseError> {
    let parsed_url = Url::parse(url)?;
    let host = parsed_url.host_str().unwrap_or_default().replace(".", "_");
    let path = parsed_url.path().replace("/", "_");
    Ok(format!("{}_{}", host, path.trim_matches('_')))
}

// Function to create a Revwalk to list commits between two commits
pub(crate) fn create_revwalk<'a>(repo: &'a Repository, old_commit: &'a str, new_commit: &'a str) -> Result<Revwalk<'a>, Error> {
    let old_commit = repo.revparse_single(old_commit)?.peel_to_commit()?;
    let new_commit = repo.revparse_single(new_commit)?.peel_to_commit()?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push(new_commit.id())?;
    revwalk.hide(old_commit.id())?;
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;
    Ok(revwalk)
}

// Function to calculate code churn between commits
pub(crate) fn calculate_code_churn(repo: &Repository, revwalk: Revwalk) -> Result<(), Error> {
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