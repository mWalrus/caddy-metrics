use prometheus_client::encoding::text::encode;
use prometheus_client::registry::Registry;
use rocket::{http::Status, Request, State};

use crate::{
    matchers::{Matcher, Method},
    parser::LogQueue,
    registry::{
        resolve_status_code, RequestLabels, HTTP_REQUESTS_TOTAL, HTTP_RESPONSE_TIME_SECONDS,
    },
};

#[catch(default)]
pub fn default(status: Status, _req: &Request<'_>) -> String {
    format!("ERROR: Something went wrong (code {status})")
}

#[get("/")]
pub fn index() -> String {
    "Wally's caddy exporter".into()
}

#[get("/metrics")]
pub fn metrics(
    log_queue: &State<LogQueue>,
    matchers: &State<Vec<Matcher>>,
    registry: &State<Registry>,
) -> String {
    if let Ok(queue) = &mut log_queue.lock() {
        for entry in queue.drain(..) {
            for matcher in matchers.iter() {
                let req = &entry.request;
                let method = Method::from(req.method.as_ref());
                if matcher.is_match(&req.host, &req.uri, &method) {
                    let uri = matcher.matched_uri_segment(&req.uri);
                    HTTP_REQUESTS_TOTAL
                        .get_or_create(&RequestLabels {
                            host: req.host.clone(),
                            uri,
                            method,
                            status_code: resolve_status_code(entry.status),
                        })
                        .inc();
                    HTTP_RESPONSE_TIME_SECONDS.observe(entry.duration);
                }
            }
        }
    }
    let mut buffer = String::new();
    encode(&mut buffer, &registry).unwrap();
    buffer
}
