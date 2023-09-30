/// Check if the git repository at `repo` exists.
pub fn check_repo_exist<P>(repo: P) -> bool
where
    P: AsRef<std::path::Path>,
{
    matches!(git2::Repository::open(repo), Ok(_))
}
