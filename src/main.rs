use core::panic;
use inquire::{Confirm, Editor, MultiSelect, Select, Text};
use strum::{Display, EnumIter, IntoEnumIterator};

fn main() {
  let contacts = vec![
    Person {
      name: String::from("Ole Johnny Pettersen"),
      age: 56,
      email: String::from("oj.pettersen@gmail.com"),
    },
    Person {
      name: String::from("Jon-Roger Valhammer"),
      age: 43,
      email: String::from("jonrog@online.no"),
    },
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

      // ACTION: delete event
      Ok(Action::Delete) => {
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
            println!("Event {} successfully deleted.", to_delete.name);
          }
          Ok(false) => println!("Ok, no events deleted."),
          Err(_) => continue,
        }
      }

      // ACTION: edit event
      Ok(Action::Edit) => {
        // Early return if no events
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
                println!("Event {} successfully edited.", event.name);
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
      Ok(Action::See) => todo!("see"),

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

#[derive(Debug, PartialEq, Clone)]
struct Event {
  name: String,
  // date: DateTime<Local>,
  notes: String,
  invitees: Vec<Person>,
  category: Category,
}

impl Event {
  fn new(contacts: &Vec<Person>) -> Option<Event> {
    // Prompt user for name
    let name = match Text::new("What should the event be called?").prompt() {
      Ok(v) => v,
      Err(_) => return None,
    };

    // Prompt user for notes
    let notes = match Confirm::new("Do you want any notes?").prompt() {
      Ok(true) => match Editor::new("Write your notes:").prompt() {
        Ok(v) => v,
        Err(_) => return None,
      },
      Ok(false) => String::from(""),
      Err(_) => return None,
    };

    // Prompt user for invitees
    let invitees = match MultiSelect::new("Whom do you want to invite?", contacts.to_vec()).prompt()
    {
      Ok(p) => p,
      Err(_) => return None,
    };

    // Prompt user for category
    let category = match Select::new(
      "How do you want to categorize this event?",
      Category::iter().collect(),
    )
    .prompt()
    {
      Ok(c) => c,
      Err(_) => return None,
    };

    // Return event
    let event = Event {
      name,
      notes,
      invitees,
      category,
    };
    Some(event)
  }

  fn edit(event: &Event, field: &str, contacts: &Vec<Person>) -> Option<Event> {
    // Edit field
    let new_event = match field {
      // Edit name
      "name" => match Text::new("What should the event be called?").prompt() {
        Ok(name) => Event {
          name,
          ..event.clone()
        },
        Err(_) => return None,
      },

      // Edit notes
      "notes" => match Editor::new("Write your notes:").prompt() {
        Ok(notes) => Event {
          notes,
          ..event.clone()
        },
        Err(_) => return None,
      },

      // Edit invitees
      "invitees" => {
        match MultiSelect::new("Whom do you want to invite?", contacts.to_vec()).prompt() {
          Ok(invitees) => Event {
            invitees,
            ..event.clone()
          },
          Err(_) => return None,
        }
      }

      // Edit category
      "category" => match Select::new(
        "How do you want to categorize this event?",
        Category::iter().collect(),
      )
      .prompt()
      {
        Ok(category) => Event {
          category,
          ..event.clone()
        },
        Err(_) => return None,
      },

      // Field not found
      _ => panic!("Unimplemeted field: {}", field),
    };

    Some(new_event)
  }
}

#[derive(Debug, EnumIter, Display, PartialEq, Clone)]
enum Category {
  Home,
  Work,
  Personal,
}

#[derive(EnumIter, Display, Clone, Copy)]
enum Action {
  New,
  Edit,
  See,
  Delete,
  Quit,
}

#[derive(Debug, PartialEq, Clone)]
struct Person {
  name: String,
  age: u16,
  email: String,
}

impl std::fmt::Display for Person {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} ({}); {}", self.name, self.age, self.email)
  }
}

impl std::fmt::Display for Event {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}
