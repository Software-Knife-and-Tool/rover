//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#[allow(unused_imports)]
use {
    crate::{Message, Tab},
    iced::widget::{
        button, checkbox, column, container, horizontal_space, image, row, rule, scrollable, text,
        text_input, toggler, vertical_space, Button, Column, Container, Row, Rule, Text,
    },
    iced::{Element, Length, Renderer, Theme},
    iced_aw::{
        menu::{self, Item, Menu, StyleSheet},
        split,
        style::MenuBarStyle,
        Split, TabLabel,
    },
    std::fmt::Display,
};

#[derive(Debug, Clone)]
pub enum ComposerMessage {
    OnResize(u16),
    EvalPressed,
    ClearPressed,
}

pub struct ComposerTab {
    divider_position: Option<u16>,
}

impl ComposerTab {
    pub fn new() -> Self {
        ComposerTab {
            divider_position: None,
        }
    }

    fn padded_button<Message: Clone>(label: &str) -> Button<'_, Message> {
        button(text(label)).padding([12, 24])
    }

    pub fn update(&mut self, message: ComposerMessage) {
        match message {
            ComposerMessage::OnResize(position) => self.divider_position = Some(position),
            ComposerMessage::EvalPressed => (),
            ComposerMessage::ClearPressed => (),
        }
    }
}

impl Tab for ComposerTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("composer")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let controls = row![]
            .push(Self::padded_button("eval").on_press(ComposerMessage::EvalPressed))
            .push(horizontal_space())
            .push(Self::padded_button("clear").on_press(ComposerMessage::ClearPressed));

        let compose = Container::new(Text::new("compose"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        let eval = Container::new(Text::new("eval"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        let content: Element<'_, ComposerMessage> = Container::new(
            Column::new()
                .push(controls)
                .push(Text::new(""))
                .push(Rule::horizontal(2))
                .push(Split::new(
                    compose,
                    eval,
                    self.divider_position,
                    split::Axis::Horizontal,
                    ComposerMessage::OnResize,
                )),
        )
        .into();

        content.map(Message::Composer)
    }
}
