// SPDX-FileCopyrightText: 2025 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Everything about flatpak-builder

use std::process::Command;

use anyhow::anyhow;

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
    let output = Command::new("flatpak-builder").args(args).output()?;
    if !output.status.success() {
        return Err(anyhow!(format!(
            "flatpak-builder error: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    Ok(output.stdout)
}
