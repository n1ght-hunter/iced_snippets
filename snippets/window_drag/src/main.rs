use button_on_press::Button;
use iced::widget::{container, horizontal_space, Column, Container, Space, Text};
use iced::{executor, Application, Color, Command, Element, Length, Settings, Size};
mod button_on_press;

pub fn main() -> iced::Result {
    CounterApp::run(Settings {
        window: iced::window::Settings {
            size: Size::new(800.0, 600.0), // Correct way to specify the size
            decorations: false,            // Example option to remove title bar
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug, Default)]
struct CounterApp {}

#[derive(Debug, Clone, Copy)]
enum Message {
    Drag,
}

impl Application for CounterApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (CounterApp::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Test App")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Drag => {
                println!("dragging");
                return iced::window::drag(iced::window::Id::MAIN);
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let content = container(
            Button::new(horizontal_space())
                .width(iced::Length::Fill)
                .height(30)
                .on_press(Message::Drag),
        )
        .style(iced::widget::container::Appearance {
            background: Some(iced::Background::Color(Color::BLACK)),
            ..Default::default()
        });

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            // .center_y()
            .into()
    }
}
