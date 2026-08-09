#[test]
fn channel_points_uses_website_session_for_watch_and_context() {
    let lib = include_str!("../src/lib.rs");
    let presence = include_str!("../src/viewer_presence.rs");
    let realtime = include_str!("../src/channel_points_realtime.rs");
    let points = include_str!("../src/channel_points.rs");
    let auth_bar = include_str!("../../src/components/AuthBar.tsx");
    let website_auth = include_str!("../../src/components/TwitchWebsiteAuth.tsx");

    assert!(!lib.contains("mod channel_points_auth;"));
    assert!(lib.contains("mod channel_points_claim_auth;"));

    assert!(presence.contains("twitch_web_auth::load_session"));
    assert!(presence.contains("twitch_web_auth::WEB_CLIENT_ID"));
    assert!(presence.contains("twitch_web_auth::client_session_id"));
    assert!(presence.contains("format!(\"OAuth {token}\")"));
    assert!(!presence.contains("channel_points_claim_auth::"));

    assert!(realtime.contains("twitch_web_auth::load_session"));
    assert!(!realtime.contains("channel_points_claim_auth::"));

    // Balance/context polling remains Website-authenticated.
    assert!(points.contains("twitch_web_auth::load_session"));
    assert!(points.contains("twitch_web_auth::WEB_CLIENT_ID"));
    assert!(points.contains("twitch_web_auth::client_session_id"));

    // Only the protected +50 claim uses the isolated TV identity.
    assert!(points.contains("channel_points_claim_auth::load_session"));
    assert!(points.contains("channel_points_claim_auth::TV_CLIENT_ID"));
    assert!(!points.contains("Client-Integrity"));

    assert!(auth_bar.contains("TwitchWebsiteAuth"));
    assert!(auth_bar.contains("ChannelPointsClaimAuth"));
    assert!(website_auth.contains("syncViewerPresence(true)"));
}
