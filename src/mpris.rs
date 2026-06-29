#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
}

impl TrackInfo {
    pub fn placeholder() -> Self {
        Self {
            title: "No track playing".into(),
            artist: "—".into(),
            album: "—".into(),
        }
    }
}
