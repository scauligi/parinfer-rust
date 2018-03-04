extern crate regex;

use regex::Regex;
use std::collections::HashMap;

//------------------------------------------------------------------------------
// Constants / Predicates
//------------------------------------------------------------------------------

const BACKSLASH : &'static str = "\\";
const BLANK_SPACE : &'static str = " ";
const DOUBLE_SPACE : &'static str = "  ";
const DOUBLE_QUOTE : &'static str = "\"";
const NEWLINE : &'static str = "\n";
const SEMICOLON : &'static str = ";";
const TAB : &'static str = "\t";

const LINE_ENDING_REGEX : &'static str = r"\r?\n";

fn match_paren(paren: &str) -> Option<&'static str> {
    match paren {
        "{" => Some("}"),
        "}" => Some("{"),
        "[" => Some("]"),
        "]" => Some("["),
        "(" => Some(")"),
        ")" => Some("("),
        _ => None
    }
}

#[cfg(test)]
#[test]
fn match_paren_works() {
    assert_eq!(match_paren("}"), Some("{"));
    assert_eq!(match_paren("x"), None);
}

//------------------------------------------------------------------------------
// Options Structure
//------------------------------------------------------------------------------

pub struct Change<'a> {
    x: u32,
    line_no: u32,
    old_text: &'a str,
    new_text: &'a str,
}

struct TransformedChange<'a> {
    x: u32,
    line_no: u32,
    old_text: &'a str,
    new_text: &'a str,
    old_end_x: u32,
    new_end_x: u32,
    new_end_line_no: u32,
    lookup_line_no: u32,
    lookup_x: u32
}

fn transform_change<'a>(change: &Change<'a>) -> TransformedChange<'a> {
    unimplemented!();
}

fn transform_changes<'a>(changes: &Vec<Change<'a>>) -> HashMap<(u32, u32), TransformedChange<'a>> {
    unimplemented!();
}

pub struct Options<'a> {
    cursor_x: u32,
    cursor_line: u32,
    prev_cursor_x: Option<u32>,
    prev_cursor_line: Option<u32>,
    selection_start_line: Option<u32>,
    changes: Vec<Change<'a>>,
    partial_result: bool,
    force_balance: bool,
    return_parens: bool
}

//------------------------------------------------------------------------------
// Result Structure
//------------------------------------------------------------------------------

// This represents the running result. As we scan through each character
// of a given text, we mutate this structure to update the state of our
// system.

struct Paren {
    line_no: u32,
    ch: char,
    x: u32,
    indent_delta: i32
}

struct ParenTrailClamped {
    start_x: u32,
    end_x: u32,
    openers: Vec<Paren>
}

struct ParenTrail {
    line_no: Option<u32>,
    start_x: Option<u32>,
    end_x: Option<u32>,
    openers: Vec<Paren>,
    clamped: Option<ParenTrailClamped>
}

pub enum Mode {
    Indent,
    Paren
}

enum TrackingArgTabStop {
    NotSearching,
    Space,
    Arg
}

pub struct Result<'a> {
    mode: Mode,
    orig_text: &'a str,
    changes: HashMap<(u32, u32), TransformedChange<'a>>,
    tracking_arg_tab_stop: TrackingArgTabStop
}

fn initial_paren_trail() -> ParenTrail {
    ParenTrail {
        line_no: None,
        start_x: None,
        end_x: None,
        openers: vec![], 
        clamped: None
    }
}

fn get_initial_result<'a>(text: &'a str, options: Options<'a>, mode: Mode) -> Result<'a> {
    unimplemented!();
}

//------------------------------------------------------------------------------
// Possible Errors
//------------------------------------------------------------------------------

pub enum Error {
    ErrorQuoteDanger,
    ErrorEolBackslash,
    ErrorUnclosedQuote,
    ErrorUnclosedParen,
    ErrorUnmatchedCloseParen,
    ErrorUnmatchedOpenParen,
    ErrorUnhandled 
}

fn error_message(error: Error) -> &'static str {
    unimplemented!();
}

fn cache_error_pos(result: &mut Result, error: Error) {
    unimplemented!();
}

fn error(result: &mut Result, name: Error) {
    unimplemented!();
}

