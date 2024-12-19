use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Stylize, Color, Style, Modifier},
    symbols::{self,border},
    text::{Line, Text, Span},
    widgets::{Block, List, ListItem, Paragraph, Widget, Tabs, Clear, Gauge, Padding, BorderType, Scrollbar, ScrollbarOrientation, Dataset, Chart, Axis, canvas::{Canvas, Map, MapResolution},},
};
use crate::app::{App, AppState, SetupState};
use crate::tasks::JobState;
use crate::config::Settings;
use sysinfo::System;


const CUSTOM_LABEL_COLOR: Color = Color::White;
const GAUGE2_COLOR: Style = Style::new().fg(Color::Rgb(10, 100, 100));

// Function to check if the terminal area is sufficient for rendering
fn is_area_sufficient(area: Rect, min_width: u16, min_height: u16) -> bool {
    area.width >= min_width && area.height >= min_height
}

pub fn draw(frame: &mut ratatui::Frame, app: &mut App) {
    app.update_jobs_status();

    // Check if terminal is too small
    if !check_terminal_size(frame) {
        return;
    }

    // Proceed with rendering if size is sufficient
    let main_chunks = create_main_layout(frame);
    draw_tabs(frame, app, main_chunks[0]);

    match app.current_tab {
        0 => draw_network_tab(frame, app, main_chunks[1]),
        1 => draw_manuscripts_tab(frame, app, main_chunks[1]),
        2 => draw_avs_tab(frame, app, main_chunks[1]),
        _ => unreachable!(),
    }

    draw_status_bar(frame);
    draw_popups(frame, app);

    if app.job_logs.is_some() {
        draw_job_logs_popup(frame, app);
    }
}

fn draw_popups(frame: &mut ratatui::Frame, app: &App) {
    if app.show_sql_window {
        draw_sql_popup(frame, app);
    }
    if app.show_search {
        draw_search_popup(frame, app);
    }
    if app.show_deploy_options {
        draw_deploy_options_popup(frame, app);
    }
    if app.show_job_options {
        draw_job_options_popup(frame, app);
    }
    if app.show_help {
        draw_help_popup(frame);
    }
    if app.show_warning {
        draw_warning_popup(frame);
    }
}

fn check_terminal_size(frame: &mut ratatui::Frame) -> bool {
    let size = frame.size();
    const MIN_WIDTH: u16 = 80;
    const MIN_HEIGHT: u16 = 28;

    if size.width < MIN_WIDTH || size.height < MIN_HEIGHT {
        // Render a warning message if the terminal is too small
        let warning = Paragraph::new("Terminal too small. Please resize to at least 80x28.")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Red));
        frame.render_widget(warning, size);
        return false;
    }
    true
}

fn draw_manuscripts_tab(frame: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    draw_jobs_list(frame, app, chunks[0]);
    draw_sql_editor(frame, app, chunks[1]);
}

fn draw_sql_popup(frame: &mut ratatui::Frame, app: &App) {
            // When SQL window is shown, render the input window
            let area = frame.area();
            let sql_window_width = (area.width as f32 * 0.8) as u16;
            let sql_window_height = (area.height as f32 * 0.6) as u16;
            let sql_window = Rect::new(
                (area.width - sql_window_width) / 2,
                (area.height - sql_window_height) / 2,
                sql_window_width,
                sql_window_height,
            );

            frame.render_widget(Clear, sql_window);

            let input_block = Block::bordered()
                .title(" Manuscript Editor (Esc → Save & Esc) ")
                .title_alignment(Alignment::Center)
                .border_set(border::THICK)
                .style(Style::default().bg(Color::Rgb(10, 100, 100)))
                .title_style(Style::default()
                    .fg(Color::Yellow)
                    .bold()
                    .add_modifier(Modifier::UNDERLINED | Modifier::ITALIC));

            let mut styled_text = Text::default();
            let input = app.sql_input.as_str();
            let lines: Vec<&str> = input.split('\n').collect();
            let mut current_pos = 0;

            for line in lines {
                let line_length = line.len() + 1; // Add 1 for newline
                let cursor_in_this_line = app.sql_cursor_position >= current_pos 
                    && app.sql_cursor_position <= current_pos + line_length - 1;
                
                if cursor_in_this_line {
                    let line_cursor_pos = app.sql_cursor_position - current_pos;
                    
                    let mut spans = Vec::new();
                    if line_cursor_pos > 0 {
                        spans.push(Span::raw(&line[..line_cursor_pos]));
                    }
                    
                    if line_cursor_pos < line.len() {
                        spans.push(Span::styled(
                            &line[line_cursor_pos..line_cursor_pos+1],
                            Style::default().bg(Color::White).fg(Color::Black)
                        ));
                        if line_cursor_pos + 1 < line.len() {
                            spans.push(Span::raw(&line[line_cursor_pos+1..]));
                        }
                    } else {
                        spans.push(Span::styled(
                            " ",
                            Style::default().bg(Color::White)
                        ));
                    }
                    
                    styled_text.extend(Text::from(Line::from(spans)));
                } else {
                    styled_text.extend(Text::from(Line::from(line.to_string())));
                }
                
                current_pos += line_length;
            }

            let sql_paragraph = Paragraph::new(styled_text)
                .block(input_block)
                .style(Style::default().fg(Color::White));

            frame.render_widget(sql_paragraph, sql_window);

            if let Some(result) = &app.sql_result {
                let result_text = Paragraph::new(result.as_str())
                    .style(Style::default().fg(Color::Green));
                
                let result_window = Rect::new(
                    sql_window.x,
                    sql_window.y + sql_window.height,
                    sql_window.width,
                    3,
                );
                
                frame.render_widget(Clear, result_window);
                frame.render_widget(result_text, result_window);
            }
}

