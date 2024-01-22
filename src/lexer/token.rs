use logos::{Lexer, Logos, Span};
use strum_macros::EnumDiscriminants;

#[derive(Clone, Debug, Logos, PartialEq, Eq, EnumDiscriminants, clap::ValueEnum)]
pub enum Token {
    /// Left curly brace `'{'` character.
    #[token("{")]
    BraceOpen,

    /// Right curly brace `'}'` character.
    #[token("}")]
    BraceClose,

    /// Left bracket `'['` character.
    #[token("]")]
    BracketClose,

    /// Right bracket `']'` character.
    #[token("[")]
    BracketOpen,

    /// Input command, matching regex "\input".
    /// This is a command that includes another file.
    #[token(r"\input")]
    InputCommand,

    /// Input command, matching regex "\input".
    /// This is a command that includes another file.
    #[token(r"\bibliography")]
    BibliographyCommand,

    /// Use package command matching "\usepackage".
    #[token(r"\usepackage")]
    UsePackage,

    /// Command for including a graphic in the final output file.
    #[token(r"\includegraphics")]
    IncludeGraphics,

    /// A valid command name, including leading backslash `\`,
    /// matching regex `r"\\[a-zA-Z]+"`.
    #[regex(r"\\[a-zA-Z]+")]
    CommandName,

    /// Indicates an ASCII-letters only word
    /// matching regex `"[a-zA-Z]+"`.
    #[regex("[a-zA-Z./\\d]+")]
    CommandContent,

    /// Indicates an invalid command name, that should match everything
    /// escaped sequence that has invalid syntax.
    #[regex(r"\\[^a-zA-Z]")]
    InvalidCommand,

    /// Indicates a newline, either with `'\n'` or `"\r\n"`.
    #[token("\n")]
    #[token("\r\n")]
    Newline,

    /// Special escaped character `'\x'` that should be interpreted as `'x'`.
    #[token(r"\{")]
    #[token(r"\}")]
    #[token(r"\_")]
    #[token(r"\$")]
    #[token(r"\&")]
    #[token(r"\%")]
    #[token(r"\#")]
    EscapedChar,

    /// A comment, matching anyway after a non-escaped percent-sign `'%'` is encountered.
    #[regex("%.*")]
    Comment,
}

pub type SpannedToken = (Token, Span);