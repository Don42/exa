use ansi_term::{ANSIString, Style};

static BLACKLIST: &str = "'\"!?\\/`´&$(){}[]<>~|;*";

fn blacklisted(ch: char) -> bool {
    ch.is_control() || ch.is_whitespace() || BLACKLIST.contains(ch)
}

pub fn escape<'a>(string: String, bits: &mut Vec<ANSIString<'a>>, good: Style, _bad: Style) {
    if !string.contains(blacklisted) {
        bits.push(good.paint(string));
    } else {
        let mut str_buffer = String::with_capacity(string.len() + 2);
        str_buffer.push('\'');
        for ch in string.chars() {
            match ch {
                '\'' => {
                    str_buffer.push_str("'\\");
                    str_buffer.push(ch);
                    str_buffer.push('\'');
                },
                _ => str_buffer.push(ch),
            }
        }
        str_buffer.push('\'');
        bits.push(good.paint(str_buffer));
    }
}
