use logos::{Lexer, Logos};

use crate::lexer::token::Token;

// TODO: The source that is passed to the lexer could be a buffer of a file that is being read.
// Then the file could be read token by token and immediately written to the output file on a
// token token by token basis until a specific command token is hit. If a specific command is hit
// then a specific logic would be applied either by initializing a new parser to look deeper in case of
// replace commands, or copy of a file and adjustment of the command contents would be performed.

enum Command {
    CopyCommand,
    ReplaceCommand,
    NoLogic,
}

impl Command {
    fn from_slice(slice: &str) -> Self {
        const COPY_COMMANDS: &[&str] = &[r"\includegraphics", r"usepackage"];
        const REPLACE_COMMANDS: &[&str] = &[r"\input"];

        if COPY_COMMANDS.contains(&slice) {
            return Self::CopyCommand;
        }

        match slice {
            slice if COPY_COMMANDS.contains(&slice) => Self::CopyCommand,
            slice if REPLACE_COMMANDS.contains(&slice) => Self::ReplaceCommand,
            _ => Self::NoLogic,
        }
    }
}

pub struct Parser<'a> {
    lexer: logos::Lexer<'a, Token>,
    output: String,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, output: String) -> Self {
        Self {
            lexer: Token::lexer(input),
            output,
        }
    }

    pub fn parse(mut self) -> Result<(), String> {
        let mut lexer = self.lexer;

        // Iterate over tokens and match specific commands:
        while let Some(result) = lexer.next() {
            match result {
                Ok(token) => {
                    // Just copy content if it is not a command:
                    if token != Token::CommandName {
                        self.output.push_str(lexer.slice());
                        continue;
                    }
                    match_commands(&mut lexer, token)?;
                }
                Err(_) => {
                    return Err(format!(
                        "Unrecognized Token! Add support for: {}",
                        lexer.slice()
                    ))
                }
            }
        }

        Ok(())
    }
}

// TODO: Add docstring
fn match_commands(lexer: &mut Lexer<Token>, token: Token) -> Result<(), String> {
    // TODO: Define logic for different types of commands:
    match Command::from_slice(lexer.slice()) {
        Command::ReplaceCommand => replace_content(lexer),
        Command::CopyCommand => copy_file(lexer),
        Command::NoLogic => Ok(()),
    }
}

// TODO: Add Docstring:
fn copy_file(lexer: &mut Lexer<Token>) -> Result<(), String> {
    // TODO: Find the opening brace
    get_command_content(lexer)?;
    // TODO: Get content up until the closing brace

    Ok(())
}

// TODO: Add Docstring
fn replace_content(lexer: &mut Lexer<Token>) -> Result<(), String> {
    // TODO: Find the opening brace
    get_command_content(lexer)?;

    // TODO: Get content up until the closing brace

    Ok(())
}

// TODO: Add docstring:
fn get_command_content(lexer: &mut Lexer<Token>) -> Result<String, String> {
    let mut content = String::new();
    let mut record = false;

    // REVIEW: This seems like a little too convoluted logic?
    // Maybe it would be better to use an if statment instead of match token?
    while let Some(result) = lexer.next() {
        match result {
            Ok(token) => match token {
                Token::BraceOpen => {
                    record = true;
                }
                Token::BraceClose => {
                    return Ok(content);
                }
                _ => {
                    if record {
                        content.push_str(lexer.slice())
                    }
                }
            },
            Err(_) => {
                return Err(format!(
                    "Unrecognized Token! Add support for: {}",
                    lexer.slice()
                ))
            }
        }
    }

    Err("Could not obtain the command content!".to_owned())
}
