//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

use {
    crate::{Message, Tab},
    iced::{
        widget::{Column, Container, Image, Slider, Text},
        Alignment, Element, Length,
    },
    iced_aw::tab_bar::TabLabel,
};

#[derive(Debug, Clone)]
pub enum ComposerMessage {
    ImageWidthChanged(f32),
}

pub struct ComposerTab {
    composer_width: f32,
}

impl ComposerTab {
    pub fn new() -> Self {
        ComposerTab {
            composer_width: 100.0,
        }
    }

    pub fn update(&mut self, message: ComposerMessage) {
        match message {
            ComposerMessage::ImageWidthChanged(value) => self.composer_width = value,
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
        let content: Element<'_, ComposerMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(composer(self.composer_width))
                .push(Text::new(if self.composer_width == 500.0 {
                    "Earth is saved!"
                } else {
                    "-= enlarge =-"
                }))
                .push(Slider::new(
                    100.0..=500.0,
                    self.composer_width,
                    ComposerMessage::ImageWidthChanged,
                )),
        )
        .align_x(iced::alignment::Horizontal::Center)
        .into();

        content.map(Message::Composer)
    }
}

fn composer<'a>(width: f32) -> Container<'a, ComposerMessage> {
    Container::new(if cfg!(target_carch = "wasm32") {
        Image::new("images/composer.png")
    } else {
        Image::new(format!("{}/images/flash.png", env!("CARGO_MANIFEST_DIR")))
            .width(Length::Fixed(width))
    })
    .width(Length::Fill)
    .center_x()
}
