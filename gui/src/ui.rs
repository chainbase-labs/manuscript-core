use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Stylize, Color, Style, Modifier},
    symbols::border,
    text::{Line, Text, Span},
    widgets::{block::{Position, Title}, Block, List, ListItem, Paragraph, Widget, Tabs, Clear, Gauge, Padding, BorderType, Scrollbar, ScrollbarOrientation, Borders},
};
use crate::app::App;
use crate::app::AppState;

// Add this helper function before the draw function
fn title_block(title: &str) -> Block<'_> {
    Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .title(title)
}

// Also need to define CUSTOM_LABEL_COLOR and GAUGE2_COLOR constants
const CUSTOM_LABEL_COLOR: Color = Color::White;
const GAUGE2_COLOR: Style = Style::new().fg(Color::Rgb(10, 100, 100));

pub fn draw(frame: &mut ratatui::Frame, app: &mut App) {


    // Create tabs
    let titles = vec!["NETWORK [1]", "MANUSCRIPTS [2]"];
    let executing_text = String::from("Executing...");
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

            let chain_names: Vec<ListItem> = app.chains
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

                    let content = if i + app.scroll_offset == app.selected_chain_index {
                        Line::from(vec![
                            format!("{:<3} {:<25}", index, chain.name).bold().white().into(),
                            format!("{:<20}", chain.status).bold().into(),
                            format!("{:<10}", time_ago_style).bold().into(),
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
                            format!("{:<20}", chain.status).bold()
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
                        .keys()
                        .enumerate()
                        .map(|(i, table_name)| {
                            let content = if Some(i) == app.selected_table_index {
                                Line::from(vec![
                                    format!("{:<1}. ", i + 1).bold().green(),
                                    table_name.clone().bold().green()
                                ])
                            } else {
                                Line::from(vec![
                                    format!("{:<1}.", i + 1).white(),
                                    table_name.clone().into()
                                ])
                            };
                            ListItem::new(content)
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
            let hints = vec![
                "Enter: Select",
                "PageUp/Down: Navigate",
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
            
            // Render hints in the bottom section
            frame.render_widget(
                hints_paragraph,
                if app.show_tables { left_chunks[2] } else { left_chunks[1] }
            );

            if let Some(selected_chain) = app.chains.get(app.selected_chain_index) {
                let mut data_lines = if app.show_tables && app.selected_table_index.is_some() {
                    let table_name = selected_chain.dataDictionary
                        .keys()
                        .nth(app.selected_table_index.unwrap())
                        .map(|s| s.as_str())
                        .unwrap_or("");

                    let fields = selected_chain.dataDictionary.get(table_name);
                    
                    let mut lines = Vec::new();
                    
                    // Add header
                    lines.push(Line::from(vec![
                        "Field Name".bold().white(),
                        " | ".into(),
                        "Data Type".bold().white(),
                        " | ".into(),
                        "Description".bold().white(),
                    ]));
                    lines.push(Line::from("─".repeat(80)));  // Separator line

                    // Add field descriptions in table format
                    if let Some(fields) = fields {
                        lines.extend(fields.iter().map(|item| {
                            Line::from(vec![
                                format!("{:<20}", item.name).yellow().into(),
                                " | ".into(),
                                format!("{:<15}", item.dataType).cyan().into(),
                                " | ".into(),
                                item.description.clone().white().into(),
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
                        lines.push(Line::from("─".repeat(80)));  // Separator line

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
                            Constraint::Percentage(15),  // Top spacing
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
                        ])
                        .split(chunks[1]);  // Use the right panel area

                    // Render the symbol logo in magenta
                    let logo = Paragraph::new(LOGO)
                        .style(Style::default().fg(Color::Cyan))
                        .alignment(Alignment::Center);
                    frame.render_widget(logo, layout[1]);

                    // Render the text logo below in cyan
                    let logo_letter = Paragraph::new(LOGO_LETTER)
                        .style(Style::default().fg(Color::Magenta))
                        // .style(Style::default().fg(Color::Cyan))
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
                    frame.render_widget(splash, layout[4]);  // Changed to layout[4]

                    // Return empty vec since we're handling the rendering directly
                    Vec::new()
                };

                // Modify the right side rendering when there's saved SQL
                if let Some(selected_chain) = app.chains.get(app.selected_chain_index) {
                    if app.show_tables && app.selected_table_index.is_some() && app.saved_sql.is_some() {

                        let right_chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Percentage(45),
                                Constraint::Percentage(55),
                            ])
                            .split(chunks[1]);

                        let sql_block = Block::bordered()
                            .border_type(BorderType::Double)
                            .title(" SQL Editor ")
                            .title_alignment(Alignment::Center)
                            .style(Style::default().bg(Color::Rgb(10, 100, 100)))
                            .title_bottom(Line::from(vec![
                                "   Press ".white(),
                                "R".green().bold(), 
                                " to run, ".white(),
                                "E".red().bold(),
                                " to edit, ".white(), 
                                "D".blue().bold(),
                                " to deploy  ".white()
                            ]).right_aligned()).padding(Padding::horizontal(1));

                        let sql_paragraph = Paragraph::new(app.saved_sql.as_ref().unwrap().as_str())
                            .block(sql_block)
                            .wrap(ratatui::widgets::Wrap { trim: true });
                        frame.render_widget(sql_paragraph, right_chunks[0]);

                            let console_block = Block::bordered()
                                .title(" Console ")
                                .title_alignment(Alignment::Center)
                                .border_set(border::THICK);
                            frame.render_widget(console_block, right_chunks[1]);

                            let gauge_chunks = Layout::default()
                                .direction(Direction::Vertical)
                                .constraints([
                                    Constraint::Length(1),
                                    Constraint::Length(1),
                                    Constraint::Length(2),  
                                    Constraint::Length(9),
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
                                "🏄🏻 Manuscript console: Debug your SQL before deploying it locally or to the network.".to_string()
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
                            let paragraph = Paragraph::new(app.get_setup_progress_msg())
                                .gray()
                                .block(Block::default()
                                    .borders(Borders::BOTTOM)
                                    .padding(Padding::horizontal(4)))
                                .scroll((app.vertical_scroll as u16, 0));
                            frame.render_widget(paragraph, gauge_chunks[4]);
                            frame.render_stateful_widget(
                                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                                    .begin_symbol(Some("↑"))
                                    .end_symbol(Some("↓")),
                                chunks[1],
                                &mut app.vertical_scroll_state,
                        );

                        
                    } else {
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
        }
        1 => {
            // Tab 2 content
            let tab2_text = Paragraph::new("tab2 text")
                .block(Block::bordered().title("Tab 2"))
                .alignment(Alignment::Center);
            frame.render_widget(tab2_text, main_chunks[1]);
        }
        _ => unreachable!(),
    }

    // Add Chainbase text to top-right corner LAST (after all other rendering)
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
            frame.size().width - 45, // Increased width to accommodate animation
            1,                       // Top of screen
            43,                      // Increased width for blocks
            1,                       // Height of text
        ),
    );

    if app.show_sql_window {
        // Create a floating SQL input window
        let area = frame.size();
        let sql_window_width = (area.width as f32 * 0.8) as u16;
        let sql_window_height = (area.height as f32 * 0.4) as u16;
        let sql_window = Rect::new(
            (area.width - sql_window_width) / 2,
            (area.height - sql_window_height) / 2,
            sql_window_width,
            sql_window_height,
        );

        // Clear the area under the SQL window
        frame.render_widget(Clear, sql_window);

        // Create input block
        let input_block = Block::bordered()
            .title(" SQL Editor (Esc → Save & Esc) ")
            .title_alignment(Alignment::Center)
            .border_set(border::THICK)
            .style(Style::default().bg(Color::Rgb(10, 100, 100)))
            .title_style(Style::default()
                .fg(Color::Yellow)
                .bold()
                .add_modifier(Modifier::UNDERLINED | Modifier::ITALIC));

        // Convert the SQL input into styled text with cursor
        let mut styled_text = Text::default();
        let input = app.sql_input.as_str();
        
        // Split input into lines and process each line
        let lines: Vec<&str> = input.split('\n').collect();
        let mut current_pos = 0;
        
        for (line_idx, line) in lines.iter().enumerate() {
            let line_length = line.len() + 1; // +1 for the newline character
            let cursor_in_this_line = app.sql_cursor_position >= current_pos 
                && app.sql_cursor_position < current_pos + line_length;
            
            if cursor_in_this_line {
                // Calculate cursor position within this line
                let line_cursor_pos = app.sql_cursor_position - current_pos;
                
                // Split the line at cursor position
                let (before_cursor, after_cursor) = line.split_at(line_cursor_pos);
                
                let mut spans = Vec::new();
                spans.push(Span::raw(before_cursor));
                
                // Add cursor
                if after_cursor.chars().next().is_some() {
                    // If there's a character at cursor position, highlight it
                    let next_char = &after_cursor[..1];
                    spans.push(Span::styled(
                        next_char,
                        Style::default().bg(Color::White).fg(Color::Black)
                    ));
                    spans.push(Span::raw(&after_cursor[1..]));
                } else {
                    // If cursor is at the end of line, show a block cursor
                    spans.push(Span::styled(
                        " ",
                        Style::default().bg(Color::White)
                    ));
                }
                
                styled_text.extend(Text::from(Line::from(spans)));
            } else {
                // Regular line without cursor
                styled_text.extend(Text::from(Line::from(line.to_string())));
            }
            
            // Add newline if this isn't the last line
            if line_idx < lines.len() - 1 {
                styled_text.extend(Text::from("\n"));
            }
            
            current_pos += line_length;
        }

        // Render SQL input with cursor
        let sql_paragraph = Paragraph::new(styled_text)
            .block(input_block)
            .style(Style::default().fg(Color::White));

        frame.render_widget(sql_paragraph, sql_window);

        // If there's a SQL result, show it below the input
        if let Some(result) = &app.sql_result {
            let result_text = Paragraph::new(result.as_str())
                .style(Style::default().fg(Color::Green));
            
            // Calculate result window position below SQL input
            let result_window = Rect::new(
                sql_window.x,
                sql_window.y + sql_window.height,
                sql_window.width,
                3, // Height for result display
            );
            
            frame.render_widget(Clear, result_window);
            frame.render_widget(result_text, result_window);
        }
    }
}
