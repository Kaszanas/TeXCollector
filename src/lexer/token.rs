use logos::{Logos, Span};
use strum_macros::EnumDiscriminants;

#[derive(Clone, Debug, Logos, PartialEq, Eq, EnumDiscriminants, clap::ValueEnum)]
pub enum Token {
    #[token("*")]
    Asterix,

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

    /// A valid command name, including leading backslash `\`,
    /// matching regex `r"\\[a-zA-Z]+"`.
    #[regex(r"\\[a-zA-Z]+")]
    CommandName,

    /// Indicates an invalid command name, that should match everything
    /// escaped sequence that has invalid syntax.
    #[regex(r"\\[^a-zA-Z]")]
    InvalidCommand,

    /// Indicates a newline, either with `'\n'` or `"\r\n"`.
    #[token("\n")]
    #[token("\r\n")]
    Newline,

    #[regex(r"[ \t]+")]
    WhitespaceOrTab,

    /// Comma `'.'` character.
    #[token(".")]
    Dot,

    /// Colon `':'` character.
    #[token(":")]
    Colon,

    /// Comma `','` character.
    #[token(",")]
    Comma,

    /// Double-or-escaped backslash `"\\"`.
    #[token(r"\\")]
    DoubleBackslash,

    /// Double-or-escaped backslash `"\\"`.
    #[token(r"/")]
    ForwardSlash,

    /// Underscore `'_'` character.
    #[token("_")]
    Underscore,

    /// Indicates a valid number matching `"[0-9]+"`.
    #[regex("[0-9]+")]
    Number,

    /// Indicates an ASCII-letters only word
    /// matching regex `"[a-zA-Z]+"`.
    #[regex(r"\p{L}+")]
    Word,

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
