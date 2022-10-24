use strum::{Display, EnumIter};

#[derive(EnumIter, Display, Clone, Copy)]
pub enum Action {
  New,
  Edit,
  See,
  Delete,
  Quit,
}
