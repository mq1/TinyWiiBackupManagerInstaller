// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

mod style;
mod util;

use crate::util::{Arch, Os};
use iced::{
    Alignment, Element, Length, Size, Task,
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
    Install(String, Os, Arch),
}

impl State {
    fn new() -> (Self, Task<Message>) {
        let initial_state = State::FetchingLatestVersion;
        let task = Task::perform(util::get_latest_version(), Message::GotLatestVersion);

        (initial_state, task)
    }

    fn view(&self) -> Element<'_, Message> {
        let content: Element<'_, Message> = match self {
            State::FetchingLatestVersion => text("Fetching latest version...").into(),
            State::GotLatestVersion(version) => {
                let os = util::get_os();
                let arch = util::get_arch();

                column![
                    text(format!("Latest version: v{}", version)),
                    text(format!("Detected OS: {}", os.as_display_str())),
                    text(format!("Detected Arch: {}", arch.as_display_str())),
                    space(),
                    space(),
                    space(),
                    space(),
                    button("Install")
                        .style(style::rounded_button)
                        .on_press(Message::Install(version.clone(), os, arch))
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
            State::Errored(msg) => text(msg).into(),
        };

        container(content).center(Length::Fill).into()
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
            Message::Install(version, os, arch) => {
                // TODO
                Task::none()
            }
        }
    }
}

fn main() -> iced::Result {
    iced::application(State::new, State::update, State::view)
        .window_size(Size::new(400.0, 300.0))
        .resizable(false)
        .title("Install TinyWiiBackupManager")
        .run()
}
