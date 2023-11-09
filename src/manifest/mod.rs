// SPDX-FileCopyrightText: 2020-2023 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod config;
mod flathub;
mod metainfo;
mod module;
mod sdk_extension;
pub(crate) mod tui;

use dialoguer::Input;
use serde::Serialize;
use serde_json::json;

use module::ModuleEntry;
use tui::Prompt;

pub(crate) use sdk_extension::SdkExtension;

#[derive(Debug, Default)]
pub(crate) enum PackageType {
    #[default]
    None,
    LinuxAudioPlugin,
    GimpPlugin,
    Application,
}

impl PackageType {
    pub fn base_id(&self) -> String {
        match *self {
            Self::None | Self::Application => "".to_string(),
            Self::GimpPlugin => "org.gimp.GIMP.Plugin.".to_string(),
            Self::LinuxAudioPlugin => "org.freedesktop.LinuxAudio.Plugins.".to_string(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Sdk {
    Freedesktop(String),
    Gnome(String),
    Kde(String),
    Qt6(String),
}

impl Default for Sdk {
    fn default() -> Sdk {
        Sdk::Freedesktop(config::FREEDESKTOP_VERSION.to_string())
    }
}

impl std::string::ToString for Sdk {
    fn to_string(&self) -> String {
        match self {
            Self::Freedesktop(s) => format!("org.freedesktop.Sdk//{s}"),
            Self::Gnome(s) => format!("org.gnome.Sdk//{s}"),
            Self::Kde(s) => format!("org.kde.Sdk//{s}"),
            Self::Qt6(s) => format!("org.kde.Sdk//{s}"),
        }
    }
}

#[derive(Debug, Default)]
pub enum Runtime {
    #[default]
    None,
    Freedesktop(String),
    Gnome(String),
    Kde(String),
    Qt6(String),
    Other(String, String),
}

impl std::string::ToString for Runtime {
    fn to_string(&self) -> String {
        format!("{}//{}", self.to_string_name(), self.to_string_version())
    }
}

impl Runtime {
    pub fn to_string_name(&self) -> &str {
        match self {
            Self::None => "",
            Self::Freedesktop(_) => "org.freedesktop.Platform",
            Self::Gnome(_) => "org.gnome.Platform",
            Self::Kde(_) => "org.kde.Platform",
            Self::Qt6(_) => "org.kde.Platform",
            Self::Other(ref s, _) => s,
        }
    }

    fn to_string_version(&self) -> &str {
        match self {
            Self::None => "",
            Self::Freedesktop(ref s) | Self::Gnome(ref s) | Self::Kde(ref s) | Self::Qt6(ref s) => {
                s
            }
            Self::Other(_, ref v) => v,
        }
    }
}

#[derive(Default, Debug)]
pub struct Manifest {
    pub short_id: String,
    pub(crate) package_type: PackageType,
    pub use_base_app: bool,
    pub id: String,
    pub runtime: Runtime,
    pub(crate) sdk: Sdk,
    pub sdk_extensions: Vec<SdkExtension>,
    pub(crate) modules: Vec<ModuleEntry>,
}

impl Manifest {
    fn is_extension(&self) -> bool {
        matches!(
            self.package_type,
            PackageType::LinuxAudioPlugin | PackageType::GimpPlugin
        )
    }

    fn get_prefix(&self) -> String {
        match self.package_type {
            PackageType::LinuxAudioPlugin => format!("/app/extensions/Plugins/{}", &self.short_id),
            PackageType::GimpPlugin => format!("/app/extensions/{}", &self.short_id),
            _ => "/app".to_string(),
        }
    }

    pub fn generate<P: AsRef<std::path::Path>>(self, dest: P) -> Result<(), std::io::Error> {
        let dest_dir = std::path::PathBuf::from(dest.as_ref());

        let mut manifest_file = dest_dir.clone();
        manifest_file.push(format!("{}.json", &self.id));

        if self.is_extension() {
            metainfo::generate(&self, &dest_dir)?;
            let flathub = flathub::Flathub {
                skip_icons_check: true,
            };
            flathub.generate(&dest_dir)?;
        }

        let data: serde_json::Value = self.into();

        let file = std::fs::File::create(manifest_file)?;
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut serializer = serde_json::Serializer::with_formatter(file, formatter);
        data.serialize(&mut serializer)?;

        Ok(())
    }

    pub(crate) fn prompt_with_id(id: Option<&str>) -> Option<Manifest> {
        let package_type = PackageType::prompt().unwrap();
        let use_base_app = matches!(
            package_type,
            PackageType::GimpPlugin | PackageType::LinuxAudioPlugin
        );
        let runtime = {
            use PackageType::*;
            match package_type {
                GimpPlugin => Runtime::Other("org.gimp.GIMP".to_string(), "stable".to_string()),
                LinuxAudioPlugin => Runtime::Other(
                    "org.freedesktop.LinuxAudio.BaseExtension".to_string(),
                    "stable".to_string(),
                ),
                _ => Runtime::prompt().unwrap(),
            }
        };

        let sdk = {
            use PackageType::*;
            match package_type {
                GimpPlugin => Sdk::Gnome("42".to_string()),
                LinuxAudioPlugin => Sdk::Freedesktop(config::FREEDESKTOP_VERSION.to_string()),
                _ => match runtime {
                    Runtime::Freedesktop(ref s) => Sdk::Freedesktop(s.clone()),
                    Runtime::Gnome(ref s) => Sdk::Gnome(s.clone()),
                    Runtime::Kde(ref s) => Sdk::Kde(s.clone()),
                    Runtime::Qt6(ref s) => Sdk::Qt6(s.clone()),
                    _ => Sdk::prompt().unwrap(),
                },
            }
        };

        let sdk_extensions = {
            let mut extensions = vec![];

            while let Some(ext) = SdkExtension::prompt() {
                extensions.push(ext);
            }

            extensions
        };

        let base_id = package_type.base_id();
        let short_id: String;

        let id = if let Some(id) = id {
            short_id = id.rsplit_once('.').unwrap_or(("", id)).1.to_string();
            id.to_string()
        } else {
            short_id = Input::new()
                .with_prompt(&format!("Package ID {base_id}"))
                .interact()
                .unwrap();
            format!("{base_id}{short_id}")
        };

        let mut modules = vec![];
        while let Some(module) = ModuleEntry::prompt() {
            modules.push(module)
        }

        match package_type {
            PackageType::GimpPlugin | PackageType::LinuxAudioPlugin => {
                if let Some(ModuleEntry::Module(main_module)) = modules.last_mut() {
                    main_module.add_metainfo(&id);
                }
            }
            _ => (),
        }

        Some(Manifest {
            short_id,
            id,
            package_type,
            use_base_app,
            runtime,
            sdk,
            sdk_extensions,
            modules,
        })
    }
}

impl From<Manifest> for serde_json::Value {
    fn from(manifest: Manifest) -> serde_json::Value {
        let mut data = json!({
            "id": manifest.id,
            "runtime": manifest.runtime.to_string_name(),
            "runtime-version": manifest.runtime.to_string_version(),
            "sdk": manifest.sdk.to_string(),
        });
        match manifest.package_type {
            PackageType::LinuxAudioPlugin | PackageType::GimpPlugin => {
                data["build-extension"] = json!(true);
                data["appstream-compose"] = json!(false);
                data["branch"] = match manifest.package_type {
                    // For Audio plugins, FREEDESKTOP_VERSION is the branch
                    PackageType::LinuxAudioPlugin => config::FREEDESKTOP_VERSION.into(),
                    PackageType::GimpPlugin => config::GIMP_PLUGIN_VERSION.into(),
                    _ => unreachable!(),
                };
                data["build-options"] = json!({
                    "prefix": manifest.get_prefix()
                });
            }
            _ => {}
        }
        let sdk_extensions: Vec<serde_json::Value> = manifest
            .sdk_extensions
            .iter()
            .map(|e| serde_json::Value::from(e.to_string()))
            .collect();
        if !sdk_extensions.is_empty() {
            data["sdk-extensions"] = sdk_extensions.into();
        }

        data["modules"] = manifest
            .modules
            .iter()
            .map(serde_json::Value::from)
            .collect();

        data
    }
}
