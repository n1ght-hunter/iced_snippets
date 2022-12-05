use external_view_function::show_counter;
use iced::{
    executor,
    widget::{button, container, text, self},
    Application, Color, Command,
};

mod external_view_function;
mod theme;

fn main() {
    App::run(Default::default()).unwrap();
}

#[derive(Clone, Debug)]
pub enum Message {
    ChangeStyle,
    IncrementPressed,
    DecrementPressed,
}

pub type Element<'a> = iced::Element<'a, Message, iced::Renderer<theme::Theme>>;

struct App {
    change_style: bool,
    value: i32,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = theme::Theme;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            App {
                change_style: false,
                value: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Custom theme")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::ChangeStyle => {
                self.change_style = !self.change_style;
            }
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        };
        Command::none()
    }

    fn view(&self) -> Element {
        let text = button(text("custome theme").style(if self.change_style {
            theme::Text::Color(Color::BLACK)
        } else {
            theme::Text::Color(Color::from_rgb(1.0, 0.0, 0.0))
        }))
        .on_press(Message::ChangeStyle);
        let content = widget::column![text, show_counter(self.value)];
        container(content).into()
    }
}
