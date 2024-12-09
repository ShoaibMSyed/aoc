use std::{
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign},
};

use anyhow::{Context, Result};

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

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct CellIndex {
    pub x: isize,
    pub y: isize,
}

impl CellIndex {
    pub fn all_indexes_for<T>(grid: &Grid<T>) -> impl Iterator<Item = CellIndex> {
        struct Iter {
            cur: CellIndex,
            size: CellIndex,
        }

        impl Iterator for Iter {
            type Item = CellIndex;
            
            fn next(&mut self) -> Option<Self::Item> {
                if self.cur.x >= self.size.x || self.cur.y >= self.size.y {
                    return None;
                }

                let out = self.cur;
                self.cur.x += 1;
                if self.cur.x >= self.size.x {
                    self.cur.x = 0;
                    self.cur.y += 1;
                }

                Some(out)
            }
        }

        Iter { cur: CellIndex::default(), size: grid.size() }
    }
}

impl Deref for CellIndex {
    type Target = [isize; 2];

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const CellIndex).cast() }
    }
}

impl DerefMut for CellIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut CellIndex).cast() }
    }
}

impl Add for CellIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for CellIndex {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for CellIndex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for CellIndex {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl From<[usize; 2]> for CellIndex {
    fn from([x, y]: [usize; 2]) -> Self {
        CellIndex { x: x.try_into().unwrap(), y: y.try_into().unwrap() }
    }
}

impl From<[isize; 2]> for CellIndex {
    fn from([x, y]: [isize; 2]) -> Self {
        CellIndex { x, y }
    }
}

impl From<(usize, usize)> for CellIndex {
    fn from((x, y): (usize, usize)) -> Self {
        CellIndex { x: x.try_into().unwrap(), y: y.try_into().unwrap() }
    }
}

impl From<(isize, isize)> for CellIndex {
    fn from((x, y): (isize, isize)) -> Self {
        CellIndex { x, y }
    }
}

impl Into<[usize; 2]> for CellIndex {
    fn into(self) -> [usize; 2] {
        [self.x.try_into().unwrap(), self.y.try_into().unwrap()]
    }
}

impl Into<[isize; 2]> for CellIndex {
    fn into(self) -> [isize; 2] {
        [self.x, self.y]
    }
}

impl Into<(usize, usize)> for CellIndex {
    fn into(self) -> (usize, usize) {
        (self.x.try_into().unwrap(), self.y.try_into().unwrap())
    }
}

impl Into<(isize, isize)> for CellIndex {
    fn into(self) -> (isize, isize) {
        (self.x, self.y)
    }
}

impl Display for CellIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    values: Vec<T>,
    width: isize,
    height: isize,
}

impl<T> Grid<T> {
    pub fn map<U, F>(self, mapper: F) -> Grid<U>
    where
        F: FnMut(T) -> U,
    {
        let values = self.values.into_iter().map(mapper).collect();

        Grid {
            values,
            width: self.width,
            height: self.height,
        }
    }

    pub fn width(&self) -> isize {
        self.width
    }

    pub fn height(&self) -> isize {
        self.height
    }

    pub fn size(&self) -> CellIndex {
        [self.width, self.height].into()
    }

    pub fn cells(&self) -> &[T] {
        &self.values
    }

    pub fn cells_mut(&mut self) -> &mut [T] {
        &mut self.values
    }

    pub fn get(&self, i: impl Into<CellIndex>) -> Option<&T> {
        let index = self.to_index(i)?;

        self.values.get(index)
    }

    pub fn get_mut(&mut self, i: impl Into<CellIndex>) -> Option<&mut T> {
        let index = self.to_index(i)?;
        
        self.values.get_mut(index)
    }

    pub fn set(&mut self, i: impl Into<CellIndex>, value: T) -> Result<T> {
        let i = i.into();
        let cell = self
            .get_mut(i)
            .with_context(|| format!("grid position '{i}' is outside of grid"))?;
        Ok(std::mem::replace(cell, value))
    }

    pub fn contains(&self, i: impl Into<CellIndex>) -> bool {
        let i = i.into();

        (0..self.width).contains(&i.x)
            && (0..self.height).contains(&i.y)
    }

    pub fn iter(&self) -> GridIter<T> {
        self.into_iter()
    }

    pub fn matches_mask_at<U>(&self, mask: &GridMask<U>, mask_pos: impl Into<CellIndex>) -> bool
    where
        T: PartialEq<U>
    {
        let offset = mask_pos.into();

        for (index, mask) in mask {
            let Some(mask_value) = mask
            else { continue };

            let Some(value) = self.get(offset + index)
            else { return false };

            if value != mask_value {
                return false;
            }
        }

        true
    }

    fn to_index(&self, i: impl Into<CellIndex>) -> Option<usize> {
        let CellIndex { x, y } = i.into();
        if x < 0 || y < 0 { return None; }
        Some(y as usize * self.width as usize + x as usize)
    }
}

impl Grid<char> {
    pub fn new(input: &str) -> Result<Self> {
        let mut width = None;
        let mut height = 0;
        let mut values = Vec::new();

        for line in input.lines() {
            match width {
                Some(width) => {
                    if line.len() != width {
                        anyhow::bail!(
                            "invalid grid: line {height} has width of '{}' instead of '{width}'",
                            line.len()
                        );
                    }
                }
                None => width = Some(line.len()),
            }

            for ch in line.chars() {
                values.push(ch);
            }

            height += 1;
        }

        let width = width.unwrap_or_default();
        let width = width.try_into().expect("grid size must fit in isize");

        Ok(Grid {
            values,
            width,
            height,
        })
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();

        for y in 0..self.height() {
            for x in 0..self.width() {
                let value = self.get([x, y]).unwrap();
                string.push(*value);
            }
            string.push('\n');
        }

        string
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = (CellIndex, &'a T);

    type IntoIter = GridIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter {
            grid: self,
            pos: CellIndex::default(),
        }
    }
}

pub type GridMask<T> = Grid<Option<T>>;

impl GridMask<char> {
    pub fn new_mask(input: &str, ignore: char) -> Result<Self> {
        let grid = Grid::new(input)?;
        Ok(grid.map(|ch| if ch == ignore { None } else { Some(ch) }))
    }

    pub fn to_string_with_empty(&self, empty: char) -> String {
        let mut string = String::new();

        for y in 0..self.height() {
            for x in 0..self.width() {
                let ch = self.get([x, y]).unwrap();
                let ch = ch.unwrap_or(empty);
                string.push(ch);
            }
            string.push('\n');
        }

        let len = string.trim_end().len();

        while string.len() > len {
            string.pop();
        }

        string
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    pos: CellIndex,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = (CellIndex, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.pos;
        let value = self.grid.get(cur)?;

        self.pos.x += 1;
        if self.pos.x == self.grid.width() {
            self.pos.x = 0;
            self.pos.y += 1;
        }

        Some((cur, value))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
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

pub trait IntoGrid<T> {
    fn grid(self) -> Result<Grid<T>>;
}

impl IntoGrid<char> for &str {
    fn grid(self) -> Result<Grid<char>> {
        Grid::new(self)
    }
}
