// SPDX-FileCopyrightText: 2020-2023 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

/// Sdk Extension
#[derive(Debug)]
pub enum SdkExtension {
    Rust,
    RustNightly,
    Java(String),
    Node(String),
    Php(String),
    GoLang,
    TexLive,
    Llvm(String),
    DotNet(String),
    Mono(String),
    Gcc(String),
}

impl ToString for SdkExtension {
    fn to_string(&self) -> String {
        match self {
            Self::Rust => "org.freedesktop.Sdk.Extension.rust-stable".to_owned(),
            Self::RustNightly => "org.freedesktop.Sdk.Extension.rust-nightly".to_owned(),
            Self::Java(s) => format!("org.freedesktop.Sdk.Extension.openjdk{s}"),
            Self::Node(s) => format!("org.freedesktop.Sdk.Extension.node{s}"),
            Self::Php(s) => format!("org.freedesktop.Sdk.Extension.php{s}"),
            Self::GoLang => "org.freedesktop.Sdk.Extension.golang".to_owned(),
            Self::TexLive => "org.freedesktop.Sdk.Extension.texlive".to_owned(),
            Self::Llvm(s) => format!("org.freedesktop.Sdk.Extension.llvm{s}"),
            Self::DotNet(s) => format!("org.freedesktop.Sdk.Extension.dotnet{s}"),
            Self::Mono(s) => format!("org.freedesktop.Sdk.Extension.mono{s}"),
            Self::Gcc(s) => format!("org.freedesktop.Sdk.Extension.gcc{s}"),
        }
    }
}
