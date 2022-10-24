use crate::category::Category;
use crate::person::Person;
use inquire::{Confirm, Editor, MultiSelect, Select, Text};
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq, Clone)]
pub struct Event {
  name: String,
  // date: DateTime<Local>,
  notes: String,
  invitees: Vec<Person>,
  category: Category,
}

impl Event {
  pub fn new(contacts: &Vec<Person>) -> Option<Event> {
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
  pub fn edit(event: &Event, field: &str, contacts: &Vec<Person>) -> Option<Event> {
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

  pub fn get_name(&self) -> &str {
    &self.name
  }
}

impl std::fmt::Display for Event {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}
