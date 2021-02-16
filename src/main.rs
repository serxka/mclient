use iced::{Application, Settings};

mod matrix;
mod ui;

fn main() -> iced::Result {
	ui::MClient::run(Settings::default())
}
