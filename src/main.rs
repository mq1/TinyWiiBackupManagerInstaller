// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod style;
mod util;

use crate::util::{Arch, Os};
use iced::{
    Alignment, Element, Length, Size, Task,
    futures::TryFutureExt,
    widget::{button, column, container, space, text},
};

enum State {
    FetchingLatestVersion,
    GotLatestVersion(String),
    Downloading(String),
    Installing(String),
    Installed(String),
    Errored(String),
}

#[derive(Clone, Debug)]
enum Message {
    GotLatestVersion(Result<String, String>),
    Download(String, Os, Arch),
    Downloaded(Result<(String, Vec<u8>), String>),
    Installed(Result<String, String>),
    LaunchTwbm,
}

impl State {
    fn new() -> (Self, Task<Message>) {
        let task = Task::perform(
            util::get_latest_version().map_err(|e| e.to_string()),
            Message::GotLatestVersion,
        );

        (State::FetchingLatestVersion, task)
    }

    fn view(&self) -> Element<'_, Message> {
        let content: Element<'_, Message> = match self {
            State::FetchingLatestVersion => text("Fetching latest version...").into(),
            State::GotLatestVersion(version) => {
                let os = util::get_os().unwrap_or_default();
                let arch = util::get_arch();
                let is_installed = util::is_installed();

                column![
                    text(format!("Latest version: v{}", version)),
                    text(format!("Detected OS: {}", os.as_display_str())),
                    text(format!("Detected arch: {}", arch.as_display_str())),
                    space(),
                    space(),
                    space(),
                    space(),
                    button(if is_installed {
                        "Update/Reinstall"
                    } else {
                        "Download and Install"
                    })
                    .style(style::rounded_button)
                    .on_press(Message::Download(version.clone(), os, arch)),
                ]
                .spacing(5)
                .align_x(Alignment::Center)
                .into()
            }
            State::Downloading(version) => text(format!("Downloading v{}", version)).into(),
            State::Installing(version) => text(format!("Installing v{}", version)).into(),
            State::Installed(version) => column![
                text(format!("TinyWiiBackupManager v{} installed", version)),
                button("→ Launch TinyWiiBackupManager")
                    .style(style::rounded_button)
                    .on_press(Message::LaunchTwbm)
            ]
            .spacing(10)
            .align_x(Alignment::Center)
            .into(),
            State::Errored(msg) => text(format!("Error: {}", msg)).into(),
        };

        container(content).center(Length::Fill).padding(10).into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GotLatestVersion(res) => {
                match res {
                    Ok(version) => *self = State::GotLatestVersion(version),
                    Err(e) => *self = State::Errored(e),
                }

                Task::none()
            }
            Message::Download(version, os, arch) => {
                *self = State::Downloading(version.clone());
                Task::perform(
                    util::download(version, os, arch).map_err(|e| e.to_string()),
                    Message::Downloaded,
                )
            }
            Message::Downloaded(res) => match res {
                Ok((version, bytes)) => {
                    *self = State::Installing(version.clone());
                    Task::perform(
                        util::install(version, bytes).map_err(|e| e.to_string()),
                        Message::Installed,
                    )
                }
                Err(e) => {
                    *self = State::Errored(e);
                    Task::none()
                }
            },
            Message::Installed(res) => match res {
                Ok(version) => {
                    *self = State::Installed(version);
                    Task::none()
                }
                Err(e) => {
                    *self = State::Errored(e);
                    Task::none()
                }
            },
            Message::LaunchTwbm => match util::launch_twbm() {
                Ok(()) => iced::exit(),
                Err(e) => {
                    *self = State::Errored(e.to_string());
                    Task::none()
                }
            },
        }
    }
}

fn main() -> iced::Result {
    iced::application(State::new, State::update, State::view)
        .window_size(Size::new(500.0, 300.0))
        .resizable(false)
        .title("Install TinyWiiBackupManager")
        .run()
}
