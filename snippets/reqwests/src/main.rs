use iced::{
    futures::{SinkExt, StreamExt}, widget::{self, container}, Application, Element, Length, Settings
};
use reqwest_websocket::RequestBuilderExt as _;
use serde::{Deserialize, Serialize};

pub fn main() -> iced::Result {
    iced::application("A cool application", update, view)
        .subscription(subscription)
        .run()
}

#[derive(Debug, Deserialize, Serialize)]
struct Reply {
    ip: String,
}

#[derive(Debug, Default)]
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

fn request_recviwer(client: reqwest::Client) -> impl iced::futures::Stream<Item = Message> {
    iced::stream::channel(10, |mut sender| async move {
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

fn subscription(state: &App) -> iced::Subscription<Message> {
    let client = state.client.clone();
    iced::Subscription::run_with_id("websocket" ,request_recviwer(client))
}

fn update(state: &mut App, message: Message) -> iced::Task<Message> {
    match message {
        Message::ButtonMessage(msm) => match msm {
            ButtonMessage::FetchIp => {
                let client = state.client.clone();
                return iced::Task::perform(
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
                state.text_input = message;
            }
            ButtonMessage::Submit => {
                if let Some(channel) = &mut state.channel {
                    channel
                        .try_send(state.text_input.clone())
                        .expect("Failed to send message");
                    state.text_input.clear();
                }
            }
        },
        Message::IpFetched(res) => {
            if let Ok(reply) = res {
                state.reply = Some(reply);
            } else {
                eprintln!("Failed to fetch IP: {:?}", res.unwrap_err());
            }
        }
        Message::NewChannel(sender) => {
            state.channel = Some(sender);
        }
        Message::NewMessage(new_message) => {
            state.messages.push(new_message);
        }
    };

    iced::Task::none()
}

fn view(state: &App) -> Element<Message> {
    let ip = Element::from(widget::row![
        widget::button("Fetch IP").on_press(ButtonMessage::FetchIp),
        match &state.reply {
            Some(reply) => widget::text(reply.ip.clone()),
            None => widget::text("No IP fetched"),
        },
    ])
    .map(Message::ButtonMessage);

    let text_input = Element::from(
        widget::text_input("text to send", &state.text_input)
            .on_input(ButtonMessage::TextInput)
            .on_submit(ButtonMessage::Submit),
    )
    .map(Message::ButtonMessage);

    let messages = widget::column![text_input];

    let messages = state
        .messages
        .iter()
        .fold(messages, |acc, message| acc.push(widget::text(message)));

    container(widget::column![ip, messages])
        .center(Length::Fill)
        .into()
}
