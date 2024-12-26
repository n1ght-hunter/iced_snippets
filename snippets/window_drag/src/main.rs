mod button_on_press;

use button_on_press::Button;
use iced::widget::{container, horizontal_space, Column, Container, Space, Text};
use iced::{Color, Element, Length, Task};

pub fn main() -> iced::Result {
    iced::application("Test App", CounterApp::update, CounterApp::view).run()
}

#[derive(Debug, Default)]
struct CounterApp {}

#[derive(Debug, Clone, Copy)]
enum Message {
    Drag,
}

impl CounterApp {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Drag => {
                println!("dragging");
                return iced::window::get_oldest().and_then(iced::window::drag);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = container(
            Button::new(horizontal_space())
                .width(iced::Length::Fill)
                .height(30)
                .on_press(Message::Drag),
        )
        .style(|_| iced::widget::container::Style {
            background: Some(iced::Color::BLACK.into()),
            ..Default::default()
        });

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            // .center_y()
            .into()
    }
}
