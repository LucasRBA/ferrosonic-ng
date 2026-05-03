use crate::error::Error;

use super::*;

impl App {
    /// Handle click on radio page
    pub(super) async fn handle_radio_click(
        &mut self,
        x: u16,
        y: u16,
        layout: &LayoutAreas,
    ) -> Result<(), Error> {
        let mut state = self.state.write().await;
        let content = layout.content;

        let row_in_viewport = y.saturating_sub(content.y + 1) as usize;
        let item_index = state.radio.scroll_offset + row_in_viewport;

        if item_index < state.radio.stations.len() {
            let was_selected = state.radio.selected == Some(item_index);
            state.radio.selected = Some(item_index);

            let is_second_click = was_selected
                && self
                    .last_click
                    .is_some_and(|(lx, ly, t)| lx == x && ly == y && t.elapsed().as_millis() < 500);

            if is_second_click {
                let station = state.radio.stations[item_index].clone();
                drop(state);
                self.last_click = Some((x, y, std::time::Instant::now()));
                return self.play_radio_station(station).await;
            }
        }

        self.last_click = Some((x, y, std::time::Instant::now()));
        Ok(())
    }
}
