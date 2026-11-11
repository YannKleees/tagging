#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    pub text: &'a str,
    pub start: usize,
    pub end: usize,
}

const LEADING: &[char] = &[
    '(', '[', '{', '"', '\'', '«', '„', '“', '‘', '‹', '¿', '¡', '*', '`', '#',
];
const TRAILING: &[char] = &[
    '.', ',', ';', ':', '!', '?', ')', ']', '}', '"', '\'', '»', '“', '”', '’', '›', '…', '*', '`',
];
/// Characters of markup and of delimited data. They end a token wherever they stand, so the names
/// in `<City>Trier</City>`, `"city": "Trier"` and `Schmit;Jhemp;Trier` are tokens of their own.
const DELIMITERS: &[char] = &[';', '|', '<', '>', '"', '\\', '{', '}', '[', ']'];
const ELISIONS: &[&str] = &["qu", "d", "l", "j", "m", "n", "s", "t", "c", "z"];
const APOSTROPHES: &[char] = &['\'', '’'];

/// Splits on whitespace and delimiters, then peels punctuation off token edges only, so emails,
/// URLs, decimals and domains stay whole. Elided articles (`D'Mett`, `l’État`) become their own token.
pub fn tokenize(text: &str) -> Vec<Token<'_>> {
    let mut out = Vec::new();
    let mut chunk = None;
    for (i, c) in text.char_indices() {
        if c.is_whitespace() || DELIMITERS.contains(&c) {
            if let Some(start) = chunk.take() {
                split_chunk(text, start, i, &mut out);
            }
            if !c.is_whitespace() {
                push(text, i, i + c.len_utf8(), &mut out);
            }
        } else if chunk.is_none() {
            chunk = Some(i);
        }
    }
    if let Some(start) = chunk {
        split_chunk(text, start, text.len(), &mut out);
    }
    out
}

fn split_chunk<'a>(text: &'a str, mut start: usize, mut end: usize, out: &mut Vec<Token<'a>>) {
    while let Some(c) = text[start..end].chars().next().filter(|c| LEADING.contains(c)) {
        push(text, start, start + c.len_utf8(), out);
        start += c.len_utf8();
    }
    let mut tail = Vec::new();
    while let Some(c) = text[start..end].chars().next_back().filter(|c| TRAILING.contains(c)) {
        tail.push((end - c.len_utf8(), end));
        end -= c.len_utf8();
    }
    if let Some(n) = possessive_len(&text[start..end]) {
        tail.push((end - n, end));
        end -= n;
    }
    if start < end {
        if let Some(n) = elision_len(&text[start..end]) {
            push(text, start, start + n, out);
            start += n;
        }
        push(text, start, end, out);
    }
    for (s, e) in tail.into_iter().rev() {
        push(text, s, e, out);
    }
}

fn push<'a>(text: &'a str, start: usize, end: usize, out: &mut Vec<Token<'a>>) {
    out.push(Token {
        text: &text[start..end],
        start,
        end,
    });
}

fn elision_len(core: &str) -> Option<usize> {
    ELISIONS.iter().find_map(|e| {
        let head = core.get(..e.len())?;
        let mut rest = core[e.len()..].chars();
        let apostrophe = rest.next().filter(|c| APOSTROPHES.contains(c))?;
        let next_is_letter = rest.next().is_some_and(char::is_alphabetic);
        (head.eq_ignore_ascii_case(e) && next_is_letter).then_some(e.len() + apostrophe.len_utf8())
    })
}

fn possessive_len(core: &str) -> Option<usize> {
    let mut rev = core.chars().rev();
    let s = rev.next().filter(|c| *c == 's')?;
    let apostrophe = rev.next().filter(|c| APOSTROPHES.contains(c))?;
    rev.next().filter(|c| c.is_alphabetic())?;
    Some(s.len_utf8() + apostrophe.len_utf8())
}
