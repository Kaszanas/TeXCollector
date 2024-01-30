use logos::Logos;
use texcollector::lexer::token::Token;

macro_rules! assert_token_positions {
    ($source:expr, $token:pat, $($pos:expr),+ $(,)?) => {
        let source = $source;

        let positions: Vec<std::ops::Range<usize>> = vec![$($pos),*];
        let spanned_token: Vec<_> = Token::lexer(source)
            .spanned()
            .filter(|(token, _)| matches!(token, $token))
            .collect();


        let strs: Vec<_> = Token::lexer(source)
            .spanned()
            .map(|(token, span)| (token, source[span].to_string()))
            .collect();

        assert_eq!(
            spanned_token.len(), positions.len(),
            "The number of tokens found did not match the expected number of positions {strs:?}"
        );

        for (pos, (token, span)) in positions.into_iter().zip(spanned_token) {
            assert_eq!(
                span,
                pos,
                "Token {token:#?} was found, but expected at {pos:?}"
            );
        }
    };
}

#[test]
fn token_test_input_command() {
    // Just a word resembling an input command name and a word enclosed in braces:
    let source = r"input{blah}";
    assert_token_positions!(source, Ok(Token::BraceOpen), 5..6);
    assert_token_positions!(source, Ok(Token::Word), 0..5, 6..10);
    assert_token_positions!(source, Ok(Token::BraceClose), 10..11);

    // Title command with two inputs:
    let source = r"\title{\input{blah} \input{blah}}";
    assert_token_positions!(source, Ok(Token::WhitespaceOrTab), 19..20);
    assert_token_positions!(source, Ok(Token::BraceOpen), 6..7, 13..14, 26..27);
    assert_token_positions!(source, Ok(Token::BraceClose), 18..19, 31..32, 32..33);
    assert_token_positions!(source, Ok(Token::CommandName), 0..6, 7..13, 20..26);
    assert_token_positions!(source, Ok(Token::Word), 14..18, 27..31);
}

#[test]
fn token_test_utf_8_word() {
    let source = "ł";
    // The range resembles the byte range of the character in the source string:
    assert_token_positions!(source, Ok(Token::Word), 0..2);
}

#[test]
fn token_test_usepackage_command() {
    let source = r"\usepackage{arxiv}";
    assert_token_positions!(source, Ok(Token::BraceOpen), 11..12);
    assert_token_positions!(source, Ok(Token::BraceClose), 17..18);
    assert_token_positions!(source, Ok(Token::CommandName), 0..11);
    assert_token_positions!(source, Ok(Token::Word), 12..17);
}

#[test]
fn test_single_tokens() {
    let tk = "*";
    assert_token_positions!(tk, Ok(Token::Asterix), 0..1);

    let tk = "{";
    assert_token_positions!(tk, Ok(Token::BraceOpen), 0..1);

    let tk = "}";
    assert_token_positions!(tk, Ok(Token::BraceClose), 0..1);

    let tk = "[";
    assert_token_positions!(tk, Ok(Token::BracketOpen), 0..1);

    let tk = "]";
    assert_token_positions!(tk, Ok(Token::BracketClose), 0..1);

    let tk = "word";
    assert_token_positions!(tk, Ok(Token::Word), 0..4);

    let tk = r"\t{}";
    assert_token_positions!(tk, Ok(Token::CommandName), 0..2);

    let tk = "\n";
    assert_token_positions!(tk, Ok(Token::Newline), 0..1);

    let tk = r"	";
    assert_token_positions!(tk, Ok(Token::WhitespaceOrTab), 0..1);

    let tk = r" ";
    assert_token_positions!(tk, Ok(Token::WhitespaceOrTab), 0..1);

    let tk = r".";
    assert_token_positions!(tk, Ok(Token::Dot), 0..1);

    let tk = r":";
    assert_token_positions!(tk, Ok(Token::Colon), 0..1);

    let tk = r",";
    assert_token_positions!(tk, Ok(Token::Comma), 0..1);

    let tk = r"\\";
    assert_token_positions!(tk, Ok(Token::DoubleBackslash), 0..2);

    let tk = r"/";
    assert_token_positions!(tk, Ok(Token::ForwardSlash), 0..1);

    let tk = r"_";
    assert_token_positions!(tk, Ok(Token::Underscore), 0..1);

    let tk = r"1";
    assert_token_positions!(tk, Ok(Token::Number), 0..1);

    let tk = r"\{";
    assert_token_positions!(tk, Ok(Token::EscapedChar), 0..2);

    let tk = r"\}";
    assert_token_positions!(tk, Ok(Token::EscapedChar), 0..2);

    let tk = r"\_";
    assert_token_positions!(tk, Ok(Token::EscapedChar), 0..2);

    let tk = r"\$";
    assert_token_positions!(tk, Ok(Token::EscapedChar), 0..2);

    let tk = r"\&";
    assert_token_positions!(tk, Ok(Token::EscapedChar), 0..2);

    let tk = r"\%";
    assert_token_positions!(tk, Ok(Token::EscapedChar), 0..2);

    let tk = r"\#";
    assert_token_positions!(tk, Ok(Token::EscapedChar), 0..2);
}

#[test]
fn token_test_document() {
    let source = r#"

    \documentclass{article}
    \usepackage{arxiv}

    \title{\input{../title.tex} \input{../title.tex}}

    \date{July 28, 2022}	% Date

    \begin{abstract}
      \input{../0_abstract.tex}
    \end{abstract}

    % Two commands in one line:
    \input{../1_introduction.tex} \input{../2_sample_content.tex}

    \input{
      ../0_abstract.tex
    }
    
    \input % { siemano.txt }
    { ../1_introduction.tex  }
    
    % \input{ siemano.txt }
    \input % { siemano.txt }
    { ../0_abstract.tex }
    
    \input{ %  siemano.txt }
      ../0_abstract.tex }

    And finally some test content.

    \bibliographystyle{IEEEtran}
    \bibliography{../sources.bib}
    
    \end{document}
    "#;

    let mut lexer = Token::lexer(source).spanned();

    while let Some((result, _range)) = lexer.next() {
        match result {
            Ok(token) => {
                println!(
                    "Got {:?} token, range {:?}, maps to {}",
                    token,
                    _range.clone(),
                    source[_range].to_string()
                );
            }
            Err(err) => {
                let message = format!(
                    "Error: {:?}, range {:?}, maps to {}",
                    err,
                    _range.clone(),
                    source[_range].to_string()
                );

                assert!(false, "{}", message)
            }
        }
    }
}
