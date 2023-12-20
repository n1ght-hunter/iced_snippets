use std::time::Duration;
use std::thread;
use tokio;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState, GlobalHotKeyEventReceiver
};
use iced::subscription::{self, Subscription};


#[derive(Debug, Clone)]
pub enum Key {
    Screenshoot,
    DelayedScreenshoot,
}

#[derive(Debug, Clone)]
pub enum Event {
    Initializing,
    Listened(Key),
    WorkFinished,
}

#[derive(Debug)]
pub enum ChannelState {
    Starting,//(GlobalHotKeyEventReceiver),
    Working(tokio::sync::mpsc::Receiver<Event>),
    Closing,
}

pub fn subscribe (
    global_receiver: GlobalHotKeyEventReceiver, 
    hotkey_screenshoot: u32, 
    hotkey_delayed_screenshoot: u32
) -> Subscription<Event> {
    struct HotKey;

    subscription::channel(
        std::any::TypeId::of::<HotKey>(),
        100,
        move |mut output| async move {
            let mut state = ChannelState::Starting;//(receiver);

            loop {
                match &mut state {
                    ChannelState::Starting => {
                        let (sender, mut receiver) = tokio::sync::mpsc::channel(100);

                        tokio::task::spawn(
                            listen(
                                global_receiver.clone(), 
                                hotkey_screenshoot, 
                                hotkey_delayed_screenshoot, 
                                sender
                            )
                        );
                        //global_receiver.close();
                        let msg = receiver.recv().await.unwrap();
                        println!("{:?}", msg);
                        state = ChannelState::Working(receiver);
                        
                    },
                    ChannelState::Working(ref mut receiver) => {
                        println!("Prima");
                        let msg = receiver.recv().await.unwrap();
                        println!("Dopo");
                        match msg {
                            Event::Listened(key) => {
                                output.try_send(Event::Listened(key)).unwrap();
                            },
                            _ => {
                                state = ChannelState::Closing;
                            }
                        }
                    },
                    ChannelState::Closing => {
                        iced::futures::future::pending().await
                    },
                }
            }
        }
    )
}

async fn listen (
    global_receiver: GlobalHotKeyEventReceiver, 
    hotkey_screenshoot: u32, 
    hotkey_delayed_screenshoot: u32, 
    tx: tokio::sync::mpsc::Sender<Event>
) -> () {
    
    loop {
        if let Ok(event) = global_receiver.try_recv() {
            println!("{event:?}");

            if event.id == hotkey_screenshoot && event.state == HotKeyState::Pressed {
                tx.send(Event::Listened(Key::Screenshoot)).await.unwrap();
            }
            else if event.id == hotkey_delayed_screenshoot && event.state == HotKeyState::Pressed {
                tx.send(Event::Listened(Key::DelayedScreenshoot)).await.unwrap();
            }
            else { () }


            tokio::time::sleep(Duration::from_millis(50)).await;

        }
    }
 
}