# dialog-box
<p align="left">
  <a href="https://travis-ci.org/pankajchaudhary5/dialog-box">
    <img alt="Build Status" src="https://travis-ci.org/PankajChaudhary5/dialog-box.svg?branch=master">
  </a>
  <a href="https://crates.io/crates/dialog-box">
    <img alt="Latest version" src="https://img.shields.io/crates/v/dialog-box.svg">
  </a>
  <img alt="MIT licensed" src="https://img.shields.io/badge/license-MIT-blue.svg">
  <img alt="Stability stable" src="https://img.shields.io/badge/stability-stable-green.svg">
</p>

A Rust Crate/Library which enables us to display different Dialog-box in rust program like warning, information, error. We can also take input from various dialog-box like calender, question, file path.

## Use
Add dependency in Cargo.toml
```sh
[dependencies]
dialog_box = "0.1.0"
```
Example to use dialog-box crate
```sh
extern crate dialog_box;
use dialog_box::{calender, information, question, progress, error, warning, file_path, pick_number};

fn main() {
    println!("{}", calender("Select a Date"));
    println!("{}", warning("The warning message you want to display"));
}
```
## Contributing
We thrive for the best and want you to contribute towards a better Project. See [`CONTRIBUTING.md`](CONTRIBUTING.md) for giving your valuable feedbacks and contributions.