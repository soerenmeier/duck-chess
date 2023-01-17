mod api;
mod cors;
mod error;
mod requests;

use clap::Parser;

mod assets {
	include!(concat!(env!("OUT_DIR"), "/assets_routes.rs"));
}

#[derive(Parser)]
struct Args {
	#[clap(long)]
	enable_cors: bool
}

#[tokio::main]
async fn main() {
	let args = Args::parse();

	let mut server = fire::build("0.0.0.0:1658").await.unwrap();

	eprintln!("listening on 0.0.0.0:1658");

	assets::add_routes(&mut server);
	requests::add_routes(&mut server);
	if args.enable_cors {
		cors::add_routes(&mut server);
	}

	server.ignite().await.unwrap();
}