//------------------------------------------------------------------------------
// String Operations
//------------------------------------------------------------------------------

fn replace_within_string(orig: &str, start: usize, end: usize, replace: &str) -> String {
    String::from(&orig[0..start]) + replace + &orig[end..]
}

#[cfg(test)]
#[test]
fn replace_within_string_works() {
    assert_eq!(replace_within_string("aaa", 0, 2, ""), "a");
    assert_eq!(replace_within_string("aaa", 0, 1, "b"), "baa");
    assert_eq!(replace_within_string("aaa", 0, 2, "b"), "ba");
}

fn repeat_string(text: &str, n: usize) -> String {
    String::from(text).repeat(n)
}

#[cfg(test)]
#[test]
fn repeat_string_works() {
    assert_eq!(repeat_string("a", 2), "aa");
    assert_eq!(repeat_string("aa", 3), "aaaaaa");
    assert_eq!(repeat_string("aa", 0), "");
    assert_eq!(repeat_string("", 0), "");
    assert_eq!(repeat_string("", 5), "");
}

fn get_line_ending(text: &str) -> &'static str {
    unimplemented!();
}

//------------------------------------------------------------------------------
// Line operations
//------------------------------------------------------------------------------

fn is_cursor_affected<'a>(result: &Result<'a>, start: u32, end: u32) -> bool {
    unimplemented!();
}

fn shift_cursor_on_edit<'a>(result: &mut Result<'a>, line_no: u32, start: u32, end: u32, replace: &str) {
    unimplemented!();
}

fn replace_within_line<'a>(result: &mut Result<'a>, line_no: u32, start: u32, end: u32, replace: &str) {
    unimplemented!();
}

fn insert_within_line<'a>(result: &mut Result<'a>, line_no: u32, idx: u32, insert: &str) {
    unimplemented!();
}

fn init_line<'a>(result: &mut Result<'a>, line: &str) {
    unimplemented!();
}

fn commit_char<'a>(result: &mut Result<'a>, orig_ch: char) {
    unimplemented!();
}

//------------------------------------------------------------------------------
// Misc Utils
//------------------------------------------------------------------------------

fn clamp<T : Clone + Ord>(val: T, min_n: Option<T>, max_n: Option<T>) -> T {
    if let Some(low) = min_n {
        if low >= val {
            return low;
        }
    }
    if let Some(high) = max_n {
        if high <= val {
            return high;
        }
    }
    val
}

#[cfg(test)]
#[test]
fn clamp_works() {
    assert_eq!(clamp(1, Some(3), Some(5)), 3);
    assert_eq!(clamp(9, Some(3), Some(5)), 5);
    assert_eq!(clamp(1, Some(3), None), 3);
    assert_eq!(clamp(5, Some(3), None), 5);
    assert_eq!(clamp(1, None, Some(5)), 1);
    assert_eq!(clamp(9, None, Some(5)), 5);
    assert_eq!(clamp(1, None, None), 1);
}

fn peek<T>(array: &Vec<T>, i: usize) -> Option<&T> {
    if i >= array.len() {
        None
    } else {
        Some(&array[array.len() - 1 - i])
    }
}

#[cfg(test)]
#[test]
fn peek_works() {
    assert_eq!(peek(&vec!['a'], 0), Some(&'a'));
    assert_eq!(peek(&vec!['a'], 1), None);
    assert_eq!(peek(&vec!['a', 'b', 'c'], 0), Some(&'c'));
    assert_eq!(peek(&vec!['a', 'b', 'c'], 1), Some(&'b'));
    assert_eq!(peek(&vec!['a', 'b', 'c'], 5), None);
    let empty : Vec<char> = vec![];
    assert_eq!(peek(&empty, 0), None);
    assert_eq!(peek(&empty, 1), None);
}

//------------------------------------------------------------------------------
// Questions about characters
//------------------------------------------------------------------------------

fn is_open_paren(paren: &str) -> bool {
    match paren {
        "{" | "[" | "(" => true,
        _ => false
    }
}

#[cfg(test)]
#[test]
fn is_open_paren_works() {
    assert!(is_open_paren("("));
    assert!(!is_open_paren("}"));
}

