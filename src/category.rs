use strum::{Display, EnumIter};

#[derive(Debug, EnumIter, Display, PartialEq, Clone)]
pub enum Category {
  Home,
  Work,
  Personal,
}
