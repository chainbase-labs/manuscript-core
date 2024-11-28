use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Stylize, Color, Style, Modifier},
    symbols::{self,border},
    text::{Line, Text, Span},
    widgets::{block::Title, Block, List, ListItem, Paragraph, Widget, Tabs, Clear, Gauge, Padding, BorderType, Scrollbar, ScrollbarOrientation, Borders, Dataset, Chart, Axis, canvas::{Canvas, Map, MapResolution},},
};
use crate::app::{App, AppState, SetupState};
use crate::tasks::JobState;


const CUSTOM_LABEL_COLOR: Color = Color::White;
const GAUGE2_COLOR: Style = Style::new().fg(Color::Rgb(10, 100, 100));

fn draw_jobs_status(app: &App) -> String {
    if app.jobs_status.is_empty() {
        return "No Jobs Running".to_string();
    }

    let mut status = String::new();
    for (name, job) in &app.jobs_status {
        status.push_str(&format!("Job: {} - {}\n", name, match job.status {
            JobState::Running => "Running".green(),
            JobState::Pending => "Pending".yellow(),
            JobState::Failed => "Failed".red(),
        }));
        
        for container in &job.containers {
            status.push_str(&format!("  └─ {} ({})\n", 
                container.name,
                match container.state.as_str() {
                    "running" => container.state.as_str().green(),
                    _ => container.state.as_str().yellow(),
                }
            ));
        }
    }
    status
}