fn draw_status_bar(frame: &mut ratatui::Frame) {
    let blocks = "▃ ▅ ▇";
    
    // Get system memory info
    let sys = System::new_all();
    let used_mem = sys.used_memory() / 1024;
    let total_mem = sys.total_memory() / 1024;
    let mem_percent = (used_mem as f64 / total_mem as f64 * 100.0) as u64;
    
    let cpu_percent = sys.global_cpu_usage() as u64;
    
    let text = vec![
        Span::styled(Settings::get_status_text(), Style::default().bold()),
        Span::raw(" | "),
        Span::styled(format!("CPU: {}%", cpu_percent), Style::default().fg(Color::Yellow)),
        Span::raw(" | "),
        Span::styled(format!("MEM: {}%", mem_percent), Style::default().fg(Color::Cyan)),
        Span::raw(" | "),
        Span::styled(blocks, Style::default().fg(Color::Green))
    ];
    
    let status_text = Paragraph::new(Line::from(text)).alignment(Alignment::Right);
    frame.render_widget(
        status_text,
        Rect::new(frame.area().width - 87, 1, 85, 1),
    );
}

fn draw_network_tab(frame: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    draw_chain_list(frame, app, chunks[0]);
    draw_chain_details(frame, app, chunks[1]);
}

fn draw_chain_list(frame: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let left_chunks = if app.show_tables {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(45),
                Constraint::Percentage(45),
                Constraint::Percentage(10),
            ])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ])
            .split(area)
    };

    draw_chains_block(frame, app, left_chunks[0]);
    
    if app.show_tables {
        draw_tables_block(frame, app, left_chunks[1]);
    }
    
    draw_network_monitoring(frame, app, left_chunks[left_chunks.len()-1]);
}

fn draw_chains_block(frame: &mut ratatui::Frame, app: &App, area: Rect) {
    let visible_height = area.height as usize - 2;
    let chains_block = Block::bordered()
        .border_set(border::THICK)
        .title_top(" Omnichain ")
        .title_alignment(Alignment::Center);

    let chain_names: Vec<ListItem> = app.filtered_chains
        .iter()
        .skip(app.scroll_offset)
        .take(visible_height)
        .enumerate()
        .map(|(i, chain)| create_chain_list_item(app, chain, i))
        .collect();

    let chain_list = List::new(chain_names).block(chains_block);
    frame.render_widget(chain_list, area);
}

fn draw_tables_block(frame: &mut ratatui::Frame, app: &App, area: Rect) {
    if let Some(selected_chain) = app.chains.get(app.selected_chain_index) {
        let table_names: Vec<ListItem> = selected_chain.dataDictionary
            .iter()
            .enumerate()
            .map(|(i, (table_name, _))| {
                let content = if Some(i) == app.selected_table_index {
                    Line::from(vec![
                        format!("{:<1}-> ", i + 1).bold().white(),
                        format!("{}.{}", selected_chain.databaseName, table_name).bold().white()
                    ])
                } else {
                    Line::from(vec![
                        format!("{:<1}-", i + 1).green(),
                        format!("{}.{}", selected_chain.databaseName, table_name).green()
                    ])
                };
                let style = if Some(i) == app.selected_table_index {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                };
                ListItem::new(content).style(style)
            })
            .collect();

        let tables_block = Block::bordered()
            .title(format!(" {} Tables ", selected_chain.name))
            .title_alignment(Alignment::Center)
            .border_set(border::THICK);

        let table_list = List::new(table_names).block(tables_block);
        frame.render_widget(table_list, area);
    }
}

