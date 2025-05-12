// SPDX-FileCopyrightText: 2020-2025 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Serialize;
use serde_json::json;

#[derive(Clone, Debug)]
pub(crate) enum ModuleEntry {
    SharedModule(String),
    Module(Module),
}

impl From<&ModuleEntry> for serde_json::Value {
    fn from(entry: &ModuleEntry) -> serde_json::Value {
        match entry {
            ModuleEntry::SharedModule(s) => json!(s),
            ModuleEntry::Module(m) => json!(m),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum SourceType {
    Archive,
    Git,
    Patch,
    File,
}

impl std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                SourceType::Archive => "archive",
                SourceType::Git => "git",
                SourceType::Patch => "patch",
                SourceType::File => "file",
            }
        )
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct Source {
    #[serde(rename = "type")]
    pub(crate) type_: SourceType,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Buildsystem {
    #[default]
    Autotools,
    Cmake,
    CmakeNinja,
    Simple,
    Meson,
    Qmake,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Module {
    pub name: String,
    pub(crate) buildsystem: Buildsystem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_opts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleanup: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_install: Option<Vec<String>>,
    pub(crate) sources: Vec<Source>,
}

impl Module {
    /// Add the metainfo to the module
    pub fn add_metainfo(&mut self, id: &str) {
        let metainfo = format!("{id}.metainfo.xml");
        self.post_install = Some(vec![
            format!(
                "install -Dm644 {} -t ${{FLATPAK_DEST}}/share/metainfo",
                &metainfo
            ),
            "appstream-compose --basename=${FLATPAK_ID} --prefix=${FLATPAK_DEST} --origin=flatpak ${FLATPAK_ID}".to_string()
        ]);
        self.sources.push(Source {
            type_: SourceType::File,
            path: metainfo,
            sha256: None,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buildsystems() {
        fn test_value(bs: Buildsystem, expected: &str) {
            let buildsystem = json!(bs);

            assert_eq!(buildsystem.as_str(), Some(expected));
        }

        // These string values are immutable from the manifest expectations.
        test_value(Buildsystem::Autotools, "autotools");
        test_value(Buildsystem::Cmake, "cmake");
        test_value(Buildsystem::CmakeNinja, "cmake-ninja");
        test_value(Buildsystem::Simple, "simple");
        test_value(Buildsystem::Meson, "meson");
        test_value(Buildsystem::Qmake, "qmake");
    }
}
