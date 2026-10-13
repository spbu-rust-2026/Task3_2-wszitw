use std::collections::VecDeque;

use task3_2_template::add_command;

fn queue(commands: &[&str]) -> VecDeque<String> {
    commands
        .iter()
        .map(|command| (*command).to_owned())
        .collect()
}

fn assert_commands(actual: &VecDeque<String>, expected: &[&str]) {
    let actual: Vec<&str> = actual.iter().map(String::as_str).collect();
    assert_eq!(actual, expected);
}

#[test]
fn command_is_added_to_empty_history() {
    let mut history = VecDeque::new();
    add_command(&mut history, "ls".to_owned(), 3);
    assert_commands(&history, &["ls"]);
}

#[test]
fn commands_keep_their_order_below_capacity() {
    let mut history = queue(&["pwd"]);
    add_command(&mut history, "ls".to_owned(), 3);
    assert_commands(&history, &["pwd", "ls"]);
}

#[test]
fn history_can_reach_its_capacity() {
    let mut history = queue(&["pwd"]);
    add_command(&mut history, "ls".to_owned(), 2);
    assert_commands(&history, &["pwd", "ls"]);
}

#[test]
fn oldest_command_is_removed_when_capacity_is_exceeded() {
    let mut history = queue(&["pwd", "ls"]);
    add_command(&mut history, "date".to_owned(), 2);
    assert_commands(&history, &["ls", "date"]);
}

#[test]
fn multiple_additions_keep_only_the_last_commands() {
    let mut history = VecDeque::new();
    for command in ["one", "two", "three", "four", "five"] {
        add_command(&mut history, command.to_owned(), 3);
    }
    assert_commands(&history, &["three", "four", "five"]);
}

#[test]
fn capacity_of_one_keeps_only_the_latest_command() {
    let mut history = queue(&["old"]);
    add_command(&mut history, "new".to_owned(), 1);
    assert_commands(&history, &["new"]);
}

#[test]
fn capacity_larger_than_history_does_not_remove_commands() {
    let mut history = queue(&["one", "two"]);
    add_command(&mut history, "three".to_owned(), 10);
    assert_commands(&history, &["one", "two", "three"]);
}

#[test]
fn zero_capacity_keeps_no_commands() {
    let mut history = VecDeque::new();
    add_command(&mut history, "discarded".to_owned(), 0);
    assert_commands(&history, &[]);
}

#[test]
fn duplicate_commands_are_kept_as_separate_entries() {
    let mut history = queue(&["ls"]);
    add_command(&mut history, "ls".to_owned(), 3);
    assert_commands(&history, &["ls", "ls"]);
}

#[test]
fn empty_command_can_be_stored() {
    let mut history = VecDeque::new();
    add_command(&mut history, String::new(), 2);
    assert_commands(&history, &[""]);
}

#[test]
fn unicode_commands_are_kept() {
    let mut history = queue(&["помощь"]);
    add_command(&mut history, "猫".to_owned(), 2);
    assert_commands(&history, &["помощь", "猫"]);
}

#[test]
fn spaces_inside_a_command_are_preserved() {
    let mut history = VecDeque::new();
    add_command(&mut history, "git status --short".to_owned(), 2);
    assert_commands(&history, &["git status --short"]);
}

#[test]
fn oldest_is_removed_once_per_new_command() {
    let mut history = queue(&["first", "second", "third"]);
    add_command(&mut history, "fourth".to_owned(), 3);
    assert_commands(&history, &["second", "third", "fourth"]);
}