fn draw_network_monitoring(frame: &mut ratatui::Frame, app: &App, area: Rect) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let window_size = area.width.min(100) as usize;
    
    let mut history = vec![' '; window_size];
    let mut compute_history = vec![' '; window_size];
    
    for i in 0..window_size {
        let past_time = now - (window_size - i - 1) as u64;
        let rand_val = (past_time * i as u64) % 100;
        let compute_val = (past_time * (i + 1) as u64) % 100;
        history[i] = if rand_val < 30 { '⣷' } else { '⣤' };
        compute_history[i] = if compute_val < 1 { '⣴' } else { '⣤' };
    }

    let cpu_power = app.network_status.cpu
        .trim_end_matches('%')
        .parse::<f64>()
        .unwrap_or(0.0);

    let traffic_line = "╌".repeat(area.width as usize - 22);
    let compute_line = "╌".repeat(area.width as usize - 22);
    let cpu_line = "╌".repeat(area.width as usize - 20);
    let storage_line = "╌".repeat(area.width as usize - 18);

    let hints_text = Text::from(vec![
        Line::from(vec![
            Span::styled("Net Traffic:", Style::default().fg(Color::White)),
            Span::styled(traffic_line, Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" {}", app.network_status.net), Style::default().fg(Color::Green)),
        ]),
        Line::from(history.iter().collect::<String>()).fg(Color::Rgb(5, 170, 171)),
        Line::from("⣿".repeat(window_size)).fg(Color::Rgb(5, 170, 171)),
        Line::from("⣿".repeat(window_size)).fg(Color::Rgb(2, 85, 85)),
        Line::default(),

        Line::from(vec![
            Span::styled("Pro Threads:", Style::default().fg(Color::White)),
            Span::styled(compute_line, Style::default().fg(Color::DarkGray)), 
            Span::styled(format!(" {}", app.network_status.thread), Style::default().fg(Color::Green)),
        ]),
        Line::from(compute_history.iter().collect::<String>()).fg(Color::Rgb(231, 186, 80)),
        Line::from("⣿".repeat(window_size)).fg(Color::Rgb(115, 93, 40)),

        Line::default(),
        Line::from(vec![
            Span::styled("Tol Power:", Style::default().fg(Color::White)),
            Span::styled(cpu_line, Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" {}%", app.network_status.cpu), Style::default().fg(Color::Green)),
        ]),
        Line::from({
            let mut spans = Vec::new();
            for i in 0..window_size {
                let progress = i as f64 / 9 as f64;
                let r = (0x10 as f64 + ((0x20 - 0x10) as f64 * progress)) as u8;
                let g = (0x40 as f64 + ((0x60 - 0x40) as f64 * progress)) as u8;
                let b = (0x10 as f64 + ((0x20 - 0x10) as f64 * progress)) as u8;
                if i as f64 / window_size as f64 > cpu_power / 100.0 {
                    spans.push(Span::styled("■", Style::default().fg(Color::Rgb(32, 32, 32))));
                } else {
                    spans.push(Span::styled("■", Style::default().fg(Color::Rgb(r, g, b))));
                }
            }
            Line::from(spans)
        }),

        Line::default(),
        Line::from(vec![
            Span::styled("Storage:", Style::default().fg(Color::White)),
            Span::styled(storage_line, Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" {}PB ", app.network_status.storage), Style::default().fg(Color::Green)),
        ]),
        Line::from({
            let mut spans = Vec::new();
            for i in 0..window_size {
                let progress = i as f64 / 9 as f64;
                let r = (0x10 as f64 + ((0x20 - 0x10) as f64 * progress)) as u8;
                let g = (0x40 as f64 + ((0x60 - 0x40) as f64 * progress)) as u8;
                let b = (0x10 as f64 + ((0x20 - 0x10) as f64 * progress)) as u8;
                if i as f64 / window_size as f64 > app.network_status.storage.parse::<f64>().unwrap_or(0.0) / 100.0 {
                    spans.push(Span::styled("■", Style::default().fg(Color::Rgb(32, 32, 32))));
                } else {
                    spans.push(Span::styled("■", Style::default().fg(Color::Rgb(r, g, b))));
                }
            }
            Line::from(spans)
        }),

    ]);

    let hints_block = Block::bordered()
        .title(" Network Resources ")
        .title_alignment(Alignment::Center)
        .border_set(border::THICK);
    
    let hints_paragraph = Paragraph::new(hints_text)
        .block(hints_block)
        .alignment(Alignment::Left);
    
    frame.render_widget(hints_paragraph, area);
}

