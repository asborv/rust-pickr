use inquire::{
    Select
};


fn main() {
    let options = vec!["home", "work", "personal"];

    let selected = match Select::new("Select one:", options).prompt() {
        Ok(v) => v,
        Err(v) => ""
    };

    println!("You selected {:?}", selected)
}
