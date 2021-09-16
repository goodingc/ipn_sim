use std::str;

#[derive(Default)]
pub struct ParsedBytesSections {
    sections: Vec<ParsedBytesSection>,
    next_section: Option<ParsedBytesSection>,
}

#[derive(Eq, PartialEq)]
pub enum ParsedBytesSection {
    Chars(Vec<char>),
    Bytes(Vec<u8>),
}

impl ParsedBytesSections {
    pub fn from_bytes(mut bytes: &[u8]) -> Vec<ParsedBytesSection> {
        let mut sections = Self::default();

        loop {
            match str::from_utf8(bytes) {
                Ok(string) => {
                    sections.push_string(string);
                    break;
                }
                Err(e) => {
                    let (good, bad) = bytes.split_at(e.valid_up_to());

                    if !good.is_empty() {
                        sections.push_string(unsafe {
                            str::from_utf8_unchecked(good)
                        });
                    }

                    if bad.is_empty() {
                        break;
                    }

                    sections.push_byte(bad[0]);
                    bytes = &bad[1..];
                }
            }
        }

        sections.sections.push(sections.next_section.take().unwrap());
        sections.sections
    }

    fn push_string(&mut self, string: &str) {
        for char in string.chars() {
            if char.is_control() {
                self.push_byte(char as u8)
            } else {
                self.push_char(char);
            }
        }
    }

    fn push_char(&mut self, char: char) {
        let mut push_new_section = false;

        if let Some(next_section) = &mut self.next_section {
            match next_section {
                ParsedBytesSection::Chars(chars) => {
                    chars.push(char);
                }
                ParsedBytesSection::Bytes(_) => {
                    push_new_section = true;
                }
            }
        } else {
            self.next_section = Some(ParsedBytesSection::Chars(vec![char]));
        }

        if push_new_section {
            self.sections.push(
                self.next_section
                    .replace(ParsedBytesSection::Chars(vec![char]))
                    .unwrap()
            )
        }
    }

    fn push_byte(&mut self, byte: u8) {
        let mut demote_next_section = false;
        let mut push_new_section = false;

        if let Some(next_section) = &mut self.next_section {
            if let ParsedBytesSection::Chars(chars) = next_section {
                if chars.len() < 3 {
                    demote_next_section = true;
                } else {
                    push_new_section = true;
                }
            }
        } else {
            self.next_section = Some(ParsedBytesSection::Bytes(vec![byte]));
            return;
        }

        if push_new_section {
            self.sections.push(
                self.next_section
                    .replace(ParsedBytesSection::Bytes(vec![byte]))
                    .unwrap()
            );
            return;
        } else if demote_next_section {
            let next_section = self.next_section.take().unwrap();

            let merge_with_last_section = if let Some(ParsedBytesSection::Bytes(_)) = self.sections.last() {
                true
            } else {
                 false
            };

            self.next_section = Some(if merge_with_last_section {
                match self.sections.pop().unwrap() {
                    ParsedBytesSection::Bytes(bytes) =>
                        match next_section {
                            ParsedBytesSection::Chars(chars) =>
                                ParsedBytesSection::Bytes(bytes
                                    .into_iter()
                                    .chain(
                                        chars
                                            .into_iter()
                                            .map(|char| char as u8)
                                    ).collect()
                                ),
                            _ => unreachable!()
                        }
                    _ => unreachable!()
                }
            } else {
                match next_section {
                    ParsedBytesSection::Chars(chars) =>
                        ParsedBytesSection::Bytes(chars
                            .into_iter()
                            .map(|char| char as u8)
                            .collect()
                        ),
                    _ => unreachable!()
                }
            });
        }

        if let Some(next_section) = &mut self.next_section {
            if let ParsedBytesSection::Bytes(bytes) = next_section {
                bytes.push(byte);
            }
        }
    }
}