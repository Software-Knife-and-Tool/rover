//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

use {
    crate::{Message, Tab},
    iced::{
        widget::{Container, Text},
        Element, Length,
    },
    iced_aw::{split, tab_bar::TabLabel, Split},
};

#[derive(Debug, Clone)]
pub enum ComposerMessage {
    OnHorResize(u16),
}

pub struct ComposerTab {
    hor_divider_position: Option<u16>,
}

impl ComposerTab {
    pub fn new() -> Self {
        ComposerTab {
            hor_divider_position: None,
        }
    }

    pub fn update(&mut self, message: ComposerMessage) {
        match message {
            ComposerMessage::OnHorResize(position) => self.hor_divider_position = Some(position),
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
        let top = Container::new(Text::new("Top"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        let bottom = Container::new(Text::new("Bottom"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        let content: Element<'_, ComposerMessage> = Container::new(Split::new(
            top,
            bottom,
            self.hor_divider_position,
            split::Axis::Horizontal,
            ComposerMessage::OnHorResize,
        ))
        .into();

        content.map(Message::Composer)
    }
}
