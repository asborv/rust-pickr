mod action;
mod category;
mod event;
mod person;

use inquire::{Confirm, Select, Text};
use strum::IntoEnumIterator;

use crate::action::Action;
use crate::event::Event;
use crate::person::Person;
use core::panic;
use std::fs::{self, File};
use std::path::Path;

fn main() {
  let contacts = vec![
    Person::new(
      String::from("John Doe"),
      20,
      String::from("john.doe@gmail.com"),
    ),
    Person::new(
      String::from("Jane Doe"),
      20,
      String::from("jane.doe@gmail.com"),
    ),
    Person::new(
      String::from("Ole Johnny Pettersen"),
      56,
      String::from("oj.pettersen@gmail.com"),
    ),
    Person::new(
      String::from("Jon-Roger Valhammer"),
      43,
      String::from("jonrog@online.no"),
    ),
  ];

  let actions: Vec<Action> = Action::iter().collect();
  let mut events: Vec<Event> = vec![];

  loop {
    // Prompt user for action
    match Select::new("What do you want to do?", actions.to_vec()).prompt() {
      // ACTION: new event
      Ok(Action::New) => {
        let event = match Event::new(&contacts) {
          Some(event) => event,
          None => continue,
        };

        events.push(event);
      }

      // ACTION: edit event
      Ok(Action::Manage) => {
        match Select::new(
          "What do you want to do?",
          vec!["edit", "delete", "save", "load"],
        )
        .prompt()
        {
          Ok("edit") => {
            // Skip if no events
            if events.is_empty() {
              println!("There are no events to edit.");
              continue;
            }

            // Prompt user for event to edit
            let to_edit =
              match Select::new("Which event do you want to edit?", events.to_vec()).prompt() {
                Ok(e) => e,
                Err(_) => continue,
              };

            // Fallback to original event if user cancels or does not edit
            let mut event = to_edit.clone();

            // Edit props of event until happy
            loop {
              event = match Select::new(
                "Select the property you want to edit? (OK to apply changes)",
                vec!["name", "notes", "invitees", "category", "OK"],
              )
              .prompt()
              {
                // Apply changes if any have been made
                Ok("OK") => {
                  if event != to_edit {
                    println!("Event {} successfully edited.", event.get_name());
                    events.retain(|e| *e != to_edit);
                    events.push(event);
                  } else {
                    println!("No changes made.");
                  }

                  break;
                }

                // Change some field
                Ok(field) => match Event::edit(&to_edit, field, &contacts) {
                  Some(e) => e,
                  None => continue,
                },

                //  Cancel
                Err(_) => break,
              };
            }
          }
          Ok("delete") => {
            // Skip if no events
            if events.is_empty() {
              println!("No events left.");
              continue;
            }

            // Prompt user for event to delete
            let to_delete =
              match Select::new("Which event to you want to delete?", events.to_vec()).prompt() {
                Ok(e) => e,
                Err(_) => continue,
              };

            // Confirm deletion
            match Confirm::new(&format!("Do you want to delete {}?", to_delete)).prompt() {
              Ok(true) => {
                events.retain(|e| *e != to_delete);
                println!("Event {} successfully deleted.", to_delete.get_name());
              }
              Ok(false) => println!("Ok, no events deleted."),
              Err(_) => continue,
            }
          }
          Ok("save") => {
            // Skip if no events
            if events.is_empty() {
              println!("No events to save.");
              continue;
            }

            // Prompt user for file to save to
            let name = match Text::new("Which file do you want to save to?").prompt() {
              Ok(f) => f,
              Err(_) => continue,
            };

            // Create path to file
            let path = Path::new("events").join(&name);

            // Check if file exists
            let file_exists = path.exists();

            // Serialize events
            let serialized = match serde_json::to_string(&events) {
              Ok(s) => s,
              Err(e) => panic!("{}", e),
            };

            // Confirm overwrite if file exists
            if file_exists {
              match Confirm::new(&format!(
                "File {} already exists. Do you want to overwrite it?",
                name
              ))
              .prompt()
              {
                Ok(true) => {}
                Ok(false) => {
                  println!("Ok, no events saved.");
                  continue;
                }
                Err(e) => panic!("{}", e),
              }
            }

            // Write to file if it does not exist or user confirms overwrite
            match fs::write(path, serialized) {
              Ok(_) => println!("Events successfully saved to {}.", name),
              Err(e) => panic!("{}", e),
            }
          }
          Ok("load") => todo!(),
          Err(e) => panic!("{}", e),
          Ok(_) => unreachable!(),
        };
      }
      // ACTION: quit
      Ok(Action::Quit) => {
        println!("Goodbye! 👋");
        break;
      }

      Err(e) => panic!("{}", e),
    };
  }
  std::process::exit(0);
}
