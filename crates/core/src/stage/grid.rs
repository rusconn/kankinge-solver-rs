use std::{error, ops::Index};

use fixedbitset::FixedBitSet;
use serde_json::Value;

use crate::Point;

use super::{Cell, symbols};

#[derive(Debug)]
pub(super) struct Grid {
    cells: Vec<&'static Cell>,
    width: usize,
}

impl Index<GridIndex> for Grid {
    type Output = &'static Cell;

    fn index(&self, index: GridIndex) -> &Self::Output {
        &self.cells[index.as_usize()]
    }
}

impl Grid {
    pub(super) fn parse(json_text: &str) -> Result<Self, Box<dyn error::Error>> {
        let value: Value = serde_json::from_str(json_text)?;

        let lines = value
            .get("map")
            .ok_or("mapキーが無い")?
            .as_array()
            .ok_or("mapは配列でなければならない")?
            .iter()
            .map(Value::as_str)
            .collect::<Option<Vec<_>>>()
            .ok_or("mapの要素は文字列でなければならない")?;
        if lines.is_empty() {
            return Err("mapが空".into());
        }

        let width = lines[0].chars().count();
        if width == 0 {
            return Err("mapは1列以上でなければならない".into());
        }
        if lines.iter().any(|line| line.chars().count() != width) {
            return Err("mapは矩形でなければならない".into());
        }

        let cells = lines
            .into_iter()
            .flat_map(str::chars)
            .map(|symbol| {
                symbols::MAP
                    .get(&symbol)
                    .ok_or_else(|| format!("未知のシンボル: {symbol}"))
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(Self { cells, width })
    }

    pub(super) fn len(&self) -> usize {
        self.cells.len()
    }

    pub(super) fn width(&self) -> usize {
        self.width
    }

    pub(super) fn iter(&self) -> impl Iterator<Item = &&'static Cell> {
        self.cells.iter()
    }

    pub(super) fn reachables(&self, start: GridIndex) -> impl Iterator<Item = GridIndex> {
        let mut reached = Vec::new();

        let mut stack = vec![start];
        let mut visited = FixedBitSet::with_capacity(self.len());
        visited.insert(start.as_usize());

        while let Some(index) = stack.pop() {
            for index in self.neighbors(index) {
                if visited.put(index.as_usize()) {
                    continue;
                }
                match self[index] {
                    Cell::Wall => {}
                    Cell::Road | Cell::Start => {
                        stack.push(index);
                    }
                    Cell::Object(_) => {
                        reached.push(index);
                    }
                }
            }
        }

        reached.into_iter()
    }

    fn neighbors(&self, index: GridIndex) -> impl Iterator<Item = GridIndex> {
        [
            index.up(self.width),
            index.right(self.width),
            index.down(self.width, self.len()),
            index.left(self.width),
        ]
        .into_iter()
        .flatten()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct GridIndex(usize);

impl From<usize> for GridIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl GridIndex {
    pub(super) fn as_usize(self) -> usize {
        self.0
    }

    pub(crate) fn as_point(self, width: usize) -> Point {
        Point {
            x: self.0 % width,
            y: self.0 / width,
        }
    }

    fn up(self, width: usize) -> Option<Self> {
        self.0.checked_sub(width).map(Self)
    }

    fn right(self, width: usize) -> Option<Self> {
        (self.0 % width != width - 1).then_some(Self(self.0 + 1))
    }

    fn down(self, width: usize, len: usize) -> Option<Self> {
        (self.0 + width < len).then_some(Self(self.0 + width))
    }

    fn left(self, width: usize) -> Option<Self> {
        (!self.0.is_multiple_of(width)).then_some(Self(self.0 - 1))
    }
}
