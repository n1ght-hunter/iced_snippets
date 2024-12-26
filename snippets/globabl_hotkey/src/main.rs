use iced::widget::Container;
use iced::widget::{button, column, text};
use iced::Task;
use iced::{Alignment, Element, Length, Renderer, Subscription, Theme};

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyEventReceiver, GlobalHotKeyManager,
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

    iced::application("Prova hotkeys", TestApp::update, TestApp::view)
        .subscription(TestApp::subscription)
        .run_with(move || {
            TestApp::new(Flags {
                receiver: global_hotkey_channel.clone(),
                hotkey_screenshoot: hotkey.id(),
                hotkey_delayed_screenshoot: hotkey2.id(),
            })
        })
        .unwrap();
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

impl TestApp {
    fn new(flags: Flags) -> (Self, Task<Message>) {
        let my_app = TestApp {
            receiver: flags.receiver,
            hotkey_screenshoot: flags.hotkey_screenshoot,
            hotkey_delayed_screenshoot: flags.hotkey_delayed_screenshoot,
            text: String::from(""),
            should_listen: false,
        };

        (my_app, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EventOccurred(event) => {
                match event {
                    Event::Listened(key) => match key {
                        Key::Screenshoot => self.text = String::from("Screenshoot Hotkey"),
                        Key::DelayedScreenshoot => {
                            self.text = String::from("Delayed Screenshoot Hotkey")
                        }
                    },
                    _ => {}
                };
            }
            Message::ToggleShouldListen => self.should_listen = !self.should_listen,
        };
        // Command::batch(
        //     [
        //         window::maximize(true),
        //         window::gain_focus(),
        //     ]
        // )
        Task::none()
    }

    fn view(&self) -> Element<'_, Message, Theme, crate::Renderer> {
        let mut content = column![button(if self.should_listen {
            "Smetti di sentire"
        } else {
            "Ascolta global hotkeys"
        })
        .on_press(Message::ToggleShouldListen)]
        .align_x(Alignment::Center)
        .spacing(10.0);

        if self.should_listen {
            content = content.push(text(format!("{}", self.text)));
        }

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        hotkeys_management::subscribe(
            self.receiver.clone(),
            self.hotkey_screenshoot,
            self.hotkey_delayed_screenshoot,
        )
        .map(Message::EventOccurred)
        // if self.should_listen {
        // }
        // else {
        //     Subscription::none()
        // }
    }
}
