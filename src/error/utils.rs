use cssparser::Token;

pub(crate) fn render_token<'a>(token: &Token<'a>) -> String {
    // THIS TOOK FOREVER TO IMPLEMENT
    // TODO: Make this easier to read, I guess

    match token {
        // TODO: Give these guys some better names
        Token::Ident(ident) => format!("{}", ident.clone()),
        Token::AtKeyword(value) => format!("@{}", value.clone()),
        Token::Hash(name) | Token::IDHash(name) => format!("#{}", name.clone()),
        Token::QuotedString(value) => format!("\"{}\"", value.clone()),
        Token::Number {
            has_sign: signed,
            value: num,
            int_value: _,
        } => {
            if *signed {
                render_int_signed(*num)
            } else {
                render_int_unsigned(*num)
            }
        }
        Token::Percentage {
            has_sign: signed,
            unit_value: num,
            int_value: _,
        } => {
            format!(
                "{}%",
                if *signed {
                    render_int_signed(*num)
                } else {
                    render_int_unsigned(*num)
                }
            )
        }
        Token::Dimension {
            has_sign: signed,
            value: num,
            int_value: _,
            unit,
        } => {
            format!(
                "{}{}",
                if *signed {
                    render_int_signed(*num)
                } else {
                    render_int_unsigned(*num)
                },
                unit
            )
        }
        Token::WhiteSpace(_) => String::from(" "),
        Token::Comment(comment) => format!("/* {} */", comment),
        Token::Function(name) => format!("{}()", name),
        Token::BadString(string) => format!("<Bad String {:?}>", string),
        Token::BadUrl(url) => format!("<Bad URL {:?}>", url),
        // Single-character token
        sc_token => String::from(match sc_token {
            Token::Colon => ":",
            Token::Semicolon => ";",
            Token::Comma => ",",
            Token::IncludeMatch => "~=",
            Token::DashMatch => "|=",
            Token::PrefixMatch => "^=",
            Token::SuffixMatch => "$=",
            Token::SubstringMatch => "*=",
            Token::CDO => "<!--",
            Token::CDC => "-->",
            Token::ParenthesisBlock => "<(",
            Token::SquareBracketBlock => "<[",
            Token::CurlyBracketBlock => "<{",
            Token::CloseParenthesis => "<)",
            Token::CloseSquareBracket => "<]",
            Token::CloseCurlyBracket => "<}",
            other => panic!(
                "Token {:?} is not supposed to match as a single-character token!",
                other
            ),
        }),
    }
}

fn render_int_signed(num: f32) -> String {
    if num > 0.0 {
        format!("+{}", num)
    } else {
        format!("-{}", num)
    }
}

fn render_int_unsigned(num: f32) -> String {
    format!("{}", num)
}
