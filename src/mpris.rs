use mpris::PlaybackStatus;
use mpris::PlayerFinder;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub art_url: Option<String>,
    pub playback_status: PlaybackStatus,
    pub position: Duration,
    pub length: Option<Duration>,
}

impl TrackInfo {
    pub fn placeholder() -> Self {
        Self {
            title: "No track playing".into(),
            artist: "—".into(),
            album: "—".into(),
            art_url: None,
            playback_status: PlaybackStatus::Stopped,
            position: Duration::ZERO,
            length: None,
        }
    }

    pub fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;

        format!("{minutes}:{seconds:02}")
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
            playback_status: player
                .get_playback_status()
                .unwrap_or(PlaybackStatus::Stopped),
            position: player.get_position().unwrap_or(Duration::ZERO),
            length: metadata.length(),
        }
    }

    pub fn art_path(&self) -> Option<String> {
        self.art_url
            .as_deref()
            .and_then(|url| url.strip_prefix("file://"))
            .map(|path| path.to_string())
    }
}
