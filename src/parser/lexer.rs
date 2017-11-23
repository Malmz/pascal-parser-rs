use super::Token;
//use std;

pub fn lexer(text: String) -> Option<Vec<Token>> {
    let mut tokens = Vec::with_capacity(text.len());
    let mut int = Integer(0);
    for c in text.chars() {
        if let Some(val) = get_digit(c) {
            int.concat(val);
            continue;
        } else if int.0 > 0 {
            tokens.push(Token::Integer(int.0));
            int.0 = 0;
        }

        tokens.push(
            match c {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Mult,
                '/' => Token::Div,
                '(' => Token::LParens,
                ')' => Token::RParens,
                ' ' => continue,
                _ => return None,
            }
        )
    }

    tokens.push(Token::Integer(int.0));

    Some(tokens)
}

#[derive(Copy, Clone, Debug)]
pub struct Integer(i32);

impl Integer {
    pub fn concat(&mut self, i: i32) {
        self.0 = self.0 * 10 + i;
    }
}

fn get_digit(character: char) -> Option<i32> {
	match character {
		'0' => Some(0),
		'1' => Some(1),
		'2' => Some(2),
		'3' => Some(3),
		'4' => Some(4),
		'5' => Some(5),
		'6' => Some(6),
		'7' => Some(7),
		'8' => Some(8),
		'9' => Some(9),
		_ => None,
	}
}