import sys
from enum import Enum

def print_promt():
    print('db >', end=' ')

class MetaCommandResult(Enum):
    META_COMMAND_SUCCESS = 0
    META_COMMAND_UNRECOGNIZED_COMMAND = 1

class PrepareResult(Enum):
    PREPARE_SUCCESS = 0
    PREPARE_UNRECOGNIZED_STATEMENT = 1

class StatementType(Enum):
    STATEMENT_INSERT = 0
    STATEMENT_SELECT = 1

class Statement:
    type = None

def do_meta_command(input_buffer):
    if input_buffer == '.exit':
        sys.exit(0)
    else:
        return MetaCommandResult.META_COMMAND_UNRECOGNIZED_COMMAND

def prepare_statement(input_buffer, statement):
    if input_buffer[:6] == 'insert':
        statement.type = StatementType.STATEMENT_INSERT
        return PrepareResult.PREPARE_SUCCESS
    elif 'select' in input_buffer:
        statement.type = StatementType.STATEMENT_SELECT
        return PrepareResult.PREPARE_SUCCESS
    return PrepareResult.PREPARE_UNRECOGNIZED_STATEMENT

def execute_statement(statement):
    if statement.type == StatementType.STATEMENT_INSERT:
        print('This is where we would do an insert.')
    elif statement.type == StatementType.STATEMENT_SELECT:
        print('This is where we would do a select.')

def main():
    while True:
        print_promt()
        user_input = input()
        if user_input[0] == '.':
            case = do_meta_command(user_input)
            if case == MetaCommandResult.META_COMMAND_SUCCESS:
                continue
            elif case == MetaCommandResult.META_COMMAND_UNRECOGNIZED_COMMAND:
                print(f'Unrecognized command {user_input}')
                continue
        
        statement = Statement()
        case = prepare_statement(user_input, statement)
        if case == PrepareResult.PREPARE_SUCCESS:
            execute_statement(statement)
            print('Executed.')
        elif case == PrepareResult.PREPARE_UNRECOGNIZED_STATEMENT:
            print(f'Unrecognized keyword at start of {user_input}')
            continue

if __name__ == '__main__':
    main()
