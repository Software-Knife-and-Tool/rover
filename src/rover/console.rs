//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

use {
    crate::{tabs::rover::RoverMessage, ROVER},
    chrono::Utc,
    iced::widget::{scrollable, Container, Scrollable, Text},
};

use {futures::executor::block_on, futures_locks::RwLock};

#[derive(Clone, Debug)]
pub struct Console {
    messages: RwLock<Vec<String>>,
}

impl Console {
    pub fn new() -> Self {
        Console {
            messages: RwLock::new(vec![]),
        }
    }

    #[allow(dead_code)]
    pub fn log(&self, msg: String) {
        let mut messages_ref = block_on(self.messages.write());
        let timestamp = Utc::now().timestamp();

        messages_ref.push(format!("{}:    {}", timestamp, msg))
    }

    pub fn _clear(&self) {
        let mut messages_ref = block_on(self.messages.write());

        messages_ref.clear()
    }

    pub fn view() -> Scrollable<'static, RoverMessage> {
        let messages_ref = block_on(ROVER.console.messages.read());

        let buffer = &messages_ref
            .iter()
            .map(|line| (*line).clone() + "\n")
            .collect::<String>();

        let console: Container<'static, RoverMessage> =
            Container::new(Text::new(buffer.clone()).width(800).height(800));

        scrollable(console)
    }
}
