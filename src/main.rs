use strum::{IntoEnumIterator, EnumIter, Display};
use inquire::{Select, Text, Confirm, Editor, MultiSelect};
use core::panic;


fn main() {
    // prompt user for action

    let actions: Vec<Action> = Action::iter().collect();
    let mut events: Vec<Event> = vec![];

    loop {
        let action = match Select::new("What do you want to do?", actions.to_vec()).prompt() {
            Ok(Action::New) => {
                let event = Event::new();
                println!("{:?}", event);
                events.push(event);
            },

            Ok(Action::Delete) => {
                if events.is_empty() {
                    println!("No events left.");
                    continue;
                }

                let to_delete = match Select::new("Which event to you want to delete", events.to_vec()).prompt() {
                    Ok(e) => e,
                    Err(e) => panic!("{}", e)
                };

                match Confirm::new(&format!("Do you want to delete {}?", to_delete)).prompt() {
                    Ok(true) => {
                        events.retain(|e| *e != to_delete);
                        println!("Event {} successfully deleted.", to_delete.name);
                    },
                    Ok(false) => println!("Ok, no events deleted."),
                    Err(e) => panic!("{}", e),
                }
            }
            Ok(Action::Edit) => todo!("edit"),
            Ok(Action::See) => todo!("see"),
            Ok(Action::Quit) => {
                println!("Goodbye! 👋");
                std::process::exit(0);
            },
            Err(e)  => panic!("{}", e)
        };
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Event {
    name: String,
    // date: DateTime<Local>,
    notes: String,
    invitees: Vec<Person>,
    category: Category
}

impl Event {
    fn new() -> Event {
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
        event
    }
}

#[derive(Debug, EnumIter, Display, PartialEq, Clone)]
enum Category {
    Home,
    Work,
    Personal
}

#[derive(EnumIter, Display, Clone, Copy)]
enum Action {
    New,
    Edit,
    See,
    Delete,
    Quit
}

#[derive(Debug, PartialEq, Clone)]
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

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} for {} with {:?}", self.name, self.category, self.invitees)
    }
}