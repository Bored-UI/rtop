use std::{
    collections::HashMap,
    sync::mpsc::{self, Receiver, Sender},
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
    components::network::draw_network_info,
    cpu::draw_cpu_info,
    disk::draw_disk_info,
    get_sys_info::spawn_system_info_collector,
    memory::draw_memory_info,
    types::{CSysInfo, MemoryData, SysInfo},
    utils::process_sys_info,
};

#[derive(PartialEq)]
pub enum SelectedContainer {
    Cpu,
    Memory,
    Disk,
    Network,
    Process,
    Menu,
    None,
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
    memory_graph_shown_range: usize,
    disk_graph_shown_range: usize,
    network_graph_shown_range: usize,
    cpu_selected_state: ListState,
    disk_selected_entry: usize,
    network_selected_entry: usize,
    is_renderable: bool,
    is_init: bool,
    container_full_screen: bool,
}

pub struct AppColorInfo {
    pub background_color: Color,
    pub base_app_text_color: Color,
    // key text was the key that triggers certain functionality, like c for selecting cpu container -/+ to chnage the refresh tick
    pub key_text_color: Color,
    pub app_title_color: Color, // this will be used for those text in the title of each main block

    // for cpu
    pub cpu_container_selected_color: Color,
    pub cpu_main_block_color: Color,
    pub cpu_selected_color: Color,
    pub cpu_base_graph_color: Color,
    pub cpu_info_border_color: Color,
    pub cpu_text_color: Color,

    // for memory
    pub memory_container_selected_color: Color,
    pub memory_main_block_color: Color,
    pub used_memory_base_graph_color: Color,
    pub available_memory_base_graph_color: Color,
    pub free_memory_base_graph_color: Color,
    pub cached_memory_base_graph_color: Color,
    pub swap_memory_base_graph_color: Color,
    pub memory_text_color: Color,

    // for disk
    pub disk_container_selected_color: Color,
    pub disk_main_block_color: Color,
    pub disk_bytes_written_base_graph_color: Color,
    pub disk_bytes_read_base_graph_color: Color,
    pub disk_text_color: Color,

    // for network
    pub network_container_selected_color: Color,
    pub network_main_block_color: Color,
    pub network_received_base_graph_color: Color,
    pub network_transmitted_base_graph_color: Color,
    pub network_text_color: Color,
}

