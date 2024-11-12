use std::{io, collections::HashMap, time::Duration, cell::RefCell};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use serde::{Deserialize, Serialize};
use reqwest;
use serde_json::json;
use reqwest::header::{HeaderMap, HeaderValue};
use tokio::sync::mpsc;
use ratatui::style::{Style, Color,palette::tailwind, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::buffer::Buffer;
use ratatui::widgets::{Gauge, Widget,block::Title,Block,Borders, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui::symbols::scrollbar;
use std::time::{Instant};
use ratatui::symbols::Marker;
use crate::ui;
use crate::setup::DockerManager;

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
    pub saved_sql: Option<String>,
    pub sql_executing: bool,
    pub sql_error: Option<String>,
    pub sql_columns: Vec<Column>,
    pub sql_data: Vec<Vec<serde_json::Value>>,
    sql_sender: Option<mpsc::Sender<Result<serde_json::Value, String>>>,
    sql_receiver: Option<mpsc::Receiver<Result<serde_json::Value, String>>>,
    pub sql_timer: u64, 
    pub docker_manager: DockerManager,
    pub docker_msg: Option<String>,
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
    pub signal1: SinSignal,
    pub data1: Vec<(f64, f64)>,
    pub signal2: SinSignal,
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
    pub deploy_options: Vec<(String, bool)>, // (option name, is_enabled)
    pub transformed_sql: Option<String>,
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
            saved_sql: self.saved_sql.clone(),
            sql_executing: self.sql_executing,
            sql_error: self.sql_error.clone(),
            sql_columns: self.sql_columns.clone(),
            sql_data: self.sql_data.clone(),
            sql_sender: self.sql_sender.clone(),
            sql_receiver: None,  // Don't clone the receiver
            sql_timer: self.sql_timer,
            docker_manager: self.docker_manager.clone(),
            docker_msg: self.docker_msg.clone(),
            docker_setup_in_progress: self.docker_setup_in_progress,
            docker_setup_timer: self.docker_setup_timer,  // Initialize the timer
            setup_progress: self.setup_progress,
            setup_state: self.setup_state.clone(),
            state: self.state,
            progress_columns: self.progress_columns,
            progress1: self.progress1,
            progress_lines: RefCell::new(Vec::new()),
            should_cancel_setup: self.should_cancel_setup,  // Clone the new field
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
    pub lastUpdate: String,
    pub time_ago: String,
    pub databaseName: String,
    pub dataDictionary: HashMap<String, Vec<DataDictionaryItem>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DataDictionaryItem {
    pub name: String,
    pub dataType: String,
    pub description: String,
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
    // CheckingDocker,
    // PullingImage,
    // StartingContainer,
    ParseManuscript,
    SubmitSQLTask,
    WaitingForExecutionResults,
}

impl SetupStep {
    fn as_str(&self) -> &'static str {
        match self {
            // SetupStep::CheckingDocker => "Checking Docker installation",
            // SetupStep::PullingImage => "Pulling required images",
            // SetupStep::StartingContainer => "Starting container",
            SetupStep::ParseManuscript => "Parse manuscript yaml",
            SetupStep::SubmitSQLTask => "Submit sql task",
            SetupStep::WaitingForExecutionResults => "Waiting for execution results",
        }
    }
}

impl App {
    pub async fn new() -> Self {
        let (sql_sender, sql_receiver) = mpsc::channel(32);
        let (update_sender, update_receiver) = mpsc::channel(32);

        let mut signal1 = SinSignal::new(0.2, 3.0, 18.0);
        let mut signal2 = SinSignal::new(0.1, 2.0, 10.0);
        let data1 = signal1.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        let data2 = signal2.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        
        let chains = match App::fetch_chains().await {
            Ok(data) => data,
            Err(_) => Vec::new()
        };

        App {
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
            saved_sql: None,
            sql_executing: false,
            sql_error: None,
            sql_columns: Vec::new(),
            sql_data: Vec::new(),
            sql_sender: Some(sql_sender),
            sql_receiver: Some(sql_receiver),
            sql_timer: 0,  // Initialize timer
            docker_manager: DockerManager::new(),
            docker_msg: None,
            docker_setup_in_progress: false,
            docker_setup_timer: 0,  // Initialize the timer
            setup_progress: 0.0,
            setup_state: SetupState::NotStarted,
            state: AppState::default(),
            progress_columns: 0,
            progress1: 0.0,
            progress_lines: RefCell::new(Vec::new()),
            should_cancel_setup: false,  // Initialize the new field
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
        }
    }

    async fn fetch_chains() -> Result<Vec<Chain>, reqwest::Error> {
        let url = "https://api.chainbase.com/api/v1/metadata/network_chains";

        match reqwest::get(url).await?.json::<Response>().await {
            Ok(response) => {
                Ok(response.graphData.into_iter()
                    .map(|graph_data| {
                        let mut tables = HashMap::new();
                        tables.insert("blocks".to_string(), graph_data.chain.dataDictionary.blocks);
                        tables.insert("transactions".to_string(), graph_data.chain.dataDictionary.transactions);
                        tables.insert("transactionLogs".to_string(), graph_data.chain.dataDictionary.transactionLogs);
                        
                        let time_ago = Self::calculate_time_diff(&graph_data.chain.lastUpdate);
                        
                        Chain {
                            name: graph_data.chain.name,
                            status: graph_data.chain.status,
                            lastUpdate: graph_data.chain.lastUpdate,
                            databaseName: graph_data.chain.databaseName,
                            time_ago,
                            dataDictionary: tables,
                        }
                    })
                    .collect())
            }
            Err(e) => {
                println!("Failed to parse JSON response: {:?}", e);
                Err(e)
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

    // TODO: Remove this mock data once we have real data
    fn mock_blocks_data() -> ExampleData {
        ExampleData {
            columns: vec![
                Column { name: "block_number".to_string(), type_: "bigint".to_string() },
                Column { name: "hash".to_string(), type_: "varchar(66)".to_string() },
                Column { name: "parent_hash".to_string(), type_: "varchar(66)".to_string() },
                Column { name: "nonce".to_string(), type_: "varchar(78)".to_string() },
                Column { name: "sha3_uncles".to_string(), type_: "varchar(66)".to_string() },
                Column { name: "logs_bloom".to_string(), type_: "varchar".to_string() },
                Column { name: "transactions_root".to_string(), type_: "varchar(66)".to_string() },
                Column { name: "state_root".to_string(), type_: "varchar(66)".to_string() },
                Column { name: "receipts_root".to_string(), type_: "varchar(66)".to_string() },
                Column { name: "miner".to_string(), type_: "varchar(42)".to_string() },
                Column { name: "difficulty".to_string(), type_: "varchar(78)".to_string() },
                Column { name: "total_difficulty".to_string(), type_: "varchar(78)".to_string() },
                Column { name: "size".to_string(), type_: "bigint".to_string() },
                Column { name: "extra_data".to_string(), type_: "varchar".to_string() },
                Column { name: "gas_limit".to_string(), type_: "varchar(78)".to_string() },
                Column { name: "gas_used".to_string(), type_: "varchar(78)".to_string() },
                Column { name: "block_timestamp".to_string(), type_: "timestamp".to_string() },
                Column { name: "transaction_count".to_string(), type_: "bigint".to_string() },
                Column { name: "base_fee_per_gas".to_string(), type_: "varchar(78)".to_string() },
                Column { name: "withdrawals_root".to_string(), type_: "varchar(66)".to_string() },
                Column { name: "__pk".to_string(), type_: "integer".to_string() },
            ],
            data: vec![
                vec![
                    json!(0),
                    json!("0x81005434635456a16f74ff7023fbe0bf423abbc8a8deb093ffff455c0ad3b741"),
                    json!("0x0000000000000000000000000000000000000000000000000000000000000000"),
                    json!("0x0000000000000000"),
                    json!("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"),
                    json!("0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
                    json!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
                    json!("0x3f86b09b43e3e49a41fc20a07579b79eba044253367817d5c241d23c0e2bc5c9"),
                    json!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
                    json!("0x0000000000000000000000000000000000000000"),
                    json!("0"),
                    json!("0"),
                    json!(505),
                    json!("0x"),
                    json!("0"),
                    json!("0"),
                    json!("2023-03-24 10:19:23.000"),
                    json!(0),
                    json!(null),
                    json!(null),
                    json!(0)
                ],
            ],
        }
    }

    fn mock_transaction_logs_data() -> ExampleData {
        ExampleData {
            columns: vec![
                Column { name: "block_number".to_string(), type_: "bigint".to_string() },
                Column { name: "block_timestamp".to_string(), type_: "timestamp".to_string() },
                Column { name: "transaction_hash".to_string(), type_: "varchar".to_string() },
                Column { name: "transaction_index".to_string(), type_: "integer".to_string() },
                Column { name: "log_index".to_string(), type_: "integer".to_string() },
                Column { name: "address".to_string(), type_: "varchar".to_string() },
                Column { name: "data".to_string(), type_: "varbinary".to_string() },
                Column { name: "topic0".to_string(), type_: "varchar".to_string() },
                Column { name: "topic1".to_string(), type_: "varchar".to_string() },
                Column { name: "topic2".to_string(), type_: "varchar".to_string() },
                Column { name: "topic3".to_string(), type_: "varchar".to_string() },
                Column { name: "pk".to_string(), type_: "integer".to_string() },
            ],
            data: vec![
                vec![
                    json!(1),
                    json!("2023-03-24 17:30:15.000"),
                    json!("0x1b1cc77d663d9176b791e94124eecffe49d1c69837ee6e9ed09356f2c70a065d"),
                    json!(0),
                    json!(0),
                    json!("0x2a3dd3eb832af982ec71669e178424b10dca2ede"),
                    json!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAHboRMQAGZLiEobojhGQVmJIlLToAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABY0V4XYoAAA=="),
                    json!("0x25308c93ceeed162da955b3f7ce3e3f93606579e40fb92029faa9efe27545983"),
                    json!(""),
                    json!(""),
                    json!(""),
                    json!(0)
                ],
            ],
        }
    }

    fn mock_transactions_data() -> ExampleData {
        ExampleData {
            columns: vec![
                Column { name: "hash".to_string(), type_: "varchar(66)".to_string() },
                Column { name: "nonce".to_string(), type_: "varchar(78)".to_string() },
                Column { name: "transaction_index".to_string(), type_: "integer".to_string() },
                Column { name: "from_address".to_string(), type_: "varchar(42)".to_string() },
                Column { name: "to_address".to_string(), type_: "varchar(42)".to_string() },
                Column { name: "value".to_string(), type_: "varchar(78)".to_string() },
                Column { name: "gas".to_string(), type_: "varchar(78)".to_string() },
                Column { name: "gas_price".to_string(), type_: "varchar(78)".to_string() },
                Column { name: "method_id".to_string(), type_: "varchar(10)".to_string() },
                Column { name: "input".to_string(), type_: "varbinary".to_string() },
                Column { name: "block_timestamp".to_string(), type_: "timestamp".to_string() },
                Column { name: "block_number".to_string(), type_: "bigint".to_string() },
                Column { name: "block_hash".to_string(), type_: "varchar(66)".to_string() },
                Column { name: "max_fee_per_gas".to_string(), type_: "varchar(78)".to_string() },
                Column { name: "max_priority_fee_per_gas".to_string(), type_: "varchar(78)".to_string() },
                Column { name: "transaction_type".to_string(), type_: "integer".to_string() },
                Column { name: "receipt_cumulative_gas_used".to_string(), type_: "varchar(78)".to_string() },
                Column { name: "receipt_gas_used".to_string(), type_: "varchar(78)".to_string() },
                Column { name: "receipt_contract_address".to_string(), type_: "varchar(42)".to_string() },
                Column { name: "receipt_status".to_string(), type_: "integer".to_string() },
                Column { name: "receipt_effective_gas_price".to_string(), type_: "varchar(78)".to_string() },
                Column { name: "__pk".to_string(), type_: "integer".to_string() },
            ],
            data: vec![
                vec![
                    json!("0x81005434635456a16f74ff7023fbe0bf423abbc8a8deb093ffff455c0ad3b741"),
                    json!("0x0"),
                    json!(0),
                    json!("0x742d35Cc6634C0532925a3b844Bc454e4438f44e"),
                    json!("0x1234567890123456789012345678901234567890"),
                    json!("1000000000000000000"),
                    json!("21000"),
                    json!("20000000000"),
                    json!("0x"),
                    json!("0x"),
                    json!("2023-10-31 03:52:35.000"),
                    json!(12345678),
                    json!("0x0000000000000000000000000000000000000000000000000000000000000000"),
                    json!("30000000000"),
                    json!("2000000000"),
                    json!(2),
                    json!("21000"),
                    json!("21000"),
                    json!(null),
                    json!(1),
                    json!("20000000000"),
                    json!(0),
                ],
            ],
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
                self.update(terminal.size()?.width);
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
                            self.docker_msg = Some(msg);
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
                            self.docker_msg = Some(format!("Error: {}", error));
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
                        self.docker_msg = Some("Channel closed".to_string());
                    }
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

    fn update(&mut self, terminal_width: u16) {
        if self.should_cancel_setup {
            // Reset everything if cancellation is requested
            self.progress1 = 0.0;
            self.docker_setup_timer = 0;
            return;
        }

        let total_duration = 1200;
        self.progress1 = (self.docker_setup_timer as f64 * 100.0 / total_duration as f64).min(100.0);
        
        if self.progress1 >= 100.0 {
            self.state = AppState::Running;
            self.docker_setup_timer = 0;
        }
    }

    pub fn update_example_data(&mut self) {
        if let Some(selected_chain) = self.chains.get(self.selected_chain_index) {
            // Check if chain is offline
            if selected_chain.status == "Offline" {
                self.example_data = None;
                return;
            }

            if let Some(table_index) = self.selected_table_index {
                let table_name = selected_chain.dataDictionary
                    .keys()
                    .nth(table_index)
                    .map(|s| s.as_str());

                self.example_data = match table_name {
                    Some("blocks") => Some(Self::mock_blocks_data()),
                    Some("transactions") => Some(Self::mock_transactions_data()),
                    Some("transactionLogs") => Some(Self::mock_transaction_logs_data()),
                    _ => None,
                };
            }
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent, visible_height: usize) {
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
                        self.create_config_file();
                    }
                    self.show_deploy_options = false;
                }
                _ => {}
            }
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
        } else if self.show_sql_window {
            match key_event.code {
                KeyCode::Esc => {
                    // Save the SQL when closing the window
                    if !self.sql_input.trim().is_empty() {
                        self.saved_sql = Some(self.sql_input.clone());
                    }
                    // Reset SQL window state
                    self.show_sql_window = false;
                    self.sql_result = None;
                    // Don't clear the selected table index anymore
                }
                KeyCode::Enter => {
                    // Execute SQL when Ctrl+Enter is pressed
                    if key_event.modifiers.contains(event::KeyModifiers::CONTROL) {
                        // tokio::spawn({
                        //     let mut app = self.clone();
                        //     async move {
                        //         app.execute_sql().await;
                        //         app
                        //     }
                        // });
                    } else {
                        // Insert newline at cursor position
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
                        self.saved_sql = Some(self.sql_input.clone());
                    }
                    self.show_sql_window = false;
                    self.sql_result = None;
                    self.current_tab = (self.current_tab + 1) % 3;
                }
                _ => {}
            }
        } else {
            match key_event.code {
                KeyCode::Char('\\') => {
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
                            // Manuscripts tab logic
                            if self.saved_sql.is_some() {
                                self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
                                self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
                            }
                        },
                        _ => {}
                    }
                },
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
                            // Manuscripts tab logic
                            if self.saved_sql.is_some() {
                                self.vertical_scroll = self.vertical_scroll.saturating_add(1);
                                self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
                            }
                        },
                        _ => {}
                    }
                },
                KeyCode::Enter => {
                    if !self.show_tables {
                        if let Some(filtered_chain) = self.filtered_chains.get(self.selected_chain_index) {
                            if let Some(original_index) = self.chains.iter().position(|c| c.name == filtered_chain.name) {
                                self.selected_chain_index = original_index;
                            }
                        }
                        self.show_tables = true;
                        self.selected_table_index = Some(0);
                        self.update_example_data();
                    }
                }
                KeyCode::Char('c') => {
                    if self.show_tables {
                        self.show_sql_window = true;
                        self.sql_input = self.generate_initial_manuscript();
                        self.sql_cursor_position = self.sql_input.len();
                        self.current_tab = 1;  // Switch to tab 2 (index 1)
                    }
                }
                KeyCode::Esc => {
                    if self.state == AppState::Started {
                        // Cancel the setup process
                        self.should_cancel_setup = true;
                        self.state = AppState::Running;
                        self.docker_setup_timer = 0;
                        self.progress1 = 0.0;
                        self.docker_setup_in_progress = false;
                    }
                    if self.saved_sql.is_some() || self.show_tables || self.sql_result.is_some() {
                        if !self.sql_input.trim().is_empty() {
                            self.saved_sql = Some(self.sql_input.clone());
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
                    if self.saved_sql.is_some() {
                        self.show_sql_window = true;
                        self.sql_input = self.saved_sql.clone().unwrap_or_default();
                        self.sql_cursor_position = self.sql_input.len();
                    }
                }
                KeyCode::Char('r') => {
                    if let Some(saved_sql) = &self.saved_sql {
                        match self.transform_yaml_to_sql(saved_sql) {
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
                                self.docker_msg = Some(format!("Error transforming SQL: {}", e));
                            }
                        }
                    }
                },
                KeyCode::Char('d') => {
                    if self.current_tab == 1 {
                        self.show_deploy_options = true;
                        self.selected_deploy_option = 0;
                        // if self.sql_result.is_none() {
                        //     self.show_warning = true;
                        // } else {
                        //     self.show_deploy_options = true;
                        //     self.selected_deploy_option = 0;
                        // }
                    }
                }
                _ => {}
            }
        }
    }

    // Add new method to generate initial SQL
    fn generate_initial_manuscript(&self) -> String {
        if let Some(chain) = self.chains.get(self.selected_chain_index) {
            if let Some(table_index) = self.selected_table_index {
                if let Some(table_name) = chain.dataDictionary.keys().nth(table_index) {
                    let table_name = if table_name == "transactionLogs" {
                        "transaction_logs"
                    } else {
                        table_name
                    };
                    let dataset_name = &chain.databaseName;
                    return format!("name: demo
specVersion: v1.0.0
parallelism: 1

sources:
  - name: {}_{} 
    type: dataset
    dataset: {}.{}

transforms:
  - name: {}_{}_{}_transform
    sql: >
      Select * From {}_{}

sinks:
  - name: {}_{}_{}_sink
    type: postgres
    from: {}_{}_{}_transform
    database: {}
    schema: public
    table: {}
    primary_key: block_number
    config:
    host: postgres
    port: 5432
    username: postgres
    password: postgres
    graphqlPort: 8082",
                        dataset_name, table_name,
                        dataset_name, table_name,
                        dataset_name, dataset_name, table_name,
                        dataset_name, table_name,
                        dataset_name, dataset_name, table_name,
                        dataset_name, dataset_name, table_name,
                        dataset_name,
                        table_name
                    );
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
        
        match self.docker_manager.setup(sender, sql).await {
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

        if let Some(status) = &self.docker_msg {
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

    fn create_config_file(&self) -> Result<(), std::io::Error> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found"))?;
        let config_path = home_dir.join(".manuscript_config.ini");
        
        let os_type = if cfg!(target_os = "darwin") {
            "darwin"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else {
            "windows"
        };

        let config_content = format!(
            "baseDir    = {}\nsystemInfo = {}\n\n\
            [demo]\n\
            baseDir     = {}/manuscript\n\
            name        = demo\n\
            specVersion =\n\
            parallelism = 0\n\
            chain       = zkevm\n\
            table       = blocks\n\
            database    = zkevm\n\
            query       = Select * From zkevm_blocks\n\
            sink        = postgres\n\
            port        = 8081\n\
            dbPort      = 15432\n\
            dbUser      = postgres\n\
            dbPassword  = postgres\n\
            graphqlPort = 8082",
            home_dir.display(),
            os_type,
            home_dir.display()
        );

        std::fs::write(config_path, config_content)?;
        Ok(())
    }

    fn transform_yaml_to_sql(&self, yaml_content: &str) -> Result<String, String> {
        let yaml: serde_yaml::Value = serde_yaml::from_str(yaml_content)
            .map_err(|e| format!("Failed to parse YAML: {}", e))?;

        let dataset = yaml["sources"]
            .as_sequence()
            .and_then(|sources| sources.first())
            .and_then(|source| source["dataset"].as_str())
            .ok_or_else(|| "Failed to extract dataset".to_string())?;

        let sql = yaml["transforms"]
            .as_sequence()
            .and_then(|transforms| transforms.first())
            .and_then(|transform| transform["sql"].as_str())
            .ok_or_else(|| "Failed to extract SQL query".to_string())?;

        let sql = sql.trim();

        // Check if SQL already contains the full dataset name (including schema)
        let sql = if sql.contains(dataset) {
            format!("{} limit 10", sql)
        } else {
            // Use the full dataset name which includes schema
            format!("Select * From {} limit 10", dataset)
        };

        Ok(sql)
    }
}


const GAUGE1_COLOR: Color = tailwind::RED.c800;
const CUSTOM_LABEL_COLOR: Color = tailwind::SLATE.c200;
const GAUGE2_COLOR: Color = tailwind::GREEN.c800;

#[derive(Debug, Deserialize)]
struct Response {
    graphData: Vec<GraphData>,
}

#[derive(Debug, Deserialize)]
struct GraphData {
    chain: ChainData,
}

#[derive(Debug, Deserialize)]
struct ChainData {
    name: String,
    status: String,
    lastUpdate: String,
    databaseName: String,
    dataDictionary: DataDictionary,
}

#[derive(Debug, Deserialize)]
struct DataDictionary {
    blocks: Vec<DataDictionaryItem>,
    transactions: Vec<DataDictionaryItem>,
    transactionLogs: Vec<DataDictionaryItem>,
}

pub fn title_block(title: &str) -> Block {
    let title = Title::from(title).alignment(Alignment::Center);
    Block::new()
        .borders(Borders::NONE)
        .padding(Padding::vertical(1))
        .title(title)
        .style(Style::default().fg(CUSTOM_LABEL_COLOR))
}
