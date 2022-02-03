use http::StatusCode;
use std::error::Error;
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request, Response};

fn handler(req: Request) -> Result<impl IntoResponse, VercelError> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(ics_filter::filter_from_query(req.uri.query().unwrap()))
        .expect("Internal Server Error");

    Ok(response)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
