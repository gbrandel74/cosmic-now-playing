use mpris::PlayerFinder;

#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub art_url: Option<String>,
}

impl TrackInfo {
    pub fn placeholder() -> Self {
        Self {
            title: "No track playing".into(),
            artist: "—".into(),
            album: "—".into(),
            art_url: None,
        }
    }

    pub fn current() -> Self {
        let Ok(finder) = PlayerFinder::new() else {
            return Self::placeholder();
        };

        let Ok(player) = finder.find_active() else {
            return Self::placeholder();
        };

        let Ok(metadata) = player.get_metadata() else {
            return Self::placeholder();
        };

        Self {
            title: metadata.title().unwrap_or("Unknown title").to_string(),
            artist: metadata
                .artists()
                .map(|artists| artists.join(", "))
                .unwrap_or_else(|| "Unknown artist".to_string()),
            album: metadata.album_name().unwrap_or("Unknown album").to_string(),
            art_url: metadata.art_url().map(|url| url.to_string()),
        }
    }
}
