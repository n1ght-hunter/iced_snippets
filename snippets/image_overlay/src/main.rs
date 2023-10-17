use iced::{
    executor,
    widget::{self, button, text},
    Application, Color, Command, Element,
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
        Container::new("testing", Some(self.image.clone()))
            .center_x()
            .center_y().padding(10)
            .into()
    }
}
