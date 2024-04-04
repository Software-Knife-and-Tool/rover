//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

#![allow(unused_imports)]
use {
    iced::{
        alignment::{self, Alignment},
        executor,
        font::Style,
        widget::{
            self, button, checkbox, column, container, keyed_column, row, rule, scrollable, text,
            text_input, Column, Container, Image, Rule, Scrollable, Space, Text,
        },
        window, Application, Color, Command, Element, Font, Length, Settings, Theme,
    },
    iced_aw::tab_bar::TabLabel,
    uname::{self, Info},
};

use crate::{Console, Rover, Message, Tab};

#[derive(Debug, Clone)]
pub enum RoverMessage {}

pub struct RoverTab {
    uname: Option<uname::Info>,
}

impl RoverTab {
    pub fn new() -> Self {
        RoverTab {
            uname: match uname::uname() {
                Err(_) => None,
                Ok(info) => Some(info),
            },
        }
    }

    pub fn update(&mut self, message: RoverMessage) {
        match message {}
    }
}

impl Tab for RoverTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("rover")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let console: Scrollable<'_, RoverMessage> = Console::view();

        let content: Element<'_, RoverMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Start)
                .max_width(600)
                .padding(20)
                .spacing(2)
                .push(
                    Text::new("Mu Rover")
                        .width(Length::Fill)
                        .size(28)
                        .horizontal_alignment(alignment::Horizontal::Center),
                )
                .push(Space::new(600, 20))
                .push(
                    Text::new("System".to_string())
                        .width(Length::Fill)
                        .size(18)
                        .horizontal_alignment(alignment::Horizontal::Left),
                )
                .push(Rule::horizontal(2))
                .push(Space::new(600, 20))
                .push(
                    Text::new(
                        "sysname: ".to_string()
                            + match &self.uname {
                                None => "unavailable",
                                Some(uname) => &uname.sysname,
                            },
                    )
                    .width(Length::Fill)
                    .size(16)
                    .horizontal_alignment(alignment::Horizontal::Left),
                )
                .push(
                    Text::new(
                        "release: ".to_string()
                            + match &self.uname {
                                None => "unavailable",
                                Some(uname) => &uname.release,
                            },
                    )
                    .width(Length::Fill)
                    .size(16)
                    .horizontal_alignment(alignment::Horizontal::Left),
                )
                .push(
                    Text::new(
                        "version: ".to_string()
                            + match &self.uname {
                                None => "unavailable",
                                Some(uname) => &uname.version,
                            },
                    )
                    .width(Length::Fill)
                    .size(16)
                    .horizontal_alignment(alignment::Horizontal::Left),
                )
                .push(
                    Text::new(
                        "machine: ".to_string()
                            + match &self.uname {
                                None => "unavailable",
                                Some(uname) => &uname.machine,
                            },
                    )
                    .width(Length::Fill)
                    .size(16)
                    .horizontal_alignment(alignment::Horizontal::Left),
                )
                .push(Space::new(600, 40))
                .push(
                    Text::new("Console".to_string())
                        .width(Length::Fill)
                        .size(18)
                        .horizontal_alignment(alignment::Horizontal::Left),
                )
                .push(Rule::horizontal(2))
                .push(Space::new(600, 20))
                .push(console),
        )
        .align_x(iced::alignment::Horizontal::Center)
        .into();

        content.map(Message::Rover)
    }
}
