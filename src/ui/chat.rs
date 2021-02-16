use matrix_sdk::events::{room::message::MessageEventContent, AnyMessageEventContent};
// use matrix_sdk::RoomInfo;
use ruma::RoomId;

use iced::{
	scrollable, text_input, Align, Column, Command, Container, Element, Length, Scrollable, Text,
	TextInput,
};

use std::convert::TryFrom;

use crate::matrix::Client;
use crate::ui::Message;

pub struct ChatView {
	input: text_input::State,
	input_content: String,
	scroll: scrollable::State,
	// room: RoomInfo,
	messages: Vec<String>,
}

impl ChatView {
	pub fn new() -> Self {
		Self {
			input: text_input::State::focused(),
			input_content: String::new(),
			scroll: scrollable::State::new(),
			messages: Vec::new(),
		}
	}

	pub fn update(&mut self, message: Message, client: Client) -> Command<Message> {
		match message {
			Message::InputSubmit => {
				self.messages.push(self.input_content.clone());
				self.input_content.clear();

				let message = self.messages.last().unwrap().clone();
				return Command::perform(
					async move {
						client
							.room_send(
								&RoomId::try_from("!RykxkpZvmufepGYwgD:matrix.org").unwrap(),
								AnyMessageEventContent::RoomMessage(
									MessageEventContent::text_plain(message),
								),
								None,
							)
							.await
					},
					|_| Message::MessageDelivered,
				);
			}
			Message::InputChanged(text) => {
				self.input_content = text;
			}
			_ => {}
		}
		Command::none()
	}

	pub fn view(&mut self) -> Element<Message> {
		let text_input = TextInput::new(
			&mut self.input,
			"Send a chat message...",
			&self.input_content,
			Message::InputChanged,
		)
		.padding(5)
		.on_submit(Message::InputSubmit);

		let mut messages = Scrollable::new(&mut self.scroll)
			.width(Length::Fill)
			.padding(5)
			.spacing(5);

		for message in self.messages.iter() {
			messages = messages.push(Text::new(message));
		}

		let content = Column::new()
			.align_items(Align::End)
			.push(Container::new(messages).height(Length::Fill))
			.push(Container::new(text_input).width(Length::Fill));

		Container::new(content)
			.width(Length::Fill)
			.height(Length::Fill)
			.align_y(Align::End)
			.into()
	}
}
