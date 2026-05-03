use crossterm::event::{self, KeyCode};

use crate::error::Error;
use crate::subsonic::models::InternetRadioStation;

use super::*;

impl App {
    /// Handle radio page keys
    pub(super) async fn handle_radio_key(&mut self, key: event::KeyEvent) -> Result<(), Error> {
        let mut state = self.state.write().await;

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if let Some(sel) = state.radio.selected {
                    if sel > 0 {
                        state.radio.selected = Some(sel - 1);
                    }
                } else if !state.radio.stations.is_empty() {
                    state.radio.selected = Some(0);
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max = state.radio.stations.len().saturating_sub(1);
                if let Some(sel) = state.radio.selected {
                    if sel < max {
                        state.radio.selected = Some(sel + 1);
                    }
                } else if !state.radio.stations.is_empty() {
                    state.radio.selected = Some(0);
                }
            }
            KeyCode::Enter => {
                if let Some(station) = selected_station(&state) {
                    drop(state);
                    return self.play_radio_station(station).await;
                }
            }
            KeyCode::Char(' ') => {
                if let Some(station) = selected_station(&state) {
                    let is_current = state
                        .now_playing
                        .radio_station
                        .as_ref()
                        .map(|current| current.id == station.id)
                        .unwrap_or(false);
                    drop(state);

                    if is_current {
                        return self.toggle_pause().await;
                    }
                    return self.play_radio_station(station).await;
                }

                drop(state);
                return self.toggle_pause().await;
            }
            _ => {}
        }

        Ok(())
    }
}

fn selected_station(state: &AppState) -> Option<InternetRadioStation> {
    state
        .radio
        .selected
        .and_then(|idx| state.radio.stations.get(idx).cloned())
}
