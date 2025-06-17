use crate::app::App;
use crate::state::TimerState;
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
            Constraint::Length(6),  // Scramble e instrucciones
            Constraint::Length(10), // Timer grande
            Constraint::Min(15),    // Historial largo
        ])
        .split(size);

    // --- SCRAMBLE + INSTRUCCIONES ---
    // Carga la fuente personalizada, y convierte el scramble a FIGure
    let small_font = FIGfont::from_file("assets/mini.flf").unwrap();
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
        TimerState::Idle => {
            if let Some(last) = app.last_solved {
                format!(
                    "                                                {:.3}",
                    last.as_secs_f64()
                )
            } else {
                "                                                00:00".into()
            }
        }
        TimerState::Inspection(start) => {
            let remaining = 20.0 - start.elapsed().as_secs_f64();
            format!(
                "                                             -{:.1}",
                remaining.max(0.0)
            )
        }
        TimerState::ReadyToStart => {
            "                                                ***".to_string()
        }
        TimerState::Solving(start) => format!(
            "                                                {:.3}",
            start.elapsed().as_secs_f64()
        ),
        TimerState::Solved(duration) => format!(
            "                                                {:.3}",
            duration.as_secs_f64()
        ),
    };

    // Carga la fuente, y usa unwrap_or para un FIGure vacío si falla
    let standard_font = FIGfont::from_file("assets/alligator2.flf").unwrap();
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

    // --- PROMEDIOS ---
    let avg5 = app
        .average(5)
        .map(|a| format!("{:.3}", a))
        .unwrap_or("--".into());
    let avg12 = app
        .average(12)
        .map(|a| format!("{:.3}", a))
        .unwrap_or("--".into());
    let avg50 = app
        .average(50)
        .map(|a| format!("{:.3}", a))
        .unwrap_or("--".into());

    let averages_str = format!("Ao5: {}  Ao12: {}  Ao50: {}", avg5, avg12, avg50);

    let standard_font = FIGfont::from_file("assets/straight.flf").unwrap();
    let figure: FIGure = standard_font.convert(&averages_str).unwrap();
    let ascii_str = figure.to_string();

    let averages_lines: Vec<Line> = ascii_str
        .lines()
        .map(|line| {
            Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(Color::Yellow).bold(),
            ))
        })
        .collect();

    let averages_paragraph = Paragraph::new(Text::from(averages_lines))
        .block(Block::default().borders(Borders::ALL).title("Promedios"));

    frame.render_widget(averages_paragraph, chunks[2]);

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

    // Ajusta el chunk para que el historial quede debajo de los promedios
    let history_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(7)])
        .split(chunks[2]);

    frame.render_widget(history, history_chunk[1]);
}
