
use crate::Data;

use fire::FireBuilder;
use fire::catcher;
use fire::http::Response;
use fire::http::header::{Method, StatusCode};

// security headers
catcher!( CorsHeaders,
	|_req, _res| {true},
	|req, res| -> Response {

		let values = &mut res.header.values;

		// if we have a options request this means we need to
		// answer with access-control-allow-origin
		if req.header().method == Method::Options {
			res.header.status_code = StatusCode::NoContent;
			values.insert("access-control-allow-methods", "POST, PUT");
		}

		values.insert("access-control-allow-origin", "*");
		values.insert("access-control-allow-headers", "content-type");
		values.insert("x-xss-protection", "0");
		res
	}
);

pub fn add_routes(server: &mut FireBuilder<Data>) {
	server.add_catcher(CorsHeaders);
}