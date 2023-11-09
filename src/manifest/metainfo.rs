// SPDX-FileCopyrightText: 2020-2023 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::prelude::*;
use std::path::Path;

use xmlwriter::*;

use crate::manifest::Manifest;

pub fn generate(manifest: &Manifest, dest_dir: &Path) -> Result<(), std::io::Error> {
    let mut metainfo_file = dest_dir.to_path_buf();
    metainfo_file.push(format!("{}.metainfo.xml", &manifest.id));

    let mut file = std::fs::File::create(metainfo_file)?;

    let mut w = XmlWriter::new(Options::default());
    w.write_declaration();

    w.start_element("component");
    w.write_attribute("type", "addon");

    w.start_element("id");
    w.set_preserve_whitespaces(true);
    w.write_text(&manifest.id);
    w.end_element();
    w.set_preserve_whitespaces(false);

    w.start_element("extends");
    w.set_preserve_whitespaces(true);
    w.write_text(manifest.runtime.to_string_name());
    w.end_element();
    w.set_preserve_whitespaces(false);

    w.start_element("name");
    w.set_preserve_whitespaces(true);
    w.write_text(&manifest.short_id);
    w.end_element();
    w.set_preserve_whitespaces(false);

    w.start_element("summary");
    w.set_preserve_whitespaces(true);
    w.write_text(&manifest.short_id);
    w.end_element();
    w.set_preserve_whitespaces(false);

    w.start_element("project_license");
    w.end_element();

    w.start_element("metadata_license");
    w.set_preserve_whitespaces(true);
    w.write_text("CC0-1.0");
    w.end_element();
    w.set_preserve_whitespaces(false);

    w.start_element("update_contact");
    w.set_preserve_whitespaces(true);
    w.write_text("CONTACT");
    w.end_element();
    w.set_preserve_whitespaces(false);

    w.start_element("url");
    w.write_attribute("type", "homepage");
    w.end_element();

    w.start_element("releases");
    w.start_element("release");
    w.end_element();
    w.end_element();

    file.write_all(w.end_document().as_bytes())?;

    Ok(())
}
