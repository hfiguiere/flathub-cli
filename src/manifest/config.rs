// SPDX-FileCopyrightText: 2023 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

use super::module::{Module, ModuleEntry, Source, SourceType};

pub(crate) const FREEDESKTOP_VERSION: &str = "23.08";
pub(crate) const GNOME_VERSION: &str = "45";
pub(crate) const KDE_VERSION: &str = "5.15-23.08";
pub(crate) const QT6_VERSION: &str = "6.6";
pub(crate) const GIMP_PLUGIN_VERSION: &str = "2-40";

fn fltk_module() -> Module {
    Module {
        name: "fltk-static".to_string(),
        config_opts: Some(vec![
            "--enable-threads".to_string(),
            "--enable-cairo".to_string(),
            "--disable-shared".to_string(),
            "--enable-static".to_string(),
        ]),
        sources: vec![Source {
            type_: SourceType::Archive,
            path: "https://github.com/fltk/fltk/archive/release-1.3.5.tar.gz".to_string(),
            sha256: Some(
                "5c534287b0e03b9520ff866704a5649268986b371bdf8f6ac003fa240e761901".to_string(),
            ),
        }],
        cleanup: Some(vec![
            "/bin".to_string(),
            "/include".to_string(),
            "/man".to_string(),
            "/share/doc".to_string(),
            "/share/man".to_string(),
            "/lib/pkgconfig".to_string(),
            "*.so".to_string(),
            "*.la".to_string(),
            "*.a".to_string(),
        ]),
        ..Module::default()
    }
}

lazy_static::lazy_static! {
    pub(crate) static ref DEFAULT_MODULES: HashMap<&'static str, ModuleEntry> = {
        HashMap::from([
            (
                "gtk2",
                ModuleEntry::Include("shared-modules/gtk2/gtk2.json".into()),
            ),
            (
                "python 2.7",
                ModuleEntry::Include("shared-modules/python2.7/python-2.7.json".into()),
            ),
            (
                "lv2",
                ModuleEntry::Include("shared-modules/linux-audio/lv2.json".into()),
            ),
            (
                "fftw3f",
                ModuleEntry::Include("shared-modules/linux-audio/fftw3f.json".into()),
            ),
            (
                "fftw3f-static",
                ModuleEntry::Include("shared-modules/linux-audio/fftw3f-static.json".into()),
            ),
            ("fltk", ModuleEntry::Module(fltk_module())),
        ])
    };
}
