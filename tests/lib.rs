/**
 * Copyright (c) 2019 Pankaj Chaudhary
 *
 * This source code is licensed under the MIT License found in
 * the LICENSE file in the root directory of this source tree.
 */

#[cfg(test)]
use dialog_box::{calender, information, question, progress, error, warning, file_path, pick_number};

#[test]
fn test_calender() {
    assert_eq!(calender("Select a Date"), "Wednesday 01 January 2020\n");
}

#[test]
fn test_information() {
    assert_eq!(information("The information you want to display"), "Information displayed.");
}

#[test]
fn test_question() {
    assert_eq!(question("What is your name?"), "Pankaj\n");
}

#[test]
fn test_error() {
    assert_eq!(error("The Error message you want to display."), "Error displayed.");
}

#[test]
fn test_progress() {
    assert_eq!(progress(), "Process complete.");
}

#[test]
fn test_warning() {
    assert_eq!(warning("The warning message you want to display"), "Warning displayed.");
}

#[test]
fn test_file_path() {
    assert_eq!(file_path(), "/home/knoldus/examples.desktop\n");
}

#[test]
fn test_pick_number() {
    assert_eq!(pick_number("Choose a number"), "100\n");
}
