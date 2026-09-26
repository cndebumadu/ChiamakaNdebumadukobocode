use crate::token::{keyword, Token, TokenType};

/// Cut `source` into tokens. Returns everything it managed to scan alongside every
/// error it found; the caller decides whether to go on.
pub fn scan(source: &str) -> (Vec<Token>, Vec<String>) {
    let mut s = Scanner {
        src: source.chars().collect(),
        start: 0,
        current: 0,
        line: 1,
        tokens: Vec::new(),
        errors: Vec::new(),
    };
    s.run();
    (s.tokens, s.errors)
}

struct Scanner {
    src: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
    errors: Vec<String>,
}

impl Scanner {
    fn run(&mut self) {
        // TODO(you): drive the scan: read one token at a time until the source runs out, then
        //            add the EOF token. Spec 6.1 says which line EOF carries.
        while self.current != self.at_end(){// run it as long as it is not at the end
            self.start = self.current; // saving the beginning position 
            self.scan_token();
            self.current +=1; // so it moves
        }

        let eof_line = self.tokens.last().map(|t| t.line).unwrap_or(1); // asked for the line number if no then use the value or 1 

        self.add(/'0'); // add eof to the end 
        todo!("run")
    }

    // TODO(you): recognise one token. Spec 1.2 lists every token type, 1.1 covers
        //            whitespace and comments, and an unrecognised character is 'Character is
        //            not part of any token.' (5.1).

    fn scan_token(&mut self) { // THIS IS  LIKE CONSUME PUNCTUATION FUNCTION AND MAYBE NEXT_TOKEN
    let c = self.advance(); 
    match c {// mathcing the characters to what it would be
     '(' => self.add(TokenType::LPAREN),
     ')' => self.add(TokenType::RPAREN),
     '{' => self.add(TokenType::LBRACE),
     '}' => self.add(TokenType::RBRACE),
     ',' => self.add(TokenType::COMMA),
     ';' => self.add(TokenType::SEMICOLON),
     '+' => self.add(TokenType::PLUS),
     '-' => self.add(TokenType::MINUS),
     '*' => self.add(TokenType::STAR),
     '!' => self.add(TokenType::BANG),
     '=' => self.add(TokenType::EQUAL),
     '<' => self.add(TokenType::LESS),
     '>' => self.add(TokenType::GREATER),

    '/' => {
        if self.matches('/') {
            while !self.at_end() && self.peek() != '\n' {
                self.advance();
            }
        } else {
            self.add(TokenType::SLASH);
        }
    }
   // these are my white spaces
   ' ' => (),
   '\r' => (),
   '\t' => (),
   '\n' => self.line += 1, // move to a new line

    '"' => self.string(), // that that word is a string these have there functions
     '0'..='9' => self.number(), // it is a number simipler to self::is_number_start in my lexer kinda
     'a'..='z' => self.identifier(),
     'A'..='Z' => self.identifier(),
      '_' => self.identifier(), 
     _ => self.error(self.line, "Character not idnetified"), // anything else error

    
}
    todo!("scan_token");
}
    // TODO(you): scan a string literal. A string may span lines (1.5); an unterminated one
        //            is reported at the line it opened on (5.1).

    fn string(&mut self) {
    while self.current < self.input.len() && self.current != '"' { //as long as it is not at the end or it is not ta ""
           if self.current == '\n' {
            self.lin += 1; // if it is a mew line move it to the next line
           }
           else {
            self.advance(); // consume it
           }
            // an erroe at this part what if it didnt hit the closing " that is an error it needs to be reported
           if self.at_end(){
            self.error(self.line, "You didnt close the string");
           }
           else{
            self.advance(); // consuming the last "  I could over look this
           }

    }       
    
}
        
        todo!("string")
    }
    // TODO(you): scan a number literal: digits, then a fractional part only when a digit
        //            follows the dot (1.4).

    fn number(&mut self) {// hmmm similer to identifier with the same consept as my consume_number
    while self.peek().is_digit(10){ 
        self.advance();
         //reflection question 
        if self.peek() == '.' && self.peek().is_digit(10){
            self.advance();
            while self.peek().is_digit(10){
                self.advance();
            }

    }

    self.add(TokenType::Number);
   
    todo!("number")
} }
    // TODO(you): scan an identifier, then decide whether it is a keyword; keyword() in
        //            token.rs does the lookup (1.2, 1.3).

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
        self.advance();
    }

    let text: String = self.src[self.start..self.current].iter().collect();

    match keyword(&text) {
        Some(kind) => self.add(kind),
        None => self.add(TokenType::Identifier),
    }
        
        todo!("identifier")
    }

    // --- primitives ---------------------------------------------------------------

    fn at_end(&self) -> bool { // checks if you have gotten to the end and returns true if there is nothing left
        self.current >= self.src.len()
    }

    fn advance(&mut self) -> char { // does the same function as cunsume in my lexer
        let c = self.src[self.current];
        self.current += 1;
        c
    }

    fn matches(&mut self, expected: char) -> bool {// peek then commit
        if self.at_end() || self.src[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char { // looks at the current character without consuming it
        if self.at_end() {// if it is at the end it returns a null character
            '\0'
        } else {
            self.src[self.current]
        }
    }

    fn peek_next(&self) -> char {// the same as peek but looks further ahead
        if self.current + 1 >= self.src.len() {
            '\0'
        } else {
            self.src[self.current + 1]
        }
    }

    fn add(&mut self, kind: TokenType) {
        self.tokens.push(Token {
            kind,
            lexeme: self.src[self.start..self.current].iter().collect(), // same as (start,end,x.to_string)
            line: self.line, // records this thing
        });
    }

    fn error(&mut self, line: usize, message: &str) { // omo only God's know sha but we will find out
        self.errors.push(format!("[line {}] Error: {}", line, message));
    }
   
}
