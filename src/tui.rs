extern crate rcui;

use rcui::curses::*;
use rcui::*;

struct DisplayEvent {
	file: String,
}

fn item_list_controls<T: ToString + Clone>(item_list: ItemList<T>) -> Box<Proxy<ItemList<T>>> {
	Proxy::wrap(
		|list, context, event| match event {
			Event::KeyStroke(key) => match *key {
				KEY_NPAGE => list.page_down(),
				KEY_PPAGE => list.page_up(),
				KEY_DOWN => list.down(),
				KEY_UP => list.up(),
				key => match key as u8 as char {
					'j' => list.down(),
					'k' => list.up(),
					'\n' => context.push_event(Event::Custom(Box::new(DisplayEvent {
						file: list.items.iter().nth(list.cursor).unwrap().to_string(),
					}))),
					_ => {}
				},
			},

			Event::Custom(event) => {
				if let Some(file) = event.downcast_ref::<DisplayEvent>() {
					panic!(
						"{}", crate::parse(&file.file).unwrap_or(vec![]).first().unwrap()
					);
				}
			}
			Event::Quit => (),
		},
		item_list,
	)
}

fn title(title: &str, widget: Box<dyn Widget>) -> Box<dyn Widget> {
	let mut title = Column::wrap(vec![
		Cell::Fixed(
			3.0,
			Box::new(Text {
				text: title.to_string(),
				halign: HAlign::Left,
				valign: VAlign::Centre,
			}),
		),
		Cell::One(widget),
	]);
	title.group.focus = 1;
	title
}

pub fn display(files: Vec<String>) {
	let file_list = ItemList::new(files);
	let right_list =
		ItemList::new(crate::parse(file_list.items.iter().nth(2).unwrap()).unwrap_or(vec![]));

	Rcui::exec(title(
		"Xniffer --- jk to move up and down, TAB to switch the focus",
		Proxy::wrap(
			|row, context, event| match event {
				Event::KeyStroke(key) => match *key as u8 as char {
					'q' => context.quit(),
					'\t' => row.focus_next(),
					_ => row.handle_event(context, event),
				},

				Event::Custom(_) => {
					assert!(row.group.cells.len() == 2);
					row.group.cells[1 - row.group.focus]
						.get_widget_mut()
						.handle_event(context, event);
				}

				_ => {}
			},
			rcui::Row::new(vec![
				Cell::One(item_list_controls(file_list)),
				Cell::One(item_list_controls(right_list)),
			]),
		),
	));
}