pub fn draw(frame: &mut ratatui::Frame, app: &mut App) {
    app.update_jobs_status();

    // Create tabs
    let titles = vec!["NETWORK [1]", "MANUSCRIPTS [2]", "AVS [3]"];
    let tabs = Tabs::new(titles)
        .block(Block::bordered().title("Tabs"))
        .select(app.current_tab)
        .style(Style::default())
        .highlight_style(Style::default().bold());

    // Create main layout with space for tabs
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Height for tabs
            Constraint::Min(0),     // Remaining space for content
        ])
        .split(frame.area());

    // Render tabs
    frame.render_widget(tabs, main_chunks[0]);

    match app.current_tab {
        0 => {
            // Original content
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(main_chunks[1]);  // Use main_chunks[1] instead of frame.area()

            let left_chunks = if app.show_tables {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(45),  // Reduced to make room for hints
                        Constraint::Percentage(45),
                        Constraint::Percentage(10),  // New space for key hints
                    ])
                    .split(chunks[0])
            } else {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(90),  // Reduced to make room for hints
                        Constraint::Percentage(10),  // Space for key hints
                    ])
                    .split(chunks[0])
            };

            let visible_height = left_chunks[0].height as usize - 2;
            let chains_block = Block::bordered()
                .border_set(border::THICK)
                .title(Title::from(" Omnichain ").alignment(Alignment::Center));

            let chain_names: Vec<ListItem> = app.filtered_chains
                .iter()
                .skip(app.scroll_offset)
                .take(visible_height)
                .enumerate()
                .map(|(i, chain)| {
                    let index = i + app.scroll_offset + 1; // Calculate the 1-based index
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
                        i + app.scroll_offset == current_filtered_index
                    } else {
                        false
                    };

                    let content = if is_selected {
                        Line::from(vec![
                            format!("{:<3}⟠ {:<25}", index, chain.name).bold().white().bg(Color::DarkGray).into(),
                            format!("{:<10}", chain.ticker).bold().white().bg(Color::DarkGray).into(),
                            format!("{:<10}", if chain.status == "Online" { "⬤" } else if chain.status == "Offline" { "⬤" } else { "◑" }).bold()
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
                            format!("{:<10}", if chain.status == "Online" { "⬤" } else if chain.status == "Offline" { "⬤" } else { "◑" }).bold()
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
                })
                .collect();

            let chain_list = List::new(chain_names).block(chains_block);
            frame.render_widget(chain_list, left_chunks[0]);

            if app.show_tables {
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
                    frame.render_widget(table_list, left_chunks[1]);
                }
            }

            // Add key hints at the bottom
            let mut hints = vec![
                "Enter: Select",
                "Esc: Back",
                "\\: Search",
                "q: Quit",
            ];
            
            let hints_text = if app.show_tables {
                Text::from(vec![
                    Line::from(hints.join(" | ")),
                    Line::from(vec![
                        " ".into(),
                        "c: Create Manuscript".magenta().bold().into()
                    ])
                ])
            } else {
                Text::from(hints.join(" | "))
            };

            let hints_block = Block::bordered()
                .title(" Controls ")
                .title_alignment(Alignment::Center)
                .border_set(border::THICK);
            let hints_paragraph = Paragraph::new(hints_text)
                .block(hints_block)
                .alignment(Alignment::Center);
            
            // Render hints in the bottom section
            frame.render_widget(
                hints_paragraph,
                if app.show_tables { left_chunks[2] } else { left_chunks[1] }
            );

            if let Some(selected_chain) = app.chains.get(app.selected_chain_index) {
                let mut data_lines = if app.show_tables && app.selected_table_index.is_some() {
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
                    if let Some(overview) = &selected_chain.overview {
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
                        .split(chunks[1]);  // Use the right panel area

                    // Render the symbol logo in magenta
                    let logo = Paragraph::new(LOGO)
                        .block(Block::default()
                            .padding(Padding::new(20, 20, 0, 0))
                            .style(Style::default().fg(Color::Rgb(200, 200, 200))))
                        .style(Style::default().fg(Color::Cyan))
                        .alignment(Alignment::Center);
                    frame.render_widget(logo, layout[1]);

                    // Render the text logo below in cyan
                    let logo_letter = Paragraph::new(LOGO_LETTER)
                        .block(Block::default()
                            .padding(Padding::new(20, 20, 0, 0))
                            .style(Style::default().fg(Color::Rgb(200, 200, 200))))
                        .style(Style::default().fg(Color::Magenta))
                        .alignment(Alignment::Center);
                    frame.render_widget(logo_letter, layout[2]);

                    let x_labels = vec![
                        Span::styled(
                            format!("{}", app.window[0]),
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                        Span::raw(format!("{}", (app.window[0] + app.window[1]) / 2.0)),
                        Span::styled(
                            format!("{}", app.window[1]),
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                    ];
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

                    // Render animated chart at the bottom
                    frame.render_widget(chart, layout[5]);

                    // Return empty vec since we're handling the rendering directly
                    Vec::new()
                };

                // Modify the right side rendering when there's saved SQL
                if let Some(selected_chain) = app.chains.get(app.selected_chain_index) {
                    let right_block = Block::bordered()
                    .title(" Data Dictionary ")
                    .title_alignment(Alignment::Center)
                    .border_set(border::THICK);

                    let data_paragraph = Paragraph::new(data_lines)
                        .block(right_block)
                        .wrap(ratatui::widgets::Wrap { trim: true });
                    frame.render_widget(data_paragraph, chunks[1]);
                }
            }
        }
        1 => {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(main_chunks[1]);

            let left_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(90),
                    Constraint::Percentage(10),
                ])
                .split(chunks[0]);

            // Left panel - Show selected chain and table
            let left_content = draw_jobs_status(app);

            let left_block = Block::bordered()
                .title(" Manuscript Jobs ")
                .title_alignment(Alignment::Center)
                .border_set(border::THICK);
            
            // Convert jobs into ListItems
            let job_list = app.jobs_status.iter().enumerate().map(|(index, (name, status))| {
                // Get the status duration from the first container
                let duration = status.containers.first()
                    .map(|c| c.status.clone())
                    .unwrap_or_default();

                let style = match status.status {
                    JobState::Running => Style::default().fg(Color::Green),
                    JobState::Pending => Style::default().fg(Color::Yellow),
                    JobState::Failed => Style::default().fg(Color::Red),
                };

                Line::from(vec![
                    Span::raw(format!("{} ", name)),
                    Span::styled(
                        format!("{} ({})",
                            match status.status {
                                JobState::Running => "Running",
                                JobState::Pending => "Pending", 
                                JobState::Failed => "Failed",
                            },
                            duration
                        ),
                        style
                    )
                ])
            }).collect::<Vec<_>>();

            let jobs_list = List::new(job_list)
                .block(left_block);
            frame.render_widget(jobs_list, left_chunks[0]);

            let right_block = Block::bordered()
                .title(" SQL Editor ")
                .title_alignment(Alignment::Center)
                .border_set(border::THICK);
            let right_paragraph = Paragraph::new("")
                .block(right_block)
                .alignment(Alignment::Left);
            frame.render_widget(right_paragraph, chunks[1]);

            if app.saved_sql.is_some() {

                let right_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(55),
                        Constraint::Percentage(45),
                    ])
                    .split(chunks[1]);

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

                let sql_paragraph = Paragraph::new(app.saved_sql.as_ref().unwrap().as_str())
                    .block(sql_block)
                    .alignment(Alignment::Left)
                    .style(Style::default().fg(Color::White));
                frame.render_widget(sql_paragraph, right_chunks[0]);

                    let console_block = Block::bordered()
                        .title(" Debug Console ")
                        .title_alignment(Alignment::Center)
                        .border_set(border::THICK);
                    frame.render_widget(console_block, right_chunks[1]);

                    // Modify the console content rendering based on setup_state
                    if app.setup_state == SetupState::Complete {
                        // When complete, use the full console area for the message
                        let paragraph_msg = Paragraph::new(app.get_setup_progress_msg())
                            .gray()
                            .block(Block::default()
                                .borders(Borders::BOTTOM)
                                .padding(Padding::horizontal(4)))
                            .scroll((app.vertical_scroll as u16, 0));
                        frame.render_widget(paragraph_msg, right_chunks[1]);
                    } else {
                        // During setup, use the original divided layout
                        let gauge_chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Length(1),
                                Constraint::Length(1),
                                Constraint::Length(2),  
                                Constraint::Length(6),
                                Constraint::Min(0),
                            ])
                            .split(right_chunks[1]);

                        // 1. Progress gauge
                        if app.state == AppState::Started {
                            let label = Span::styled(
                                format!("{:.1}%", app.progress1()),
                                Style::new().italic().bold().fg(CUSTOM_LABEL_COLOR),
                            );
                            let gauge = Gauge::default()
                                .block(Block::default().padding(Padding::horizontal(1)))
                                .gauge_style(GAUGE2_COLOR)
                                .ratio(app.progress1 / 100.0)
                                .label(label);
                            frame.render_widget(gauge, gauge_chunks[1]);
                        }

                        // 2. Docker status
                        let docker_status = if app.docker_setup_in_progress {
                            format!("Docker setup in progress... ({} seconds)", app.docker_setup_timer / 10)
                        } else {
                            "🏄🏻 Manuscript console: Debug your manuscript before deploying it locally or to the network.".to_string()
                        };

                        let docker_status_widget = Paragraph::new(Text::from(
                            Span::styled(docker_status, Style::default().fg(Color::Yellow))
                        ))
                        .alignment(Alignment::Center)
                        .block(Block::default()
                            .padding(Padding::horizontal(1)));
                        frame.render_widget(docker_status_widget, gauge_chunks[2]);

                        // 3. Setup progress
                        let steup_msg_lines = app.get_setup_progress_lines();
                        let progress_widget = Paragraph::new(steup_msg_lines)
                            .alignment(Alignment::Left)
                            .wrap(ratatui::widgets::Wrap { trim: true })
                            .block(Block::default().padding(Padding::horizontal(4)));
                        frame.render_widget(progress_widget, gauge_chunks[3]);

                        // 4. Setup progress msg
                        let paragraph_msg = Paragraph::new(app.get_setup_progress_msg())
                            .gray()
                            .block(Block::default()
                                .borders(Borders::BOTTOM)
                                .padding(Padding::new(4, 1, 1, 1)))
                            .scroll((app.vertical_scroll as u16, 0));
                        frame.render_widget(paragraph_msg, gauge_chunks[4]);
                        frame.render_stateful_widget(
                            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                                .begin_symbol(Some("↑"))
                                .end_symbol(Some("↓")),
                            chunks[1],
                            &mut app.vertical_scroll_state,
                        );

                }
            }

            // Add key hints at the bottom
            let hints = vec![
                "R: Run",
                "E: Edit",
                "D: Deploy",
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

            // Add job options popup if show_job_options is true
            if app.show_job_options {
                let area = frame.size();
                let popup_width = 20;
                let popup_height = 5; // Increased height to accommodate new option
                let popup_area = Rect::new(
                    (area.width - popup_width) / 2,
                    (area.height - popup_height) / 2,
                    popup_width,
                    popup_height,
                );

                frame.render_widget(Clear, popup_area);

                let items: Vec<ListItem> = vec!["start", "stop", "graphql"]
                    .iter()
                    .enumerate()
                    .map(|(i, &action)| {
                        if i == app.selected_job_option {
                            ListItem::new(Line::from(
                                Span::styled(action, Style::default().bg(Color::Blue).fg(Color::White))
                            ))
                        } else {
                            ListItem::new(action)
                        }
                    })
                    .collect();

                let options_list = List::new(items)
                    .block(Block::bordered().title(" Actions "));

                frame.render_widget(options_list, popup_area);
            }
        }
        2 => {
            // AVS tab content (moved from old tab 2)
            let tab3_text = Paragraph::new("Manuscript Jobs")
                .block(Block::bordered())
                .alignment(Alignment::Center);
            frame.render_widget(tab3_text, main_chunks[1]);

            let horizontal = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
            let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
            let [left, right] = horizontal.areas(frame.area());
            let [draw, map] = vertical.areas(right);
            frame.render_widget(map_canvas(&app), map);
        }
        _ => unreachable!(),
    }

    // Calculate how many blocks to show based on time
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();  // Get current time in milliseconds
    let num_blocks = ((now / 1500) % 5 + 1) as usize;  // Cycle every 1.5 seconds (1500ms)
    
    // Create the loading animation string
    let blocks: String = "▊".repeat(num_blocks) + &" ".repeat(5 - num_blocks);
    
    // Create text spans with different colors
    let text = vec![
        Span::styled(
            "Chainbase Network [TestNet] [v1.1.0] ",
            Style::default().bold()
        ),
        Span::styled(
            blocks,
            Style::default().fg(ratatui::style::Color::Green)
        )
    ];
    
    let chainbase_text = Paragraph::new(Line::from(text))
        .alignment(Alignment::Right);
        
    frame.render_widget(
        chainbase_text,
        Rect::new(
            frame.size().width - 45,
            1,
            43,
            1,
        ),
    );

    if app.show_sql_window {
        // Create a floating SQL input window
        let area = frame.size();
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

    // Add search window rendering at the end of the function
    if app.show_search {
        let area = frame.size();
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

    // Add warning window rendering at the end
    if app.show_warning {
        let area = frame.size();
        let warning_window_width = 90;
        let warning_window_height = 3;
        let warning_window = Rect::new(
            (area.width - warning_window_width) / 2,
            (area.height - warning_window_height) / 2,
            warning_window_width,
            warning_window_height,
        );

        // Clear the area under the warning window
        frame.render_widget(Clear, warning_window);

        // Create warning block
        let warning_block = Block::bordered()
            .title(" Warning ")
            .title_alignment(Alignment::Center)
            .border_set(border::THICK)
            .border_style(Style::default().fg(Color::Yellow));

        let warning_text = Paragraph::new("Run and verify the results in debug mode before proceeding with deployment..")
            .block(warning_block)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));

        frame.render_widget(warning_text, warning_window);
    }

    // Add deployment options window rendering at the end
    if app.show_deploy_options {
        let area = frame.size();
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