fn create_chain_list_item<'a>(app: &'a App, chain: &'a crate::app::Chain, index: usize) -> ListItem<'a> {
    let index = index + app.scroll_offset + 1; // Calculate the 1-based index
    let time_ago_style = if chain.time_ago.contains("min") && 
        chain.time_ago.as_str().trim_end_matches(" min").parse::<u64>().unwrap_or(0) > 10 {
        chain.time_ago.as_str().yellow()
    } else {
        let display_time = if chain.time_ago == "unknown" { "-" } else { &chain.time_ago };
        display_time.white()
    };
    // Get the current filtered index
    let is_selected = if let Some(current_filtered_index) = app
        .filtered_chains
        .iter()
        .position(|c| c.name == app.chains[app.selected_chain_index].name)
    {
        index - 1 == current_filtered_index
    } else {
        false
    };

    let content = if is_selected {
        Line::from(vec![
            format!("{:<3}⟠ {:<25}", index, chain.name).bold().white().bg(Color::DarkGray).into(),
            format!("{:<10}", chain.ticker).bold().white().bg(Color::DarkGray).into(),
            format!("{:<10}", if chain.status == "Online" { format!("↿⇂{}", chain.net) } else if chain.status == "Offline" { "↿⇂".to_string() } else { "◑".to_string() }).bold()
                .style(if chain.status == "Online" && chain.time_ago.contains("min") { 
                    Style::default().fg(Color::Green).bg(Color::DarkGray)
                } else if chain.status == "Offline" {
                    Style::default().fg(Color::Red).bg(Color::DarkGray)
                } else { 
                    Style::default().fg(Color::Yellow).bg(Color::DarkGray)
                }).into(),
            format!("{:<10}", time_ago_style).bold().bg(Color::DarkGray).into(),
        ])
    } else {
        Line::from(vec![
            format!("{:<3}⟠ {:<25}", index, chain.name).bold()
                .style(if chain.status == "Online" && chain.time_ago.contains("min") { 
                    Style::default().fg(Color::Green)
                } else if chain.status == "Offline" {
                    Style::default().fg(Color::Red)
                } else { 
                    Style::default().fg(Color::Yellow) 
                }).into(),
            format!("{:<10}", chain.ticker).bold()
                .style(if chain.status == "Online" && chain.time_ago.contains("min") { 
                    Style::default().fg(Color::Green)
                } else if chain.status == "Offline" {
                    Style::default().fg(Color::Red)
                } else { 
                    Style::default().fg(Color::Yellow) 
                }).into(),
                format!("{:<10}", if chain.status == "Online" { format!("↿⇂{}", chain.net) } else if chain.status == "Offline" { "↿⇂".to_string() } else { "◑".to_string() }).bold()
                .style(if chain.status == "Online" && chain.time_ago.contains("min") { 
                    Style::default().fg(Color::Green)
                } else if chain.status == "Offline" {
                    Style::default().fg(Color::Red)
                } else { 
                    Style::default().fg(Color::Yellow) 
                }).into(),
            format!("{:<10}", time_ago_style).bold()
                .style(if chain.status == "Online" && chain.time_ago.contains("min") { 
                    Style::default().fg(Color::Green)
                } else if chain.status == "Offline" {
                    Style::default().fg(Color::Red)
                } else { 
                    Style::default().fg(Color::Yellow) 
                }).into(),
        ])
    };
    ListItem::new(content)
}

fn create_main_layout(frame: &mut ratatui::Frame) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(frame.area())
        .to_vec()
}

fn draw_tabs(frame: &mut ratatui::Frame, app: &App, area: Rect) {
    let titles = vec!["NETWORK [1]", "MANUSCRIPTS [2]", "ZONE [3]"];
    let tabs = Tabs::new(titles)
        .block(Block::bordered().title("Tabs"))
        .select(app.current_tab)
        .style(Style::default())
        .highlight_style(Style::default().bold());
    frame.render_widget(tabs, area);
}

