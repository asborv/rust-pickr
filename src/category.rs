use strum::{Display, EnumIter};
use serde::{Serialize, Deserialize};

#[derive(Debug, EnumIter, Display, PartialEq, Clone, Serialize, Deserialize)]
pub enum Category {
  Home,
  Work,
  Personal,
}
