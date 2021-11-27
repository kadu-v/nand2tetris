use crate::{loc::Loc, token::*};

#[derive(Debug, PartialEq, Eq)]
pub struct Lexer<'a> {
    input: &'a [u8],
    line: usize,
    pos: usize,
    next_pos: usize,
    ch: Option<u8>,
}

impl<'a> Lexer<'a> {
    // Lexer のコンストラクター
    pub fn new(input: &'a str) -> Self {
        let input = input.as_bytes();
        let mut l = Self {
            input: input,
            line: 0,
            pos: 0,
            next_pos: 0,
            ch: None,
        };
        l.read_char();
        l
    }

    // 次のトークンを返すメソッド
    // 現在の文字を検査して、次の文字をせっとしてから返す
    pub fn next_token(&mut self) -> Token {
        // 空白を読み飛ばす
        self.skip_whitespaces();

        // Tokenを取り出す
        match self.ch {
            None => Token {
                kind: TokenKind::Eof,
                loc: Loc::new(self.line, self.pos, 0),
            },
            Some(c) => match c {
                b'\n' => {
                    self.line += 1;
                    let tok = Token {
                        kind: TokenKind::NewLine,
                        loc: Loc::new(self.line, self.pos, 1),
                    };
                    self.read_char();
                    tok
                }
                _ => {
                    if self.is_digit() {
                        let s = String::from_utf8(self.read_number().to_vec()).unwrap();
                        let n = s.parse::<usize>().unwrap();
                        Token {
                            kind: TokenKind::Number(n),
                            loc: Loc::new(self.line, self.pos - s.len(), s.len()),
                        }
                    } else if self.is_letter() {
                        let ident = String::from_utf8(self.read_identifier().to_vec()).unwrap();
                        let width = ident.len();
                        match lookup_keyword(&ident) {
                            Some(kind) => Token {
                                kind: kind,
                                loc: Loc::new(self.line, self.pos - width, width),
                            },
                            None => Token {
                                kind: TokenKind::Symbol(ident),
                                loc: Loc::new(self.line, self.pos - width, width),
                            },
                        }
                    } else {
                        Token {
                            kind: TokenKind::Illegal,
                            loc: Loc::new(self.line, self.pos, 1),
                        }
                    }
                }
            },
        }
    }

    // 次の文字を読み込む
    fn read_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.next_pos]);
        }
        self.pos = self.next_pos;
        self.next_pos += 1;
    }

    // 次の文字を先読み
    fn peek_char(&self) -> Option<u8> {
        if self.next_pos >= self.input.len() {
            return None;
        }

        Some(self.input[self.next_pos])
    }

    //

    // 空白を読み飛ばす
    fn skip_whitespaces(&mut self) {
        while self.is_whitespace() {
            self.read_char()
        }
    }

    // Identifierを読み取るメソッド
    fn read_identifier(&mut self) -> &[u8] {
        let pos = self.pos;
        while self.is_letter() || self.is_digit() {
            self.read_char();
        }

        &self.input[pos..self.pos]
    }

    // 数字を読み取るメソッド
    // 負の数字にも対応
    pub fn read_number(&mut self) -> &[u8] {
        let pos = self.pos;
        // 先頭が '-' の時は一文字読み飛ばす
        if self.is_minus_lit() {
            self.read_char();
        }

        while self.is_digit() {
            self.read_char()
        }

        &self.input[pos..self.pos]
    }

    // 空白を判定するメソッド
    fn is_whitespace(&self) -> bool {
        match self.ch {
            None => false,
            Some(c) => c as char == ' ' || c as char == '\t',
        }
    }

    // 改行を判定するメソッド
    fn is_newline(&self) -> bool {
        match self.ch {
            None => false,
            Some(c) => c as char == '\r' || c as char == '\n',
        }
    }

    // 文字を判定するメソッド
    fn is_letter(&self) -> bool {
        match self.ch {
            None => false,
            Some(c) => (b'a' <= c && c <= b'z') || (b'A' <= c && c <= b'Z'),
        }
    }

    // 数字を判定するメソッド
    fn is_digit(&self) -> bool {
        match self.ch {
            None => false,
            Some(c) => b'0' <= c && c <= b'9',
        }
    }

    // '-' を判定するメソッド
    fn is_minus_lit(&mut self) -> bool {
        match self.ch {
            None => false,
            Some(c) => c == b'-',
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    // EOF の代わりにNoneを返す
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        }
        Some(self.next_token())
    }
}