fn draw_chain_details(frame: &mut ratatui::Frame, app: &mut App, area: Rect) {
    if let Some(selected_chain) = app.chains.get(app.selected_chain_index) {
        let data_lines = if app.show_tables && app.selected_table_index.is_some() {
            let table_name = selected_chain.dataDictionary
                .get(app.selected_table_index.unwrap())
                .map(|(name, _)| name.as_str())
                .unwrap_or("");

            let fields = selected_chain.dataDictionary
                .get(app.selected_table_index.unwrap())
                .map(|(_, items)| items);
            
            let mut lines = Vec::new();

            // Add chain name and overview
            lines.push(Line::from(selected_chain.name.clone().bold().magenta()));
            lines.push(Line::from(""));
            if let Some(overview) = &selected_chain.overview {
                lines.push(Line::from(vec![
                    "֎ ⋙ ".into(),
                    overview.clone().white().into()
                ]));
            }
            lines.push(Line::from(""));
            if !table_name.is_empty() {
                lines.push(Line::from(vec![
                    "Ⓣ ⋙ ".into(),
                    table_name.yellow().bold().into()
                ]));
            }
            
            // Add header
            lines.push(Line::from(vec![
                "Field Name".bold().white(),
                " | ".into(),
                "Data Type".bold().white(),
                " | ".into(),
                "Description".bold().white(),
            ]));
            lines.push(Line::from("─".repeat(80)));

            // Add field descriptions in table format
            if let Some(fields) = fields {
                lines.extend(fields.iter().map(|item| {
                    Line::from(vec![
                        format!("{:<20}", item.name).yellow().into(),
                        " | ".into(),
                        format!("{:<15}", item.dataType).cyan().into(),
                        " | ".into(),
                        item.dataType.clone().white().into(),
                    ])
                }));
            }

            // Add example data if available
            if let Some(example_data) = &app.example_data {
                lines.push(Line::from(""));
                lines.push(Line::from("Example Data:".bold().yellow()));
                
                // Add header
                lines.push(Line::from(vec![
                    "Column Name".bold().white(),
                    " | ".into(),
                    "Value".bold().white(),
                ]));
                lines.push(Line::from("─".repeat(80)));

                // Show first row of data as example
                if let Some(first_row) = example_data.data.first() {
                    for (i, value) in first_row.iter().enumerate() {
                        if let Some(column) = example_data.columns.get(i) {
                            lines.push(Line::from(vec![
                                format!("{:<30}", column.name).yellow().into(),
                                " | ".into(),
                                value.to_string().white().into(),
                            ]));
                        }
                    }
                }
            } else if selected_chain.status == "Offline" {
                lines.push(Line::from(""));
                lines.push(Line::from("No data available - Chain is currently offline".red().bold()));
            }

            lines
        } else {
            // Show welcome screen with logos and description
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(13),  // Top spacing
                    Constraint::Length(
                        TryInto::<u16>::try_into(LOGO.lines().count())
                            .unwrap_or_default()
                            .saturating_add(2),
                    ),
                    Constraint::Length(
                        TryInto::<u16>::try_into(LOGO_LETTER.lines().count())
                            .unwrap_or_default()
                            .saturating_add(2),
                    ),
                    Constraint::Length(3),  // Spacing between logos and description
                    Constraint::Fill(1),    // Description area
                    Constraint::Percentage(20), // Space for animated chart
                ])
                .split(area);

            // Render the symbol logo in cyan
            let logo = Paragraph::new(LOGO)
                .block(Block::default()
                    .padding(Padding::new(20, 20, 0, 0))
                    .style(Style::default().fg(Color::Rgb(200, 200, 200))))
                .style(Style::default().fg(Color::Cyan))
                .alignment(Alignment::Center);
            frame.render_widget(logo, layout[1]);

            // Render the text logo below in magenta
            let logo_letter = Paragraph::new(LOGO_LETTER)
                .block(Block::default()
                    .padding(Padding::new(20, 20, 0, 0))
                    .style(Style::default().fg(Color::Rgb(200, 200, 200))))
                .style(Style::default().fg(Color::Magenta))
                .alignment(Alignment::Center);
            frame.render_widget(logo_letter, layout[2]);

            // Add descriptive text below both logos
            let gray = Color::Rgb(80, 80, 100);
            let description = Text::from(vec![
                Line::from(vec![
                    "Welcome to ".white(),
                    "Manuscript".magenta().bold(),
                ]),
                Line::from(vec![
                    "Build The World's ".white(),
                    "Largest".green().bold(),
                    " Omnichain ".white(),
                    "Data Network".yellow().bold(),
                ]),
                Line::from(vec![
                    "Select a chain from the left panel to explore".fg(gray),
                ]),
                Line::from(""),
                Line::from(vec![
                    "GitHub: ".fg(Color::Rgb(200, 200, 200)),
                    "chainbase-labs/manuscript-core".fg(Color::Rgb(100, 200, 200)).bold(),
                ]),
            ]);

            let splash = Paragraph::new(description)
                .alignment(Alignment::Center);
            frame.render_widget(splash, layout[4]);

                    let datasets = vec![
                        Dataset::default()
                            .marker(symbols::Marker::Dot)
                            .style(Style::default().fg(Color::Cyan))
                            .data(&app.data1),
                        Dataset::default()
                            .marker(symbols::Marker::Braille)
                            .style(Style::default().fg(Color::Yellow))
                            .data(&app.data2),
                    ];

                    let chart = Chart::new(datasets)
                        .x_axis(
                            Axis::default()
                                .title("time line")
                                .style(Style::default().fg(Color::Gray))
                                .bounds(app.window),
                        )
                        .y_axis(
                            Axis::default()
                                .style(Style::default().fg(Color::Gray))
                                .bounds([-20.0, 20.0]),
                        );
                        frame.render_widget(chart, layout[5]);

            // Return empty vec since we're handling the rendering directly
            Vec::new()
        };

        let right_block = Block::bordered()
                .title(" Data Dictionary ")
                .title_alignment(Alignment::Center)
                .border_set(border::THICK);

            let data_paragraph = Paragraph::new(data_lines)
                .block(right_block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            frame.render_widget(data_paragraph, area);
    }
}

