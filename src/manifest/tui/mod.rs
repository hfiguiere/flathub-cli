// SPDX-FileCopyrightText: 2023 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Implements the Text UI

mod prompt;

use dialoguer::{theme::ColorfulTheme, Input, Select};

use super::config;
use super::module::{Buildsystem, Module, ModuleEntry};
use super::{PackageType, Runtime, Sdk, SdkExtension};
pub(crate) use prompt::Prompt;

fn prompt_sdk_ext_version(idx: usize) -> Option<SdkExtension> {
    match idx {
        1 => Some(SdkExtension::Rust),
        2 => Some(SdkExtension::RustNightly),
        3 => Input::<String>::new()
            .with_prompt("Java SDK version")
            .interact()
            .ok()
            .map(SdkExtension::Java),
        4 => Input::<String>::new()
            .with_prompt("Node version")
            .interact()
            .ok()
            .map(SdkExtension::Node),
        5 => Input::<String>::new()
            .with_prompt("PHP version")
            .interact()
            .ok()
            .map(SdkExtension::Php),
        6 => Some(SdkExtension::GoLang),
        7 => Some(SdkExtension::TexLive),
        8 => Input::<String>::new()
            .with_prompt("LLVM version")
            .interact()
            .ok()
            .map(SdkExtension::Llvm),
        9 => Input::<String>::new()
            .with_prompt(".Net version")
            .interact()
            .ok()
            .map(SdkExtension::DotNet),
        10 => Input::<String>::new()
            .with_prompt("Mono version")
            .interact()
            .ok()
            .map(SdkExtension::Mono),
        11 => Input::<String>::new()
            .with_prompt("gcc version")
            .interact()
            .ok()
            .map(SdkExtension::Gcc),
        _ => None,
    }
}

impl Prompt for SdkExtension {
    fn prompt() -> Option<SdkExtension> {
        let choices = &[
            "None",
            "Rust (stable)",
            "Rust (nightly)",
            "Java",
            "Node",
            "Php",
            "Go",
            "TexLive",
            "LLvm",
            ".Net",
            "Mono",
            "Gcc",
        ];

        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select the SDK Extension:")
            .default(0)
            .items(choices)
            .interact()
            .ok()
            .and_then(|selection| match selection {
                1..=11 => prompt_sdk_ext_version(selection),
                _ => None,
            })
    }
}

impl Prompt for Module {
    fn prompt() -> Option<Module> {
        let name = Input::<String>::new()
            .with_prompt("Module name")
            .interact()
            .ok()?;
        let buildsystem = Buildsystem::prompt()?;

        Some(Module {
            name,
            buildsystem,
            ..Module::default()
        })
    }
}

impl Prompt for Buildsystem {
    fn prompt() -> Option<Buildsystem> {
        use Buildsystem::*;

        let choices = &[
            "autotools",
            "cmake",
            "cmake-ninja",
            "simple",
            "meson",
            "qmake",
        ];

        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a build system:")
            .default(0)
            .items(choices)
            .interact()
            .ok()
            .and_then(|selection| match selection {
                0 => Some(Autotools),
                1 => Some(Cmake),
                2 => Some(CmakeNinja),
                3 => Some(Simple),
                4 => Some(Meson),
                5 => Some(Qmake),
                _ => None,
            })
    }
}

impl Prompt for ModuleEntry {
    fn prompt() -> Option<ModuleEntry> {
        let mut choices = vec!["None", "Custom"];
        config::DEFAULT_MODULES.keys().for_each(|k| choices.push(k));

        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a module:")
            .default(0)
            .items(&choices)
            .interact()
            .ok()
            .and_then(|selection| match selection {
                0 => None,
                1 => Module::prompt().map(ModuleEntry::Module),
                _ => config::DEFAULT_MODULES.get(choices[selection]).cloned(),
            })
    }
}

fn prompt_runtime_version(idx: usize) -> Option<Runtime> {
    match idx {
        0 => Some(Runtime::Freedesktop(
            config::FREEDESKTOP_VERSION.to_string(),
        )),
        1 => Some(Runtime::Gnome(config::GNOME_VERSION.to_string())),
        2 => Some(Runtime::Kde(config::KDE_VERSION.to_string())),
        3 => Some(Runtime::Qt6(config::QT6_VERSION.to_string())),
        _ => None,
    }
}

impl Prompt for Runtime {
    fn prompt() -> Option<Runtime> {
        let choices = &["Freedesktop", "GNOME", "KDE/Qt 5.15", "Qt 6", "Other"];

        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select the runtime:")
            .default(0)
            .items(choices)
            .interact()
            .ok()
            .and_then(|selection| {
                match selection {
                    0..=3 => prompt_runtime_version(selection),
                    4 =>
                    // prompt other runtime
                    {
                        Some(Runtime::Other("undefined".into(), "undefined".into()))
                    }
                    _ => None,
                }
            })
    }
}

fn prompt_sdk_version(idx: usize) -> Option<Sdk> {
    match idx {
        0 => Some(Sdk::Freedesktop(config::FREEDESKTOP_VERSION.to_string())),
        1 => Some(Sdk::Gnome(config::GNOME_VERSION.to_string())),
        2 => Some(Sdk::Kde(config::KDE_VERSION.to_string())),
        3 => Some(Sdk::Qt6(config::QT6_VERSION.to_string())),
        _ => None,
    }
}

impl Prompt for Sdk {
    fn prompt() -> Option<Sdk> {
        let choices = &["Freedesktop", "GNOME", "KDE/Qt 5.15", "Qt 6"];

        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select the SDK:")
            .default(0)
            .items(choices)
            .interact()
            .ok()
            .and_then(|selection| match selection {
                0..=3 => prompt_sdk_version(selection),
                _ => None,
            })
    }
}

impl Prompt for PackageType {
    fn prompt() -> Option<PackageType> {
        let choices = &["Application", "Linux Audio plugin", "GIMP Plugin"];

        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select the type of package:")
            .default(0)
            .items(choices)
            .interact()
            .ok()
            .and_then(|selection| match selection {
                0 => Some(PackageType::Application),
                1 => Some(PackageType::LinuxAudioPlugin),
                2 => Some(PackageType::GimpPlugin),
                _ => None,
            })
    }
}
