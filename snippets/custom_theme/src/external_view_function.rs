use iced::{
    widget::{self, button, container, text},
    Alignment, Color,
};

use crate::{theme, Message};

pub fn show_counter<'a>(value: i32) -> crate::Element<'a> {
    let counter = widget::column![
        button("Increment")
            .on_press(Message::IncrementPressed)
            .style(theme::Button::Black),
        text(value).size(50).style(theme::Text::Color(Color::BLACK)),
        button("Decrement")
            .on_press(Message::DecrementPressed)
            .style(theme::Button::Black)
    ]
    .padding(20)
    .align_items(Alignment::Center);
    container(counter).into()
}
