use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Row, Table, TableState, Wrap},
    Frame, Terminal,
};
use std::time::{Duration, Instant};
use tokio_postgres::Config as PgConfig;

use crate::config::Config;
use crate::db::DbClient;
use crate::model::Statement;

pub struct App {
    pub config: Config,
    pub pg_config: PgConfig,
    pub statements: Vec<Statement>,
    pub table_state: TableState,
    pub page: i64,
    pub selected_index: usize,
    pub error_message: Option<String>,
    pub is_loading: bool,
    pub show_detail: bool,
    pub last_tick: Instant,
}

impl App {
    pub fn new(config: Config, pg_config: PgConfig) -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        Self {
            config,
            pg_config,
            statements: Vec::new(),
            table_state,
            page: 0,
            selected_index: 0,
            error_message: None,
            is_loading: false,
            show_detail: false,
            last_tick: Instant::now(),
        }
    }

    pub fn selected_statement(&self) -> Option<&Statement> {
        self.statements.get(self.selected_index)
    }

    pub fn next(&mut self) {
        if self.statements.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.statements.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.selected_index = i;
    }

    pub fn previous(&mut self) {
        if self.statements.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.statements.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.selected_index = i;
    }

    pub fn next_page(&mut self) {
        self.page += 1;
        self.selected_index = 0;
        self.table_state.select(Some(0));
    }

    pub fn previous_page(&mut self) {
        if self.page > 0 {
            self.page -= 1;
            self.selected_index = 0;
            self.table_state.select(Some(0));
        }
    }
}

async fn fetch_or_reconnect(app: &mut App, db_client: &mut Option<DbClient>) {
    app.is_loading = true;
    if db_client.is_none() {
        match DbClient::connect(&app.pg_config).await {
            Ok(client) => {
                *db_client = Some(client);
            }
            Err(e) => {
                app.error_message = Some(format!("Connection error: {:#}", e));
                app.is_loading = false;
                return;
            }
        }
    }

    if let Some(ref db) = db_client {
        match db.fetch_statements(app.config.page_size, app.page).await {
            Ok(stats) => {
                app.statements = stats;
                app.error_message = None;
            }
            Err(e) => {
                app.error_message = Some(format!("Query error: {:#}", e));
                // Discard broken client so next attempt reconnects
                *db_client = None;
            }
        }
    }
    app.is_loading = false;
}

