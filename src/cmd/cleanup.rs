// SPDX-FileCopyrightText: 2025 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};
use clap::Parser;
use multimap::MultiMap;
use serde_json::Value as JsonValue;
use url::Url;

use crate::builder;
use crate::project::Project;
use crate::{Error, Result};

#[derive(Parser)]
pub struct Args {
    /// Dry-run: do not remove anything.
    #[arg(short = 'n', long)]
    dry_run: bool,
    /// Verbose
    #[arg(short = 'v', long)]
    verbose: bool,
    /// Subcommand: "downloads"
    command: String,
}

fn cleanup_downloads(dry_run: bool, verbose: bool) -> Result<()> {
    let current_dir = std::env::current_dir().context("Get current dir")?;
    let project = Project::open(&current_dir).context("Open project")?;

    // Get download dir
    let downloads_dir = current_dir.join(crate::builder::downloads_dir());
    if !downloads_dir.exists() || !downloads_dir.is_dir() {
        eprintln!("No download dir found");
        return Err(Error::NotFound.into());
    }

    // List downloads
    let mut downloads = MultiMap::new();
    if let Ok(dir) = std::fs::read_dir(&downloads_dir) {
        for entry in dir {
            let path = entry.context("Get dir entry")?.path();
            if !path.is_dir() {
                continue;
            }
            if let Some(name) = path.file_name() {
                if let Ok(dir) = std::fs::read_dir(&path) {
                    for entry in dir {
                        let path = entry?.path();
                        if !path.is_file() {
                            continue;
                        }
                        if verbose {
                            let path = path.strip_prefix(&current_dir)?;
                            println!("Found {path:?} (checksum {name:?})");
                        }
                        downloads.insert(name.to_owned(), path.canonicalize().unwrap());
                    }
                }
            }
        }
    }

    // Get git dir
    // let git_dir = current_dir.join(crate::builder::git_dir());
    // List git repos (canonicalize from dir name). Problem: managing submodules.
    let mut git_repos = MultiMap::new();
    if let Ok(dir) = std::fs::read_dir(&downloads_dir) {
        for entry in dir {
            let path = entry?.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    git_repos.insert(name.to_owned(), path.to_owned());
                }
            }
        }
    }

    let sources = declared_sources(&project)?;
    sources
        .iter()
        .filter_map(|source| {
            let type_ = source.get("type")?;
            if type_ != "archive" {
                return None;
            }
            let sha256 = source.get("sha256")?.as_str()?;
            let url = source.get("url")?.as_str()?;
            let url = Url::parse(url).ok()?;
            let names: Vec<_> = url.path_segments()?.collect::<Vec<_>>();
            let name = names.iter().last()?.to_string();
            let path = build_download_path(&current_dir, &name, sha256);
            Some((std::ffi::OsString::from(sha256), path))
        })
        .for_each(|source| {
            if let Some(v) = downloads.get_vec_mut(&source.0) {
                if let Some(idx) = v.iter().position(|s| s == &source.1) {
                    v.remove(idx);
                }
            }
        });

    // Go through the lefover and remove them.
    let mut total_size = 0_u64;
    for download in downloads.flat_iter() {
        let metadata = std::fs::metadata(download.1)?;
        total_size += metadata.len();
        if dry_run {
            // display what to do
            let path = download.1.strip_prefix(&current_dir)?;
            println!("Would delete {:?}", path);
        } else {
            // remove
            if verbose {
                let path = download.1.strip_prefix(&current_dir)?;
                println!("Deleting {:?}", path);
            }
            std::fs::remove_file(download.1)?;
        }
    }
    if dry_run {
        println!(
            "Would have saved {}.",
            humanize_bytes::humanize_bytes_decimal!(total_size)
        );
    } else if verbose {
        println!(
            "Deleted {}.",
            humanize_bytes::humanize_bytes_decimal!(total_size)
        );
    }

    Ok(())
}

fn build_download_path(current_dir: &Path, name: &str, sha: &str) -> PathBuf {
    let downloads_dir = current_dir.join(crate::builder::downloads_dir());
    let download_path = downloads_dir.join(sha);
    download_path.join(name)
}

/// Get all the sources declared in the manifest.
fn declared_sources(project: &Project) -> Result<Vec<JsonValue>> {
    let manifest_file = project.manifest_file().to_string_lossy().to_string();
    // Parse manifest
    let manifest_text = builder::run(&["--show-manifest", &manifest_file])?;
    // List all download. Mark them in the existing list
    let manifest: JsonValue = serde_json::from_slice(&manifest_text)?;
    manifest
        .get("modules")
        .ok_or(Error::NotFound.into())
        .and_then(all_sources_from_modules)
}

/// Get all sources for the module JSON. This will go down recursively.
fn all_sources_from_modules(modules: &JsonValue) -> Result<Vec<JsonValue>> {
    if !modules.is_array() {
        return Err(anyhow!("JSON: modules not an array"));
    }
    let mut sources = vec![];
    for module in modules.as_array().unwrap() {
        if let Some(s) = module.get("sources") {
            if !s.is_array() {
                // XXX error
                continue;
            }
            for source in s.as_array().unwrap() {
                sources.push(source.clone());
            }
        }
        // Recursively dig down submodules.
        if let Some(m) = module.get("modules") {
            if let Ok(mut s) = all_sources_from_modules(m) {
                sources.append(&mut s);
            }
        }
    }
    Ok(sources)
}

/// Run the command
pub fn run(args: Args) -> Result<()> {
    let command = args.command.as_str();
    match command {
        "downloads" => cleanup_downloads(args.dry_run, args.verbose),
        _ => Err(Error::InvalidArgument.into()),
    }
}
