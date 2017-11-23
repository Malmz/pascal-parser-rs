use super::Token;

type Chars<'a> = &'a [u8];

pub fn lexer(chars: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::with_capacity(chars.len());
    let mut codepoints = chars.as_bytes();
    loop {
        if let Some((val, points)) = integer(codepoints) {
            tokens.push(Token::Integer(val));
            codepoints = points;
            continue;
        }

        if let Some((val, points)) = 
            plus(codepoints)
            .or(minus(codepoints))
            .or(mult(codepoints))
            .or(div(codepoints))
        {
            tokens.push(val);
            codepoints = points;
            continue;
        }


        break;
    }

    Ok(tokens)
}

fn integer(mut chars: Chars) -> Option<(i32, Chars)> {
    let mut result: i32 = match chars.first() {
        Some(val) => match get_digit(*val) {
            Some(val) if val != => {
                chars = pop(chars);
                val as i32
            },
            _ => return None,
        },
        None => return None,
    };
    loop {
        match chars.first() {
            Some(val) => match get_digit(*val) {
                Some(val) => {
                    result = result * 10 + val as i32;
                    chars = pop(chars);
                },
                None => break,
            },
            None => return None
        }
    }
    Some((result, chars))
}

fn get_digit(character: u8) -> Option<u8> {
	if character >= 48 && character <= 57 {
        Some(character - 48)
    } else {
        None
    }
}

fn plus(chars: Chars) -> Option<(Token, Chars)> {
    match chars.first() {
        Some(val) if *val == 43u8 => Some((Token::Plus, pop(chars))),
        _ => None,
    }
}

fn minus(chars: Chars) -> Option<(Token, Chars)> {
    match chars.first() {
        Some(val) if *val == 45u8 => Some((Token::Minus, pop(chars))),
        _ => None,
    }
}

fn div(chars: Chars) -> Option<(Token, Chars)> {
    match chars.first() {
        Some(val) if *val == 47u8 => Some((Token::Div, pop(chars))),
        _ => None,
    }
}

fn mult(chars: Chars) -> Option<(Token, Chars)> {
    match chars.first() {
        Some(val) if *val == 42u8 => Some((Token::Mult, pop(chars))),
        _ => None,
    }
}

fn lparens(chars: Chars) -> Option<(Token, Chars)> {
    match chars.first() {
        Some(val) if *val == 40u8 => Some((Token::LParens, pop(chars))),
        _ => None,
    }
}

fn rparens(chars: Chars) -> Option<(Token, Chars)> {
    match chars.first() {
        Some(val) if *val == 41u8 => Some((Token::LParens, pop(chars))),
        _ => None,
    }
}

fn pop(chars: Chars) -> Chars { &chars[1..] }