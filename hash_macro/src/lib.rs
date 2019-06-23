extern crate proc_macro;

use proc_macro::{Literal, TokenStream, TokenTree};

#[inline]
fn byte(s: &impl AsRef<[u8]>, index: usize) -> u8
{
    let s = s.as_ref();
    if index >= s.len()
    {
        0
    }
    else
    {
        s[index]
    }
}

fn parse_string_literal(mut s: &str) -> String
{
    if byte(&s, 0) != b'"'
    {
        panic!("Not a string literal");
    }

    s = &s[1..];

    let mut output = String::new();

    loop
    {
        let chr = match byte(&s, 0)
        {
            b'"' => break,
            b'\\' =>
            {
                let chr = byte(&s, 1);
                s = &s[2..];
                match chr
                {
                    b'n' => '\n',
                    b'r' => '\r',
                    b't' => '\t',
                    b'\\' => '\\',
                    b'0' => '\0',
                    b'"' => '"',
                    b'\'' => '\'',
                    b'x' =>
                    {
                        let chr =
                            u8::from_str_radix(&s[0..2], 16).expect("Invalid escaped literal");
                        s = &s[2..];

                        if chr > 0x7f
                        {
                            panic!("Invalid escaped literal");
                        }

                        char::from(chr)
                    }
                    b'u' =>
                    {
                        if byte(&s, 0) != b'{'
                        {
                            panic!("Invalid escaped literal");
                        }

                        let mut value = 0;
                        s = &s[1..];

                        for _ in 0..6
                        {
                            let hex_digit = match byte(&s, 0)
                            {
                                b @ b'0'..=b'9' => (b - b'0') as u32,
                                b @ b'a'..=b'f' => (b - b'a') as u32 + 10,
                                b @ b'A'..=b'F' => (b - b'A') as u32 + 10,
                                b'}' => break,
                                _ => panic!("Invalid escaped literal"),
                            };

                            value = value * 0x10 + hex_digit;
                            s = &s[1..];
                        }

                        if value > 0xffffff || byte(&s, 0) != b'}'
                        {
                            panic!("Invalid escaped literal");
                        }

                        s = &s[1..];

                        core::char::from_u32(value).expect("Invalid char")
                    }
                    _ => panic!("Unknown escape character"),
                }
            }
            _ =>
            {
                let chr = s.chars().next().unwrap_or('\0');
                s = &s[chr.len_utf8()..];
                chr
            }
        };

        output.push(chr);
    }

    return output;
}

#[proc_macro]
pub fn murmur3_32(input: TokenStream) -> TokenStream
{
    let first = input
        .into_iter()
        .nth(0)
        .expect("Should contain a string literal");

    let first = match first
    {
        TokenTree::Literal(l) =>
        {
            let string = parse_string_literal(&l.to_string());
            let hash = fnd::hash::murmur3_32(&string);
            TokenTree::from(Literal::u32_suffixed(hash))
        }
        _ => panic!("Should contain a string literal"),
    };

    return TokenStream::from(first);
}
