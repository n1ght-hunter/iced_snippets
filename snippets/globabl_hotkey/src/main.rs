use iced::{executor, window};
use iced::widget::{button, column, text};
use iced::widget::Container;
use iced::{Alignment, Application, Command, Element, Length, Renderer, Settings, Subscription, Theme};

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState, GlobalHotKeyEventReceiver
};

mod hotkeys_management;
use hotkeys_management::{Event, Key};
mod hotkey_manager;


fn main() {
    let manager = GlobalHotKeyManager::new().unwrap();

    let hotkey = HotKey::new(Some(Modifiers::CONTROL), Code::KeyT);
    let hotkey2 = HotKey::new(Some(Modifiers::SHIFT | Modifiers::CONTROL), Code::KeyT);

    manager.register(hotkey).unwrap();
    manager.register(hotkey2).unwrap();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    TestApp::run(
        Settings {
            id: None, 
            flags: Flags {
                receiver: global_hotkey_channel.clone(),
                hotkey_screenshoot: hotkey.id(),
                hotkey_delayed_screenshoot: hotkey2.id()
            },
            window: iced::window::Settings::default(),
            default_font: iced::Font::DEFAULT,
            default_text_size: 16.0,
            antialiasing: false,
            exit_on_close_request: true
        }
    ).unwrap()
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    ToggleShouldListen,
}

struct Flags {
    receiver: GlobalHotKeyEventReceiver,
    hotkey_screenshoot: u32,
    hotkey_delayed_screenshoot: u32,
}

#[derive(Debug)]
struct TestApp {
    receiver: GlobalHotKeyEventReceiver,
    hotkey_screenshoot: u32,
    hotkey_delayed_screenshoot: u32,
    text: String,
    should_listen: bool,
}

impl Application for TestApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let my_app = TestApp {
            receiver: flags.receiver,
            hotkey_screenshoot: flags.hotkey_screenshoot,
            hotkey_delayed_screenshoot: flags.hotkey_delayed_screenshoot,
            text: String::from(""),
            should_listen: false,
        };

        (my_app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Prova hotkeys")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::EventOccurred(event) => {
                match event {
                    Event::Listened(key) => {
                        match key {
                            Key::Screenshoot => self.text = String::from("Screenshoot Hotkey"),
                            Key::DelayedScreenshoot => self.text = String::from("Delayed Screenshoot Hotkey"),
                        }
                    },
                    _ => {},
                };
            },
            Message::ToggleShouldListen => self.should_listen = !self.should_listen,
        };
        // Command::batch(
        //     [
        //         window::maximize(true),
        //         window::gain_focus(),
        //     ]
        // )
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let mut content = column![
            button(if self.should_listen {"Smetti di sentire"} else {"Ascolta global hotkeys"}).on_press(Message::ToggleShouldListen)
        ]
        .align_items(Alignment::Center)
        .spacing(10.0);

        if self.should_listen {
            content = content.push(text(format!("{}", self.text)));
        }

        Container::new(
            content
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        hotkeys_management::subscribe(self.receiver.clone(), self.hotkey_screenshoot, self.hotkey_delayed_screenshoot).map(Message::EventOccurred)
        // if self.should_listen {
        // }
        // else {
        //     Subscription::none()
        // }
    }
}