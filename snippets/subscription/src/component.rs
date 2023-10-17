use iced::{
    widget::{self, text_input, Component, component},
    Element, Length, Padding,
};

#[derive(Debug, Clone)]
pub enum Event {
    InputChanged(String),
}

pub struct NumericInput<Message> {
    value: Option<usize>,
    on_change: Box<dyn Fn(Option<usize>) -> Message>,
    padding: Padding,
    width: Length,
}

pub fn numeric_input<Message>(
    value: Option<usize>,
    on_change: impl Fn(Option<usize>) -> Message + 'static,
    padding: Padding,
    width: Length,
) -> NumericInput<Message> {
    NumericInput::new(value, on_change, padding, width)
}

impl<Message> NumericInput<Message> {
    pub fn new(
        value: Option<usize>,
        on_change: impl Fn(Option<usize>) -> Message + 'static,
        padding: Padding,
        width: Length,
    ) -> Self {
        Self {
            value,
            on_change: Box::new(on_change),
            padding,
            width,
        }
    }
}

impl<Message, Renderer> Component<Message, Renderer> for NumericInput<Message>
where
    Renderer: iced::advanced::text::Renderer + 'static,
    Renderer::Theme:
        widget::button::StyleSheet + widget::text_input::StyleSheet + widget::text::StyleSheet,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
        match event {
            Event::InputChanged(value) => {
                if value.is_empty() {
                    Some((self.on_change)(None))
                } else {
                    value.parse().ok().map(Some).map(self.on_change.as_ref())
                }
            }
        }
    }

    fn view(&self, _state: &Self::State) -> iced::Element<'_, Self::Event, Renderer> {
        text_input(
            "Type a number",
            self.value
                .as_ref()
                .map(usize::to_string)
                .as_deref()
                .unwrap_or(""),
        )
        .on_input(Event::InputChanged)
        .width(self.width)
        .padding(self.padding)
        .into()
    }
}

impl<'a, Message, Renderer> From<NumericInput<Message>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'static + iced::advanced::text::Renderer,
    Renderer::Theme:
        widget::button::StyleSheet + widget::text_input::StyleSheet + widget::text::StyleSheet,
{
    fn from(numeric_input: NumericInput<Message>) -> Self {
        component(numeric_input)
    }
}
