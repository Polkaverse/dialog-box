/**
 * Copyright (c) 2019 Pankaj Chaudhary
 *
 * This source code is licensed under the MIT License found in
 * the LICENSE file in the root directory of this source tree.
 */
use std::process::Command;

/// The calender function can give the dialog-box/popup on the screen by which we can put the desired output as per our requirement. This function can return the input value.
pub fn calender(title: &str) -> String {
    match Command::new("zenity")
        .args(&["--calendar", "--text=Pick a day" , "--title", title])
        .output() {
            Ok(success) => String::from_utf8(success.stdout).unwrap(),
            Err(error) => error.to_string(),
    }
}

/// The information function can display the dialog-box/popup on the screen with the given argument.
pub fn information(info: &str ) -> String {
    match Command::new("zenity")
        .args(&["--info", "--text", info])
        .output() {
        Ok(_success) => "Information displayed.".to_string(),
        Err(error) => error.to_string(),
    }
}

/// The question function can give the dialog-box/popup on the screen by which we can put the desired answer for that question. This function can return the answer.
pub fn question(question: &str ) -> String {
    match Command::new("zenity")
        .args(&["--entry", "--text", question])
        .output() {
        Ok(success) => String::from_utf8(success.stdout).unwrap(),
        Err(error) => error.to_string(),
    }
}

/// The progress function can display the progress dialog-box/popup on the screen.
pub fn progress() -> String {
    match Command::new("zenity")
        .args(&["--progress", "--pulsate"])
        .output() {
        Ok(_success) => "Process complete.".to_string(),
        Err(error) => error.to_string(),
    }
}

/// The error function can display the error dialog-box/popup on the screen with the given error message.
pub fn error(error_message: &str) -> String {
    match Command::new("zenity")
        .args(&["--error", "--text", error_message])
        .output() {
        Ok(_success) => "Error displayed.".to_string(),
        Err(error) => error.to_string(),
    }
}

/// The warning function can display the warning dialog-box/popup on the screen with the given warning message.
pub fn warning(message: &str) -> String {
    match Command::new("zenity")
        .args(&["--warning", "--text", message])
        .output() {
        Ok(_success) => "Warning displayed.".to_string(),
        Err(error) => error.to_string(),
    }
}

/// The file_path function can give the dialog-box/popup on the screen by which we can path of our directory. This function can return the entered path of directory.
pub fn file_path() -> String {
    match Command::new("zenity")
        .args(&["--file-selection"])
        .output() {
        Ok(success) => String::from_utf8(success.stdout).unwrap(),
        Err(error) => error.to_string(),
    }
}

/// The pick_number function can give the dialog-box/popup on the screen by which we can choose any number between 1 to 100. This function can return that number.
pub fn pick_number(title: &str) -> String {
    match Command::new("zenity")
        .args(&["--scale", "--text", title])
        .output() {
        Ok(success) => String::from_utf8(success.stdout).unwrap(),
        Err(error) => error.to_string(),
    }
}
