use std::io::{self, Write};
use std::process;

enum MetaCommandResult {
    #[allow(non_camel_case_types)]
    META_COMMAND_SUCCESS,
    #[allow(non_camel_case_types)]
    META_COMMAND_UNRECOGNIZED_COMMAND
}
use crate::MetaCommandResult::META_COMMAND_SUCCESS;
use crate::MetaCommandResult::META_COMMAND_UNRECOGNIZED_COMMAND;

enum PrepareResult {
    #[allow(non_camel_case_types)]
    PREPARE_SUCCESS,
    #[allow(non_camel_case_types)]
    PREPARE_UNRECOGNIZED_STATEMENT
}
use crate::PrepareResult::PREPARE_SUCCESS;
use crate::PrepareResult::PREPARE_UNRECOGNIZED_STATEMENT;

enum StatementType {
    #[allow(non_camel_case_types)]
    STATEMENT_INSERT,
    #[allow(non_camel_case_types)]
    STATEMENT_SELECT,
    NONE
}
use crate::StatementType::STATEMENT_INSERT;
use crate::StatementType::STATEMENT_SELECT;
use crate::StatementType::NONE;

struct Statement{
    _type: StatementType
}

fn print_prompt(){
    print!("db >");
    io::stdout()
        .flush()
        .unwrap();
}


fn do_meta_command(input: &String) -> MetaCommandResult{
    if input.eq(".exit"){
        process::exit(0);
    } else {
        return META_COMMAND_UNRECOGNIZED_COMMAND;
    }
}

fn prepare_statement(input: &String, statement: &mut Statement) -> PrepareResult{
    if input.starts_with("insert"){
        statement._type = STATEMENT_INSERT;
        return PREPARE_SUCCESS;
    } else if input.starts_with("select"){
        statement._type = STATEMENT_SELECT;
        return PREPARE_SUCCESS;
    } else{
        return PREPARE_UNRECOGNIZED_STATEMENT;
    }
}

fn execute_statement(statement: &Statement){
    match statement._type{
        STATEMENT_INSERT => {
            println!("This is where we would do an insert.");
        },
        STATEMENT_SELECT => {
            println!("This is where we would do a select.");
        }
        NONE => {
            println!("NONE.");
        }
    }
}

fn main(){
    loop {
        let mut input = String::new();
        print_prompt();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();
        if input.starts_with("."){
            match do_meta_command(&input){
                META_COMMAND_SUCCESS => {
                    println!("SUCCESS");
                    continue;
                },
                META_COMMAND_UNRECOGNIZED_COMMAND => {
                    println!("Unrecognized command '{}'", input);
                    continue;
                }
            }
        }
        let mut statement = Statement{_type:NONE};
        match prepare_statement(&input, &mut statement){
            PREPARE_SUCCESS => (),
            PREPARE_UNRECOGNIZED_STATEMENT =>{
                println!("Unrecognized keyword at start of '{}'.", input);
                continue;
            }
        }
        execute_statement(&statement);
        println!("Executed.");
    }
}
