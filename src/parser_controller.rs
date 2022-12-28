pub struct ParserController {
    pub chars: Vec<char>,
    pub index: usize,
}

impl ParserController {
    pub fn from_str(source: &str) -> Self {
        ParserController {
            chars: source.chars().collect(),
            index: 0,
        }
    }

    pub fn is_end(&self) -> bool {
        self.chars.len() == self.index
    }

    pub fn current(&self) -> char {
        self.chars[self.index]
    }

    pub fn skip_ws(&mut self) {
        while !self.is_end()
            && (self.current() == '\n'
                || self.current() == '\r'
                || self.current() == '\n'
                || self.current() == ' ')
        {
            self.index += 1;
        }
    }

    /// Look into the future and see if pattern arise
    /// right after current index
    pub fn expect(&self, pattern: &str) -> bool {
        if self.index + pattern.len() >= self.chars.len() {
            return false;
        }

        let mut resp = true;
        let mut currindex = self.index;
        for c in pattern.chars() {
            if c != self.chars[currindex] {
                resp = false;
                break;
            }
            currindex += 1;
        }
        resp
    }

    pub fn eat(&mut self, pattern: &str) {
        if self.index + pattern.len() > self.chars.len() {
            let mut currline = 0;
            for c in &self.chars[0..(self.index + pattern.len())] {
                if *c == '\n' {
                    currline += 1;
                }
            }
            panic!(
                "Cannot eat '{}', it goes outside source, Trying to eat at {}, but maxlen is {}, current line is {}",
                pattern, self.index + pattern.len(), self.chars.len(), currline
            );
        }
        let pattern_chars: Vec<char> = pattern.chars().collect();
        for i in 0..pattern.len() {
            if self.chars[self.index + i] != pattern_chars[i] {
                panic!(
                    "Cannot eat pattern '{}', symbol '{}' not match to {}",
                    pattern, pattern_chars[i], self.chars[self.index + i]
                );
            }
        }
        self.index += pattern.len()
    }

    pub fn read_until_not_equal(&mut self, c: char) -> Option<char> {
        if self.current() != c {
            let val = Some(self.current());
            self.index += 1;
            val
        } else {
            None
        }
    }

    pub fn take_until_not_equal(&mut self, n: char) -> String {
        let mut resp = String::new();
        while let Some(c) = self.read_until_not_equal(n) {
            resp.push(c);
        }
        resp
    }
}
