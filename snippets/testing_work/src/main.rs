use iced::theme::{self, Theme};
use iced::widget::{column, container, radio, row, text, Row};
use iced::{Application, Color, Command, Length, Sandbox, Settings, executor};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}


#[derive(Clone, Debug)]
enum Message {}

struct App {}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (App {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Testing")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {};
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        text("testing").into()
    }
}
