use anyhow::{Context, Result};

use super::either::Either;

#[derive(Clone)]
pub struct Reader<'a> {
    input: &'a str,
    skip_whitespace: bool,
}

impl<'a> Reader<'a> {
    pub fn new(input: &'a str) -> Self {
        Reader {
            input,
            skip_whitespace: true,
        }
    }

    pub fn keep_whitespace(mut self) -> Self {
        self.skip_whitespace = false;
        self
    }

    pub fn ident(&mut self) -> Result<&'a str> {
        self.skip_whitespace();

        let ident = self.take_while(|ch| ch.is_alphanumeric() || ch == '-' || ch == '_');

        if ident.is_empty() {
            anyhow::bail!("ident not found");
        }

        Ok(ident)
    }

    pub fn digit(&mut self) -> Result<usize> {
        self.skip_whitespace();

        let digit = self.take(1);

        if digit.is_empty() || !digit.chars().next().unwrap().is_ascii_digit() {
            anyhow::bail!("digit not found");
        }

        Ok(digit.parse().unwrap())
    }

    pub fn unsigned(&mut self) -> Result<usize> {
        self.skip_whitespace();

        let unsigned = self.take_while(|ch| ch.is_ascii_digit());

        if unsigned.is_empty() {
            anyhow::bail!("unsigned not found");
        }

        Ok(unsigned.parse().unwrap())
    }

    pub fn signed(&mut self) -> Result<isize> {
        self.skip_whitespace();

        let mut end = 0;
        if self.input.starts_with('-') {
            end += 1;
            self.input = &self.input[1..];
        }

        for ch in self.input.chars() {
            if ch.is_ascii_digit() {
                end += ch.len_utf8();
            } else {
                break;
            }
        }

        let string = (end != 0)
            .then_some(&self.input[..end])
            .context("signed not found")?;
        self.input = &self.input[end..];

        Ok(string.parse().unwrap())
    }

    pub fn text(&mut self, text: &str) -> Result<&'a str> {
        self.skip_whitespace();

        if self.input.starts_with(text) {
            let text = &self.input[..text.len()];
            self.input = &self.input[text.len()..];
            Ok(text)
        } else {
            Err(anyhow::anyhow!("input was not '{text}'"))
        }
    }

    pub fn list<T, F>(&mut self, separator: &str, mut body: F) -> Result<Vec<T>>
    where
        F: FnMut(&mut Self) -> Result<T>,
    {
        let mut values = Vec::new();

        self.skip_whitespace();
        values.push(body(self)?);
        self.skip_whitespace();

        while self.input.starts_with(separator) {
            self.input = &self.input[separator.len()..];

            self.skip_whitespace();
            values.push(body(self)?);
            self.skip_whitespace();
        }

        Ok(values)
    }

    pub fn array<const N: usize, T, F>(&mut self, separator: &str, mut body: F) -> Result<[T; N]>
    where
        F: FnMut(&mut Self) -> Result<T>,
    {
        if N == 0 {
            return Ok([0; N].map(|_| loop {}));
        }

        let mut values = Vec::new();

        self.skip_whitespace();
        values.push(Some(body(self)?));
        self.skip_whitespace();

        let mut count = 1;

        while self.input.starts_with(separator) && count < N {
            self.input = &self.input[separator.len()..];

            self.skip_whitespace();
            values.push(Some(body(self)?));
            self.skip_whitespace();

            count += 1;
        }

        if count < N {
            anyhow::bail!("invalid array: expected '{N}' occurences, got '{count}'");
        }

        let mut array = [0; N];
        for i in 0..N {
            array[i] = i;
        }

        let array = array.map(|i| values[i].take().unwrap());

        Ok(array)
    }

    pub fn lines<T, F>(&mut self, mut body: F) -> Result<Vec<T>>
    where
        F: FnMut(&mut Self) -> Result<T>,
    {
        let mut out = Vec::new();

        for line in self.input.lines() {
            self.input = line;
            let item = body(self)?;
            out.push(item);
        }

        Ok(out)
    }

    pub fn while_ok<T, F>(&mut self, mut body: F) -> Vec<T>
    where
        F: FnMut(&mut Self) -> Result<T>,
    {
        let mut out = Vec::new();
        while let Ok(item) = body(self) {
            out.push(item);
        }
        out
    }

    pub fn get_matches<T, F>(&mut self, mut body: F) -> Vec<T>
    where
        F: FnMut(&mut Self) -> Result<T>
    {
        let mut out = Vec::new();

        loop {
            let cur = self.input;

            match body(self) {
                Ok(item) => {
                    out.push(item);
                }
                Err(_) => {
                    let Some(ch) = self.input.chars().next()
                    else { break };
                    self.input = &cur[ch.len_utf8()..];
                }
            }
        }

        out
    }

    pub fn race<T, U, F1, F2>(&mut self, mut body1: F1, mut body2: F2) -> Result<Either<T, U>>
    where
        F1: FnMut(&mut Self) -> Result<T>,
        F2: FnMut(&mut Self) -> Result<U>,
    {
        let mut last_error;

        loop {
            let cur = self.input;

            let e1;
            let e2;

            match body1(self) {
                Ok(t) => return Ok(Either::Left(t)),
                Err(err) => e1 = err,
            }

            self.input = cur;

            match body2(self) {
                Ok(u) => return Ok(Either::Right(u)),
                Err(err) => e2 = err,
            }

            last_error = anyhow::anyhow!("errors:\n  left: {e1:?}\n  right: {e2:?}");

            let Some(ch) = self.input.chars().next()
            else { break };

            self.input = &cur[ch.len_utf8()..];
        }

        Err(last_error)
    }

    pub fn take(&mut self, n: usize) -> &'a str {
        let rest = n.min(self.input.len());
        let out = &self.input[..rest];
        self.input = &self.input[rest..];
        out
    }

    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    fn take_while<F>(&mut self, mut cond: F) -> &'a str
    where
        F: FnMut(char) -> bool,
    {
        let mut end = 0;
        for ch in self.input.chars() {
            if cond(ch) {
                end += ch.len_utf8();
            } else {
                break;
            }
        }

        let out = &self.input[..end];
        self.input = &self.input[end..];

        out
    }

    fn skip_whitespace(&mut self) {
        if !self.skip_whitespace {
            return;
        }

        let mut end = 0;
        for ch in self.input.chars() {
            if ch.is_whitespace() && ch != '\n' {
                end += ch.len_utf8();
            } else {
                break;
            }
        }

        self.take(end);
    }
}

pub trait IntoReader<'a> {
    fn reader(self) -> Reader<'a>;
}

impl<'a, T> IntoReader<'a> for &'a T
where
    T: AsRef<str>,
{
    fn reader(self) -> Reader<'a> {
        Reader::new(self.as_ref())
    }
}