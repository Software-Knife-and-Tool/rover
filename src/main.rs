//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[macro_use]
extern crate lazy_static;

mod rover;
mod tabs;

use {
    crate::{
        rover::{
            console::Console,
            // mu::Mu,
        },
        tabs::{
            composer::{ComposerMessage, ComposerTab},
            debugger::{DebuggerMessage, DebuggerTab},
            rover::{RoverMessage, RoverTab},
            inspector::{InspectorMessage, InspectorTab},
            listener::{ListenerMessage, ListenerTab},
        },
    },
    hostname::{self},
    iced::{
        alignment::{self, Horizontal, Vertical},
        widget::{container, text, Column, Container},
        Application, Command, Element, Length, Settings, Theme,
    },
    iced_aw::{TabLabel, Tabs},
};

lazy_static! {
    static ref ROVER: Rover = Rover {
        console: Console::new(),
        // mu: Mu::new(),
    };
}

const TAB_PADDING: u16 = 16;

// rover.log("main: UI initialized");

#[derive(Clone, Debug)]
struct Rover {
    console: Console,
    // mu: Mu,
}

#[allow(clippy::large_enum_variant)]
enum RoverState {
    Loading,
    Loaded(State),
}

struct State {
    active_tab: TabId,
    rover_tab: RoverTab,
    listener_tab: ListenerTab,
    composer_tab: ComposerTab,
    inspector_tab: InspectorTab,
    debugger_tab: DebuggerTab,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum TabId {
    Rover,
    Listener,
    Composer,
    Inspector,
    Debugger,
}

#[derive(Clone, Debug)]
enum Message {
    TabSelected(TabId),
    Rover(RoverMessage),
    Listener(ListenerMessage),
    Composer(ComposerMessage),
    Inspector(InspectorMessage),
    Debugger(DebuggerMessage),
    #[allow(dead_code)]
    Loaded(Result<(), String>),
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Rover {
    #[allow(dead_code)]
    pub fn log(&self, msg: &str) {
        ROVER.console.log(msg.to_string())
    }
}

impl Application for RoverState {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (RoverState, Command<Message>) {
        (
            RoverState::Loading,
            Command::batch(vec![Command::perform(load(), Message::Loaded)]),
        )
    }

    fn title(&self) -> String {
        format!(
            "mu-rover v0.0.1 running on {}",
            hostname::get().unwrap().to_str().unwrap()
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match self {
            RoverState::Loading => {
                if let Message::Loaded(_) = message {
                    *self = RoverState::Loaded(State {
                        active_tab: TabId::Rover,
                        rover_tab: RoverTab::new(),
                        listener_tab: ListenerTab::new(),
                        composer_tab: ComposerTab::new(),
                        inspector_tab: InspectorTab::new(),
                        debugger_tab: DebuggerTab::new(),
                    })
                }
            }
            RoverState::Loaded(state) => match message {
                Message::TabSelected(selected) => state.active_tab = selected,
                Message::Rover(message) => state.rover_tab.update(message),
                Message::Listener(message) => state.listener_tab.update(message),
                Message::Composer(message) => state.composer_tab.update(message),
                Message::Inspector(message) => state.inspector_tab.update(message),
                Message::Debugger(message) => state.debugger_tab.update(message),
                _ => {}
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self {
            RoverState::Loading => container(
                text("Loading...")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(50),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into(),
            RoverState::Loaded(state) => Tabs::new(Message::TabSelected)
                .push(
                    TabId::Rover,
                    state.rover_tab.tab_label(),
                    state.rover_tab.view(),
                )
                .push(
                    TabId::Listener,
                    state.listener_tab.tab_label(),
                    state.listener_tab.view(),
                )
                .push(
                    TabId::Composer,
                    state.composer_tab.tab_label(),
                    state.composer_tab.view(),
                )
                .push(
                    TabId::Inspector,
                    state.inspector_tab.tab_label(),
                    state.inspector_tab.view(),
                )
                .push(
                    TabId::Debugger,
                    state.debugger_tab.tab_label(),
                    state.debugger_tab.view(),
                )
                .set_active_tab(&state.active_tab)
                .tab_bar_position(iced_aw::TabBarPosition::Top)
                .into(),
        }
    }
}

trait Tab {
    type Message;

    fn title(&self) -> String;
    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(self.content())
            .align_items(iced::Alignment::Center);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Top)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}

fn main() -> iced::Result {
    RoverState::run(Settings::default())
}
