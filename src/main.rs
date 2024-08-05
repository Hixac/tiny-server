use tiny_http::{Header, Method, Request, Response, Server, StatusCode};

fn form_response(request: Request, bytes: &[u8], content_type: &str, status_code: u16) -> std::io::Result<()> {
	let header = Header::from_bytes("Content-Type", content_type)
		.expect("Shit fucked up header");
	let response = Response::from_data(bytes).with_header(header).with_status_code(StatusCode(status_code));
	
	request.respond(response)
}

fn explore(request: Request) -> std::io::Result<()> {
	match (request.method(), request.url()) {
		(Method::Get, "/main") => {
			form_response(request, std::include_bytes!("index.html"), "text/html; charset=utf-8", 200)
		}
		(Method::Get, "/index.css") => {
			form_response(request, std::include_bytes!("index.css"), "text/css; charset=utf-8", 200)
		}
		(Method::Get, "/index.js") => {
			form_response(request, std::include_bytes!("index.js"), "text/javascript; charset=utf-8", 200)
		}
		(Method::Get, "/404.css") => {
			form_response(request, std::include_bytes!("404.css"), "text/css; charset=utf-8", 200)
		}
		_ => {
			let html_bytes = std::include_bytes!("404.html");

			form_response(request, html_bytes, "text/html; charset=utf-8", 404)
		}
	}
}

fn main() -> std::process::ExitCode {
    let server = Server::http("127.0.0.1:8888").unwrap();
	for request in server.incoming_requests() {
		explore(request).map_err(|err| {
			eprintln!("ERROR: something went wrong exploring: {err}")
		}).ok();
	}

	eprintln!("FATAL: server is down");
	std::process::ExitCode::FAILURE
}
