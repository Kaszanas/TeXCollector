use logos::Logos;
use texcollector::lexer::token::Token;

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
                eprintln!(
                    "Error: {:?}, range {:?}, maps to {}",
                    err,
                    _range.clone(),
                    source[_range].to_string()
                );
                // Handle the error as needed
            }
        }
    }

    // assert_eq!(lexer.next(), Some(Ok(Token::Newline)));
    // println!("Got {:?} token", lexer.next());
}
