use std::convert::Infallible;

use futures::future::Future;
use lambda_http::{Body, IntoResponse, Request, Response};
use serde::Serialize;

use crate::errors::{errors::Error, http_errors::HTTPError};

pub async fn middleware_handle<F, Fut, T>(
    event: Request,
    handler: F,
) -> Result<impl IntoResponse, Infallible>
where
    F: Fn(Request) -> Fut,
    Fut: Future<Output = Result<T, Error>>,
    T: Serialize,
{
    match handler(event).await {
        Ok(result) => {
            let body = serde_json::to_string(&result).unwrap();
            let response = Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap();

            Ok(response)
        }
        Err(error) => {
            let http_error = HTTPError::new(error);
            let body = serde_json::to_string(&http_error).unwrap();
            let response = Response::builder()
                .status(http_error.code)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap();

            Ok(response)
        }
    }
}
