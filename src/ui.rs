pub mod chat;

use iced::{
	executor, button, Application, Column,  Button, Command, Container, Element, HorizontalAlignment, Length, Row, Text,
};

use crate::matrix;
use crate::matrix::Client;
use crate::ui::chat::ChatView;

pub enum MClient {
	Login,
	Loading,
	Loaded(Chat),
}

pub struct Chat {
	client: Client,
	views: Vec<ChatView>,
	current_view: usize,
	button_state: button::State
}

#[derive(Debug, Clone)]
pub enum Message {
	Loaded(Client),
	InputChanged(String),
	InputSubmit,
	MessageDelivered,
	ButtonPress,
}

impl Application for MClient {
	type Executor = executor::Default;
	type Message = crate::ui::Message;
	type Flags = ();

	fn new(_flags: ()) -> (MClient, Command<Message>) {
		(
			MClient::Loading,
			Command::perform(matrix::connect(), Message::Loaded),
		)
	}

	fn title(&self) -> String {
		String::from("Matrix Chat")
	}

	fn update(&mut self, message: Message) -> Command<Message> {
		match self {
			MClient::Login => {}
			MClient::Loading => match message {
				// When we are finally logged in and loaded
				Message::Loaded(client) => {
					*self = MClient::Loaded(Chat {
						client,
						views: vec![ChatView::new()],
						current_view: 0,
						button_state: button::State::new()
					});
				}
				_ => {}
			},
			MClient::Loaded(chat) => {
				match message {
					Message::ButtonPress => {
						// do thing
					}
					_ => {}
				}
				return chat.views[chat.current_view].update(message, chat.client.clone());
			}
		}

		Command::none()
	}

	fn view(&mut self) -> Element<Message> {
		match self {
			MClient::Login => Row::new().into(),
			MClient::Loading => loading_screen_view(),
			MClient::Loaded(chat) => {
				let x = chat.views[chat.current_view].view();
				Column::new().push(x).push(Button::new(&mut chat.button_state, Text::new("do thing")).on_press(Message::ButtonPress)).width(Length::Fill).height(Length::Fill).into()
			},
		}
	}
}

fn loading_screen_view<'a>() -> Element<'a, Message> {
	Container::new(
		Text::new("Loading...")
			.horizontal_alignment(HorizontalAlignment::Center)
			.width(Length::Fill)
			.size(25),
	)
	.width(Length::Fill)
	.center_y()
	.into()
}
