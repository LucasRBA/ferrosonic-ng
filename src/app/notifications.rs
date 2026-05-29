use notify_rust::{Hint, Notification, Timeout};
use std::sync::atomic::{AtomicU32, Ordering};
use tracing::error;

/// Holds the ID of the last notification so we can replace it.
/// A value of 0 means no notification has been sent yet.
static LAST_NOTIFICATION_ID: AtomicU32 = AtomicU32::new(0);

pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
}

pub fn notify_track_change(track: &TrackInfo) {
    let mut builder = Notification::new();
    builder
        .appname("Ferrosonic")
        .summary(&track.title)
        .body(&format!("{} — {}", track.artist, track.album))
        .hint(Hint::Category("music".to_owned()))
        .hint(Hint::Transient(true)) // don't persist in notification center
        .timeout(Timeout::Milliseconds(5000));

    // Replace the previous notification instead of stacking
    let previous_id = LAST_NOTIFICATION_ID.load(Ordering::SeqCst);
    if previous_id != 0 {
        builder.id(previous_id);
    }

    match builder.show() {
        Ok(handle) => {
            LAST_NOTIFICATION_ID.store(handle.id(), Ordering::SeqCst);
        }
        Err(e) => error!("Failed to show notification: {}", e),
    }
}
