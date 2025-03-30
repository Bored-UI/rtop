use ratatui::{layout::Rect, Frame};

use crate::{tui::AppColorInfo, types::NetworkData};

// width smaller than this will be consider small width for the memory container
const SMALL_WIDTH: u16 = 40;
const MEDIUM_HEIGHT: u16 = 16;
const LARGE_HEIGHT: u16 = 21;
// const MEMORY_GRAPH_HEIGHT_PRCENTAGE: u16 = 70;

// this was to indicate that the memory graph y axis will be either shown as 25% or 100% (based on the widget size)
const SMALL_WIDGET_PERCENTAGE: f64 = 25.0;
const BIG_WIDGET_PERCENTAGE: f64 = 100.0;

pub fn draw_network_info(
    tick: u64,
    memory: &NetworkData,
    area: Rect,
    frame: &mut Frame,
    graph_show_range: usize,
    is_selected: bool,
    app_color_info: &AppColorInfo,
    is_full_screen: bool,
) {
}
