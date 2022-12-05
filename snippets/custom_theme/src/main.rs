use iced::{
    executor,
    widget::{button, container, text},
    Application, Command,Color,
};

mod theme;

fn main() {
    App::run(Default::default()).unwrap();
}

#[derive(Clone, Debug)]
enum Message {
    ChangeStyle,
}

struct App {
    change_style: bool,
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
        };
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let text = button(text("custome theme").style(if self.change_style {
            theme::Text::Color(Color::BLACK)
        } else {
            theme::Text::Color(Color::from_rgb(1.0, 0.0, 0.0))
        }))
            .style(if self.change_style {
                theme::Button::Cyan
            } else {
                theme::Button::Yellow
            })
            .on_press(Message::ChangeStyle);
        container(text).into()
    }
}
