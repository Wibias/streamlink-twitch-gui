#[test]
fn bonus_claim_uses_web_integrity_without_touching_watch_transport() {
    let source = include_str!("../src/channel_points.rs");

    assert!(source.contains("ClaimCommunityPoints"));
    assert!(source.contains("46aaeebe02c99afdf4fc97c7c0cba964124bf6b0af229395f1f6d1feed05b3d0"));
    assert!(source.contains("https://gql.twitch.tv/integrity"));
    assert!(source.contains("\"Client-Integrity\""));
    assert!(source.contains("async fn fetch_integrity_token("));
    assert!(source.contains("async fn claim_bonus("));
    assert!(source.contains("crate::channel_points_realtime::is_ready()"));
}
