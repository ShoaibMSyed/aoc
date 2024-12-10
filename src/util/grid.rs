use std::{fmt::Display, ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign}};

use anyhow::{Context, Result};

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
    
    pub fn surrounding(self) -> impl Iterator<Item = Self> {
        struct Iter {
            offset: CellIndex,
            mid: CellIndex,
        }

        impl Iterator for Iter {
            type Item = CellIndex;

            fn next(&mut self) -> Option<Self::Item> {
                if self.offset.x > 1 || self.offset.y > 1 {
                    return None;
                }

                let ret = self.mid + self.offset;

                self.offset.x += 1;
                if self.offset.x == 0 && self.offset.y == 0 {
                    self.offset.x += 1;
                }
                if self.offset.x > 1 {
                    self.offset.x = -1;
                    self.offset.y += 1;
                }

                Some(ret)
            }
        }

        Iter { offset: CellIndex { x: -1, y: -1 }, mid: self }
    }

    pub fn cardinal(self) -> impl Iterator<Item = Self> {
        [[-1isize, 0], [0, -1], [1, 0], [0, 1]]
            .map(CellIndex::from)
            .map(|i| self + i)
            .into_iter()
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
pub struct Grid<T = char> {
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

    pub fn surrounding(&self, cell: CellIndex) -> impl Iterator<Item = (CellIndex, &T)> {
        cell
            .surrounding()
            .filter_map(|i| self.get(i).map(|c| (i, c)))
    }

    pub fn cardinal(&self, cell: CellIndex) -> impl Iterator<Item = (CellIndex, &T)> {
        cell
            .cardinal()
            .filter_map(|i| self.get(i).map(|c| (i, c)))
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
        let i = i.into();
        let CellIndex { x, y } = i;

        if !self.contains(i) { return None; }

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

pub trait IntoGrid<T> {
    fn grid(self) -> Result<Grid<T>>;
}

impl IntoGrid<char> for &str {
    fn grid(self) -> Result<Grid<char>> {
        Grid::new(self)
    }
}
