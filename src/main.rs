use std::collections::HashMap;

use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response,Dictionary};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MessagePayload {
    response_type: String,
    text: String,
}

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    match req.get_method() {
        &Method::GET | &Method::HEAD | &Method::POST => (),

        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD")
                .with_body_text_plain("This method is not allowed\n"))
        }
    };

    match req.get_path() {
        "/command/fastly-echo" => {
            // Below are some common patterns for Compute@Edge services using Rust.
            // Head to https://developer.fastly.com/learning/compute/rust/ to discover more.

            // Create a new request.
            // let mut bereq = Request::get("http://httpbin.org/headers")
            //     .with_header("X-Custom-Header", "Welcome to Compute@Edge!")
            //     .with_ttl(60);

            // Add request headers.
            // bereq.set_header(
            //     "X-Another-Custom-Header",
            //     "Recommended reading: https://developer.fastly.com/learning/compute",
            // );

            // Forward the request to a backend.
            // let mut beresp = bereq.send("backend_name")?;

            // Remove response headers.
            // beresp.remove_header("X-Another-Custom-Header");

            // Log to a Fastly endpoint.
            // use std::io::Write;
            // let mut endpoint = fastly::log::Endpoint::from_name("my_endpoint");
            // writeln!(endpoint, "Hello from the edge!").unwrap();

            // Send a default synthetic response.

            let env = Dictionary::open("env");
            let slack_token = env.get("SLACK_TOKEN").unwrap();

            let body_form = req.take_body_form::<HashMap<String, String>>().unwrap();
            if body_form.get("token").unwrap() != &slack_token {
                return Ok(Response::from_status(StatusCode::UNAUTHORIZED)
                    .with_body_text_plain("Unauthorized\n"));
            }

            let response_type = "in_channel".to_string();
            let text = body_form.get("text").unwrap().clone();
            let message = MessagePayload { response_type, text };

            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::APPLICATION_JSON)
                .with_body(serde_json::to_string(&message).unwrap()))
        }

        // Catch all other requests and return a 404.
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n")),
    }
}
