use strum::{IntoEnumIterator, EnumIter};
use inquire::{Select, Text, Confirm, Editor, MultiSelect};
use chrono::{DateTime, Local};
use enum_display_derive::{Display};
use core::panic;
use std::{fmt::Display, future::pending};



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
        Ok(v)  => v,
        Err(e) => panic!("{}", e)
    };

    let notes = match Confirm::new("Do you want any notes?").prompt() {
        Ok(true) => {
            match Editor::new("Write your notes:").prompt() {
                Ok(v)  => v,
                Err(e) => panic!("{}", e)
            }
        },
        Ok(false) => String::from(""),
        Err(e)    => panic!("{}", e)
    };

    let contacts = vec![
        Person { name: String::from("Ole Johnny Pettersen"), age: 56, email: String::from("oj.pettersen@gmail.com")},
        Person { name: String::from("Jon-Roger Valhammer"), age: 43, email: String::from("jonrog@online.no") }
    ];
    
    let invitees = match MultiSelect::new("Whom do you want to invite?", contacts).prompt() {
        Ok(p)  => p,
        Err(e) => panic!("{}", e)
    };

    let category = match Select::new("How do you want to categorize this event?", Category::iter().collect()).prompt() {
        Ok(c)  => c,
        Err(e) => panic!("{}", e)
    };

    let event = Event { name, notes, invitees, category };
    Ok(event)
}

#[derive(Debug)]
struct Event {
    name: String,
    // date: DateTime<Local>,
    notes: String,
    invitees: Vec<Person>,
    category: Category
}

#[derive(Debug, EnumIter, Display)]
enum Category {
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

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}); {}", self.name, self.age, self.email)
    }
}