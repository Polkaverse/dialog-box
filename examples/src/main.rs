extern crate dialog_box;
use dialog_box::{calender, information, question, progress, error, warning, file_path, pick_number};

fn main() {
    println!("{}", calender("Select a Date"));
    println!("{}", file_path());
    println!("{}", information("The information you want to display"));
    println!("{}", pick_number("Choose a number"));
    println!("{}", question("What is your name??"));
    println!("{}", progress());
    println!("{}", error("The Error message you want to display."));
    println!("{}", warning("The warning message you want to display"));
}
