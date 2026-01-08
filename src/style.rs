// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    Background, Theme, border,
    widget::{button, container},
};

pub fn rounded_button(theme: &Theme, status: button::Status) -> button::Style {
    let mut style = button::primary(theme, status);
    style.border.radius = border::radius(30);
    style
}

pub fn rounded_secondary_button(theme: &Theme, status: button::Status) -> button::Style {
    let mut style = button::secondary(theme, status);
    style.border.radius = border::radius(30);
    style
}

pub fn card(theme: &Theme) -> container::Style {
    let bg = theme.palette().background;

    let mut style = container::bordered_box(theme);
    style.background = Some(Background::Color(bg));
    style.border.radius = border::radius(10);
    style
}
