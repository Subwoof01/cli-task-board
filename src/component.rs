use chrono::{Utc, DateTime};
use std::fs::*;
use color_eyre::Result;


pub trait Component {
    fn draw(&self);
}

pub struct Card {
    name: String,
    creation_date: DateTime<Utc>,
    due_date: DateTime<Utc>,
    assigned_to: String,
    board: String,
    lane: String,
    description: String
}

impl Card {
    fn save() -> Result<()> {
       let mut file = File::create("") 
    }

}

impl Component for Card {
    fn draw(&self) {
        
    }
}
