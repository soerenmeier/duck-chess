
mod api;
mod cors;
mod error;
mod requests;

use clap::Parser;

use fire::{data_struct, static_file, static_files};

data_struct!{
	pub struct Data {}
}

static_file!(Index, "/" => "./public/index.html");

static_files!(Public, "/" => "./public");

#[derive(Parser)]
struct Args {
	#[clap(long)]
	enable_cors: bool
}

#[tokio::main]
async fn main() {
	let args = Args::parse();

	let data = Data {};

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