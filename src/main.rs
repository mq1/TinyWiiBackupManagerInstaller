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
    widget::{button, column, container, row, space, text},
};
use native_dialog::DialogBuilder;
use std::path::PathBuf;

enum State {
    FetchingLatestVersion,
    GotLatestVersion(String),
    Downloading(String),
    Installing(String),
    Installed(String),
    InstalledPortable(String, PathBuf),
    Errored(String),
}

#[derive(Clone, Debug)]
enum Message {
    GotLatestVersion(Result<String, String>),
    Download(String, Os, Arch),
    Downloaded(Result<(String, Vec<u8>), String>),
    Installed(Result<String, String>),
    DownloadPortable(String, Os, Arch),
    DownloadedPortable(Result<(String, PathBuf), String>),
    LaunchTwbm,
    LaunchTwbmPortable(PathBuf),
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
                let is_installed = util::is_installed().unwrap_or(false);
                let install_str = match is_installed {
                    true => "Update/Reinstall",
                    false => "Download and Install",
                };

                column![
                    text(format!("Latest version: v{}", version)),
                    text(format!("Detected OS: {}", os.as_display_str())),
                    text(format!("Detected arch: {}", arch.as_display_str())),
                    space(),
                    space(),
                    space(),
                    space(),
                    row![
                        button(install_str)
                            .style(style::rounded_button)
                            .on_press(Message::Download(version.clone(), os, arch)),
                        button("Download Portable")
                            .style(style::rounded_secondary_button)
                            .on_press(Message::DownloadPortable(version.clone(), os, arch)),
                    ]
                    .spacing(10)
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
            State::InstalledPortable(version, path) => column![
                text(format!("TinyWiiBackupManager v{} installed", version)),
                button("→ Launch TinyWiiBackupManager")
                    .style(style::rounded_button)
                    .on_press(Message::LaunchTwbmPortable(path.clone()))
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
            Message::DownloadPortable(version, os, arch) => {
                let dest_dir = DialogBuilder::file()
                    .set_title("Select destination directory")
                    .open_single_dir()
                    .show()
                    .unwrap_or_default();

                if let Some(dest_dir) = dest_dir {
                    *self = State::Downloading(version.clone());
                    Task::perform(
                        util::download_to_dir(version, os, arch, dest_dir)
                            .map_err(|e| e.to_string()),
                        Message::DownloadedPortable,
                    )
                } else {
                    Task::none()
                }
            }
            Message::DownloadedPortable(res) => {
                match res {
                    Ok((version, path)) => {
                        *self = State::InstalledPortable(version.clone(), path);
                    }
                    Err(e) => {
                        *self = State::Errored(e);
                    }
                }
                Task::none()
            }
            Message::LaunchTwbm => match util::launch_twbm() {
                Ok(()) => iced::exit(),
                Err(e) => {
                    *self = State::Errored(e.to_string());
                    Task::none()
                }
            },
            Message::LaunchTwbmPortable(path) => match util::launch_twbm_portable(path) {
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
