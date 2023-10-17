use std::{
    fs::{File, OpenOptions},
    ops::RangeInclusive, io::BufWriter,
};
use crate::Message;

fn update(&mut self, message: Message) {
    match message {
        Message::CreateFile => {
            let path = "./out/example.txt";
            let file = OpenOptions::new().append(true).create(true).open(path);
            if let Ok(mut file) = file {
                for i in 1..=self.lines.unwrap_or(0) {
                    self.update(Message::UpdatingProgressBar(self.progress + 1.0));
                    file.write_all(format!("linea {i}\n").as_bytes()).unwrap();
                }
            }
        }
        Message::DeleteText => {
            self.lines = None;
            self.update(Message::UpdatingProgressBar(0.0));
        }
        Message::OnInputChange(message) => self.lines = message,
        Message::OnNoteChange(message) => self.notes = message,
        Message::UpdatingProgressBar(val) => self.progress = val,
    }
}

// state of async thread
enum FileState {
    Opening(String, BufWriter),
    Reading(File, RangeInclusive<i32>, i32),
    Error,
}
// a message to be passed back to the application from the async thread
enum FileEvent {
    Opened,
    Read,
    Error,
    UpdatingProgressBar(i32),
}

fn subscription(lines: Option<i32>) -> iced::Subscription<Message> {

    // a function that if run by the rust runtime the thing that handles external threads
    // if an id is the same evertime the runtime checks then is will not make a new future
    iced::subscription::unfold(
        // the id that is return with the function for the runtime
        "some id",
        // the start state of this future
        FileState::Opening("./out/example.txt".to_string()),
        // a function that is run by the runtime continually
        // when returning your return type is (Option<Message>, async_thread_state)
        // in this case the async thread state is the File state enum
        |state| async move {
            match state {
                FileState::Opening(path) => {
                    let file = OpenOptions::new().append(true).create(true).open(path);
                    if let Ok(file) = file {
                        let reader = 1..=lines.unwrap_or(0);
                        (FileEvent::Opened, FileState::Reading(file, reader, 0))
                    } else {
                        (FileEvent::Error, FileState::Error)
                    }
                }
                FileState::Reading(mut file, reader, mut progress) => {
                    if let Some(line) = reader.next() {
                        progress += 1;
                        (
                            FileEvent::UpdatingProgressBar(progress),
                            FileState::Reading(file, reader, progress),
                        )
                    } else {
                        (None, FileState::Error)
                    }
                }
                FileState::Error => (None, FileState::Error),
            }
        },
    ).map(Message::FileMessage)
}

impl application for T {
    fn subscription(&self) -> iced::Subscription<Message> {
 // a function that if run by the rust runtime the thing that handles external threads
    // if an id is the same evertime the runtime checks then is will not make a new future
    iced::subscription::unfold(
        // the id that is return with the function for the runtime
        "some id",
        // the start state of this future
        FileState::Opening("./out/example.txt".to_string()),
            subscription::unfold(&path, FileState::Opening(path, bytes), run)

    }
}

fn my_suscription(path: String, bytes: BufWriter) -> iced::Subscription<Message> {
    iced::subscription::unfold(
        "some id",
        FileState::Opening(path, bytes),
        run,
    ).map(Message::FileMessage)
}

async fn run(state: FileState) -> (Option<FileEvent>, FileState) {
    match state {
        FileState::Opening(path, lines) => {
            let file = OpenOptions::new().append(true).create(true).open(path);
            match file {
                Ok(file) => {
                    let reader = 1..=lines;
                    (Some(FileEvent::Opened), FileState::Writing(file, reader, 0))
                }
                Err(_) => (None, FileState::Error),
            }
        }
        FileState::Writing(mut file, range, mut progress) => {
            file.write_all(format!("linea {progress}\n").as_bytes())
                .unwrap();

            if !range.contains(&progress) {
                return (None, FileState::Error);
            }
            progress += 1;
            (
                Some(FileEvent::UpdatingProgressBar(progress as i32)),
                FileState::Writing(file, range, progress),
            )
        }
        FileState::Error => (None, FileState::Error),
    }
}

enum Message {
    FileMessage(FileEvent),
    
}

fn update(&mut self, message: Message) -> iced::Command {
    match message {
      FileMessage(file_event: FileEvent) -> {
      match   file_event {
        // do somthing with teh message
      }
    }
      {..}
  }