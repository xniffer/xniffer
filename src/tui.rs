use crossterm::{
	event::{self, Event as CEvent, KeyCode},
	terminal::{disable_raw_mode, enable_raw_mode},
};
use std::fs;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
	backend::CrosstermBackend,
	layout::{Alignment, Constraint, Direction, Layout},
	style::{Color, Modifier, Style},
	text::{Span, Spans},
	widgets::{
		Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
	},
	Terminal,
};

pub fn display(files: Vec<String>) {
	// setup terminal
	enable_raw_mode().expect("Raw mode");
}
