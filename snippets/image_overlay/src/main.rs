use iced::{
    executor,
    widget::{self, button, text},
    Application, Color, Command, Element, Length,
};

mod image_container;

use image_container::Container;

fn main() {
    App::run(Default::default()).unwrap();
}

#[derive(Clone, Debug)]
pub enum Message {}

struct App {
    image: iced::widget::image::Handle,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            App {
                image: iced::widget::image::Handle::from_memory(
                    include_bytes!("../../../assets/ferris.png").to_vec(),
                ),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Custom theme")
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Dark
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {};
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        widget::container(
            Container::new(widget::text("Any Content").size(44), Some(self.image.clone()))
                .center_x()
                .center_y()
                .height(Length::Fill)
                .width(Length::Fill)
                .style(iced::theme::Container::Custom(Box::new(
                    CustomContainerStyle::default(),
                )))
                .padding(10),
        )
        .padding(50)
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

#[derive(Default)]
struct CustomContainerStyle;

impl iced::widget::container::StyleSheet for CustomContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> widget::container::Appearance {
        widget::container::Appearance {
            background: Some(Color::WHITE.into()),
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: Some(Color::from_rgb(0.0, 0.0, 1.0)),
        }
    }
}