pub async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    mut db_client: Option<DbClient>,
) -> Result<()> {
    let tick_rate = Duration::from_millis(250);

    // Initial fetch
    fetch_or_reconnect(&mut app, &mut db_client).await;

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(app.last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            if app.show_detail {
                                app.show_detail = false;
                            } else {
                                return Ok(());
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.next();
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.previous();
                        }
                        KeyCode::Right | KeyCode::Char('n') | KeyCode::Char('L') => {
                            app.next_page();
                            fetch_or_reconnect(&mut app, &mut db_client).await;
                        }
                        KeyCode::Left | KeyCode::Char('p') | KeyCode::Char('H') => {
                            if app.page > 0 {
                                app.previous_page();
                                fetch_or_reconnect(&mut app, &mut db_client).await;
                            }
                        }
                        KeyCode::Enter | KeyCode::Char(' ') => {
                            if !app.statements.is_empty() {
                                app.show_detail = !app.show_detail;
                            }
                        }
                        KeyCode::Char('r') => {
                            fetch_or_reconnect(&mut app, &mut db_client).await;
                        }
                        _ => {}
                    }
                }
            }
        }

        if app.config.refresh_interval > 0
            && app.last_tick.elapsed() >= Duration::from_secs(app.config.refresh_interval)
        {
            if db_client.is_some() {
                fetch_or_reconnect(&mut app, &mut db_client).await;
            }
            app.last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title / Header
    let header_text = format!(
        " PostgreSQL Statements Monitor (Page {}, Page Size: {}) ",
        app.page, app.config.page_size
    );
    let title_block = Block::default()
        .borders(Borders::ALL)
        .title(header_text)
        .style(Style::default().fg(Color::Cyan));

    let subtitle = if let Some(ref err) = app.error_message {
        Paragraph::new(format!(" Error: {}", err)).style(Style::default().fg(Color::Red))
    } else if app.is_loading {
        Paragraph::new(" Loading...").style(Style::default().fg(Color::Yellow))
    } else {
        Paragraph::new(format!(" Connected. Querying pg_stat_statements (total items in page: {})", app.statements.len()))
            .style(Style::default().fg(Color::Green))
    };
    f.render_widget(title_block, chunks[0]);
    f.render_widget(
        subtitle,
        Rect {
            x: chunks[0].x + 1,
            y: chunks[0].y + 1,
            width: chunks[0].width.saturating_sub(2),
            height: 1,
        },
    );

    // Table view
    let header_cells = [
        "User ID",
        "DB ID",
        "Calls",
        "Total Time",
        "Mean Time",
        "Min Time",
        "Max Time",
        "Rows",
        "Query",
    ]
    .iter()
    .map(|h| Span::styled(*h, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.statements.iter().map(|item| {
        let cells = vec![
            item.userid.to_string(),
            item.dbid.to_string(),
            item.calls.to_string(),
            item.formatted_total_time(),
            item.formatted_mean_time(),
            item.formatted_min_time(),
            item.formatted_max_time(),
            item.rows.to_string(),
            item.query.replace('\n', " "),
        ];
        Row::new(cells)
    });

    let widths = [
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(14),
        Constraint::Length(12),
        Constraint::Length(12),
        Constraint::Length(12),
        Constraint::Length(10),
        Constraint::Min(20),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(" Queries "))
        .row_highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(table, chunks[1], &mut app.table_state);

    // Footer / Controls
    let footer_text = Line::from(vec![
        Span::styled(" ▲/▼ (j/k)", Style::default().fg(Color::Yellow)),
        Span::raw(": Navigate | "),
        Span::styled("◄/► (p/n)", Style::default().fg(Color::Yellow)),
        Span::raw(": Prev/Next Page | "),
        Span::styled("Enter/Space", Style::default().fg(Color::Yellow)),
        Span::raw(": Detail View | "),
        Span::styled("r", Style::default().fg(Color::Yellow)),
        Span::raw(": Refresh/Reconnect | "),
        Span::styled("q / Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": Quit"),
    ]);

    let footer = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::ALL).title(" Controls "));
    f.render_widget(footer, chunks[2]);

    // Detail modal overlay
    if app.show_detail {
        if let Some(stmt) = app.selected_statement() {
            let area = centered_rect(80, 70, f.area());
            f.render_widget(Clear, area);

            let detail_text = vec![
                Line::from(vec![Span::styled("User ID: ", Style::default().fg(Color::Yellow)), Span::raw(stmt.userid.to_string())]),
                Line::from(vec![Span::styled("Database ID: ", Style::default().fg(Color::Yellow)), Span::raw(stmt.dbid.to_string())]),
                Line::from(vec![Span::styled("Query ID: ", Style::default().fg(Color::Yellow)), Span::raw(stmt.queryid.to_string())]),
                Line::from(vec![Span::styled("Calls: ", Style::default().fg(Color::Yellow)), Span::raw(stmt.calls.to_string())]),
                Line::from(vec![Span::styled("Rows: ", Style::default().fg(Color::Yellow)), Span::raw(stmt.rows.to_string())]),
                Line::from(vec![Span::styled("Total Exec Time: ", Style::default().fg(Color::Yellow)), Span::raw(stmt.formatted_total_time())]),
                Line::from(vec![Span::styled("Mean Exec Time: ", Style::default().fg(Color::Yellow)), Span::raw(stmt.formatted_mean_time())]),
                Line::from(vec![Span::styled("Min Exec Time: ", Style::default().fg(Color::Yellow)), Span::raw(stmt.formatted_min_time())]),
                Line::from(vec![Span::styled("Max Exec Time: ", Style::default().fg(Color::Yellow)), Span::raw(stmt.formatted_max_time())]),
                Line::from(vec![Span::styled("Stddev Exec Time: ", Style::default().fg(Color::Yellow)), Span::raw(stmt.formatted_stddev_time())]),
                Line::from(""),
                Line::from(Span::styled("Query:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
                Line::from(stmt.query.as_str()),
            ];

            let detail_paragraph = Paragraph::new(detail_text)
                .block(
                    Block::default()
                        .title(" Query Details (Press Esc/q or Enter to close) ")
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black)),
                )
                .wrap(Wrap { trim: false });

            f.render_widget(detail_paragraph, area);
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
