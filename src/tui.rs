use crossterm::{
	event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::{Duration, Instant}, sync::mpsc};
use tui::{
	backend::CrosstermBackend,
	layout::{Constraint, Direction, Layout},
	widgets::{Block, Borders, Widget},
	Terminal,
};

pub fn display(files: Vec<String>) {
	// setup terminal
	enable_raw_mode().expect("can run in raw mode");

}
