//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

use {
    crate::{Message, Tab},
    iced::{
        widget::{Column, Container, Radio, Text},
        Element,
    },
    iced_aw::{style::TabBarStyles, tab_bar::TabLabel},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabBarPosition {
    #[default]
    Top,
    Bottom,
}

impl TabBarPosition {
    pub const ALL: [TabBarPosition; 2] = [TabBarPosition::Top, TabBarPosition::Bottom];
}

impl From<TabBarPosition> for String {
    fn from(position: TabBarPosition) -> Self {
        String::from(match position {
            TabBarPosition::Top => "Top",
            TabBarPosition::Bottom => "Bottom",
        })
    }
}

//#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TabInspector {
    pub tab_bar_position: Option<TabBarPosition>,
    pub tab_bar_theme: Option<TabBarStyles>,
    pub tab_bar_theme_id: Option<usize>,
}

impl TabInspector {
    pub fn new() -> Self {
        TabInspector {
            tab_bar_position: Some(TabBarPosition::Top),
            tab_bar_theme: Some(TabBarStyles::default()),
            tab_bar_theme_id: Some(0),
        }
    }
}

#[derive(Debug, Clone)]
pub enum InspectorMessage {
    PositionSelected(TabBarPosition),
    ThemeSelected(usize),
}

pub struct InspectorTab {
    settings: TabInspector,
}

impl InspectorTab {
    pub fn new() -> Self {
        InspectorTab {
            settings: TabInspector::new(),
        }
    }

    pub fn settings(&self) -> &TabInspector {
        &self.settings
    }

    pub fn update(&mut self, message: InspectorMessage) {
        match message {
            InspectorMessage::PositionSelected(position) => {
                self.settings.tab_bar_position = Some(position)
            }
            InspectorMessage::ThemeSelected(index) => {
                self.settings.tab_bar_theme_id = Some(index);
                self.settings.tab_bar_theme = Some(match index {
                    0 => TabBarStyles::Default,
                    1 => TabBarStyles::Dark,
                    2 => TabBarStyles::Red,
                    3 => TabBarStyles::Blue,
                    4 => TabBarStyles::Green,
                    5 => TabBarStyles::Purple,
                    _ => TabBarStyles::Default,
                })
            }
        }
    }
}

impl Tab for InspectorTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("inspector")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, InspectorMessage> = Container::new(
            Column::new()
                .push(Text::new("TabBar position:").size(20))
                .push(TabBarPosition::ALL.iter().cloned().fold(
                    Column::new().padding(10).spacing(10),
                    |column, position| {
                        column.push(
                            Radio::new(
                                position,
                                position,
                                self.settings().tab_bar_position,
                                InspectorMessage::PositionSelected,
                            )
                            .size(16),
                        )
                    },
                ))
                .push(Text::new("TabBar color:").size(20))
                .push(
                    (0..6).fold(Column::new().padding(10).spacing(10), |column, id| {
                        column.push(
                            Radio::new(
                                predefined_style(id),
                                id,
                                self.settings().tab_bar_theme_id,
                                InspectorMessage::ThemeSelected,
                            )
                            .size(16),
                        )
                    }),
                ),
        )
        .into();

        content.map(Message::Inspector)
    }
}

fn predefined_style(index: usize) -> String {
    match index {
        0 => "Default".to_owned(),
        1 => "Dark".to_owned(),
        2 => "Red".to_owned(),
        3 => "Blue".to_owned(),
        4 => "Green".to_owned(),
        5 => "Purple".to_owned(),
        _ => "Default".to_owned(),
    }
}
