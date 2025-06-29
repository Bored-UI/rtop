use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    symbols::{border, Marker},
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
    Frame,
};

use crate::{
    types::{AppColorInfo, MemoryData},
    utils::{get_tick_line_ui, process_to_kib_mib_gib},
};

// width smaller than this will be consider small width for the memory container
const SMALL_WIDTH: u16 = 20;
const MEDIUM_HEIGHT: u16 = 16;
const LARGE_HEIGHT: u16 = 21;
const GRAPH_PERCENTAGE: f64 = 100.0;

pub fn draw_memory_info(
    tick: u64,
    memory: &MemoryData,
    area: Rect,
    frame: &mut Frame,
    graph_show_range: usize,
    is_selected: bool,
    app_color_info: &AppColorInfo,
    is_full_screen: bool,
) {
    let select_instruction = Line::from(vec![
        Span::styled(" ", Style::default().fg(app_color_info.app_title_color)),
        Span::styled("M", Style::default().fg(app_color_info.key_text_color))
            .bold()
            .underlined(),
        Span::styled(
            "emory ",
            Style::default().fg(app_color_info.app_title_color).bold(),
        ),
    ]);

    let mut main_block = Block::bordered()
        .title(select_instruction.left_aligned())
        .style(app_color_info.memory_main_block_color)
        .border_set(border::ROUNDED);
    if is_selected {
        main_block = main_block
            .style(app_color_info.memory_container_selected_color)
            .border_set(border::DOUBLE);
    }

    if is_full_screen {
        let refresh_tick = get_tick_line_ui(tick, app_color_info);

        main_block = main_block.title(refresh_tick.right_aligned());
    }

    // this will be the layout for the memory usage graph
    let [_, memory_block, _] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(area);

    // padded the layout for the memory usage graph to have some space on the left and right
    let [_, padded_memory_block, _] = Layout::horizontal([
        Constraint::Length(2),
        Constraint::Fill(1),
        Constraint::Length(2),
    ])
    .areas(memory_block);

    // top label will be the label for total memory
    // bottom graph will be the statistics for memory usage like used, free, available memory, etc
    let [top_label, bottom_graphs] =
        Layout::vertical([Constraint::Length(2), Constraint::Fill(1)]).areas(padded_memory_block);

    let total_memory_label = Line::from("Total:")
        .style(app_color_info.app_title_color)
        .bold();
    let total_memory = Line::from(process_to_kib_mib_gib(memory.total_memory))
        .style(app_color_info.app_title_color)
        .bold();
    let top_inner_block = Block::new()
        .title(total_memory_label.left_aligned())
        .title(total_memory.right_aligned())
        .style(app_color_info.memory_main_block_color)
        .borders(Borders::NONE);

    frame.render_widget(main_block, area);
    frame.render_widget(top_inner_block, top_label);

    // we will show the metrics baseed on the height of the terminal
    // so that the rendering will fit nicely
    let mut cached_memory_layout = Rect::default();
    let mut swap_memory_layout = Rect::default();
    let [mut used_memory_layout, mut available_memory_layout, mut free_memory_layout] =
        Layout::vertical([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .areas(bottom_graphs);

    if area.height >= MEDIUM_HEIGHT {
        let [new_used_memory_layout, new_available_memory_layout, new_free_memory_layout, new_swap_memory_layout] =
            Layout::vertical([
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
            ])
            .areas(bottom_graphs);
        used_memory_layout = new_used_memory_layout;
        available_memory_layout = new_available_memory_layout;
        free_memory_layout = new_free_memory_layout;
        swap_memory_layout = new_swap_memory_layout;
    }
    if area.height >= LARGE_HEIGHT {
        let [new_used_memory_layout, new_available_memory_layout, new_free_memory_layout, new_cached_memory_layout, new_swap_memory_layout] =
            Layout::vertical([
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
            ])
            .areas(bottom_graphs);
        used_memory_layout = new_used_memory_layout;
        available_memory_layout = new_available_memory_layout;
        free_memory_layout = new_free_memory_layout;
        cached_memory_layout = new_cached_memory_layout;
        swap_memory_layout = new_swap_memory_layout;
    }

    let border_type = if bottom_graphs.width < SMALL_WIDTH {
        Borders::NONE
    } else {
        Borders::TOP
    };

    // ----------------------------------------
    //
    //          FOR USED MEMORY LAYOUT
    //
    // ----------------------------------------
    let [_, used_memory_graph] =
        Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(used_memory_layout);
    let used_memory_label = if used_memory_layout.width < SMALL_WIDTH {
        Line::from("U").style(app_color_info.base_app_text_color)
    } else {
        Line::from("Used:").style(app_color_info.base_app_text_color)
    };

    let used_memory_usage = Line::from(process_to_kib_mib_gib(
        memory.used_memory_vec[memory.used_memory_vec.len() - 1],
    ))
    .style(app_color_info.memory_text_color)
    .bold();

    let used_memory_block = Block::new()
        .title(used_memory_label.left_aligned())
        .title(used_memory_usage.right_aligned())
        .style(app_color_info.memory_main_block_color)
        .borders(border_type);

    let used_memory_history = memory.used_memory_vec.clone();
    let num_points_to_display = graph_show_range.min(used_memory_history.len());
    let start_idx = used_memory_history
        .len()
        .saturating_sub(num_points_to_display);
    let mut used_memory_data_points: Vec<(f64, f64)> = used_memory_history[start_idx..]
        .iter()
        .enumerate()
        .map(|(i, &usage)| {
            let x = i as f64;
            let y = (usage / memory.total_memory) * GRAPH_PERCENTAGE as f64;
            (x, y)
        })
        .collect();

    used_memory_data_points = used_memory_data_points
        .iter()
        .map(|(x, y)| {
            (
                graph_show_range as f64 - (used_memory_data_points.len() as f64 - x),
                *y,
            )
        })
        .collect();

    let dataset = Dataset::default()
        .data(&used_memory_data_points)
        .graph_type(GraphType::Bar)
        .marker(Marker::Braille)
        .style(Style::default().fg(app_color_info.used_memory_base_graph_color));

    let x_axis = Axis::default().bounds([0.0, graph_show_range as f64]);

    let y_axis = Axis::default().bounds([0.0, GRAPH_PERCENTAGE]);

    let used_memory_chart = Chart::new(vec![dataset])
        .x_axis(x_axis)
        .y_axis(y_axis)
        .bg(app_color_info.background_color);

    frame.render_widget(used_memory_block, used_memory_layout);
    frame.render_widget(used_memory_chart, used_memory_graph);

    drop(used_memory_history);
    drop(used_memory_data_points);

    // ----------------------------------------
    //
    //      FOR AVAILABLE MEMORY LAYOUT
    //
    // ----------------------------------------
    let [_, available_memory_graph] =
        Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
            .areas(available_memory_layout);
    let available_memory_label = if available_memory_layout.width < SMALL_WIDTH {
        Line::from("A").style(app_color_info.base_app_text_color)
    } else {
        Line::from("Available:").style(app_color_info.base_app_text_color)
    };

    let available_memory_usage = Line::from(process_to_kib_mib_gib(
        memory.available_memory_vec[memory.available_memory_vec.len() - 1],
    ))
    .style(app_color_info.memory_text_color)
    .bold();

    let available_memory_block = Block::new()
        .title(available_memory_label.left_aligned())
        .title(available_memory_usage.right_aligned())
        .style(app_color_info.memory_main_block_color)
        .borders(border_type);

    let available_memory_history = memory.available_memory_vec.clone();
    let num_points_to_display = graph_show_range.min(available_memory_history.len());
    let start_idx = available_memory_history
        .len()
        .saturating_sub(num_points_to_display);
    let mut available_memory_data_points: Vec<(f64, f64)> = available_memory_history[start_idx..]
        .iter()
        .enumerate()
        .map(|(i, &remain)| {
            let x = i as f64;
            let y = (remain / memory.total_memory) * GRAPH_PERCENTAGE as f64;
            (x, y)
        })
        .collect();

    available_memory_data_points = available_memory_data_points
        .iter()
        .map(|(x, y)| {
            (
                graph_show_range as f64 - (available_memory_data_points.len() as f64 - x),
                *y,
            )
        })
        .collect();

    let dataset = Dataset::default()
        .data(&available_memory_data_points)
        .graph_type(GraphType::Bar)
        .marker(Marker::Braille)
        .style(Style::default().fg(app_color_info.available_memory_base_graph_color));

    let x_axis = Axis::default().bounds([0.0, graph_show_range as f64]);

    let y_axis = Axis::default().bounds([0.0, GRAPH_PERCENTAGE]);

    let available_memory_chart = Chart::new(vec![dataset])
        .x_axis(x_axis)
        .y_axis(y_axis)
        .bg(app_color_info.background_color);

    frame.render_widget(available_memory_block, available_memory_layout);
    frame.render_widget(available_memory_chart, available_memory_graph);

    drop(available_memory_history);
    drop(available_memory_data_points);

    // ----------------------------------------
    //
    //        FOR FREE MEMORY LAYOUT
    //
    // ----------------------------------------
    let [_, free_memory_graph] =
        Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(free_memory_layout);
    let free_memory_label = if free_memory_layout.width < SMALL_WIDTH {
        Line::from("F").style(app_color_info.base_app_text_color)
    } else {
        Line::from("Free:").style(app_color_info.base_app_text_color)
    };

    let free_memory_usage = Line::from(process_to_kib_mib_gib(
        memory.free_memory_vec[memory.free_memory_vec.len() - 1],
    ))
    .style(app_color_info.memory_text_color)
    .bold();

    let free_memory_block = Block::new()
        .title(free_memory_label.left_aligned())
        .title(free_memory_usage.right_aligned())
        .style(app_color_info.memory_main_block_color)
        .borders(border_type);

    let free_memory_history = memory.free_memory_vec.clone();
    let num_points_to_display = graph_show_range.min(free_memory_history.len());
    let start_idx = free_memory_history
        .len()
        .saturating_sub(num_points_to_display);
    let mut free_memory_data_points: Vec<(f64, f64)> = free_memory_history[start_idx..]
        .iter()
        .enumerate()
        .map(|(i, &free)| {
            let x = i as f64;
            let y = (free / memory.total_memory) * GRAPH_PERCENTAGE as f64;
            (x, y)
        })
        .collect();

    free_memory_data_points = free_memory_data_points
        .iter()
        .map(|(x, y)| {
            (
                graph_show_range as f64 - (free_memory_data_points.len() as f64 - x),
                *y,
            )
        })
        .collect();

    let dataset = Dataset::default()
        .data(&free_memory_data_points)
        .graph_type(GraphType::Bar)
        .marker(Marker::Braille)
        .style(Style::default().fg(app_color_info.free_memory_base_graph_color));

    let x_axis = Axis::default().bounds([0.0, graph_show_range as f64]);

    let y_axis = Axis::default().bounds([0.0, GRAPH_PERCENTAGE]);

    let free_memory_chart = Chart::new(vec![dataset])
        .x_axis(x_axis)
        .y_axis(y_axis)
        .bg(app_color_info.background_color);

    frame.render_widget(free_memory_block, free_memory_layout);
    frame.render_widget(free_memory_chart, free_memory_graph);

    drop(free_memory_history);
    drop(free_memory_data_points);

    // ----------------------------------------
    //
    //        FOR SWAP MEMORY LAYOUT
    //
    // ----------------------------------------
    if swap_memory_layout.height > 0 {
        let [_, swap_memory_graph] = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
            .areas(swap_memory_layout);
        let swap_memory_label = if swap_memory_layout.width < SMALL_WIDTH {
            Line::from("S").style(app_color_info.base_app_text_color)
        } else {
            Line::from("Swap:").style(app_color_info.base_app_text_color)
        };

        let swap_memory_usage = Line::from(process_to_kib_mib_gib(
            memory.used_swap_vec[memory.used_swap_vec.len() - 1],
        ))
        .style(app_color_info.memory_text_color)
        .bold();

        let swap_memory_block = Block::new()
            .title(swap_memory_label.left_aligned())
            .title(swap_memory_usage.right_aligned())
            .style(app_color_info.memory_main_block_color)
            .borders(border_type);

        let swap_memory_history = memory.used_swap_vec.clone();
        let num_points_to_display = graph_show_range.min(swap_memory_history.len());
        let start_idx = swap_memory_history
            .len()
            .saturating_sub(num_points_to_display);
        let mut swap_memory_data_points: Vec<(f64, f64)> = swap_memory_history[start_idx..]
            .iter()
            .enumerate()
            .map(|(i, &swap)| {
                let x = i as f64;
                let y =
                    (swap.min(memory.total_memory) / memory.total_memory) * GRAPH_PERCENTAGE as f64;
                (x, y)
            })
            .collect();

        swap_memory_data_points = swap_memory_data_points
            .iter()
            .map(|(x, y)| {
                (
                    graph_show_range as f64 - (swap_memory_data_points.len() as f64 - x),
                    *y,
                )
            })
            .collect();

        let dataset = Dataset::default()
            .data(&swap_memory_data_points)
            .graph_type(GraphType::Bar)
            .marker(Marker::Braille)
            .style(Style::default().fg(app_color_info.swap_memory_base_graph_color));

        let x_axis = Axis::default().bounds([0.0, graph_show_range as f64]);

        let y_axis = Axis::default().bounds([0.0, GRAPH_PERCENTAGE]);

        let swap_memory_chart = Chart::new(vec![dataset])
            .x_axis(x_axis)
            .y_axis(y_axis)
            .bg(app_color_info.background_color);

        frame.render_widget(swap_memory_block, swap_memory_layout);
        frame.render_widget(swap_memory_chart, swap_memory_graph);

        drop(swap_memory_history);
        drop(swap_memory_data_points);
    }

    // ----------------------------------------
    //
    //       FOR CACHED MEMORY LAYOUT
    //
    // ----------------------------------------
    if cached_memory_layout.height > 0 {
        let [_, cached_memory_graph] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
                .areas(cached_memory_layout);
        let cached_memory_label = if cached_memory_layout.width < SMALL_WIDTH {
            Line::from("C").style(app_color_info.base_app_text_color)
        } else {
            Line::from("Cached:").style(app_color_info.base_app_text_color)
        };

        let cached_memory_usage = Line::from(process_to_kib_mib_gib(
            memory.cached_memory_vec[memory.cached_memory_vec.len() - 1],
        ))
        .style(app_color_info.memory_text_color)
        .bold();

        let cached_memory_block = Block::new()
            .title(cached_memory_label.left_aligned())
            .title(cached_memory_usage.right_aligned())
            .style(app_color_info.memory_main_block_color)
            .borders(border_type);

        let cached_memory_history = memory.cached_memory_vec.clone();
        let num_points_to_display = graph_show_range.min(cached_memory_history.len());
        let start_idx = cached_memory_history
            .len()
            .saturating_sub(num_points_to_display);
        let mut cached_memory_data_points: Vec<(f64, f64)> = cached_memory_history[start_idx..]
            .iter()
            .enumerate()
            .map(|(i, &cached)| {
                let x = i as f64;
                let y = (cached.min(memory.total_memory) / memory.total_memory)
                    * GRAPH_PERCENTAGE as f64;
                (x, y)
            })
            .collect();

        cached_memory_data_points = cached_memory_data_points
            .iter()
            .map(|(x, y)| {
                (
                    graph_show_range as f64 - (cached_memory_data_points.len() as f64 - x),
                    *y,
                )
            })
            .collect();

        let dataset = Dataset::default()
            .data(&cached_memory_data_points)
            .graph_type(GraphType::Bar)
            .marker(Marker::Braille)
            .style(Style::default().fg(app_color_info.cached_memory_base_graph_color));

        let x_axis = Axis::default().bounds([0.0, graph_show_range as f64]);

        let y_axis = Axis::default().bounds([0.0, GRAPH_PERCENTAGE]);

        let cached_memory_chart = Chart::new(vec![dataset])
            .x_axis(x_axis)
            .y_axis(y_axis)
            .bg(app_color_info.background_color);

        frame.render_widget(cached_memory_block, cached_memory_layout);
        frame.render_widget(cached_memory_chart, cached_memory_graph);

        drop(cached_memory_history);
        drop(cached_memory_data_points);
    }
}
