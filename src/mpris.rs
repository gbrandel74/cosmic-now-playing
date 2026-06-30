use mpris::PlaybackStatus;
use mpris::PlayerFinder;
use std::time::Duration;

impl std::fmt::Display for PlayerInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.identity)
    }
}

fn friendly_player_name(identity: &str) -> String {
    match identity {
        "com.github.th-ch.youtube-music" => "YouTube Music".to_string(),
        "Mozilla firefox" => "Firefox".to_string(),
        other => other.to_string(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerInfo {
    pub bus_name: String,
    pub identity: String,
}

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

        Self::from_player(&player)
    }

    pub fn art_path(&self) -> Option<String> {
        self.art_url
            .as_deref()
            .and_then(|url| url.strip_prefix("file://"))
            .map(|path| path.to_string())
    }
    pub fn from_player(player: &mpris::Player) -> Self {
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
                .unwrap_or(mpris::PlaybackStatus::Stopped),
            position: player.get_position().unwrap_or(std::time::Duration::ZERO),
            length: metadata.length(),
        }
    }
}

pub fn available_players() -> Vec<PlayerInfo> {
    let Ok(finder) = PlayerFinder::new() else {
        return Vec::new();
    };

    let Ok(players) = finder.find_all() else {
        return Vec::new();
    };

    players
        .into_iter()
        .map(|player| PlayerInfo {
            bus_name: player.bus_name().to_string(),
            identity: friendly_player_name(player.identity()),
        })
        .collect()
}

pub fn player_by_bus_name(bus_name: &str) -> Option<mpris::Player> {
    let finder = PlayerFinder::new().ok()?;
    let players = finder.find_all().ok()?;

    players
        .into_iter()
        .find(|player| player.bus_name() == bus_name)
}
