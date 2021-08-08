use arrayvec::ArrayString;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::process;
#[macro_use]
extern crate educe;

enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

enum PrepareResult {
    Success,
    SyntaxError,
    UnrecognizedStatement,
}

enum ExecuteResult {
    Success,
    TableFull,
    DefaultVariant,
}

#[derive(Educe)]
#[educe(Default)]
enum StatementType {
    #[educe(Default)]
    DefaultVariant,
    Insert,
    Select,
}

const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 255;
const TABLE_MAX_PAGES: usize = 100;
const TABLE_MAX_ROWS: usize = 2;

#[derive(Default, Serialize, Deserialize, Copy, Clone, Debug)]
struct Row {
    id: u32,
    username: ArrayString<COLUMN_USERNAME_SIZE>,
    email: ArrayString<COLUMN_EMAIL_SIZE>,
}

#[derive(Default)]
struct Statement {
    _type: StatementType,
    row_to_insert: Row,
}

struct Table {
    pages: Vec<Row>,
}

fn new_table() -> Table {
    return Table {
        pages: Vec::<Row>::with_capacity(TABLE_MAX_PAGES),
    };
}

fn print_prompt() {
    print!("db >");
    io::stdout().flush().unwrap();
}

fn do_meta_command(input: &String) -> MetaCommandResult {
    if input.eq(".exit") {
        process::exit(0);
    } else {
        return MetaCommandResult::UnrecognizedCommand;
    }
}

fn prepare_statement(input: &String, statement: &mut Statement) -> PrepareResult {
    if input.starts_with("insert") {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^insert (?P<id>[[:digit:]]*) (?P<username>[[:word:]]*) (?P<email>[[:word:]]*)+$"
            )
            .unwrap();
        }
        match RE.captures(input) {
            Some(caps) => {
                statement._type = StatementType::Insert;
                statement.row_to_insert.id = caps["id"].parse::<u32>().unwrap();
                statement.row_to_insert.username.push_str(&caps["username"]);
                statement.row_to_insert.email.push_str(&caps["email"]);
                return PrepareResult::Success;
            }
            None => {
                return PrepareResult::SyntaxError;
            }
        }
    } else if input.starts_with("select") {
        statement._type = StatementType::Select;
        return PrepareResult::Success;
    } else {
        return PrepareResult::UnrecognizedStatement;
    }
}

fn execute_statement(statement: &Statement, table: &mut Table) -> ExecuteResult {
    match statement._type {
        StatementType::Insert => {
            return execute_insert(statement, table);
        }
        StatementType::Select => {
            return execute_select(table);
        }
        StatementType::DefaultVariant => {
            return ExecuteResult::DefaultVariant;
        }
    }
}

fn execute_insert(statement: &Statement, table: &mut Table) -> ExecuteResult {
    if table.pages.len() >= TABLE_MAX_ROWS {
        return ExecuteResult::TableFull;
    } else {
        table.pages.push(statement.row_to_insert);
        return ExecuteResult::Success;
    }
}

fn execute_select(table: &mut Table) -> ExecuteResult {
    for row in &mut table.pages {
        println!("{} {} {}", row.id, row.username, row.email);
    }
    return ExecuteResult::Success;
}

fn main() {
    let mut table = new_table();
    loop {
        let mut input = String::new();
        print_prompt();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();
        if input.starts_with(".") {
            match do_meta_command(&input) {
                MetaCommandResult::Success => {
                    println!("SUCCESS");
                    continue;
                }
                MetaCommandResult::UnrecognizedCommand => {
                    println!("Unrecognized command '{}'", input);
                    continue;
                }
            }
        }
        let mut statement = Statement::default();
        match prepare_statement(&input, &mut statement) {
            PrepareResult::Success => (),
            PrepareResult::SyntaxError => {
                println!("Syntax error. Could not parse statement.");
                continue;
            }
            PrepareResult::UnrecognizedStatement => {
                println!("Unrecognized keyword at start of '{}'.", input);
                continue;
            }
        }
        match execute_statement(&statement, &mut table) {
            ExecuteResult::Success => {
                println!("Executed.");
            }
            ExecuteResult::TableFull => {
                println!("Error: Table full.")
            }
            ExecuteResult::DefaultVariant => {
                println!("Default")
            }
        }
    }
}
