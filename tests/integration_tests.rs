use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_basic_error_analysis() {
    let mut cmd = Command::cargo_bin("popat").unwrap();
    cmd.arg("analyze")
        .arg("--error")
        .arg("SyntaxError: unexpected EOF while parsing")
        .arg("--language")
        .arg("python");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Popat says:"))
        .stdout(predicate::str::contains("parentheses"));
}

#[test]
fn test_javascript_error_analysis() {
    let mut cmd = Command::cargo_bin("popat").unwrap();
    cmd.arg("analyze")
        .arg("--error")
        .arg("ReferenceError: myVariable is not defined")
        .arg("--language")
        .arg("javascript");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Popat says:"))
        .stdout(predicate::str::contains("myVariable"));
}

#[test]
fn test_config_show() {
    let mut cmd = Command::cargo_bin("popat").unwrap();
    cmd.arg("config").arg("show");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Popat Configuration"))
        .stdout(predicate::str::contains("Personality:"));
}

#[test]
fn test_personality_change() {
    let mut cmd = Command::cargo_bin("popat").unwrap();
    cmd.arg("config")
        .arg("set-personality")
        .arg("sarcastic");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Personality set to: Sarcastic"));
}

#[test]
fn test_shell_setup_bash() {
    let temp_dir = tempdir().unwrap();
    let bashrc_path = temp_dir.path().join(".bashrc");
    
    // Set temporary HOME
    std::env::set_var("HOME", temp_dir.path());
    
    let mut cmd = Command::cargo_bin("popat").unwrap();
    cmd.arg("setup").arg("--shell").arg("bash");
    
    cmd.assert().success();
    
    // Check that hooks were added
    let bashrc_content = fs::read_to_string(bashrc_path).unwrap();
    assert!(bashrc_content.contains("# Popat Error Helper Integration"));
}

#[test]
fn test_interactive_mode() {
    let mut cmd = Command::cargo_bin("popat").unwrap();
    cmd.arg("interactive")
        .write_stdin("SyntaxError: unexpected EOF\nquit\n");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Welcome to Popat Interactive Mode"));
}
