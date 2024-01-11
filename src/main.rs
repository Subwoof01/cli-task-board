pub mod app;
pub mod event;
pub mod ui;
pub mod tui;
pub mod update;
pub mod component;
pub mod settings;

use config::Config;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use color_eyre::eyre::{Result, Ok};
use ratatui::{backend::CrosstermBackend, Terminal};
use event::{Event, EventHandler};
use app::App;
use tui::Tui;
use update::update;
use settings::Settings;

fn main() -> Result<()> {
    // Set up colourful error reporting :)
    color_eyre::install()?;

    // Load settings from `config.ini`
    let settings = Config::builder()
        .add_source(config::File::with_name("config.ini"))
        .build()
        .unwrap();

    let s: HashMap<String, String> = settings.try_deserialize().unwrap();
    println!("{:?}", &s);
    let stng = Settings {
        parent_directory: String::from(&s[0]),
    };
    println!("{:?}", stng.parent_directory);

    // Create the application.
    let mut app = App::new();

    // Initialise the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app);
        //Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())

    // TODO implement card creation
    // maybe have each board be a directory
    // then each lane be a directory
    // and then each card be a file holding its own metadata,
    // metadata type depends on line number?
    // card.txt e.g.:
    // 1    name
    // 2    date made
    // 3    due date
    // 4    assigned_to
    // 5    description-->can be n lines: 
    // n    all lines from 5 onwards are counted as description

    
    // let args: Vec<String> = env::args().collect();

    // if args.len() < 2 {
        // println!("No arguments supplied.");
        // return;
    // }

    // let command = &args[1];
    // match command.as_str() {
        // "new" => new_task(&args[2], &args[3]),
        // "remove" => println!("remove"),
        // _ => println!("none"),
    // }
}

fn new_task(name: &str, description: &str) {
    println!("---New Task---");
    println!("Name: {:?}", name);
    println!("Desc: {:?}", description);
}
