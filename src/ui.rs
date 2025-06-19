use crate::app::App;
use crate::state::TimerState;
use figlet_rs::{FIGfont, FIGure};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation},
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
    let font_data = include_str!("../assets/mini.flf");
    let small_font = FIGfont::from_content(font_data).unwrap();
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
                Style::default().fg(Color::LightBlue).bold(),
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

    let font_data = include_str!("../assets/alligator2.flf");
    let alligator_font = FIGfont::from_content(font_data).unwrap();

    let figure: FIGure = alligator_font.convert(&timer_text).unwrap();

    // Convierte el FIGure a string con to_string()
    let ascii_str = figure.to_string();

    // Convierte el string en líneas para ratatui
    let lines: Vec<Line> = ascii_str
        .lines()
        .map(|line| {
            Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(Color::LightGreen).bold(),
            ))
        })
        .collect();

    let timer_paragraph = Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::ALL).title("Timer"));

    frame.render_widget(timer_paragraph, chunks[1]);

    // --- PANEL PRINCIPAL: Promedios a la izquierda, historial a la derecha ---
    let panel_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(80), // Promedios
            Constraint::Min(30),    // Historial
        ])
        .split(chunks[2]);

    // --- PROMEDIOS (panel izquierdo) ---
    let avg5 = app
        .average(5)
        .map(|a| format!("{:.3}", a))
        .unwrap_or("--".into());
    let avg12 = app
        .average(12)
        .map(|a| format!("{:.3}", a))
        .unwrap_or("--".into());
    let avg25 = app
        .average(25)
        .map(|a| format!("{:.3}", a))
        .unwrap_or("--".into());
    let avg50 = app
        .average(50)
        .map(|a| format!("{:.3}", a))
        .unwrap_or("--".into());

    let font_data = include_str!("../assets/small.flf");
    let small_font = FIGfont::from_content(font_data).unwrap();

    let avg5_fig = small_font
        .convert(&format!("Ao5: {}", avg5))
        .unwrap()
        .to_string();
    let avg12_fig = small_font
        .convert(&format!("Ao12: {}", avg12))
        .unwrap()
        .to_string();
    let avg25_fig = small_font
        .convert(&format!("Ao25: {}", avg25))
        .unwrap()
        .to_string();
    let avg50_fig = small_font
        .convert(&format!("Ao50: {}", avg50))
        .unwrap()
        .to_string();

    let averages_lines: Vec<Line> = avg5_fig
        .lines()
        .chain(avg12_fig.lines())
        .chain(avg25_fig.lines())
        .chain(avg50_fig.lines())
        .map(|line| {
            Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(Color::LightMagenta).bold(),
            ))
        })
        .collect();

    let averages_paragraph = Paragraph::new(Text::from(averages_lines))
        .block(Block::default().borders(Borders::ALL).title("Promedios"));

    frame.render_widget(averages_paragraph, panel_chunks[0]);

    // --- HISTORIAL DE TIEMPOS + SCRAMBLES (panel derecho) ---
    let total = app.times.len();
    let scroll = app.history_scroll;
    let visible = 19;

    let items: Vec<ListItem> = app
        .times
        .iter()
        .rev()
        .skip(scroll)
        .take(visible)
        .enumerate()
        .map(|(i, (duration, scramble))| {
            let num = total - (scroll + i);
            let content = Text::from(Line::from(vec![
                Span::raw(format!("{}: {:.3}s - ", num, duration.as_secs_f64())),
                Span::styled(scramble.clone(), Style::default().fg(Color::LightRed)),
            ]));
            ListItem::new(content)
        })
        .collect();

    let history = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Historial de tiempos"),
        )
        .highlight_style(Style::default());

    frame.render_widget(history, panel_chunks[1]);

    // --- BARRA DE SCROLL ---
    let total_items = app.times.len();
    let visible_items = visible;
    let mut scroll_state = app.history_scroll_state.content_length(total_items);

    if total_items > visible_items {
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight),
            panel_chunks[1],
            &mut scroll_state,
        );
    }
}
