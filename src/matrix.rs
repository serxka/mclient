use matrix_sdk::Session;
use std::sync::Arc;

use std::io::Write;

pub type Client = Arc<matrix_sdk::Client>;

pub async fn connect() -> Client {
	let client = matrix_sdk::Client::new("https://matrix.org").unwrap();

	if std::path::Path::new("./user.ron").exists() {
		let string = std::fs::read_to_string("./user.ron").expect("unable to read user.ron");
		let sess: Session = ron::from_str(&string).expect("unable to parse user.ron");
		client.restore_login(sess).await.expect("unable to login");
	} else {
		let args = std::env::args().collect::<Vec<_>>();
		if args.len() < 3 {
			eprintln!("too few arguments, prog [username] [password]");
			std::process::exit(1);
		}
		let res = client
			.login(&args[1], &args[2], None, Some("mclient dev"))
			.await
			.expect("unable to login");
		let mut file = std::fs::File::create("./user.ron").expect("unable to create user.ron");
		let session = Session {
			access_token: res.access_token,
			user_id: res.user_id,
			device_id: res.device_id,
		};
		file.write(
			ron::to_string(&session)
				.expect("unable to seralize session data")
				.as_bytes(),
		)
		.expect("unable to write user.ron");
	}

	Arc::new(client)
}
