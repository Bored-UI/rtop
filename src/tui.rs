use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode},
    },
    init,
    layout::{Alignment, Constraint, Layout},
    restore,
    style::{Color, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, ListState, Paragraph},
    DefaultTerminal, Frame,
};

use crate::{
    cpu::draw_cpu_info,
    get_sys_info::spawn_cpu_info_collector,
    types::{CSysInfo, CpuData, SysInfo},
};

#[derive(PartialEq)]
pub enum SelectedContainer {
    None,
    Cpu,
    Memory,
    Disk,
    Network,
    Process,
    Menu,
}

#[derive(PartialEq)]
pub enum AppState {
    View,
    Typing,
    Menu,
}

struct App {
    is_quit: bool,
    tick: u32,
    tx: Sender<CSysInfo>,
    rx: Receiver<CSysInfo>,
    tick_tx: Sender<u32>,
    sys_info: SysInfo,
    selected_container: SelectedContainer,
    state: AppState,
    cpu_graph_shown_range: usize,
    cpu_selected_state: ListState,
    is_renderable: bool,
}

pub struct AppColorInfo {
    pub background_color: Color,
    pub text_color: Color,
    // key text was the key that triggers certain functionality
    pub key_text_color: Color,

    // for cpu
    pub cpu_container_selected_color: Color,
    pub cpu_main_block_color: Color,
    pub cpu_selected_color: Color,
    pub cpu_low_usage_color: Color,
    pub cpu_medium_usage_color: Color,
    pub cpu_high_usage_color: Color,
    pub cpu_graph_color: Color,
    pub cpu_info_border_color: Color,
}

const MIN_HEIGHT: u16 = 30;
const MIN_WIDTH: u16 = 90;

pub fn tui() {
    enable_raw_mode().unwrap();
    let mut terminal = init();
    let (tx, rx) = mpsc::channel();
    let (tick_tx, tick_rx) = mpsc::channel();

    let mut app = App {
        is_quit: false,
        tick: 100,
        tx,
        rx,
        tick_tx,
        sys_info: SysInfo { cpus: vec![] },
        selected_container: SelectedContainer::None,
        state: AppState::View,
        cpu_graph_shown_range: 100,
        cpu_selected_state: ListState::default(),
        is_renderable: true,
    };

    let app_color_info = AppColorInfo {
        // Background color: A dark grayish-blue for the entire terminal
        background_color: Color::Rgb(20, 30, 40), // Dark grayish-blue

        // Text color: A soft white for general text readability
        text_color: Color::Rgb(220, 220, 220), // Soft white

        // Key text color: A bright magenta for key text (e.g., "C" in "Cpu") to highlight functionality triggers
        key_text_color: Color::Rgb(200, 50, 200), // Bright magenta

        // CPU container selected color: A bright cyan for selected container (e.g., main_block when selected)
        cpu_container_selected_color: Color::Rgb(0, 255, 255), // Cyan

        // CPU main block: A slightly lighter grayish-blue to contrast with the background
        cpu_main_block_color: Color::Rgb(40, 50, 60), // Darker grayish-blue

        // CPU selected color: A bright teal for selected CPU items in the list
        cpu_selected_color: Color::Rgb(0, 200, 200), // Teal

        // CPU usage colors: Gradient from green to red
        cpu_low_usage_color: Color::Rgb(50, 200, 50), // Green for low usage (< 30%)
        cpu_medium_usage_color: Color::Rgb(200, 200, 50), // Yellow for medium usage (30-70%)
        cpu_high_usage_color: Color::Rgb(200, 50, 50), // Red for high usage (> 70%)

        // CPU graph color: A muted blue to represent graph lines
        cpu_graph_color: Color::Rgb(70, 130, 180), // Steel blue

        // CPU info border color: A subtle silver for borders
        cpu_info_border_color: Color::Rgb(150, 150, 150), // Silver
    };
    app.run(&mut terminal, tick_rx, app_color_info);
    disable_raw_mode().unwrap();
    restore();
}

