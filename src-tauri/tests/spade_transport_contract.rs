#[test]
fn spade_transport_avoids_stale_idle_connections_and_preserves_error_cause() {
    let source = include_str!("../src/viewer_presence.rs");
    let start = source
        .find("async fn send_minute_watched(")
        .expect("send_minute_watched exists");
    let tail = &source[start..];
    let end = tail
        .find("pub(crate) fn build_minute_watched_payload")
        .expect("payload helper follows send_minute_watched");
    let body = &tail[..end];

    assert!(source.contains("fn spade_client()"));
    assert!(source.contains(".pool_max_idle_per_host(0)"));
    assert!(body.contains("let response = spade_client()"));
    assert!(body.contains(".post(endpoint.clone())"));
    assert!(source.contains("fn telemetry_transport_detail("));
    assert!(source.contains("error.source()"));
}
