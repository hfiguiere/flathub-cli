// SPDX-FileCopyrightText: 2025 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Everything about flatpak-builder

use std::process::Command;

use crate::Result;

/// Return the directory for the downloads relative to the top-level.
pub fn downloads_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(".flatpak-builder/downloads")
}

/// Return the directory for the git repositories relative to the top-level.
pub fn git_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(".flatpak-builder/git")
}

/// Run the flatpak-builder with all the arguments `args`.
pub fn run(args: &[&str]) -> Result<Vec<u8>> {
    Ok(Command::new("flatpak-builder").args(args).output()?.stdout)
}
