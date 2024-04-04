//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

use {
    crate::{Message, Tab},
    iced::{
        alignment::{Horizontal, Vertical},
        widget::{Button, Column, Container, Row, Text, TextInput},
        Alignment, Element, Length,
    },
    iced_aw::tab_bar::TabLabel,
};

#[derive(Debug, Clone)]
pub enum ListenerMessage {
    UsernameChanged(String),
    PasswordChanged(String),
    ClearPressed,
    ListenerPressed,
}

pub struct ListenerTab {
    username: String,
    password: String,
}

impl ListenerTab {
    pub fn new() -> Self {
        ListenerTab {
            username: String::new(),
            password: String::new(),
        }
    }

    pub fn update(&mut self, message: ListenerMessage) {
        match message {
            ListenerMessage::UsernameChanged(value) => self.username = value,
            ListenerMessage::PasswordChanged(value) => self.password = value,
            ListenerMessage::ClearPressed => {
                self.username = String::new();
                self.password = String::new();
            }
            ListenerMessage::ListenerPressed => {}
        }
    }
}

impl Tab for ListenerTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("listener")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, ListenerMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(
                    TextInput::new("Username", &self.username)
                        .on_input(ListenerMessage::UsernameChanged)
                        .padding(10)
                        .size(32),
                )
                .push(
                    TextInput::new("Password", &self.password)
                        .on_input(ListenerMessage::PasswordChanged)
                        .padding(10)
                        .size(32)
                        .secure(true),
                )
                .push(
                    Row::new()
                        .spacing(10)
                        .push(
                            Button::new(
                                Text::new("Clear").horizontal_alignment(Horizontal::Center),
                            )
                            .width(Length::Fill)
                            .on_press(ListenerMessage::ClearPressed),
                        )
                        .push(
                            Button::new(
                                Text::new("Listener").horizontal_alignment(Horizontal::Center),
                            )
                            .width(Length::Fill)
                            .on_press(ListenerMessage::ListenerPressed),
                        ),
                ),
        )
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into();

        content.map(Message::Listener)
    }
}
