use crate::parser::token::LogosToken;
use logos::Lexer as LogosLexer;
use logos::Logos;
use std::fmt;

// pub type Spanned = (Location, Token);
pub type Spanned<'input> = (Location, LogosToken<'input>, Location);
pub type LexResult<'input> = Result<Spanned<'input>, LexerError>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    row: usize,
    column: usize,
}

impl Default for Location {
    fn default() -> Self {
        Location { row: 0, column: 0 }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {} colomn {}", self.row, self.column)
    }
}

impl Location {
    pub fn short_show(&self) -> String {
        format!("{}:{}", self.row, self.column)
    }
}

pub struct LexerError {
    // pub error: String,
// pub location: Location,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LexerError")
    }
}

impl fmt::Debug for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LexerError")
    }
}

pub struct Lexer<'input> {
    lex: LogosLexer<'input, LogosToken<'input>>,
    curcol: usize,
    currow: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(instr: &'input str) -> Lexer<'input> {
        Lexer {
            lex: LogosToken::lexer(instr),
            curcol: 0,
            currow: 1,
        }
    }
}

impl<'input> fmt::Display for Lexer<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lex = &self.lex;
        write!(f, "{:?}", lex.span())
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = LexResult<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let op_tok = self.lex.next();
            if op_tok.is_none() {
                return None;
            }
            let tok: LogosToken<'input> = op_tok.unwrap();
            let span = self.lex.span();
            match tok {
                LogosToken::Comment | LogosToken::Blank => continue,
                LogosToken::Newline => {
                    self.currow += 1;
                    self.curcol = span.start;
                    continue;
                }
                _ => {
                    let spanned: Spanned<'input> = (
                        Location {
                            row: self.currow,
                            column: span.start - self.curcol,
                        },
                        tok,
                        Location {
                            row: self.currow,
                            column: span.end - self.curcol,
                        },
                    );
                    return Some(LexResult::Ok(spanned));
                }
            };
        }
    }
}
