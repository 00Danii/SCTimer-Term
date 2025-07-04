use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::state::TimerState;
use crate::ui::render_ui;

use crate::scramble::generate_scramble;
use ratatui::widgets::ScrollbarState;

#[derive(Debug)]
pub struct App {
    pub state: TimerState,
    pub times: Vec<(Duration, String)>,
    pub space_pressed: bool,
    pub exit: bool,
    pub scramble: String,
    pub last_solved: Option<Duration>,
    pub history_scroll: usize,
    pub history_scroll_state: ScrollbarState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: TimerState::Idle,
            times: Vec::new(),
            space_pressed: false,
            exit: false,
            scramble: generate_scramble(),
            last_solved: None,
            history_scroll: 0,
            history_scroll_state: ScrollbarState::new(0),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.kind {
                    KeyEventKind::Press => self.on_key_press(key_event),
                    KeyEventKind::Release => self.on_key_release(key_event),
                    _ => {}
                }
            }
        }

        if let TimerState::Inspection(start) = self.state {
            if start.elapsed().as_secs() >= 20 && !self.space_pressed {
                self.state = TimerState::Solving(Instant::now());
            }
        }

        Ok(())
    }

    fn on_key_press(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('q') {
            self.exit = true;
            return;
        }

        if key.code == KeyCode::Char(' ') {
            self.space_pressed = true;

            match self.state {
                TimerState::Idle => {
                    self.state = TimerState::Inspection(Instant::now());
                }
                TimerState::Inspection(_) => {
                    self.state = TimerState::ReadyToStart;
                }
                TimerState::Solving(start) => {
                    let elapsed = start.elapsed();
                    self.times.push((elapsed, self.scramble.clone()));
                    self.last_solved = Some(elapsed);
                    self.state = TimerState::Solved(elapsed);
                }
                _ => {}
            }
        }

        // Scroll historial
        if key.code == KeyCode::Up {
            if self.history_scroll + 1 < self.times.len() {
                self.history_scroll += 1;
                self.history_scroll_state = self.history_scroll_state.position(self.history_scroll);
            }
        }
        if key.code == KeyCode::Down {
            if self.history_scroll > 0 {
                self.history_scroll -= 1;
                self.history_scroll_state = self.history_scroll_state.position(self.history_scroll);
            }
        }
    }

    fn on_key_release(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char(' ') {
            self.space_pressed = false;

            match self.state {
                TimerState::ReadyToStart => {
                    self.state = TimerState::Solving(Instant::now());
                }
                TimerState::Solved(_) => {
                    self.scramble = generate_scramble();
                    self.state = TimerState::Idle;
                }
                _ => {}
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        render_ui(self, frame);
    }

    // calculo de tiempos promedios
    pub fn average(&self, n: usize) -> Option<f64> {
        let len = self.times.len();
        if len < n {
            return None;
        }
        let sum: f64 = self
            .times
            .iter()
            .rev()
            .take(n)
            .map(|(d, _)| d.as_secs_f64())
            .sum();
        Some(sum / n as f64)
    }
}
