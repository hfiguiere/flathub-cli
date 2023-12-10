// SPDX-FileCopyrightText: 2023 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

/// Check if the git repository at `repo` exists.
pub fn check_repo_exist<P>(repo: P) -> bool
where
    P: AsRef<std::path::Path>,
{
    git2::Repository::open(repo).is_ok()
}
