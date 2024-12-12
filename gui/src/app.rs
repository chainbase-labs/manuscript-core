use std::{io, collections::HashMap, time::Duration, cell::RefCell};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use serde::Deserialize;
use reqwest;
use tokio::sync::mpsc;
use ratatui::style::{Style, Color,palette::tailwind};
use ratatui::text::{Line, Span, Text};
use ratatui::layout::Alignment;
use ratatui::widgets::{block::Title,Block,Borders, Padding};
use std::time::Instant;
use ratatui::symbols::Marker;
use crate::ui;
use crate::prerun::PreRun;
use crate::config::Settings;
use crate::tasks::JobManager;
use crate::tasks::tasks::{JobsCommand, JobsUpdate, JobStatus};
use aes_gcm::{Aes128Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use base64;
use std::process::Command;

#[derive(Debug)]
pub struct App {
    pub chains: Vec<Chain>,
    pub selected_chain_index: usize,
    pub selected_table_index: Option<usize>,
    pub show_tables: bool,
    pub scroll_offset: usize,
    pub exit: bool,
    pub current_tab: usize,
    pub marker: Marker,
    pub example_data: Option<ExampleData>,
    pub sql_input: String,
    pub show_sql_window: bool,
    pub sql_cursor_position: usize,
    pub sql_result: Option<String>,
    pub saved_manuscript: Option<String>,
    pub sql_executing: bool,
    pub sql_error: Option<String>,
    pub sql_columns: Vec<Column>,
    pub sql_data: Vec<Vec<serde_json::Value>>,
    pub sql_timer: u64, 
    pub pre_run: PreRun,
    pub debug_result: Option<String>,
    pub docker_setup_in_progress: bool,
    pub docker_setup_timer: u64,
    pub setup_progress: f64, 
    pub setup_state: SetupState,
    pub state: AppState,
    progress_columns: u16,
    pub progress1: f64,
    pub progress_lines: RefCell<Vec<String>>,
    pub should_cancel_setup: bool,
    pub update_sender: Option<mpsc::Sender<AppUpdate>>,
    pub update_receiver: Option<mpsc::Receiver<AppUpdate>>,
    pub current_setup_step: Option<SetupStep>,
    pub vertical_scroll: usize,
    pub vertical_scroll_state: ratatui::widgets::ScrollbarState,
    signal1: SinSignal,
    pub data1: Vec<(f64, f64)>,
    signal2: SinSignal,
    pub data2: Vec<(f64, f64)>,
    pub window: [f64; 2],
    pub x: f64,
    pub y: f64,
    pub show_search: bool,
    pub search_input: String,
    pub search_cursor_position: usize,
    pub filtered_chains: Vec<Chain>,
    pub show_warning: bool,
    pub show_deploy_options: bool,
    pub selected_deploy_option: usize,
    pub deploy_options: Vec<(String, bool)>,
    pub transformed_sql: Option<String>,
    pub jobs_status: Vec<JobStatus>,
    jobs_monitor_sender: Option<mpsc::Sender<JobsUpdate>>,
    jobs_monitor_receiver: Option<mpsc::Receiver<JobsUpdate>>,
    pub selected_job_index: usize,
    pub show_job_options: bool,
    pub selected_job_option: usize,
    pub job_options: Vec<&'static str>,
    pub job_logs: Option<String>,
    pub job_manager: JobManager,
    action_sender: Option<mpsc::Sender<(String, String)>>,
    action_receiver: Option<mpsc::Receiver<(String, String)>>,
    pub logs_scroll_position: usize,
    pub show_help: bool,
    pub network_status: NetworkStatus,
}

#[derive(Debug, Clone,)]
struct SinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
}

impl SinSignal {
    const fn new(interval: f64, period: f64, scale: f64) -> Self {
        Self {
            x: 0.0,
            interval,
            period,
            scale,
        }
    }
}

