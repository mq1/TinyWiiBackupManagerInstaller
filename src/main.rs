// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod reg;
mod style;
mod util;

use crate::util::{Arch, Os};
use iced::{
    Alignment, Element, Length, Size, Task,
    futures::TryFutureExt,
    widget::{button, column, container, row, space, text},
};
use std::ffi::OsStr;

enum State {
    FetchingLatestVersion,
    CouldNotFetchLatestVersion(String),
    GotLatestVersion(String),
    Downloading(String),
    Installing(String),
    Installed(String),
    Errored(String),
    AskingUninstallConfirmation(bool),
    Uninstalling,
    Uninstalled,
}

#[derive(Clone, Debug)]
enum Message {
    GotLatestVersion(Result<String, String>),
    Download(String, Os, Arch),
    Downloaded(Result<(String, Vec<u8>), String>),
    Installed(Result<String, String>),
    AskUninstallConfirmation,
    Uninstall(bool),
    Uninstalled(Result<(), String>),
}

impl State {
    fn new() -> (Self, Task<Message>) {
        if let Ok(path) = std::env::current_exe()
            && let Some(name) = path.file_name().and_then(OsStr::to_str)
            && name == "uninstall.exe"
        {
            (State::AskingUninstallConfirmation(true), Task::none())
        } else {
            (
                State::FetchingLatestVersion,
                Task::perform(
                    util::get_latest_version().map_err(|e| e.to_string()),
                    Message::GotLatestVersion,
                ),
            )
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content: Element<'_, Message> = match self {
            State::FetchingLatestVersion => text("Fetching latest version...").into(),
            State::CouldNotFetchLatestVersion(error) => {
                let is_installed = util::is_installed();

                let uninstall_button: Element<'_, Message> = match is_installed {
                    true => row![
                        text("TinyWiiBackupManager installation detected"),
                        button("Uninstall")
                            .style(style::rounded_danger_button)
                            .on_press(Message::AskUninstallConfirmation)
                    ]
                    .spacing(5)
                    .into(),
                    false => row![].into(),
                };

                column![
                    space::vertical(),
                    text(format!("Could not fetch latest version: {}", error)),
                    space::vertical(),
                    uninstall_button
                ]
                .align_x(Alignment::Center)
                .into()
            }
            State::GotLatestVersion(version) => {
                let os = util::get_os();
                let arch = util::get_arch();
                let is_installed = util::is_installed();

                let uninstall_button: Element<'_, Message> = match is_installed {
                    true => row![
                        text("TinyWiiBackupManager installation detected"),
                        button("Uninstall")
                            .style(style::rounded_danger_button)
                            .on_press(Message::AskUninstallConfirmation)
                    ]
                    .spacing(5)
                    .align_y(Alignment::Center)
                    .into(),
                    false => row![].into(),
                };

                column![
                    space::vertical(),
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
                    space::vertical(),
                    uninstall_button
                ]
                .spacing(5)
                .align_x(Alignment::Center)
                .into()
            }
            State::Downloading(version) => text(format!("Downloading v{}", version)).into(),
            State::Installing(version) => text(format!("Installing v{}", version)).into(),
            State::Installed(version) => {
                text(format!("TinyWiiBackupManager v{} installed", version)).into()
            }
            State::Errored(msg) => text(format!("Error: {}", msg)).into(),
            State::AskingUninstallConfirmation(is_uninstaller) => column![
                text("Are you sure you want to uninstall TinyWiiBackupManager?"),
                button("Proceed")
                    .style(style::rounded_danger_button)
                    .on_press(Message::Uninstall(*is_uninstaller)),
            ]
            .spacing(10)
            .align_x(Alignment::Center)
            .into(),
            State::Uninstalling => text("Uninstalling TinyWiiBackupManager...").into(),
            State::Uninstalled => text("TinyWiiBackupManager successfully uninstalled").into(),
        };

        container(content).center(Length::Fill).padding(10).into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GotLatestVersion(res) => {
                match res {
                    Ok(version) => *self = State::GotLatestVersion(version),
                    Err(e) => *self = State::CouldNotFetchLatestVersion(e),
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
            Message::AskUninstallConfirmation => {
                *self = State::AskingUninstallConfirmation(false);
                Task::none()
            }
            Message::Uninstall(is_uninstaller) => {
                *self = State::Uninstalling;
                Task::perform(
                    util::uninstall(is_uninstaller).map_err(|e| e.to_string()),
                    Message::Uninstalled,
                )
            }
            Message::Uninstalled(res) => {
                match res {
                    Ok(()) => *self = State::Uninstalled,
                    Err(e) => *self = State::Errored(e),
                }
                Task::none()
            }
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
