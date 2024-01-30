use std::path::Path;

use logos::{Lexer, Logos};

use crate::lexer::token::Token;

// TODO: The source that is passed to the lexer could be a buffer of a file that is being read.
// Then the file could be read token by token and immediately written to the output file on a
// token token by token basis until a specific command token is hit. If a specific command is hit
// then a specific logic would be applied either by initializing a new parser to look deeper in case of
// replace commands, or copy of a file and adjustment of the command contents would be performed.

// TODO: Make this a ReplaceCommand, and CopyCommand. These can be a set of commands.
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
    source: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: Token::lexer(source),
            source,
        }
    }

    pub fn parse(self) -> Result<(), String> {
        let mut lexer = self.lexer;

        // Iterate over tokens and match specific commands:
        while let Some(result) = lexer.next() {
            match result {
                Ok(token) => {
                    // Logic is only applied for commands:
                    if token != Token::CommandName {
                        // TODO: everything that is not a command name
                        // needs to be copied to the output:
                        todo!();
                        continue;
                    }

                    match_commands(&lexer, token)?;
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

fn match_commands(lexer: &Lexer<Token>, token: Token) -> Result<(), String> {
    // TODO: Define logic for different types of commands:
    match Command::from_slice(lexer.slice()) {
        Command::ReplaceCommand => todo!(),
        Command::CopyCommand => todo!(),
        Command::NoLogic => Ok(()),
    }
}
