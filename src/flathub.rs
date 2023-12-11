// SPDX-FileCopyrightText: 2020-2023 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub(crate) const SHARED_MODULES_REPO: &str = "https://github.com/flathub/shared-modules.git";
pub(crate) const SHARED_MODULES: &str = "shared-modules";

pub fn repo_for_package(package: &str) -> String {
    format!("https://github.com/flathub/{package}.git")
}
