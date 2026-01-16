// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use winresource::WindowsResource;

fn main() {
    let mut res = WindowsResource::new();
    res.set_icon("assets/download.ico");
    res.compile().unwrap();
}
