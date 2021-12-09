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
            let body_form = req.take_body_form::<HashMap<String, String>>().unwrap();

            // let env = Dictionary::open("env");
            // let slack_token = env.get("SLACK_TOKEN").unwrap();
            // if body_form.get("token").unwrap() != &slack_token {
            //     return Ok(Response::from_status(StatusCode::UNAUTHORIZED)
            //         .with_body_text_plain("Unauthorized\n"));
            // }

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
