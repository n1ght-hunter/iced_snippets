use iced::{
    executor,
    widget::{self, button, row, text},
    Alignment, Application, Color, Command, Element, Length,
};

mod event_stopper;

fn main() {
    App::run(Default::default()).unwrap();
}

struct App {
    value: i32,
    events_enabled: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    ToggleEvents,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            App {
                value: 0,
                events_enabled: true,
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
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::ToggleEvents => {
                self.events_enabled = !self.events_enabled;
            }
        };
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let all_children = widget::column![
            button("Increment").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("Decrement").on_press(Message::DecrementPressed)
        ]
        .padding(20)
        .align_items(Alignment::Center);

        row![
            event_stopper::Container::new(all_children, self.events_enabled),
            button("Toggle events").on_press(Message::ToggleEvents)
        ]
        .into()
    }
}
