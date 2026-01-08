// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

mod util;

enum State {
    Started,
    Downloading,
    Installing,
    Installed,
    Errored(String),
}

fn main() {
    println!("Hello, world!");
}