fn draw_jobs_list(frame: &mut ratatui::Frame, app: &mut App, area: Rect) {
    // Check if the area is too small to render the jobs list
    if !is_area_sufficient(area, 30, 10) {
        let warning = Paragraph::new("Insufficient space for jobs list.")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Red));
        frame.render_widget(warning, area);
        return;
    }

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(90),
            Constraint::Percentage(10),
        ])
        .split(area);

    // Left panel - Show jobs list
    let left_block = Block::bordered()
        .title(" Manuscript Jobs ")
        .title_alignment(Alignment::Center)
        .border_set(border::THICK);

    // Convert jobs into ListItems
    let job_list = app.jobs_status.iter().enumerate().map(|(index, job)| {
        let duration = job.containers.first()
            .map(|c| c.status.clone())
            .unwrap_or_default();

        let style = match job.status {
            JobState::Running => Style::default().fg(Color::Green),
            JobState::Pending => Style::default().fg(Color::Yellow),
            JobState::PullingImage => Style::default().fg(Color::Yellow),
            JobState::Failed => Style::default().fg(Color::Red),
            JobState::NotStarted => Style::default().fg(Color::Yellow),
            JobState::Creating => Style::default().fg(Color::Yellow),
        };

        let is_selected = index == app.selected_job_index;
        let content = Line::from(vec![
            Span::raw(format!("{:<3}", index + 1)),
            if is_selected {
                Span::styled("⟠ ", Style::default().fg(Color::White))
            } else {
                Span::raw("⟠ ")
            },
            Span::styled(
                format!("{:<10}", job.name),
                if is_selected {
                    style.add_modifier(Modifier::BOLD)
                } else {
                    style
                }
            ),
            Span::styled(
                format!("{:<10}", 
                    match job.status {
                        JobState::Running => "Running",
                        JobState::Pending => "Pulling Image...",
                        JobState::PullingImage => "Pulling Image...",
                        JobState::Failed => "Failed",
                        JobState::NotStarted => "Not Started (pull images while take few minutes..)",
                        JobState::Creating => "Creating (pull images while take few minutes..)",
                    }
                ),
                style
            ),
            Span::styled(format!("{}", duration), style),
        ]);

        if is_selected {
            ListItem::new(content).style(Style::default().bg(Color::DarkGray))
        } else {
            ListItem::new(content)
        }
    }).collect::<Vec<_>>();

    let jobs_list = List::new(job_list)
        .block(left_block)
        .highlight_style(Style::default().bg(Color::DarkGray));
    frame.render_widget(jobs_list, left_chunks[0]);

    // Add key hints at the bottom
    let hints = vec![
        "Enter: Actions",
        "↑/↓: Navigate",
        "q: Quit",
    ];
    let hints_text = Text::from(hints.join(" | "));
    let hints_block = Block::bordered()
        .title(" Controls ")
        .title_alignment(Alignment::Center)
        .border_set(border::THICK);
    let hints_paragraph = Paragraph::new(hints_text)
        .block(hints_block)
        .alignment(Alignment::Center);
    frame.render_widget(hints_paragraph, left_chunks[1]);
}

fn draw_sql_editor(frame: &mut ratatui::Frame, app: &mut App, area: Rect) {
    // Check if the area is too small to render the SQL editor
    if !is_area_sufficient(area, 40, 15) {
        let warning = Paragraph::new("Insufficient space for SQL editor.")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Red));
        frame.render_widget(warning, area);
        return;
    }

    let right_block = Block::bordered()
        .title(" SQL Editor ")
        .title_alignment(Alignment::Center)
        .border_set(border::THICK);

    if app.saved_manuscript.is_some() {
        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(55),
                Constraint::Percentage(45),
            ])
            .split(area);

        let mut sql_block = Block::bordered()
            .border_type(BorderType::Double)
            .title(" Manuscript Editor ")
            .title_alignment(Alignment::Center)
            .padding(Padding::new(1, 1, 0, 1))
            .title_bottom(Line::from(vec![
                "   Press ".white(),
                "R".green().bold(), 
                " to run, ".white(),
                "E".red().bold(),
                " to edit, ".white(), 
                "D".blue().bold(),
                " to deploy  ".white()
            ]).right_aligned());

        if !app.show_sql_window {
            sql_block = sql_block.style(Style::default().bg(Color::Rgb(10, 100, 100)));
        }

        let sql_paragraph = Paragraph::new(app.saved_manuscript.as_ref().unwrap().as_str())
            .block(sql_block)
            .alignment(Alignment::Left)
            .style(Style::default().fg(Color::White));
        frame.render_widget(sql_paragraph, right_chunks[0]);

        let console_block = Block::bordered()
            .title(" Debug Console ")
            .title_alignment(Alignment::Center)
            .border_set(border::THICK);

        let console_area = right_chunks[1];
        frame.render_widget(console_block.clone(), console_area);

        // Modify the console content rendering based on setup_state
        if app.setup_state == SetupState::Complete {
            // When complete, use the full console area for the message
            let inner_area = console_block.inner(console_area);
            let paragraph_msg = Paragraph::new(app.get_setup_progress_msg())
                .gray()
                .block(Block::default().padding(Padding::horizontal(4)))
                .scroll((app.vertical_scroll as u16, 0));
            frame.render_widget(paragraph_msg, inner_area);
        } else {
            // During setup, use the original divided layout
            let inner_area = console_block.inner(console_area);
            let gauge_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(0),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(2),  
                    Constraint::Length(6),
                    Constraint::Min(0),
                ])
                .split(inner_area);

            // 1. Progress gauge
            if app.state == AppState::Started {
                let label = Span::styled(
                    format!("{:.1}%", app.progress1),
                    Style::new().italic().bold().fg(CUSTOM_LABEL_COLOR),
                );
                let gauge = Gauge::default()
                    .block(Block::default().padding(Padding::horizontal(0)))
                    .gauge_style(GAUGE2_COLOR)
                    .ratio(app.progress1 / 100.0)
                    .label(label);
                frame.render_widget(gauge, gauge_chunks[1]);
            }

            // 2. Status
            let docker_status = if app.docker_setup_in_progress {
                format!("Docker setup in progress... ({} seconds)", app.docker_setup_timer / 10)
            } else {
                format!("🏄🏻 Manuscript console: Debug your manuscript before deploying it locally or to the network.")
            };

            let docker_status_widget = Paragraph::new(Text::from(
                Span::styled(docker_status, Style::default().fg(Color::Yellow))
            ))
            .alignment(Alignment::Center)
            .block(Block::default().padding(Padding::horizontal(1)));
            frame.render_widget(docker_status_widget, gauge_chunks[2]);

            // Add a tips message
            let tips_message = "📣 If you are using substreams with solana, it will automatically synchronize all tables.";
            let tips_widget = Paragraph::new(Text::from(
                Span::styled(tips_message, Style::default().fg(Color::Yellow))
            ))
            .alignment(Alignment::Center)
            .block(Block::default().padding(Padding::horizontal(1)));
            frame.render_widget(tips_widget, gauge_chunks[3]);

            // 3. Setup progress
            let setup_msg_lines = app.get_setup_progress_lines();
            let progress_widget = Paragraph::new(setup_msg_lines)
                .alignment(Alignment::Left)
                .wrap(ratatui::widgets::Wrap { trim: true })
                .block(Block::default().padding(Padding::horizontal(3)));
            frame.render_widget(progress_widget, gauge_chunks[4]);

            // 4. Setup progress msg
            let paragraph_msg = Paragraph::new(app.get_setup_progress_msg())
                .gray()
                .block(Block::default().padding(Padding::horizontal(4)))
                .scroll((app.vertical_scroll as u16, 0));
            frame.render_widget(paragraph_msg, gauge_chunks[5]);

            frame.render_stateful_widget(
                Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("↑"))
                    .end_symbol(Some("↓")),
                area,
                &mut app.vertical_scroll_state,
            );
        }
    } else {
        let right_paragraph = Paragraph::new("")
            .block(right_block)
            .alignment(Alignment::Left);
        frame.render_widget(right_paragraph, area);
    }
}

