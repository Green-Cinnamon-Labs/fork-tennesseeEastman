// dashboard.rs

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Terminal,
};
use std::io;

// (index 0-based, tag, name, unit)
const XMEAS_META: &[(usize, &str, &str, &str)] = &[
    (0,  "XMEAS(1)",  "A Feed",                   "kscmh"),
    (1,  "XMEAS(2)",  "D Feed",                   "kg/hr"),
    (2,  "XMEAS(3)",  "E Feed",                   "kg/hr"),
    (3,  "XMEAS(4)",  "A&C Feed",                 "kscmh"),
    (4,  "XMEAS(5)",  "Recycle Flow",              "kscmh"),
    (5,  "XMEAS(6)",  "Reactor Feed Rate",         "kscmh"),
    (6,  "XMEAS(7)",  "Reactor Pressure",          "kPa"),
    (7,  "XMEAS(8)",  "Reactor Level",             "%"),
    (8,  "XMEAS(9)",  "Reactor Temperature",       "°C"),
    (9,  "XMEAS(10)", "Purge Rate",                "kscmh"),
    (10, "XMEAS(11)", "Sep Temperature",           "°C"),
    (11, "XMEAS(12)", "Sep Level",                 "%"),
    (12, "XMEAS(13)", "Sep Pressure",              "kPa"),
    (13, "XMEAS(14)", "Sep Underflow",             "m3/hr"),
    (14, "XMEAS(15)", "Stripper Level",            "%"),
    (15, "XMEAS(16)", "Stripper Pressure",         "kPa"),
    (16, "XMEAS(17)", "Stripper Underflow",        "m3/hr"),
    (17, "XMEAS(18)", "Stripper Temperature",      "°C"),
    (18, "XMEAS(19)", "Stripper Steam Flow",       "kg/hr"),
    (19, "XMEAS(20)", "Compressor Work",           "kW"),
    (20, "XMEAS(21)", "Reactor CW Outlet Temp",    "°C"),
    (21, "XMEAS(22)", "Sep CW Outlet Temp",        "°C"),
];

const XMV_META: &[(usize, &str, &str, &str)] = &[
    (0,  "XMV(1)",  "D Feed Flow",              "%"),
    (1,  "XMV(2)",  "E Feed Flow",              "%"),
    (2,  "XMV(3)",  "A Feed Flow",              "%"),
    (3,  "XMV(4)",  "A&C Feed Flow",            "%"),
    (4,  "XMV(5)",  "Compressor Recycle",       "%"),
    (5,  "XMV(6)",  "Purge Valve",              "%"),
    (6,  "XMV(7)",  "Sep Pot Liquid Flow",      "%"),
    (7,  "XMV(8)",  "Stripper Liquid Product",  "%"),
    (8,  "XMV(9)",  "Stripper Steam Valve",     "%"),
    (9,  "XMV(10)", "Reactor CW Flow",          "%"),
    (10, "XMV(11)", "Condenser CW Flow",        "%"),
    (11, "XMV(12)", "Agitator Speed",           "%"),
];

pub struct Dashboard {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Dashboard {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    /// Renders one frame. Returns false when the user presses q or Ctrl+C.
    pub fn render(&mut self, sim_time: f64, xmeas: &[f64], xmv: &[f64]) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(false),
                    KeyCode::Char('c')
                        if key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        return Ok(false);
                    }
                    _ => {}
                }
            }
        }

        self.terminal.draw(|f| {
            let area = f.area();

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(area);

            // Header
            let header = Block::default()
                .title(format!(
                    " Tennessee Eastman Process  ·  t = {:.2} s  ·  [q] quit ",
                    sim_time
                ))
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(header, layout[0]);

            // Two-column body
            let columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(layout[1]);

            // XMEAS table
            let header_style = Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::White);

            let xmeas_rows: Vec<Row> = XMEAS_META
                .iter()
                .map(|(idx, tag, name, unit)| {
                    let val = xmeas.get(*idx).copied().unwrap_or(f64::NAN);
                    Row::new(vec![
                        Cell::from(*tag).style(Style::default().fg(Color::Yellow)),
                        Cell::from(format!("{:>10.3}", val)),
                        Cell::from(*unit).style(Style::default().fg(Color::DarkGray)),
                        Cell::from(*name).style(Style::default().fg(Color::Gray)),
                    ])
                })
                .collect();

            let xmeas_table = Table::new(
                xmeas_rows,
                [
                    Constraint::Length(10),
                    Constraint::Length(11),
                    Constraint::Length(7),
                    Constraint::Fill(1),
                ],
            )
            .header(
                Row::new(["Tag", "Value", "Unit", "Name"]).style(header_style),
            )
            .block(
                Block::default()
                    .title(" Measurements (XMEAS) ")
                    .borders(Borders::ALL),
            );
            f.render_widget(xmeas_table, columns[0]);

            // XMV table
            let xmv_rows: Vec<Row> = XMV_META
                .iter()
                .map(|(idx, tag, name, unit)| {
                    let val = xmv.get(*idx).copied().unwrap_or(f64::NAN);
                    Row::new(vec![
                        Cell::from(*tag).style(Style::default().fg(Color::Green)),
                        Cell::from(format!("{:>7.2}", val)),
                        Cell::from(*unit).style(Style::default().fg(Color::DarkGray)),
                        Cell::from(*name).style(Style::default().fg(Color::Gray)),
                    ])
                })
                .collect();

            let xmv_table = Table::new(
                xmv_rows,
                [
                    Constraint::Length(8),
                    Constraint::Length(8),
                    Constraint::Length(4),
                    Constraint::Fill(1),
                ],
            )
            .header(
                Row::new(["Tag", "Value", "Unit", "Name"]).style(header_style),
            )
            .block(
                Block::default()
                    .title(" Manipulated Variables (XMV) ")
                    .borders(Borders::ALL),
            );
            f.render_widget(xmv_table, columns[1]);
        })?;

        Ok(true)
    }
}

impl Drop for Dashboard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
    }
}