const MIN_HEIGHT: u16 = 25;
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
        sys_info: SysInfo {
            cpus: vec![],
            memory: MemoryData::default(),
            disks: HashMap::new(),
            networks: HashMap::new(),
        },
        selected_container: SelectedContainer::None,
        state: AppState::View,
        cpu_graph_shown_range: 100,
        memory_graph_shown_range: 100,
        disk_graph_shown_range: 100,
        network_graph_shown_range: 100,
        cpu_selected_state: ListState::default(),
        disk_selected_entry: 0,
        network_selected_entry: 0,
        is_renderable: true,
        is_init: false,
        container_full_screen: false,
    };

    let app_color_info = AppColorInfo {
        // Background color: A dark grayish-blue for the entire terminal
        background_color: Color::Rgb(46, 52, 64), // Polar Knight
        // Text color: A soft white for general text readability
        base_app_text_color: Color::Rgb(216, 222, 233), // Snow Storm
        // Key text color: A bright magenta for key text (e.g., "C" in "Cpu")
        key_text_color: Color::Rgb(94, 129, 172), // Bright magenta
        app_title_color: Color::Rgb(143, 188, 187), // Frost

        cpu_container_selected_color: Color::Rgb(94, 129, 172),
        // CPU main block: A slightly lighter grayish-blue to contrast with the background
        cpu_main_block_color: Color::Rgb(76, 86, 106),
        // CPU selected color: A bright teal for selected CPU items in the list
        cpu_selected_color: Color::Rgb(94, 129, 172),
        // CPU graph color: A muted blue to represent graph lines
        cpu_base_graph_color: Color::Rgb(70, 130, 180), // Steel blue
        // CPU info border color: A subtle silver for borders
        cpu_info_border_color: Color::Rgb(150, 150, 150), // Silver
        cpu_text_color: Color::Rgb(94, 129, 172),         // color for cpu related text

        memory_container_selected_color: Color::Rgb(94, 129, 172),
        // Memory main block: A slightly lighter grayish-blue to contrast with the background
        memory_main_block_color: Color::Rgb(76, 86, 106),
        // Memory related graph color
        used_memory_base_graph_color: Color::Rgb(180, 80, 80), // Muted reddish coral
        available_memory_base_graph_color: Color::Rgb(80, 160, 160), // Muted teal
        free_memory_base_graph_color: Color::Rgb(80, 180, 80), // Muted green
        cached_memory_base_graph_color: Color::Rgb(120, 100, 180), // Muted purple-blue
        swap_memory_base_graph_color: Color::Rgb(180, 140, 60), // Muted golden orange
        memory_text_color: Color::Rgb(143, 188, 187),          // color for memory related text

        disk_container_selected_color: Color::Rgb(94, 129, 172),
        // Disk main block: A slightly lighter grayish-blue to contrast with the background
        disk_main_block_color: Color::Rgb(76, 86, 106),
        // Disk selected color: A bright teal for selected Memory items in the list
        disk_bytes_written_base_graph_color: Color::Rgb(180, 80, 80), // Muted reddish coral
        disk_bytes_read_base_graph_color: Color::Rgb(80, 160, 160),   // Muted teal
        disk_text_color: Color::Rgb(143, 188, 187), //  color for disk related text

        network_container_selected_color: Color::Rgb(94, 129, 172),
        // Network main block: A slightly lighter grayish-blue to contrast with the background
        network_main_block_color: Color::Rgb(76, 86, 106),
        // Network selected color: A bright teal for selected Memory items in the list
        network_received_base_graph_color: Color::Rgb(180, 80, 80), // Muted reddish coral
        network_transmitted_base_graph_color: Color::Rgb(80, 160, 160), // Muted teal
        network_text_color: Color::Rgb(143, 188, 187), //  color for network related text
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
        // when the program start, we let the info collector to collect at 100ms
        // only after the initial collection, we reset to the user selected tick ( this will be able to be configure at a later stage )
        spawn_system_info_collector(tick_rx, self.tx.clone(), 100);

        while !self.is_init {
            match self.rx.try_recv() {
                Ok(c_sys_info) => {
                    process_sys_info(&mut self.sys_info, c_sys_info);
                    self.is_init = true;
                }
                Err(_) => {}
            }
        }
        self.cpu_selected_state.select(Some(0));

        let _ = self.tick_tx.send(self.tick);

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
        //
        //                       The TUI Layout
        //
        //   ------------------------------------------------------------
        //   |                                                          |
        //   |                  CPU INFO (top 30.0%)                    |
        //   |                                                          |
        //   ------------------------------------------------------------
        //   |   (MEMORY AND DIKS)     |                                |
        //   |    Bottom left (45%)    |   (PROCESS bottom right 55%)   |
        //   |      & top (65%)        |                                |
        //   |--------------------(BOTTOM 70%)                          |
        //   |      (NETWORK)          |                                |
        //   |    Bottom left (45%)    |                                |
        //   |     & bottom (35%)      |                                |
        //   ------------------------------------------------------------

        let top_and_bottom =
            Layout::vertical([Constraint::Percentage(30), Constraint::Percentage(70)]);
        let [cpu_area, bottom] = top_and_bottom.areas(frame.area());
        let [bottom_left, process_area] =
            Layout::horizontal([Constraint::Percentage(45), Constraint::Percentage(55)])
                .areas(bottom);
        let [memory_disk_area, network_area] =
            Layout::vertical([Constraint::Percentage(65), Constraint::Percentage(35)])
                .areas(bottom_left);
        let [memory_area, disk_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(memory_disk_area);

        // set the bg
        let background =
            Block::default().style(Style::default().bg(app_color_info.background_color)); // Set your desired background color
        frame.render_widget(background, frame.area());

        // check if the terminal size is valid
        let full_frame_view_rect = frame.area();
        if full_frame_view_rect.width < MIN_WIDTH || full_frame_view_rect.height < MIN_HEIGHT {
            self.is_renderable = false;
            draw_not_renderable_message(frame, app_color_info);
            return;
        } else {
            self.is_renderable = true;
        }

        if self.is_renderable {
            // we check the selcted disk entry to prevent selecting a disk that got removed
            //
            // default to the first disk entry
            let mut selected_disk = self.sys_info.disks.iter().nth(0).unwrap().1;
            // if the selected disk is valid, override the selected default disk
            if let Some((_, value)) = self.sys_info.disks.iter().nth(self.disk_selected_entry) {
                selected_disk = value;
            } else {
                self.disk_selected_entry = 0;
            }

            // default to the first network entry
            let mut selected_network = self.sys_info.networks.iter().nth(0).unwrap().1;
            // if the selected network is valid, override the selected default network
            if let Some((_, value)) = self
                .sys_info
                .networks
                .iter()
                .nth(self.network_selected_entry)
            {
                selected_network = value;
            } else {
                self.network_selected_entry = 0;
            }

            // handling for full screen mode
            if self.container_full_screen {
                if self.selected_container == SelectedContainer::Cpu {
                    draw_cpu_info(
                        self.tick as u64,
                        &self.sys_info.cpus,
                        full_frame_view_rect,
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
                } else if self.selected_container == SelectedContainer::Memory {
                    draw_memory_info(
                        self.tick as u64,
                        &self.sys_info.memory,
                        full_frame_view_rect,
                        frame,
                        self.memory_graph_shown_range,
                        if self.selected_container == SelectedContainer::Memory {
                            true
                        } else {
                            false
                        },
                        app_color_info,
                        true,
                    )
                } else if self.selected_container == SelectedContainer::Disk {
                    draw_disk_info(
                        self.tick as u64,
                        &selected_disk,
                        full_frame_view_rect,
                        frame,
                        self.disk_graph_shown_range,
                        if self.selected_container == SelectedContainer::Disk {
                            true
                        } else {
                            false
                        },
                        app_color_info,
                        true,
                    )
                } else if self.selected_container == SelectedContainer::Network {
                    draw_network_info(
                        self.tick as u64,
                        &selected_network,
                        full_frame_view_rect,
                        frame,
                        self.network_graph_shown_range,
                        if self.selected_container == SelectedContainer::Network {
                            true
                        } else {
                            false
                        },
                        app_color_info,
                        true,
                    )
                }
            } else {
                draw_cpu_info(
                    self.tick as u64,
                    &self.sys_info.cpus,
                    cpu_area,
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

                draw_memory_info(
                    self.tick as u64,
                    &self.sys_info.memory,
                    memory_area,
                    frame,
                    self.memory_graph_shown_range,
                    if self.selected_container == SelectedContainer::Memory {
                        true
                    } else {
                        false
                    },
                    app_color_info,
                    false,
                );

                draw_disk_info(
                    self.tick as u64,
                    &selected_disk,
                    disk_area,
                    frame,
                    self.disk_graph_shown_range,
                    if self.selected_container == SelectedContainer::Disk {
                        true
                    } else {
                        false
                    },
                    app_color_info,
                    false,
                );

                draw_network_info(
                    self.tick as u64,
                    &selected_network,
                    network_area,
                    frame,
                    self.network_graph_shown_range,
                    if self.selected_container == SelectedContainer::Network {
                        true
                    } else {
                        false
                    },
                    app_color_info,
                    false,
                );
            }
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
                    if self.container_full_screen {
                        self.container_full_screen = false;
                    } else {
                        self.selected_container = SelectedContainer::None;
                    }
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
                } else if self.selected_container == SelectedContainer::Memory {
                    if self.memory_graph_shown_range > 100 {
                        self.memory_graph_shown_range -= 10;
                    }
                } else if self.selected_container == SelectedContainer::Disk {
                    if self.disk_graph_shown_range > 100 {
                        self.disk_graph_shown_range -= 10;
                    }
                }
            }
            KeyCode::Right => {
                if self.selected_container == SelectedContainer::Cpu {
                    if self.cpu_graph_shown_range < 10000 {
                        self.cpu_graph_shown_range += 10;
                    }
                } else if self.selected_container == SelectedContainer::Memory {
                    if self.memory_graph_shown_range < 10000 {
                        self.memory_graph_shown_range += 10;
                    }
                } else if self.selected_container == SelectedContainer::Disk {
                    if self.disk_graph_shown_range < 10000 {
                        self.disk_graph_shown_range += 10;
                    }
                }
            }

            // c and C for selecting the Cpu Block
            KeyCode::Char('c') => {
                if self.state == AppState::View {
                    if self.selected_container == SelectedContainer::None
                        || self.selected_container != SelectedContainer::Cpu
                    {
                        self.selected_container = SelectedContainer::Cpu;
                    } else {
                        self.container_full_screen = false;
                        self.selected_container = SelectedContainer::None;
                    }
                }
            }
            KeyCode::Char('C') => {
                if self.state == AppState::View {
                    if self.selected_container == SelectedContainer::None
                        || self.selected_container != SelectedContainer::Cpu
                    {
                        self.selected_container = SelectedContainer::Cpu;
                    } else {
                        self.container_full_screen = false;
                        self.selected_container = SelectedContainer::None;
                    }
                }
            }

            // m and M for selecting the Memory Block
            KeyCode::Char('m') => {
                if self.state == AppState::View {
                    if self.selected_container == SelectedContainer::None
                        || self.selected_container != SelectedContainer::Memory
                    {
                        self.selected_container = SelectedContainer::Memory;
                    } else {
                        self.container_full_screen = false;
                        self.selected_container = SelectedContainer::None;
                    }
                }
            }
            KeyCode::Char('M') => {
                if self.state == AppState::View {
                    if self.selected_container == SelectedContainer::None
                        || self.selected_container != SelectedContainer::Memory
                    {
                        self.selected_container = SelectedContainer::Memory;
                    } else {
                        self.container_full_screen = false;
                        self.selected_container = SelectedContainer::None;
                    }
                }
            }

            // d and D for selecting the Disk Block
            KeyCode::Char('d') => {
                if self.state == AppState::View {
                    if self.selected_container == SelectedContainer::None
                        || self.selected_container != SelectedContainer::Disk
                    {
                        self.selected_container = SelectedContainer::Disk;
                    } else {
                        self.container_full_screen = false;
                        self.selected_container = SelectedContainer::None;
                    }
                }
            }
            KeyCode::Char('D') => {
                if self.state == AppState::View {
                    if self.selected_container == SelectedContainer::None
                        || self.selected_container != SelectedContainer::Disk
                    {
                        self.selected_container = SelectedContainer::Disk;
                    } else {
                        self.container_full_screen = false;
                        self.selected_container = SelectedContainer::None;
                    }
                }
            }

            // n and N for selecting the Disk Block
            KeyCode::Char('n') => {
                if self.state == AppState::View {
                    if self.selected_container == SelectedContainer::None
                        || self.selected_container != SelectedContainer::Network
                    {
                        self.selected_container = SelectedContainer::Network;
                    } else {
                        self.container_full_screen = false;
                        self.selected_container = SelectedContainer::None;
                    }
                }
            }
            KeyCode::Char('N') => {
                if self.state == AppState::View {
                    if self.selected_container == SelectedContainer::None
                        || self.selected_container != SelectedContainer::Network
                    {
                        self.selected_container = SelectedContainer::Network;
                    } else {
                        self.container_full_screen = false;
                        self.selected_container = SelectedContainer::None;
                    }
                }
            }

            KeyCode::Char('<') => {
                if self.state == AppState::View {
                    if self.selected_container == SelectedContainer::Disk {
                        if self.disk_selected_entry == 0 {
                            self.disk_selected_entry = self.sys_info.disks.len() - 1;
                        } else {
                            self.disk_selected_entry -= 1;
                        }
                    } else if self.selected_container == SelectedContainer::Network {
                        if self.network_selected_entry == 0 {
                            self.network_selected_entry = self.sys_info.networks.len() - 1;
                        } else {
                            self.network_selected_entry -= 1;
                        }
                    }
                }
            }
            KeyCode::Char('>') => {
                if self.state == AppState::View {
                    if self.selected_container == SelectedContainer::Disk {
                        if self.disk_selected_entry == self.sys_info.disks.len() - 1 {
                            self.disk_selected_entry = 0
                        } else {
                            self.disk_selected_entry += 1;
                        }
                    } else if self.selected_container == SelectedContainer::Network {
                        if self.network_selected_entry == self.sys_info.networks.len() - 1 {
                            self.network_selected_entry = 0;
                        } else {
                            self.network_selected_entry += 1;
                        }
                    }
                }
            }

            KeyCode::Tab => {
                // for a container to be full screen, it need to be selected first
                if self.container_full_screen && self.selected_container != SelectedContainer::None
                {
                    self.container_full_screen = false;
                } else if !self.container_full_screen
                    && self.selected_container != SelectedContainer::None
                {
                    self.container_full_screen = true;
                }
            }
            _ => {}
        }
    }
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
            .style(app_color_info.base_app_text_color),
        Line::from(vec![
            Span::styled(
                "Width =",
                Style::default().fg(app_color_info.base_app_text_color),
            ),
            Span::styled(
                format!(" {} ", width),
                Style::default().fg(if width >= MIN_WIDTH {
                    Color::Green
                } else {
                    Color::Red
                }),
            ),
            Span::styled(
                "Height =",
                Style::default().fg(app_color_info.base_app_text_color),
            ),
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
        Line::from("Need Size for current config.").style(app_color_info.base_app_text_color),
        Line::from(format!("Width = {} Height = {}  ", MIN_WIDTH, MIN_HEIGHT))
            .style(app_color_info.base_app_text_color),
    ];

    let warning_paragraph = Paragraph::new(text_lines)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(warning_paragraph, frame.area());
}
