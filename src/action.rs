use strum::{Display, EnumIter};

#[derive(EnumIter, Display, Clone, Copy)]
pub enum Action {
  New,
  Manage,
  Quit,
}