impl App {
    // runs the application's main loop until the user quits
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        tick_rx: Receiver<u32>,
        app_color_info: AppColorInfo,
    ) {
        spawn_cpu_info_collector(tick_rx, self.tx.clone(), self.tick);
        thread::sleep(Duration::from_millis(self.tick as u64 + 100));
        let c_sys_info = self.rx.try_recv().unwrap();
        process_sys_info(&mut self.sys_info, c_sys_info);
        self.cpu_selected_state.select(Some(0));
        while !self.is_quit {
            let c_sys_info = self.rx.try_recv();
            if c_sys_info.is_ok() {
                process_sys_info(&mut self.sys_info, c_sys_info.unwrap());
            }
            let _ = terminal.draw(|frame| self.draw(frame, &app_color_info));

            // we only handle event if the tui is renderable
            if self.is_renderable {
                self.handle_events();
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame, app_color_info: &AppColorInfo) {
        let top_and_bottom =
            Layout::vertical([Constraint::Percentage(40), Constraint::Percentage(60)]);
        // let down_left_and_right = Layout::horizontal([Constraint::Percentage(45), Constraint::Percentage(55)]);
        let [top, _] = top_and_bottom.areas(frame.area());
        // let [bottom_left, bottom_right] = down_left_and_right.areas(bottom);

        let background =
            Block::default().style(Style::default().bg(app_color_info.background_color)); // Set your desired background color
        frame.render_widget(background, frame.area());

        let view_rect = frame.area();
        if view_rect.width < MIN_WIDTH || view_rect.height < MIN_HEIGHT {
            self.is_renderable = false;
            draw_not_renderable_message(frame, app_color_info);
            return;
        } else {
            self.is_renderable = true;
        }

        if self.is_renderable {
            draw_cpu_info(
                self.tick as u64,
                &self.sys_info.cpus,
                top,
                frame,
                &mut self.cpu_selected_state,
                self.cpu_graph_shown_range,
                if self.selected_container == SelectedContainer::Cpu {
                    true
                } else {
                    false
                },
                app_color_info,
            );
        }
    }

    fn handle_events(&mut self) {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            match event::read().unwrap() {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                // quit the ratatui terminal user interface
                if self.selected_container == SelectedContainer::None {
                    self.is_quit = true;
                } else {
                    self.selected_container = SelectedContainer::None;
                }
            }
            KeyCode::Char('-') => {
                if self.state == AppState::View {
                    if self.tick > 100 {
                        self.tick -= 100;
                        self.tick_tx.send(self.tick).unwrap();
                    }
                }
            }
            KeyCode::Char('+') => {
                if self.state == AppState::View {
                    if self.tick < 10000 {
                        self.tick += 100;
                        self.tick_tx.send(self.tick).unwrap();
                    }
                }
            }
            KeyCode::Up => {
                if self.selected_container == SelectedContainer::Cpu {
                    if let Some(selected) = self.cpu_selected_state.selected() {
                        if selected > 0 {
                            self.cpu_selected_state.select(Some(selected - 1));
                        } else {
                            self.cpu_selected_state
                                .select(Some(self.sys_info.cpus.len() - 1))
                        }
                    }
                }
            }
            KeyCode::Down => {
                if self.selected_container == SelectedContainer::Cpu {
                    if let Some(selected) = self.cpu_selected_state.selected() {
                        if selected < self.sys_info.cpus.len().saturating_sub(1) {
                            self.cpu_selected_state.select(Some(selected + 1));
                        } else {
                            self.cpu_selected_state.select(Some(0))
                        }
                    }
                }
            }
            KeyCode::Left => {
                if self.selected_container == SelectedContainer::Cpu {
                    if self.cpu_graph_shown_range > 100 {
                        self.cpu_graph_shown_range -= 10;
                    }
                }
            }
            KeyCode::Right => {
                if self.selected_container == SelectedContainer::Cpu {
                    if self.cpu_graph_shown_range < 10000 {
                        self.cpu_graph_shown_range += 10;
                    }
                }
            }
            KeyCode::Char('c') => {
                if self.state == AppState::View {
                    if self.selected_container == SelectedContainer::None {
                        self.selected_container = SelectedContainer::Cpu;
                    } else {
                        self.selected_container = SelectedContainer::None;
                    }
                }
            }
            KeyCode::Char('C') => {
                if self.state == AppState::View {
                    if self.selected_container == SelectedContainer::None {
                        self.selected_container = SelectedContainer::Cpu;
                    } else {
                        self.selected_container = SelectedContainer::None;
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn process_sys_info(current_sys_info: &mut SysInfo, collected_sys_info: CSysInfo) {
    // process for each cpu
    let mut total_usage = 0.0;
    let total_cpu_cores = collected_sys_info.cpus.len() as f32;
    let mut brand = String::new();
    if current_sys_info.cpus.len() == 0 {
        for cpu in collected_sys_info.cpus.iter() {
            total_usage += cpu.usage;
            brand = cpu.brand.clone();
            let cpu = CpuData::new(cpu.id as i8, cpu.brand.clone(), cpu.usage);
            current_sys_info.cpus.push(cpu);
        }
        let cpu_avg = ((total_usage / total_cpu_cores) * 100.0).round() / 100.0;
        let cpu = CpuData::new(-1, brand.clone(), cpu_avg);
        current_sys_info.cpus.insert(0, cpu);
    } else {
        for cpu in collected_sys_info.cpus.iter() {
            total_usage += cpu.usage;
            current_sys_info.cpus[cpu.id as usize + 1].update(cpu.id as i8, cpu.usage);
        }
        let cpu_avg = ((total_usage / total_cpu_cores) * 100.0).round() / 100.0;
        current_sys_info.cpus[0].update(-1, cpu_avg);
    }

    drop(collected_sys_info);
}

fn draw_not_renderable_message(frame: &mut Frame, app_color_info: &AppColorInfo) {
    let block = Block::bordered()
        .style(Color::LightYellow)
        .border_set(border::ROUNDED);

    let view_rect = frame.area();
    let height = view_rect.height;
    let width = view_rect.width;

    // Define multiple paragraphs
    let text_lines = vec![
        Line::from("UI can't be rendered, terminal size too small")
            .style(app_color_info.text_color),
        Line::from(vec![
            Span::styled("Width =", Style::default().fg(app_color_info.text_color)),
            Span::styled(
                format!(" {} ", width),
                Style::default().fg(if width >= MIN_WIDTH {
                    Color::Green
                } else {
                    Color::Red
                }),
            ),
            Span::styled("Height =", Style::default().fg(app_color_info.text_color)),
            Span::styled(
                format!(" {} ", height),
                Style::default().fg(if height >= MIN_HEIGHT {
                    Color::Green
                } else {
                    Color::Red
                }),
            ),
        ]),
        Line::from(""),
        Line::from("Need Size for current config.").style(app_color_info.text_color),
        Line::from(format!("Width = {} Height = {}  ", MIN_WIDTH, MIN_HEIGHT))
            .style(app_color_info.text_color),
    ];

    let warning_paragraph = Paragraph::new(text_lines)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(warning_paragraph, frame.area());
}