fn is_close_paren(paren: &str) -> bool {
    match paren {
        "}" | "]" | ")" => true,
        _ => false
    }
}

fn is_valid_close_paren<'a>(paren_stack: &Vec<Paren>, ch: char) {
    unimplemented!();
}

fn is_whitespace<'a>(result: &Result<'a>) -> bool {
    unimplemented!();
}

fn is_closable<'a>(result: &Result<'a>) -> bool {
    unimplemented!();
}

//------------------------------------------------------------------------------
// Advanced operations on characters
//------------------------------------------------------------------------------

fn check_cursor_holding<'a>(result: &Result<'a>) -> bool {
    unimplemented!();
}

fn track_arg_tab_stop<'a>(result: &Result<'a>, state: TrackingArgTabStop) -> bool {
    unimplemented!();
}

//------------------------------------------------------------------------------
// Literal character events
//------------------------------------------------------------------------------

fn on_open_paren<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

// set_closer

fn on_matched_close_paren<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn on_unmatched_close_paren<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn on_close_paren<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn on_tab<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn on_semicolon<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn on_newline<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn on_quote<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn on_backslash<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn after_backslash<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

//------------------------------------------------------------------------------
// Character dispatch
//------------------------------------------------------------------------------

fn on_char<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

//------------------------------------------------------------------------------
// Cursor functions
//------------------------------------------------------------------------------

fn is_cursor_on_left<'a>(result: &Result<'a>) -> bool {
    unimplemented!();
}

fn is_cursor_on_right<'a>(result: &Result<'a>) -> bool {
    unimplemented!();
}

fn is_cursor_in_comment<'a>(result: &Result<'a>) -> bool {
    unimplemented!();
}

fn handle_change_delta<'a>(result: &Result<'a>) {
    unimplemented!();
}

//------------------------------------------------------------------------------
// Paren Trail functions
//------------------------------------------------------------------------------

fn reset_paren_trail<'a>(result: &mut Result<'a>, line_no: u32, x: u32) {
    unimplemented!();
}

fn clamp_paren_trail_to_cursor<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn pop_paren_trail<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn get_parent_opener_index<'a>(result: &mut Result<'a>, index_x: u32) -> u32 {
    unimplemented!();
}

fn correct_paren_trail<'a>(result: &mut Result<'a>, index_x: u32) {
    unimplemented!();
}

fn clean_paren_trail<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn append_paren_trail<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn invalidate_paren_trail<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn check_unmatched_outside_paren_trail<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn finish_new_paren_trail<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

//------------------------------------------------------------------------------
// Indentation functions
//------------------------------------------------------------------------------

fn change_indent<'a>(result: &mut Result<'a>, delta: i32) {
    unimplemented!();
}

fn correct_indent<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn on_indent<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn on_leading_close_paren<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn shift_comment_line<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn check_indent<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn init_indent<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

fn set_tab_stops<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

//------------------------------------------------------------------------------
// High-level processing functions
//------------------------------------------------------------------------------

fn process_char<'a>(result: &mut Result<'a>, ch: char) {
    unimplemented!();
}

fn process_line<'a>(reuslt: &mut Result<'a>, line_no: u32) {
    unimplemented!();
}

fn finalize_result<'a>(result: &mut Result<'a>) {
    unimplemented!();
}

// process_error

fn process_text<'a>(text: &'a str, options: Options<'a>, mode: Mode, smart: bool) -> Result<'a> {
    unimplemented!();
}

//------------------------------------------------------------------------------
// Public API
//------------------------------------------------------------------------------

fn public_result<'a>(result: Result<'a>) -> Result<'a> {
    unimplemented!();
}

pub fn indent_mode<'a>(text: &'a str, options: Options<'a>) {
    public_result(process_text(text, options, Mode::Indent, false));
}

pub fn paren_mode<'a>(text: &'a str, options: Options<'a>) {
    public_result(process_text(text, options, Mode::Paren, false));
}

pub fn smart_mode<'a>(text: &'a str, options: Options<'a>) {
    let smart = options.selection_start_line == None;
    public_result(process_text(text, options, Mode::Indent, smart));
}
