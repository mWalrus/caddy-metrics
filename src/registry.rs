use lazy_static::lazy_static;
use prometheus_client::{
    encoding::EncodeLabelSet,
    metrics::{counter::Counter, family::Family, histogram::Histogram},
    registry::Registry,
};

use crate::matchers::Method;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct RequestLabels {
    pub host: String,
    pub uri: String,
    pub method: Method,
    pub status_code: u16,
}

lazy_static! {
    pub static ref HTTP_REQUESTS_TOTAL: Family<RequestLabels, Counter> =
        Family::<RequestLabels, Counter>::default();
    pub static ref HTTP_RESPONSE_TIME_SECONDS: Histogram = {
        // Default values from go client(https://github.com/prometheus/client_golang/blob/5d584e2717ef525673736d72cd1d12e304f243d7/prometheus/histogram.go#L68)
        let custom_buckets = [
            0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
        ];
        Histogram::new(custom_buckets.into_iter())
    };
}

pub fn init() -> Registry {
    let mut registry = <Registry>::default();

    registry.register(
        "cm_http_requests",
        "Number of HTTP requests received",
        HTTP_REQUESTS_TOTAL.clone(),
    );

    registry.register(
        "cm_http_response_time_seconds",
        "Duration of HTTP responses",
        HTTP_RESPONSE_TIME_SECONDS.clone(),
    );

    registry
}

pub fn resolve_status_code(code: u16) -> u16 {
    match code {
        100..=199 => 100,
        200..=299 => 200,
        300..=399 => 300,
        400..=499 => 400,
        500..=599 => 500,
        _ => 0,
    }
}
