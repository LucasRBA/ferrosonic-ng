//! Radio page showing internet radio stations

use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::state::{AppState, RenderMutations};

/// Render the radio page
pub fn render(frame: &mut Frame, area: Rect, state: &AppState, mutations: &mut RenderMutations) {
    let colors = *state.settings_state.theme_colors();
    let radio = &state.radio;

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Radio ({}) ", radio.stations.len()))
        .border_style(Style::default().fg(colors.border_focused));

    if radio.stations.is_empty() {
        let hint = Paragraph::new("No radio stations found")
            .style(Style::default().fg(colors.muted))
            .block(block);
        frame.render_widget(hint, area);
        return;
    }

    let current_station_id = state
        .now_playing
        .radio_station
        .as_ref()
        .map(|station| station.id.as_str());

    let items: Vec<ListItem> = radio
        .stations
        .iter()
        .enumerate()
        .map(|(i, station)| {
            let is_selected = radio.selected == Some(i);
            let is_current = current_station_id == Some(station.id.as_str());
            let detail = station
                .home_page_url
                .as_deref()
                .unwrap_or(station.stream_url.as_str());

            let (name_style, detail_style) = if is_selected {
                (
                    Style::default()
                        .fg(colors.highlight_fg)
                        .add_modifier(Modifier::BOLD),
                    Style::default().fg(colors.highlight_fg),
                )
            } else if is_current {
                (
                    Style::default()
                        .fg(colors.playing)
                        .add_modifier(Modifier::BOLD),
                    Style::default().fg(colors.muted),
                )
            } else {
                (
                    Style::default().fg(colors.song),
                    Style::default().fg(colors.muted),
                )
            };

            let indicator = if is_current { "▶ " } else { "  " };
            let line = Line::from(vec![
                Span::styled(format!("{:3}. ", i + 1), Style::default().fg(colors.muted)),
                Span::styled(indicator, Style::default().fg(colors.playing)),
                Span::styled(&station.name, name_style),
                Span::styled(format!(" — {}", detail), detail_style),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(colors.highlight_bg)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▸ ");

    let mut list_state = ListState::default();
    list_state.select(radio.selected);

    frame.render_stateful_widget(list, area, &mut list_state);
    mutations.radio_scroll_offset = list_state.offset();
}
