//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

use {
    crate::{Message, Tab},
    iced::{
        widget::{Button, Column, Container, Row, Text},
        Alignment, Element,
    },
    iced_aw::tab_bar::TabLabel,
};

#[derive(Debug, Clone)]
pub enum DebuggerMessage {
    Increase,
    Decrease,
}

pub struct DebuggerTab {
    value: i32,
}

impl DebuggerTab {
    pub fn new() -> Self {
        DebuggerTab { value: 0 }
    }

    pub fn update(&mut self, message: DebuggerMessage) {
        match message {
            DebuggerMessage::Increase => self.value += 1,
            DebuggerMessage::Decrease => self.value -= 1,
        }
    }
}

impl Tab for DebuggerTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("debugger")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, DebuggerMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(Text::new(format!("Count: {}", self.value)).size(32))
                .push(
                    Row::new()
                        .spacing(10)
                        .push(
                            Button::new(Text::new("Decrease")).on_press(DebuggerMessage::Decrease),
                        )
                        .push(
                            Button::new(Text::new("Increase")).on_press(DebuggerMessage::Increase),
                        ),
                ),
        )
        .into();

        content.map(Message::Debugger)
    }
}
