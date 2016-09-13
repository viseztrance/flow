extern crate flow;

use flow::ui::ansi_converter::AnsiStr;

#[test]
fn detects_ansi_escape_sequence() {
    let text_with_ansi = "[1m[36mHello[0m,[1m ncurses![0m";
    assert!(text_with_ansi.has_ansi_escape_sequence());

    let text_without_ansi = "Hello, ncurses!";
    assert!(!text_without_ansi.has_ansi_escape_sequence());
}

#[test]
fn strip_ansi_colors_from_ansi_string() {
    let expected = "Hello, ncurses!".to_string();
    let actual = "[1m[36mHello[0m,[1m ncurses![0m";

    assert_eq!(expected, actual.strip_ansi());
}

#[test]
fn strip_ansi_colors_from_normal_string() {
    let expected = "Hello, ncurses!".to_string();
    let actual = "Hello, ncurses!";

    assert_eq!(expected, actual.strip_ansi());
}

#[test]
fn strip_ansi_colors_from_fake_ansi_string() {
    let expected = "^[[1m^[[36mHello^[[0m,^[[1m ncurses!^[[0m".to_string();
    let actual = "^[[1m^[[36mHello^[[0m,^[[1m ncurses!^[[0m";

    assert_eq!(expected, actual.strip_ansi());
}

#[test]
fn breaks_text_into_ncurses_components() {
    let text_with_ansi = "[1m[36mHello[0m,[1m ncurses![0m";
    let components = text_with_ansi.break_to_ncurses_components();
    assert_eq!(8, components.len());

    let text_with_unknown_ansi = "[1m[99mHello[0m,[1m ncurses![0m";
    let components = text_with_unknown_ansi.break_to_ncurses_components();
    assert_eq!(7, components.len());
}
