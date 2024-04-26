use iced::{
    futures::{SinkExt, StreamExt},
    widget::{self, container},
    Application, Element, Settings,
};
use reqwest_websocket::RequestBuilderExt;
use serde::{Deserialize, Serialize};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Debug, Deserialize, Serialize)]
struct Reply {
    ip: String,
}

struct App {
    client: reqwest::Client,
    reply: Option<Reply>,
    channel: Option<iced::futures::channel::mpsc::Sender<String>>,
    messages: Vec<String>,
    text_input: String,
}

#[derive(Debug)]
enum Message {
    NewMessage(String),
    ButtonMessage(ButtonMessage),
    IpFetched(Result<Reply, reqwest::Error>),
    NewChannel(iced::futures::channel::mpsc::Sender<String>),
}

#[derive(Debug, Clone)]
enum ButtonMessage {
    FetchIp,
    TextInput(String),
    Submit,
}

impl Application for App {
    type Executor = iced::executor::Default;

    type Theme = iced::theme::Theme;

    type Flags = ();

    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            App {
                client: reqwest::Client::new(),
                reply: None,
                channel: None,
                messages: Vec::new(),
                text_input: String::new(),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let client = self.client.clone();
        iced::subscription::channel("echo ws", 10, |mut sender| async move {
            let (tx, mut rx) = iced::futures::channel::mpsc::channel(10);
            sender.send(Message::NewChannel(tx)).await.unwrap();

            let res = client
                .get("wss://echo.websocket.org")
                .upgrade()
                .send()
                .await
                .unwrap();

            let (mut websocket, mut web_recv) = res.into_websocket().await.unwrap().split();

            tokio::spawn(async move {
                while let Some(message) = web_recv.next().await {
                    if let Ok(message) = message {
                        if let reqwest_websocket::Message::Text(text) = message {
                            sender.send(Message::NewMessage(text)).await.unwrap();
                        }
                    }
                }
            });

            loop {
                let message = rx.next().await;
                if let Some(message) = message {
                    websocket
                        .send(reqwest_websocket::Message::Text(message))
                        .await
                        .unwrap();
                }
            }
        })
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::ButtonMessage(msm) => match msm {
                ButtonMessage::FetchIp => {
                    let client = self.client.clone();
                    return iced::Command::perform(
                        async move {
                            let response = client
                                .get("https://api.ipify.org/?format=json")
                                .send()
                                .await?;
                            let reply = response.json::<Reply>().await?;
                            Ok(reply)
                        },
                        Message::IpFetched,
                    );
                }
                ButtonMessage::TextInput(message) => {
                    self.text_input = message;
                }
                ButtonMessage::Submit => {
                    if let Some(channel) = &mut self.channel {
                        channel
                            .try_send(self.text_input.clone())
                            .expect("Failed to send message");
                        self.text_input.clear();
                    }
                }
            },
            Message::IpFetched(res) => {
                if let Ok(reply) = res {
                    self.reply = Some(reply);
                } else {
                    eprintln!("Failed to fetch IP: {:?}", res.unwrap_err());
                }
            }
            Message::NewChannel(sender) => {
                self.channel = Some(sender);
            }
            Message::NewMessage(new_message) => {
                self.messages.push(new_message);
            }
        };

        iced::Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let ip = Element::from(widget::row![
            widget::button("Fetch IP").on_press(ButtonMessage::FetchIp),
            match &self.reply {
                Some(reply) => widget::text(reply.ip.clone()),
                None => widget::text("No IP fetched"),
            },
        ])
        .map(Message::ButtonMessage);

        let text_input = Element::from(
            widget::text_input("text to send", &self.text_input)
                .on_input(ButtonMessage::TextInput)
                .on_submit(ButtonMessage::Submit),
        )
        .map(Message::ButtonMessage);

        let messages = widget::column![text_input];

        let messages = self
            .messages
            .iter()
            .fold(messages, |acc, message| acc.push(widget::text(message)));

        container(widget::column![ip, messages])
            .center_x()
            .center_y()
            .into()
    }
}
