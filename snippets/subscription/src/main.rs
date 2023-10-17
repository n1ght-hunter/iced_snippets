mod component;
use iced::time;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::ops::RangeInclusive;
use std::time::Duration;

use component::numeric_input;

use iced::theme::Button;
use iced::widget::{button, column, container, progress_bar, row, text, text_input};
use iced::{
    executor, subscription, Alignment, Application, Command, Element, Length, Padding, Settings,
    Subscription, Theme,
};
pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    lines: Option<usize>,
    notes: String,
    progress: f32,
    start: bool,
}

#[derive(Debug, Clone)]
enum Message {
    CreateFile,
    DeleteText,
    OnInputChange(Option<usize>),
    OnNoteChange(String),
    MySubscription(Option<FileEvent>),
}

#[derive(Debug)]
enum FileState {
    Opening(String, usize),
    Writing(File, RangeInclusive<usize>, usize),
    Error,
}

#[derive(Debug, Clone, Copy)]
enum FileEvent {
    Opened,
    Read,
    Error,
    UpdatingProgressBar(i32),
}

impl Application for Counter {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Counter, Command<Message>) {
        (
            Self {
                lines: None,
                notes: "".into(),
                progress: 0.0,
                start: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::CreateFile => {
                self.start = true;
            }
            Message::DeleteText => {
                self.start = false;
                self.lines = None;
                self.progress = 0.0;
            }
            Message::OnInputChange(message) => self.lines = message,
            Message::OnNoteChange(message) => self.notes = message,
            Message::MySubscription(event) => {
                if let Some(event) = event {
                    match event {
                        FileEvent::Opened => {
                            self.progress = 0.0;
                        }
                        FileEvent::UpdatingProgressBar(val) => {
                            self.progress = val as f32;
                        }
                        FileEvent::Error => self.start = false,
                        _ => {}
                    }
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let padding: Padding = 10.into();
        let width: Length = 160.into();
        let height: Length = 40.into();
        container(
            column![
                progress_bar(0.0..=self.progress, self.progress).height(height),
                row![
                    button(
                        text("Create file")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .vertical_alignment(iced::alignment::Vertical::Center)
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                    )
                    .padding(padding)
                    .width(width)
                    .style(Button::Primary)
                    .on_press(Message::CreateFile),
                    numeric_input(self.lines, Message::OnInputChange, padding, 300.into()),
                    button(
                        text("Cancel")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .vertical_alignment(iced::alignment::Vertical::Center)
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                    )
                    .padding(padding)
                    .width(width)
                    .style(Button::Destructive)
                    .on_press(Message::DeleteText)
                ]
                .spacing(20)
                .align_items(Alignment::Center),
                text_input("Write something...", self.notes.as_str(),)
                    .on_input(Message::OnNoteChange)
                    .padding(padding)
            ]
            .spacing(20)
            .max_width(660),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::default()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        if self.start {
            subscription::unfold(
                self.lines,
                FileState::Opening("./out/example.txt".into(), self.lines.unwrap_or(0)),
                run,
            )
            .map(Message::MySubscription)
        } else {
            Subscription::none()
        }
    }
}

async fn run(state: FileState) -> (Option<FileEvent>, FileState) {
    match state {
        FileState::Opening(path, lines) => {
            let file = OpenOptions::new().append(true).create(true).open(path);
            match file {
                Ok(file) => {
                    let reader = 1..=lines;
                    (Some(FileEvent::Opened), FileState::Writing(file, reader, 1))
                }
                Err(_) => (None, FileState::Error),
            }
        }
        FileState::Writing(mut file, range, mut progress) => {
            file.write_all(format!("linea {progress}\n").as_bytes())
                .unwrap();
            progress += 1;
            if !range.contains(&progress) {
                return (None, FileState::Error);
            }
            (
                Some(FileEvent::UpdatingProgressBar(progress as i32)),
                FileState::Writing(file, range, progress),
            )
        }
        FileState::Error => (None, FileState::Error),
    }
}
