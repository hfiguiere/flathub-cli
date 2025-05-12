// SPDX-FileCopyrightText: 2020-2025 Hubert Figuière
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

impl std::fmt::Display for SdkExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rust => write!(f, "org.freedesktop.Sdk.Extension.rust-stable"),
            Self::RustNightly => write!(f, "org.freedesktop.Sdk.Extension.rust-nightly"),
            Self::Java(s) => write!(f, "org.freedesktop.Sdk.Extension.openjdk{s}"),
            Self::Node(s) => write!(f, "org.freedesktop.Sdk.Extension.node{s}"),
            Self::Php(s) => write!(f, "org.freedesktop.Sdk.Extension.php{s}"),
            Self::GoLang => write!(f, "org.freedesktop.Sdk.Extension.golang"),
            Self::TexLive => write!(f, "org.freedesktop.Sdk.Extension.texlive"),
            Self::Llvm(s) => write!(f, "org.freedesktop.Sdk.Extension.llvm{s}"),
            Self::DotNet(s) => write!(f, "org.freedesktop.Sdk.Extension.dotnet{s}"),
            Self::Mono(s) => write!(f, "org.freedesktop.Sdk.Extension.mono{s}"),
            Self::Gcc(s) => write!(f, "org.freedesktop.Sdk.Extension.gcc{s}"),
        }
    }
}
