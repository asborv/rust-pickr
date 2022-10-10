use strum::{IntoEnumIterator, EnumIter};
use inquire::{Select, Text, Confirm, Editor};
use chrono::{DateTime, Local};
use enum_display_derive::{Display};
use std::fmt::Display;



fn main() {
    // prompt user for action

    let actions = Action::iter().collect();
    let action = match Select::new("What do you want to do?", actions).prompt() {
        Ok(Action::New) => {
            let event = new();
            println!("{:?}", event);
        },
        Err(e) => panic!("{}", e),
        _  => panic!("Not implemented yet")
    };


    // NEW
    // prompt user for name, date, invitees and area
}

fn new<'a>() -> Result<Event, &'a str> {
    let name = match Text::new("What should the event be called?").prompt() {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };

    let notes = match Confirm::new("Do you want any notes?").prompt() {
        Ok(true) => {
            match Editor::new("Write your notes:").prompt() {
                Ok(v) => v,
                Err(e) => panic!("{}", e)
            }
        },
        Ok(false) => String::from(""),
        Err(e) => panic!("{}", e)
    };

    // TODO invitees, area

    let event = Event { name, notes, invitees, area };
    
    Ok(event)
}

#[derive(Debug)]
struct Event {
    name: String,
    // date: DateTime<Local>,
    notes: String,
    invitees: Vec<Person>,
    area: Area
}

#[derive(Debug)]
enum Area {
    Home,
    Work,
    Personal
}

#[derive(EnumIter, Display)]
enum Action {
    New,
    Edit,
    See,
    Delete,
    Quit
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u16,
    email: String
}