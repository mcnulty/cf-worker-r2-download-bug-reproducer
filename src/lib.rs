//! Entry point for auth API service.

#![deny(warnings)]

use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    match entry(req, env).await {
        Ok(r) => Ok(r),
        Err(e) => {
            console_error!("Failed to process request: {:?}", e);
            Ok(empty_with_status(503))
        }
    }
}

async fn entry(req: Request, env: Env) -> Result<Response> {
    let url = match req.url() {
        Ok(u) => u,
        Err(_) => {
            return bad_request();
        }
    };

    if req.method() != Method::Get && req.method() != Method::Head {
        return bad_request();
    }

    let path = url.path();
    if !path.starts_with('/') {
        return bad_request();
    }
    let r2_key = path.trim_start_matches('/');
    if r2_key.is_empty() {
        return bad_request();
    }

    let bucket = env.bucket("BIN")?;

    let object = match bucket.get(r2_key).execute().await? {
        Some(o) => o,
        None => {
            return not_found();
        }
    };

    let body = match object.body() {
        Some(body) => body,
        None => {
            return not_found();
        }
    };

    let mut headers = Headers::new();
    let http_metadata = object.http_metadata();
    if let Some(content_type) = &http_metadata.content_type {
        headers.set("Content-Type", content_type)?;
    }
    if let Some(content_encoding) = &http_metadata.content_encoding {
        headers.set("Content-Encoding", content_encoding)?;
    }
    if let Some(content_language) = &http_metadata.content_language {
        headers.set("Content-Language", content_language)?;
    }
    if let Some(content_disposition) = &http_metadata.content_disposition {
        headers.set("Content-Disposition", content_disposition)?;
    }
    if let Some(cache_control) = &http_metadata.cache_control {
        headers.set("Cache-Control", cache_control)?;
    }
    headers.set("ETag", &object.http_etag())?;

    if req.method() == Method::Head {
        headers.set("Content-Length", &object.size().to_string())?;
        return Ok(empty_with_status(200).with_headers(headers));
    }

    let response_body = body.stream()?;

    Ok(Response::from_stream(response_body)?
        .with_headers(headers)
        .with_status(200))
}

fn empty_with_status(status: u16) -> Response {
    Response::empty()
        .expect("Failed to construct empty Response")
        .with_status(status)
}

fn not_found() -> std::result::Result<Response, Error> {
    problem_details_response(404, "Not found")
}

fn bad_request() -> Result<Response> {
    problem_details_response(400, "Bad Request")
}

fn problem_details_response(status: u16, title: &str) -> Result<Response> {
    let body = format!(r#"{{"status":{},"title":"{}"}}"#, status, title)
        .as_bytes()
        .to_vec();

    Ok(Response::from_bytes(body)?.with_status(status))
}
