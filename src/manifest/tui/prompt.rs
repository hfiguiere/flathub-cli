// SPDX-FileCopyrightText: 2020-2023 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub trait Prompt
where
    Self: Sized,
{
    fn prompt() -> Option<Self>;
}
