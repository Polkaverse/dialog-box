/**
 * Copyright (c) 2019 Pankaj Chaudhary
 *
 * This source code is licensed under the MIT License found in
 * the LICENSE file in the root directory of this source tree.
 */
use std::process::Command;

pub fn calender(title: &str) -> String {
    match Command::new("zenity")
        .args(&["--calendar", "--text=Pick a day" , "--title", title])
        .output() {
            Ok(success) => String::from_utf8(success.stdout).unwrap(),
            Err(error) => error.to_string(),
    }
}

pub fn information(info: &str ) -> String {
    match Command::new("zenity")
        .args(&["--info", "--text", info])
        .output() {
        Ok(_success) => "Information displayed.".to_string(),
        Err(error) => error.to_string(),
    }
}

pub fn question(question: &str ) -> String {
    match Command::new("zenity")
        .args(&["--entry", "--text", question])
        .output() {
        Ok(success) => String::from_utf8(success.stdout).unwrap(),
        Err(error) => error.to_string(),
    }
}

pub fn progress() -> String {
    match Command::new("zenity")
        .args(&["--progress", "--pulsate"])
        .output() {
        Ok(_success) => "Process complete.".to_string(),
        Err(error) => error.to_string(),
    }
}

pub fn error(error_message: &str) -> String {
    match Command::new("zenity")
        .args(&["--error", "--text", error_message])
        .output() {
        Ok(_success) => "Error displayed.".to_string(),
        Err(error) => error.to_string(),
    }
}

pub fn warning(message: &str) -> String {
    match Command::new("zenity")
        .args(&["--warning", "--text", message])
        .output() {
        Ok(_success) => "Warning displayed.".to_string(),
        Err(error) => error.to_string(),
    }
}

pub fn file_path() -> String {
    match Command::new("zenity")
        .args(&["--file-selection"])
        .output() {
        Ok(success) => String::from_utf8(success.stdout).unwrap(),
        Err(error) => error.to_string(),
    }
}

pub fn pick_number(title: &str) -> String {
    match Command::new("zenity")
        .args(&["--scale", "--text", title])
        .output() {
        Ok(success) => String::from_utf8(success.stdout).unwrap(),
        Err(error) => error.to_string(),
    }
}
