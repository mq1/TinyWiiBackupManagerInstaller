// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{Theme, border, widget::button};

pub fn rounded_button(theme: &Theme, status: button::Status) -> button::Style {
    let mut style = button::primary(theme, status);
    style.border.radius = border::radius(30);
    style
}

pub fn rounded_secondary_button(theme: &Theme, status: button::Status) -> button::Style {
    let mut style = button::background(theme, status);
    style.border.width = 1.0;
    style.border.color = theme.extended_palette().background.strong.color;
    style.border.radius = border::radius(30);
    style
}