impl Default for SinSignal {
    fn default() -> Self {
        Self {
            x: 0.0,
            interval: 0.2,
            period: 20.0,
            scale: 10.0,
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(point)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetupState {
    NotStarted,
    InProgress,
    Complete,
    Failed(String),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Running,
    Started,
    Quitting,
}

impl Clone for App {
    fn clone(&self) -> Self {
        Self {
            chains: self.chains.clone(),
            selected_chain_index: self.selected_chain_index,
            selected_table_index: self.selected_table_index,
            show_tables: self.show_tables,
            scroll_offset: self.scroll_offset,
            exit: self.exit,
            current_tab: self.current_tab,
            marker: self.marker,
            example_data: self.example_data.clone(),
            sql_input: self.sql_input.clone(),
            show_sql_window: self.show_sql_window,
            sql_cursor_position: self.sql_cursor_position,
            sql_result: self.sql_result.clone(),
            saved_manuscript: self.saved_manuscript.clone(),
            sql_executing: self.sql_executing,
            sql_error: self.sql_error.clone(),
            sql_columns: self.sql_columns.clone(),
            sql_data: self.sql_data.clone(),
            sql_timer: self.sql_timer,
            pre_run: self.pre_run.clone(),
            debug_result: self.debug_result.clone(),
            docker_setup_in_progress: self.docker_setup_in_progress,
            docker_setup_timer: self.docker_setup_timer,
            setup_progress: self.setup_progress,
            setup_state: self.setup_state.clone(),
            state: self.state,
            progress_columns: self.progress_columns,
            progress1: self.progress1,
            progress_lines: RefCell::new(Vec::new()),
            should_cancel_setup: self.should_cancel_setup,
            update_sender: self.update_sender.clone(),
            update_receiver: None,
            current_setup_step: self.current_setup_step.clone(),
            vertical_scroll: self.vertical_scroll,
            vertical_scroll_state: self.vertical_scroll_state.clone(),
            signal1: self.signal1.clone(),
            data1: self.data1.clone(),   
            signal2: self.signal2.clone(),
            data2: self.data2.clone(),
            window: self.window.clone(),
            x: self.x,
            y: self.y,
            show_search: self.show_search,
            search_input: self.search_input.clone(),
            search_cursor_position: self.search_cursor_position,
            filtered_chains: self.filtered_chains.clone(),
            show_warning: self.show_warning,
            show_deploy_options: self.show_deploy_options,
            selected_deploy_option: self.selected_deploy_option,
            deploy_options: self.deploy_options.clone(),
            transformed_sql: self.transformed_sql.clone(),
            jobs_status: self.jobs_status.clone(),
            jobs_monitor_sender: self.jobs_monitor_sender.clone(),
            jobs_monitor_receiver: None,
            selected_job_index: self.selected_job_index,
            show_job_options: self.show_job_options,
            selected_job_option: self.selected_job_option,
            job_options: self.job_options.clone(),
            job_logs: self.job_logs.clone(),
            job_manager: self.job_manager.clone(),
            action_sender: self.action_sender.clone(),
            action_receiver: None,
            logs_scroll_position: self.logs_scroll_position,
            show_help: self.show_help,
            network_status: self.network_status.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ExampleData {
    pub columns: Vec<Column>,
    pub data: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub type_: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Chain {
    pub name: String,
    pub status: String,
    pub ticker: String,
    pub lastUpdate: String,
    pub time_ago: String,
    pub databaseName: String,
    pub dataDictionary: Vec<(String, Vec<DataDictionaryItem>)>,
    pub example: Option<HashMap<String, Vec<serde_json::Value>>>,
    pub overview: Option<String>,
    pub net: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DataDictionary {
    #[serde(flatten)]
    tables: HashMap<String, Vec<DataDictionaryItem>>,
}

impl IntoIterator for DataDictionary {
    type Item = (String, Vec<DataDictionaryItem>);
    type IntoIter = std::collections::hash_map::IntoIter<String, Vec<DataDictionaryItem>>;

    fn into_iter(self) -> Self::IntoIter {
        self.tables.into_iter()
    }
}

#[derive(Clone, Debug)]
pub enum AppUpdate {
    SteupResult(String),
    SetupProgress(SetupStep, SetupStepStatus),
    SetupComplete,
    SetupFailed(String, SetupStep),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SetupStepStatus {
    Pending,
    InProgress,
    Complete,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SetupStep {
    ParseManuscript,
    SubmitSQLTask,
    WaitingForExecutionResults,
}

impl SetupStep {
    fn as_str(&self) -> &'static str {
        match self {
            SetupStep::ParseManuscript => "Parse manuscript yaml",
            SetupStep::SubmitSQLTask => "Submit sql task",
            SetupStep::WaitingForExecutionResults => "Waiting for execution results",
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct NetworkStatus {
    pub cpu: String,
    pub storage: String,
    pub net: String,
    pub thread: String,
}

impl App {
    pub async fn new() -> Self {
        let (update_sender, update_receiver) = mpsc::channel(32);
        let (jobs_command_tx, jobs_command_rx) = mpsc::channel(32);
        let (jobs_update_tx, mut jobs_update_rx) = mpsc::channel(32);
        let (action_sender, action_receiver) = mpsc::channel(32);
        
        let mut signal1 = SinSignal::new(0.2, 3.0, 18.0);
        let mut signal2 = SinSignal::new(0.1, 2.0, 10.0);
        let data1 = signal1.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        let data2 = signal2.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        
        let (chains, network_status) = match App::fetch_chains().await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to fetch chains: {}", e);
                (Vec::new(), NetworkStatus::default())
            }
        };

        let job_manager = JobManager::new();
        
        // Create a clone of job_manager for the monitor task
        let monitor_job_manager = job_manager.clone();
        let tx1 = jobs_update_tx.clone();
        let tx2 = jobs_update_tx.clone();
        
        // Start the jobs monitor in a separate task
        tokio::spawn(async move {
            monitor_job_manager.jobs_monitor(jobs_command_rx, tx1).await;
        });

        let job_options = vec!["edit", "logs", "start", "stop", "graphql", "delete"];
        
        let mut pre_run = PreRun::new();
        let (un, pw) = App::fetch_and_decrypt_credentials().await;
        pre_run.set_auth(un, pw);

        let app = App {
            chains: chains.clone(),
            selected_chain_index: 0,
            selected_table_index: None,
            show_tables: false,
            scroll_offset: 0,
            exit: false,
            current_tab: 0,
            marker: Marker::Braille,
            example_data: None,
            sql_input: String::new(),
            show_sql_window: false,
            sql_cursor_position: 0,
            sql_result: None,
            saved_manuscript: None,
            sql_executing: false,
            sql_error: None,
            sql_columns: Vec::new(),
            sql_data: Vec::new(),
            sql_timer: 0,
            pre_run,
            debug_result: None,
            docker_setup_in_progress: false,
            docker_setup_timer: 0,
            setup_progress: 0.0,
            setup_state: SetupState::NotStarted,
            state: AppState::default(),
            progress_columns: 0,
            progress1: 0.0,
            progress_lines: RefCell::new(Vec::new()),
            should_cancel_setup: false,
            update_sender: Some(update_sender),
            update_receiver: Some(update_receiver),
            current_setup_step: None,
            vertical_scroll: 0,
            vertical_scroll_state: ratatui::widgets::ScrollbarState::default(),
            signal1: signal1,
            data1: data1,   
            signal2: signal2,
            data2: data2,
            window: [0.0, 20.0],
            x: 0.0,
            y: 0.0,
            show_search: false,
            search_input: String::new(),
            search_cursor_position: 0,
            filtered_chains: chains,
            show_warning: false,
            show_deploy_options: false,
            selected_deploy_option: 0,
            deploy_options: vec![
                ("Local".to_string(), true),
                ("Network (Coming Soon)".to_string(), false),
            ],
            transformed_sql: None,
            jobs_status: Vec::new(),
            jobs_monitor_sender: Some(tx2),
            jobs_monitor_receiver: Some(jobs_update_rx),
            selected_job_index: 0,
            show_job_options: false,
            selected_job_option: 0,
            job_options: job_options,
            job_logs: None,
            job_manager: job_manager,
            action_sender: Some(action_sender),
            action_receiver: Some(action_receiver),
            logs_scroll_position: 0,
            show_help: false,
            network_status,
        };

        app
    }

    async fn fetch_chains() -> Result<(Vec<Chain>, NetworkStatus), reqwest::Error> {
        let url = Settings::get_chains_url();

        match reqwest::get(url).await?.json::<Response>().await {
            Ok(response) => {
                let chains = response.graphData.into_iter()
                    .map(|graph_data| {
                        let mut tables: Vec<(String, Vec<DataDictionaryItem>)> = graph_data.chain.dataDictionary
                            .into_iter()
                            .collect();
                        tables.sort_by(|a, b| a.0.cmp(&b.0));
                        
                        let time_ago = Self::calculate_time_diff(&graph_data.chain.lastUpdate);
                        
                        Chain {
                            name: graph_data.chain.name,
                            ticker: graph_data.chain.ticker,
                            status: graph_data.chain.status,
                            lastUpdate: graph_data.chain.lastUpdate,
                            databaseName: graph_data.chain.databaseName,
                            time_ago,
                            dataDictionary: tables,
                            example: graph_data.chain.example,
                            overview: graph_data.chain.overview,
                            net: graph_data.chain.net,
                        }
                    })
                    .collect();
                Ok((chains, response.status))
            }
            Err(e) => {
                eprintln!("Fatal error: Failed to parse JSON response: {:?}", e);
                std::process::exit(1);
            }
        }
    }

    fn calculate_time_diff(time_str: &str) -> String {
        if let Ok(time) = chrono::DateTime::parse_from_rfc3339(time_str) {
            let now = chrono::Utc::now();
            let duration = now.signed_duration_since(time);
            
            if duration.num_hours() > 24 {
                format!("{} days", duration.num_days())
            } else if duration.num_hours() > 0 {
                format!("{} hrs", duration.num_hours())
            } else {
                format!("{} min", duration.num_minutes())
            }
        } else {
            "unknown".to_string()
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let tick_rate = Duration::from_millis(150);
        let mut last_tick = Instant::now();
        while !self.exit {
            let visible_height = terminal.size()?.height as usize - 2;
            
            terminal.draw(|frame| ui::draw(frame, self))?;
            
            // Update timer if Docker setup is in progress
            if self.state == AppState::Started {
                self.docker_setup_timer = self.docker_setup_timer.saturating_add(1);
                self.update();
            }

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.kind == KeyEventKind::Press {
                        self.handle_key_event(key_event, visible_height);
                    }
                }
            }

            if let Some(receiver) = &mut self.update_receiver {
                match receiver.try_recv() {
                    Ok(update) => match update {
                        AppUpdate::SteupResult(msg) => {
                            self.debug_result = Some(msg);
                        },
                        AppUpdate::SetupProgress(step, status) => {
                            self.current_setup_step = Some(step.clone());
                            self.setup_state = match status {
                                SetupStepStatus::Pending => SetupState::NotStarted,
                                SetupStepStatus::InProgress => SetupState::InProgress,
                                SetupStepStatus::Complete => SetupState::Complete,
                                SetupStepStatus::Failed => SetupState::Failed(format!("Step {} failed", step.as_str())),
                            };
                        },
                        AppUpdate::SetupComplete => {
                            self.state = AppState::Running;
                            self.setup_state = SetupState::Complete;
                            self.docker_setup_timer = 0;
                            self.progress1 = 0.0;
                            self.docker_setup_in_progress = false;
                            self.current_setup_step = None;
                        },
                        AppUpdate::SetupFailed(error, step) => {
                            self.debug_result = Some(format!("Error: {}", error));
                            self.state = AppState::Running;
                            self.setup_state = SetupState::Failed(error);
                            self.docker_setup_timer = 0;
                            self.progress1 = 0.0;
                            self.docker_setup_in_progress = false;
                            self.current_setup_step = Some(step);
                        }
                    },
                    Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {},
                    Err(_) => {
                        self.debug_result = Some("Channel closed".to_string());
                    }
                }
            }

            if let Some(receiver) = &mut self.action_receiver {
                match receiver.try_recv() {
                    Ok((action, content)) => {
                        match action.as_str() {
                            "edit" => {
                                self.saved_manuscript = Some(content.clone());
                                self.show_sql_window = true;
                                self.sql_input = content;
                                self.sql_cursor_position = self.sql_input.len();
                            }
                            "logs" => {
                                self.job_logs = Some(content);
                            }
                            _ => {}
                        }
                    }
                    Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {}
                    Err(_) => {}
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn on_tick(&mut self) {
        self.data1.drain(0..5);
        self.data1.extend(self.signal1.by_ref().take(5));

        self.data2.drain(0..10);
        self.data2.extend(self.signal2.by_ref().take(10));

        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }

    fn update(&mut self) {
        if self.should_cancel_setup {
            // Reset everything if cancellation is requested
            self.progress1 = 0.0;
            self.docker_setup_timer = 0;
            return;
        }

        let total_duration = 300;
        self.progress1 = (self.docker_setup_timer as f64 * 100.0 / total_duration as f64).min(100.0);
        
        if self.progress1 >= 100.0 {
            self.state = AppState::Running;
            self.docker_setup_timer = 0;
        }
    }

    pub fn update_example_data(&mut self) {
        if let Some(selected_chain) = self.chains.get(self.selected_chain_index) {
            if selected_chain.status == "Offline" {
                self.example_data = None;
                return;
            }

            if let Some(table_index) = self.selected_table_index {
                let table_name = selected_chain.dataDictionary
                    .get(table_index)
                    .map(|(name, _)| name.as_str());

                if let Some(table_name) = table_name {
                    if let Some(examples) = &selected_chain.example {
                        if let Some(example_data) = examples.get(table_name) {
                            let columns = selected_chain.dataDictionary
                                .iter()
                                .find(|(name, _)| name == table_name)
                                .map(|(_, items)| {
                                    items.iter()
                                        .map(|item| Column {
                                            name: item.name.clone(),
                                            type_: item.dataType.clone(),
                                        })
                                        .collect()
                                })
                                .unwrap_or_default();

                            self.example_data = Some(ExampleData {
                                columns,
                                data: vec![example_data.clone()],
                            });
                            return;
                        }
                    }
                }
            }
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent, visible_height: usize) {
        match key_event.code {
            KeyCode::Char('?') => {
                self.show_help = !self.show_help;
                return;
            }
            _ => {}
        }

        if self.show_help {
            if let KeyCode::Esc = key_event.code {
                self.show_help = false;
            }
            return;
        }

        if self.show_warning {
            self.show_warning = false;
        }

        if self.show_deploy_options {
            match key_event.code {
                KeyCode::Esc => {
                    self.show_deploy_options = false;
                }
                KeyCode::Enter => {
                    if self.selected_deploy_option == 0 {
                        // Check if docker is installed
                        if !self.check_docker_installed() {
                            self.show_warning = true;
                            self.show_deploy_options = false;
                            return;
                        }
                        
                        if let Some(yaml_content) = &self.saved_manuscript {
                            let yaml_content = yaml_content.clone();
                            let job_manager = self.job_manager.clone();


                            match self.jobs_monitor_sender.clone() {
                                Some(tx) => {
                                    tokio::spawn(async move {
                                        if let Err(e) = job_manager.create_config_file(&yaml_content, tx).await {
                                            eprintln!("Failed to create config file: {}", e);
                                        }
                                    });
                                }
                                None => {
                                    println!("No sender available!");
                                }
                            }
                        }
                    }
                    self.show_deploy_options = false;
                }
                _ => {}
            }
            return;
        }

        if self.show_search {
            match key_event.code {
                KeyCode::Esc => {
                    self.show_search = false;
                    self.search_input.clear();
                    self.filtered_chains = self.chains.clone();
                }
                KeyCode::Enter => {
                    self.show_search = false;
                    self.filter_chains();
                }
                KeyCode::Char(c) => {
                    self.search_input.insert(self.search_cursor_position, c);
                    self.search_cursor_position += 1;
                }
                KeyCode::Backspace => {
                    if self.search_cursor_position > 0 {
                        self.search_input.remove(self.search_cursor_position - 1);
                        self.search_cursor_position -= 1;
                    }
                }
                KeyCode::Left => {
                    if self.search_cursor_position > 0 {
                        self.search_cursor_position -= 1;
                    }
                }
                KeyCode::Right => {
                    if self.search_cursor_position < self.search_input.len() {
                        self.search_cursor_position += 1;
                    }
                }
                _ => {}
            }
            return;
        }

        if self.show_sql_window {
            match key_event.code {
                KeyCode::Esc => {
                    if !self.sql_input.trim().is_empty() {
                        self.saved_manuscript = Some(self.sql_input.clone());
                    }
                    self.show_sql_window = false;
                    self.sql_result = None;
                }
                KeyCode::Enter => {
                    if key_event.modifiers.contains(event::KeyModifiers::CONTROL) {
                    } else {
                        self.sql_input.insert(self.sql_cursor_position, '\n');
                        self.sql_cursor_position += 1;
                    }
                }
                KeyCode::Char(c) => {
                    self.sql_input.insert(self.sql_cursor_position, c);
                    self.sql_cursor_position += 1;
                }
                KeyCode::Backspace => {
                    if self.sql_cursor_position > 0 {
                        self.sql_input.remove(self.sql_cursor_position - 1);
                        self.sql_cursor_position -= 1;
                    }
                }
                KeyCode::Left => {
                    if self.sql_cursor_position > 0 {
                        self.sql_cursor_position -= 1;
                    }
                }
                KeyCode::Right => {
                    if self.sql_cursor_position < self.sql_input.len() {
                        self.sql_cursor_position += 1;
                    }
                }
                KeyCode::Up => {
                    // Find the previous newline before cursor
                    let before_cursor = &self.sql_input[..self.sql_cursor_position];
                    if let Some(current_line_start) = before_cursor.rfind('\n') {
                        // Get the previous line's start
                        if let Some(prev_line_start) = before_cursor[..current_line_start].rfind('\n') {
                            let current_col = self.sql_cursor_position - current_line_start - 1;
                            let prev_line_length = current_line_start - prev_line_start - 1;
                            let new_col = current_col.min(prev_line_length);
                            self.sql_cursor_position = prev_line_start + 1 + new_col;
                        } else {
                            // Move to the first line
                            let current_col = self.sql_cursor_position - current_line_start - 1;
                            let first_line_length = current_line_start;
                            let new_col = current_col.min(first_line_length);
                            self.sql_cursor_position = new_col;
                        }
                    }
                }
                KeyCode::Down => {
                    // Find the next newline after cursor
                    if let Some(current_line_end) = self.sql_input[self.sql_cursor_position..].find('\n') {
                        let current_line_end = current_line_end + self.sql_cursor_position;
                        // Find the current line start to calculate column position
                        let before_cursor = &self.sql_input[..self.sql_cursor_position];
                        let current_line_start = before_cursor.rfind('\n')
                            .map(|pos| pos + 1)
                            .unwrap_or(0);
                        let current_col = self.sql_cursor_position - current_line_start;

                        // Find the next line's end
                        if let Some(next_line_end) = self.sql_input[current_line_end + 1..].find('\n') {
                            let next_line_end = next_line_end + current_line_end + 1;
                            let next_line_length = next_line_end - (current_line_end + 1);
                            let new_col = current_col.min(next_line_length);
                            self.sql_cursor_position = current_line_end + 1 + new_col;
                        } else {
                            // Move to the last line
                            let next_line_length = self.sql_input.len() - (current_line_end + 1);
                            let new_col = current_col.min(next_line_length);
                            self.sql_cursor_position = current_line_end + 1 + new_col;
                        }
                    }
                }
                KeyCode::Tab => {
                    if !self.sql_input.trim().is_empty() {
                        self.saved_manuscript = Some(self.sql_input.clone());
                    }
                    self.show_sql_window = false;
                    self.sql_result = None;
                    self.current_tab = (self.current_tab + 1) % 3;
                }
                _ => {}
            }
            return;
        }

        // Main key handling
        match key_event.code {
            KeyCode::Char('/') => {
                if self.current_tab == 0 && !self.show_sql_window {
                    self.show_search = true;
                    self.search_input.clear();
                    self.search_cursor_position = 0;
                }
            }
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Up => {
                match self.current_tab {
                    0 => {
                        // Network tab logic
                        if !self.show_tables {
                            // Get current index in filtered_chains
                            if let Some(current_filtered_index) = self
                                .filtered_chains
                                .iter()
                                .position(|c| c.name == self.chains[self.selected_chain_index].name)
                            {
                                if current_filtered_index > 0 {
                                    // Move up in filtered list
                                    let prev_filtered_chain = &self.filtered_chains[current_filtered_index - 1];
                                    // Find corresponding index in original chains
                                    if let Some(original_index) = self.chains
                                        .iter()
                                        .position(|c| c.name == prev_filtered_chain.name)
                                    {
                                        self.selected_chain_index = original_index;
                                        if current_filtered_index < self.scroll_offset {
                                            self.scroll_offset = current_filtered_index;
                                        }
                                    }
                                }
                            }
                        } else {
                            if let Some(index) = self.selected_table_index {
                                if index > 0 {
                                    self.selected_table_index = Some(index - 1);
                                    self.update_example_data();
                                }
                            }
                        }
                    },
                    1 => {
                        // Jobs tab logic
                        if !self.jobs_status.is_empty() {
                            if self.show_job_options {
                                if self.selected_job_option > 0 {
                                    self.selected_job_option -= 1;
                                }
                            } else {
                                if self.selected_job_index > 0 {
                                    self.selected_job_index -= 1;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Down => {
                match self.current_tab {
                    0 => {
                        // Network tab logic
                        if !self.show_tables {
                            // Get current index in filtered_chains
                            if let Some(current_filtered_index) = self
                                .filtered_chains
                                .iter()
                                .position(|c| c.name == self.chains[self.selected_chain_index].name)
                            {
                                if current_filtered_index < self.filtered_chains.len() - 1 {
                                    // Move down in filtered list
                                    let next_filtered_chain = &self.filtered_chains[current_filtered_index + 1];
                                    // Find corresponding index in original chains
                                    if let Some(original_index) = self.chains
                                        .iter()
                                        .position(|c| c.name == next_filtered_chain.name)
                                    {
                                        self.selected_chain_index = original_index;
                                        if current_filtered_index >= self.scroll_offset + visible_height {
                                            self.scroll_offset = current_filtered_index - visible_height + 1;
                                        }
                                    }
                                }
                            }
                        } else {
                            if let Some(index) = self.selected_table_index {
                                let tables_len = self.chains[self.selected_chain_index].dataDictionary.len();
                                if index < tables_len - 1 {
                                    self.selected_table_index = Some(index + 1);
                                    self.update_example_data();
                                }
                            }
                        }
                    },
                    1 => {
                        // Jobs tab logic
                        if !self.jobs_status.is_empty() {
                            if self.show_job_options {
                                if self.selected_job_option < self.job_options.len() - 1 {
                                    self.selected_job_option += 1;
                                }
                            } else {
                                if self.selected_job_index < self.jobs_status.len() - 1 {
                                    self.selected_job_index += 1;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Enter => {
                match self.current_tab {
                    0 => {
                        // Network tab logic
                        if !self.show_tables {
                            if let Some(filtered_chain) = self.filtered_chains.get(self.selected_chain_index) {
                                if let Some(original_index) = self.chains.iter().position(|c| c.name == filtered_chain.name) {
                                    self.selected_chain_index = original_index;
                                }
                            }
                            self.show_tables = true;
                            self.selected_table_index = Some(0);
                            self.update_example_data();
                        } else {
                            self.show_sql_window = true;
                            self.sql_input = self.generate_initial_manuscript();
                            self.sql_cursor_position = self.sql_input.len();
                            self.current_tab = 1;
                        }
                    },
                    1 => {
                        // Jobs tab logic
                        if !self.jobs_status.is_empty() {
                            if self.show_job_options {
                                let action = self.job_options[self.selected_job_option];
                                if let Some(job) = self.jobs_status.get(self.selected_job_index) {
                                    if let Some(action_sender) = &self.action_sender {
                                        tokio::spawn({
                                            let job_manager = self.job_manager.clone();
                                            let job_name = job.name.clone();
                                            let action = action.to_string();
                                            let sender = action_sender.clone();
                                            async move {
                                                if let Ok(Some(content)) = job_manager.handle_action(&job_name, &action).await {
                                                    let _ = sender.send((action, content)).await;
                                                }
                                            }
                                        });
                                    }
                                }
                                self.show_job_options = false;
                            } else {
                                self.show_job_options = true;
                                self.selected_job_option = 0;
                            }
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Esc => {
                if self.show_job_options {
                    self.show_job_options = false;
                } else {
                    if self.job_logs.is_some() {
                        // Clear job logs when ESC is pressed
                        self.job_logs = None;
                        self.logs_scroll_position = 0;
                    } else if self.state == AppState::Started {
                        // Cancel the setup process
                        self.should_cancel_setup = true;
                        self.state = AppState::Running;
                        self.docker_setup_timer = 0;
                        self.progress1 = 0.0;
                        self.docker_setup_in_progress = false;
                    }
                    if self.saved_manuscript.is_some() || self.show_tables || self.sql_result.is_some() {
                        if !self.sql_input.trim().is_empty() {
                            self.saved_manuscript = Some(self.sql_input.clone());
                        }
                        self.show_tables = false;
                        self.show_sql_window = false;
                    }
                    if !self.search_input.is_empty() && !self.show_tables {
                        self.show_search = false;
                        self.search_input.clear();
                        self.filtered_chains = self.chains.clone();
                    }
                }
            }
            KeyCode::PageUp => {
                if !self.show_tables {
                    if self.selected_chain_index > visible_height {
                        self.selected_chain_index -= visible_height;
                    } else {
                        self.selected_chain_index = 0;
                    }
                    if self.selected_chain_index < self.scroll_offset {
                        self.scroll_offset = self.selected_chain_index;
                    }
                }
            }
            KeyCode::PageDown => {
                if !self.show_tables {
                    let new_index = self.selected_chain_index + visible_height;
                    if new_index < self.chains.len() {
                        self.selected_chain_index = new_index;
                    } else {
                        self.selected_chain_index = self.chains.len() - 1;
                    }
                    if self.selected_chain_index >= self.scroll_offset + visible_height {
                        self.scroll_offset = self.selected_chain_index - visible_height + 1;
                    }
                }
            }
            KeyCode::Tab => {
                self.current_tab = (self.current_tab + 1) % 3;
            }
            KeyCode::Char('1') => {
                self.current_tab = 0;
            }
            KeyCode::Char('2') => {
                self.current_tab = 1;
            }
            KeyCode::Char('3') => {
                self.current_tab = 2;
            }
            KeyCode::Char('e') => {
                if self.saved_manuscript.is_some() {
                    self.show_sql_window = true;
                    self.sql_input = self.saved_manuscript.clone().unwrap_or_default();
                    self.sql_cursor_position = self.sql_input.len();
                }
            }
            KeyCode::Char('r') => {
                if let Some(saved_manuscript) = &self.saved_manuscript {
                    match self.job_manager.transform_yaml_to_sql(saved_manuscript) {
                        Ok(sql) => {
                            self.transformed_sql = Some(sql);
                            self.state = AppState::Started;
                            self.should_cancel_setup = false;
                            if !self.docker_setup_in_progress {
                                tokio::spawn({
                                    let mut app = self.clone();
                                    async move {
                                        app.setup_docker().await;
                                    }
                                });
                            }
                        }
                        Err(e) => {
                            self.debug_result = Some(format!("Error transforming SQL: {}", e));
                        }
                    }
                }
            }
            KeyCode::Char('d') => {
                if self.current_tab == 1 {
                    self.show_deploy_options = true;
                    self.selected_deploy_option = 0;
                }
            }
            KeyCode::Char('c') => {
                if !self.show_tables {
                    self.selected_chain_index = 0;
                    self.selected_table_index = Some(0);
                }
                self.show_sql_window = true;
                self.sql_input = self.generate_initial_manuscript();
                self.sql_cursor_position = self.sql_input.len();
                self.current_tab = 1;
            }
            _ => {}
        }

        if self.job_logs.is_some() {
            match key_event.code {
                KeyCode::Esc => {
                    self.job_logs = None;
                    self.logs_scroll_position = 0;
                }
                KeyCode::Up => {
                    if self.logs_scroll_position > 0 {
                        self.logs_scroll_position -= 1;
                    }
                }
                KeyCode::Down => {
                    if let Some(logs) = &self.job_logs {
                        let total_lines = logs.lines().count();
                        if self.logs_scroll_position < total_lines.saturating_sub(1) {
                            self.logs_scroll_position += 1;
                        }
                    }
                }
                KeyCode::PageUp => {
                    if self.logs_scroll_position > visible_height {
                        self.logs_scroll_position -= visible_height;
                    } else {
                        self.logs_scroll_position = 0;
                    }
                }
                KeyCode::PageDown => {
                    if let Some(logs) = &self.job_logs {
                        let total_lines = logs.lines().count();
                        let new_position = self.logs_scroll_position + visible_height;
                        self.logs_scroll_position = new_position.min(total_lines.saturating_sub(1));
                    }
                }
                _ => {}
            }
            return;
        }
    }

    fn generate_initial_manuscript(&self) -> String {
        if let Some(chain) = self.chains.get(self.selected_chain_index) {
            if let Some(table_index) = self.selected_table_index {
                if let Some((table_name, _)) = chain.dataDictionary.get(table_index) {
                    let manuscript = self.job_manager.generate_initial_manuscript(&chain.databaseName, table_name);
                    return manuscript;
                }
            }
        }
        String::new()
    }

    pub async fn setup_docker(&mut self) {
        self.docker_setup_in_progress = true;
        self.docker_setup_timer = 0;
        
        if let Some(sender) = &self.update_sender {
            let _ = sender.send(AppUpdate::SteupResult(
                "Executing in the debug environment...".to_string()
            )).await;
        }
        
        let sender = self.update_sender.clone();
        let sql = self.transformed_sql.clone();
        
        match self.pre_run.setup(sender, sql).await {
            Ok(msg) => {
                if !self.should_cancel_setup {
                    if let Some(sender) = &self.update_sender {
                        let _ = sender.send(AppUpdate::SetupComplete).await;
                        let _ = sender.send(AppUpdate::SteupResult(msg)).await;
                    }
                }
            },
            Err(e) => {
                if !self.should_cancel_setup {
                    if let Some(sender) = &self.update_sender {
                        let _ = sender.send(AppUpdate::SetupFailed(
                            e.to_string(),
                            self.current_setup_step.clone().unwrap_or(SetupStep::SubmitSQLTask)
                        )).await;
                    }
                }
            }
        }
        
        self.docker_setup_in_progress = false;
    }

    // Add new method to get formatted setup progress
    pub fn get_setup_progress_lines(&self) -> Text {
        let mut lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                format!("Setting up debug environment... ({:.1}s)", 
                    self.docker_setup_timer as f64 / 10.0),
                Style::default().fg(Color::Yellow)
            )),
            Line::from("")
        ];

        let all_steps = vec![
            // SetupStep::CheckingDocker,
            // SetupStep::PullingImage,
            // SetupStep::StartingContainer,
            SetupStep::SubmitSQLTask,
            SetupStep::WaitingForExecutionResults,
        ];

        for step in all_steps {
            let (prefix, style) = match &self.current_setup_step {
                Some(current) => {
                    if *current == step {
                        match &self.setup_state {
                            SetupState::Failed(_) => ("✗", Style::default().fg(Color::Red)),
                            _ => ("⋯", Style::default().fg(Color::Yellow))
                        }
                    } else if *current > step {
                        ("✓", Style::default().fg(Color::Green))
                    } else {
                        ("○", Style::default().fg(Color::DarkGray))
                    }
                },
                None => ("○", Style::default().fg(Color::DarkGray))
            };

            lines.push(Line::from(Span::styled(
                format!("{} {}", prefix, step.as_str()),
                style
            )));
        }

        Text::from(lines)
    }

    pub fn get_setup_progress_msg(&self) -> Text {
        let mut lines = Vec::new();

        if let Some(status) = &self.debug_result {
            // Split status by both \n and literal "\\n"
            let split_lines = status.split(|c| c == '\n')
                .flat_map(|line| line.split(r"\n"));

            for line in split_lines {
                if let SetupState::Failed(_) = self.setup_state {
                    lines.push(Line::from(
                        Span::styled(line, Style::default().fg(Color::Red))
                    ));
                } else {
                    lines.push(Line::from(line));
                }
            }
        }

        Text::from(lines)
    }

    pub fn progress1(&self) -> f64 {
        self.progress1
    }

    // Add new method to filter chains
    fn filter_chains(&mut self) {
        let search_term = self.search_input.to_lowercase();
        self.filtered_chains = self.chains
            .iter()
            .filter(|chain| chain.name.to_lowercase().contains(&search_term))
            .cloned()
            .collect();
        
        // Reset selection and scroll
        if !self.filtered_chains.is_empty() {
            self.selected_chain_index = 0;
            self.scroll_offset = 0;
            
            // Update selected_chain_index to point to the first filtered chain in original array
            if let Some(first_filtered_chain) = self.filtered_chains.first() {
                if let Some(original_index) = self.chains.iter().position(|c| c.name == first_filtered_chain.name) {
                    self.selected_chain_index = original_index;
                }
            }
        }
    }

    pub fn update_jobs_status(&mut self) {
        if let Some(receiver) = &mut self.jobs_monitor_receiver {
            match receiver.try_recv() {
                Ok(JobsUpdate::Status(new_status)) => {
                    self.jobs_status = new_status;
                }
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {}
                Err(_) => {
                }
            }
        }
    }

    async fn fetch_and_decrypt_credentials() -> (String, String) {
        let response = reqwest::get(format!("{}/api/v1/metadata/np", Settings::get_chainbase_url()))
            .await
            .expect("Failed to fetch metadata")
            .json::<HashMap<String, String>>()
            .await
            .expect("Failed to parse JSON response");

        let un = response.get("un").cloned().unwrap_or_else(|| "".to_string());
        let encrypted_pw = response.get("pw").cloned().unwrap_or_else(|| "".to_string());
        let pw = App::decrypt_aes(&encrypted_pw).expect("Failed to decrypt password");

        (un, pw)
    }

    fn decrypt_aes(encrypted_text: &str) -> Result<String, Box<dyn std::error::Error>> {
        let k: &str = "a1b2c3d4e5f6g7h8";
        let key = Key::<Aes128Gcm>::from_slice(k.as_bytes());
        let decoded = base64::decode(encrypted_text)?;
        let (nonce, ciphertext) = decoded.split_at(12);
    
        let cipher = Aes128Gcm::new(key);
        let plaintext = cipher.decrypt(Nonce::from_slice(nonce), ciphertext).map_err(|_| "Failed to decrypt password")?;
        Ok(String::from_utf8(plaintext).map_err(|_| "Failed to convert plaintext to string")?)
    }

    fn check_docker_installed(&self) -> bool {
        let docker_check = Command::new("docker")
            .arg("--version")
            .output();
        
        let compose_check = Command::new("docker")
            .args(["compose", "--version"])
            .output();

        docker_check.is_ok() && compose_check.is_ok()
    }
}

const CUSTOM_LABEL_COLOR: Color = tailwind::SLATE.c200;

#[derive(Debug, Deserialize)]
struct Response {
    graphData: Vec<GraphData>,
    status: NetworkStatus,
}

#[derive(Debug, Deserialize)]
struct GraphData {
    chain: ChainData,
}

#[derive(Debug, Deserialize)]
struct ChainData {
    name: String,
    ticker: String,
    status: String,
    lastUpdate: String,
    databaseName: String,
    dataDictionary: DataDictionary,
    example: Option<HashMap<String, Vec<serde_json::Value>>>,
    overview: Option<String>,
    net: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DataDictionaryItem {
    pub name: String,
    pub dataType: String,
}

pub fn title_block(title: &str) -> Block {
    let title = Title::from(title).alignment(Alignment::Center);
    Block::new()
        .borders(Borders::NONE)
        .padding(Padding::vertical(1))
        .title(title)
        .style(Style::default().fg(CUSTOM_LABEL_COLOR))
}
