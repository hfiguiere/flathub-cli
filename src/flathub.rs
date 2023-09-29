pub fn repo_for_package(package: &str) -> String {
    format!("https://github.com/flathub/{package}.git")
}
