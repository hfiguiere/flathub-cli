// SPDX-FileCopyrightText: 2020-2023 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde_json::json;

pub struct Flathub {
    pub skip_icons_check: bool,
}

impl Flathub {
    /// Generate the flathub.json file and return its path.
    pub fn generate(
        &self,
        dest_dir: &std::path::Path,
    ) -> Result<std::path::PathBuf, std::io::Error> {
        let mut flathub_file = dest_dir.to_path_buf();
        flathub_file.push("flathub.json");

        let mut j = json!({});
        if self.skip_icons_check {
            j["skip-icons-check"] = json!(self.skip_icons_check);
        }

        let mut file = std::fs::File::create(&flathub_file)?;
        serde_json::to_writer_pretty(&mut file, &j)?;

        Ok(flathub_file)
    }
}
