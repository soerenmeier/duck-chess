
mod api;
mod cors;
mod error;
mod requests;
mod runner;

use runner::Runner;

use std::sync::Arc;
use tokio::sync::Mutex;
use clap::Parser;

use fire::{data_struct, static_file, static_files};

data_struct!{
	pub struct Data {
		runner: Arc<Mutex<Runner>>
	}
}

static_file!(Index, "/" => "./public/index.html");

static_files!(Public, "/" => "./public");

#[derive(Parser)]
struct Args {
	#[clap(long)]
	enable_cors: bool,
	#[clap(long, default_value = "500")]
	buffer_len: usize
}

#[tokio::main]
async fn main() {
	let args = Args::parse();

	let data = Data {
		runner: Arc::new(Mutex::new(runner))
	};

	let mut server = fire::build("0.0.0.0:1658", data)
		.expect("Address could not be parsed");

	eprintln!("listening on 0.0.0.0:1658");

	server.add_route(Index::new());
	requests::add_routes(&mut server);
	server.add_route(Public::new());
	if args.enable_cors {
		cors::add_routes(&mut server);
	}

	server.light().await
		.expect("failed to start server");
}