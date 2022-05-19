use rcui::curses::*;
use rcui::*;

struct DisplayEvent {
    file: String,
}

fn item_list_controls<T: ToString + Clone>(item_list: ItemList<T>) -> Box<Proxy<ItemList<T>>> {
	Proxy::wrap(
		|list, context, event| match event{
			Event::KeyStroke(key) => match *key {
				KEY_NPAGE => list.page_down(),
				KEY_PPAGE => list.page_up(),
				KEY_DOWN => list.down(),
				KEY_UP => list.up(),
				KEY_ENTER => {
					context.push_event(Event::Custom(Box::new(DisplayEvent { file: list.items.iter().nth(list.cursor).unwrap().to_string()})))
				},
				key => match key as u8 as char {
					'j' => list.down(),
					'k' => list.up(),
					_ => {}
				},
            },

			Event::Custom(event) => {
                if let Some(file) = event.downcast_ref::<DisplayEvent>() {
                    list = &mut ItemList::new(crate::parse(&file.file).unwrap_or(vec!()));
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
	let file_list = ItemList::new(crate::utils::vec_str_path_name(files));
	let right_list = ItemList::new(crate::parse(file_list.items.iter().nth(2).unwrap()).unwrap_or(vec!()));

	Rcui::exec(title(
		"Xniffer --- jk to move up and down, TAB to switch the focus",
		Proxy::wrap(
			|hbox, context, event| {
				if let Event::KeyStroke(key) = event {
					match *key {
						_ => ()
					}
				}
				if let Event::KeyStroke(key) = event {
					match *key as u8 as char {
						'q' => context.quit(),
						'\t' => hbox.focus_next(),
						_ => hbox.handle_event(context, event),
					}
				}
			},
			rcui::Row::new(vec![
				Cell::One(item_list_controls(file_list)),
				Cell::One(item_list_controls(right_list)),
			]),
		),
	));
}
