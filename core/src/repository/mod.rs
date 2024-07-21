use git2::{Error, Repository, Revwalk, DiffOptions};
use crate::models::CodeChurn;

pub fn create_revwalk<'a>(repo: &'a Repository, old_commit: &'a str, new_commit: &'a str) -> Result<Revwalk<'a>, Error> {
    let old_commit = repo.revparse_single(old_commit)?.peel_to_commit()?;
    let new_commit = repo.revparse_single(new_commit)?.peel_to_commit()?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push(new_commit.id())?;
    revwalk.hide(old_commit.id())?;
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;
    Ok(revwalk)
}

pub fn calculate_code_churn(repo: &Repository, revwalk: Revwalk) -> Result<Vec<CodeChurn>, Error> {
    let mut churn_data = Vec::new();

    for id in revwalk {
        let commit = repo.find_commit(id?)?;
        let tree = commit.tree()?;
        let parent_tree = if let Ok(parent) = commit.parent(0) {
            parent.tree()?
        } else {
            repo.treebuilder(None)?.write().map(|oid| repo.find_tree(oid))?? // Create an empty tree if no parent
        };

        let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), Some(&mut DiffOptions::new()))?;
        let stats = diff.stats()?;

        churn_data.push(CodeChurn::new(commit.id().to_string(), stats.insertions(), stats.deletions()));
    }

    Ok(churn_data)
}
