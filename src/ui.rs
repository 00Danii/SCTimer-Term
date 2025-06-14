use crate::app::App;
use crate::state::TimerState;
// use ratatui::prelude::Widget;
use figlet_rs::{FIGfont, FIGure};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render_ui(app: &App, frame: &mut Frame) {
    let size = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(10), // Scramble e instrucciones
            Constraint::Length(8),  // Timer grande
            Constraint::Min(10),    // Historial largo
        ])
        .split(size);

    // --- SCRAMBLE + INSTRUCCIONES ---
    // Carga la fuente estándar, y convierte el scramble a FIGure
    // let standard_font = FIGfont::standard().unwrap();
    let small_font = FIGfont::from_file("assets/lcd.flf").unwrap();
    let scramble_figure = small_font.convert(&app.scramble);

    let scramble_ascii = match scramble_figure {
        Some(fig) => fig.to_string(),
        None => app.scramble.clone(),
    };

    let scramble_lines: Vec<Line> = scramble_ascii
        .lines()
        .map(|line| {
            Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(Color::Green).bold(),
            ))
        })
        .collect();

    let scramble_paragraph = Paragraph::new(Text::from(scramble_lines))
        .block(Block::default().borders(Borders::ALL).title("Scramble"));

    frame.render_widget(scramble_paragraph, chunks[0]);

    // --- TIMER GRANDE ASCII ---
    let timer_text = match &app.state {
        TimerState::Idle => "00:00".into(),
        TimerState::Inspection(start) => {
            let remaining = 20.0 - start.elapsed().as_secs_f64();
            format!("Inspección: {:.1}s", remaining.max(0.0))
        }
        TimerState::ReadyToStart => "...".to_string(),
        TimerState::Solving(start) => format!("{:.3}s", start.elapsed().as_secs_f64()),
        TimerState::Solved(duration) => format!("Resuelto {:.3}s", duration.as_secs_f64()),
    };

    // Carga la fuente estándar, y usa unwrap_or para un FIGure vacío si falla
    let standard_font = FIGfont::standard().unwrap();
    let figure: FIGure = standard_font.convert(&timer_text).unwrap();

    // Convierte el FIGure a string con to_string()
    let ascii_str = figure.to_string();

    // Convierte el string en líneas para ratatui
    let lines: Vec<Line> = ascii_str
        .lines()
        .map(|line| {
            Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(Color::Yellow).bold(),
            ))
        })
        .collect();

    let timer_paragraph = Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::ALL).title("Timer"));

    frame.render_widget(timer_paragraph, chunks[1]);

    // --- HISTORIAL DE TIEMPOS + SCRAMBLES ---
    let items: Vec<ListItem> = app
        .times
        .iter()
        .rev()
        .take(50)
        .enumerate()
        .map(|(i, (duration, scramble))| {
            let content = Text::from(Line::from(vec![
                Span::raw(format!("{}: {:.3}s - ", i + 1, duration.as_secs_f64())),
                Span::styled(scramble.clone(), Style::default().fg(Color::LightMagenta)),
            ]));
            ListItem::new(content)
        })
        .collect();

    let history = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Historial (últimos 50)"),
        )
        .highlight_style(Style::default().bg(Color::Magenta));

    frame.render_widget(history, chunks[2]);
}