fn draw_search_popup(frame: &mut ratatui::Frame, app: &App) {
    let area = frame.area();
    let search_window_width = 40;
    let search_window_height = 3;
    let search_window = Rect::new(
        (area.width - search_window_width) / 2,
        (area.height - search_window_height) / 2,
        search_window_width,
        search_window_height,
    );

    // Clear the area under the search window
    frame.render_widget(Clear, search_window);

    // Create search input block
    let input_block = Block::bordered()
        .title(" Search Chain ")
        .title_alignment(Alignment::Center)
        .border_set(border::THICK);

    // Create the search text with cursor
    let mut search_text = app.search_input.clone();
    if app.search_cursor_position == search_text.len() {
        search_text.push('█');
    } else {
        search_text.insert(app.search_cursor_position, '█');
    }

    let search_paragraph = Paragraph::new(search_text)
        .block(input_block)
        .alignment(Alignment::Left);

    frame.render_widget(search_paragraph, search_window);
}

fn draw_deploy_options_popup(frame: &mut ratatui::Frame, app: &App) {
    let area = frame.area();
    let window_width = 40;
    let window_height = 4;
    let window = Rect::new(
        (area.width - window_width) / 2,
        (area.height - window_height) / 2,
        window_width,
        window_height,
    );

    // Clear the area under the window
    frame.render_widget(Clear, window);

    // Create the options list
    let items: Vec<ListItem> = app.deploy_options
        .iter()
        .enumerate()
        .map(|(i, (option, enabled))| {
            let style = if i == app.selected_deploy_option {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else if !enabled {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default()
            };
            
            ListItem::new(option.as_str()).style(style)
        })
        .collect();

    let options_list = List::new(items)
        .block(Block::bordered()
            .title(" Deploy Options ")
            .title_alignment(Alignment::Center)
            .border_set(border::THICK))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    frame.render_widget(options_list, window);
}

fn draw_job_options_popup(frame: &mut ratatui::Frame, app: &App) {
    let area = frame.area();
    let popup_width = 20;
    let popup_height = app.job_options.len() as u16 + 2;
    let popup_area = Rect::new(
        (area.width - popup_width) / 2,
        (area.height - popup_height) / 2,
        popup_width,
        popup_height,
    );

    frame.render_widget(Clear, popup_area);

    let items: Vec<ListItem> = app.job_options
        .iter()
        .enumerate()
        .map(|(i, &action)| {
            let style = if i == app.selected_job_option {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else if action == "delete" {
                // Make delete option red
                Style::default().fg(Color::Red)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(
                Span::styled(action, style)
            ))
        })
        .collect();

    let options_list = List::new(items)
        .block(Block::bordered().title(" Actions "));

    frame.render_widget(options_list, popup_area);
}

fn draw_avs_tab(frame: &mut ratatui::Frame, app: &mut App, area: Rect) {
    // AVS tab content
    let tab3_text = Paragraph::new("Coming soon...")
        .block(Block::bordered())
        .alignment(Alignment::Center);
    frame.render_widget(tab3_text, area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
        .split(area);

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
        .split(horizontal[1]);

    frame.render_widget(map_canvas(app), vertical[1]);
}

fn map_canvas(app: &App) -> impl Widget + '_ {
    Canvas::default()
        .block(Block::bordered())
        .marker(app.marker)
        .paint(|ctx| {
            ctx.draw(&Map {
                color: Color::Green,
                resolution: MapResolution::High,
            });
            ctx.print(app.x, -app.y, "Avs Node".yellow());
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
}

const LOGO: &str = "
 ########   #######   ######## 
 #########  #######  ######### 
 ##########  #####  ########## 
  ########## ##### ##########  
      ####### ### #######      
        ####  #  #####         
 #########            ######## 
 ###########       ########### 
 ########             ######## 
        #####  #  #####        
     ######## ###  #######     
  ########## ##### ##########  
 ##########  #####  ########## 
 #########  #######  ######### 
 ########   #######   ########";

const LOGO_LETTER: &str = "
  █████╗██╗  ██╗ █████╗ ██╗███╗   ██╗██████╗  █████╗ ███████╗███████╗
██╔════╝██║  ██║██╔══██╗██║████╗  ██║██╔══██╗██╔══██╗██╔════╝██╔════╝
██║     ███████║███████║██║██╔██╗ ██║██████╔╝███████║███████╗█████╗  
██║     ██╔══██║██╔══██║██║██║╚██╗██║██╔══██╗██╔══██║╚════██║██╔══╝  
╚██████╗██║  ██║██║  ██║██║██║ ╚████║██████╔╝██║  ██║███████║███████╗
  ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝╚═════╝ ╚═╝  ╚═╝╚══════╝╚══════╝";

fn draw_job_logs_popup(frame: &mut ratatui::Frame, app: &App) {
    let area = frame.area();
    let popup_width = (area.width as f32 * 0.8) as u16;
    let popup_height = (area.height as f32 * 0.8) as u16;
    let popup_area = Rect::new(
        (area.width - popup_width) / 2,
        (area.height - popup_height) / 2,
        popup_width,
        popup_height,
    );

    frame.render_widget(Clear, popup_area);

    let logs_block = Block::bordered()
        .title(" Job Logs ")
        .title_alignment(Alignment::Center)
        .border_set(border::THICK)
        .style(Style::default().bg(Color::Rgb(10, 100, 100)));

    let logs = app.job_logs.as_ref().unwrap();
    let filtered_logs = logs.chars()
        .filter(|c| c.is_ascii_graphic() || c.is_ascii_whitespace())
        .collect::<String>();

    let logs_paragraph = Paragraph::new(filtered_logs)
        .block(logs_block)
        .wrap(ratatui::widgets::Wrap { trim: true })
        .style(Style::default().bg(Color::Rgb(10, 100, 100)).fg(Color::White))
        .scroll((app.logs_scroll_position as u16, 0));

    frame.render_widget(logs_paragraph, popup_area);
}

fn draw_help_popup(frame: &mut ratatui::Frame) {
    let area = frame.area();
    let help_window_width = 50;
    let help_window_height = 15;
    let help_window = Rect::new(
        (area.width - help_window_width) / 2,
        (area.height - help_window_height) / 2,
        help_window_width,
        help_window_height,
    );

    frame.render_widget(Clear, help_window);

    let help_text = vec![
        "Global Shortcuts",
        "────────────────",
        "",
        "Enter      Select/Confirm", 
        "Esc        Back/Cancel",
        "/          Search",
        "q          Quit",
        "",
        "Tab/1/2/3  Switch tabs",
        "↑/↓        Navigate",
        "c          Create manuscript",
        "e          Edit manuscript", 
        "r          Run manuscript",
        "d          Deploy options",
    ];

    let help_block = Block::bordered()
        .title(" Help (? to close) ")
        .title_alignment(Alignment::Center)
        .border_set(border::THICK)
        .style(Style::default().bg(Color::Rgb(10, 100, 100)));

    let help_paragraph = Paragraph::new(help_text.join("\n"))
        .block(help_block)
        .alignment(Alignment::Left)
        .style(Style::default().bg(Color::Rgb(10, 100, 100)).fg(Color::White));

    frame.render_widget(help_paragraph, help_window);
}

fn draw_warning_popup(frame: &mut ratatui::Frame) {
    let area = frame.area();
        let warning_width = 80;
        let warning_height = 4;
        let warning_x = (area.width - warning_width) / 2;
        let warning_y = (area.height - warning_height) / 2;

        let warning_area = Rect::new(
            warning_x,
            warning_y,
            warning_width,
            warning_height,
        );

        frame.render_widget(Clear, warning_area);

        let warning_block = Block::bordered()
            .title(" Warning ")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Thick)
            .style(Style::default().bg(Color::Red));

        let warning_text = Paragraph::new("`docker` or `docker compose` are required but not installed:\nhttps://docs.chainbase.com/core-concepts/manuscript/QuickStart/prerequisites")
            .block(warning_block)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White));

        frame.render_widget(warning_text, warning_area);
}
