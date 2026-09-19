use std::{error, num::NonZeroUsize, ops::Index};

use fixedbitset::FixedBitSet;
use serde_json::Value;

use crate::Point;

use super::{Cell, symbols};

#[derive(Debug)]
pub(super) struct Grid {
    cells: Vec<&'static Cell>,
    width: NonZeroUsize,
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

        let Ok(width) = NonZeroUsize::try_from(lines[0].chars().count()) else {
            return Err("mapは1列以上でなければならない".into());
        };
        if lines.iter().any(|line| line.chars().count() != width.get()) {
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

    pub(super) fn point_of(&self, index: GridIndex) -> Point {
        Point {
            x: index.as_usize() % self.width.get(),
            y: index.as_usize() / self.width.get(),
        }
    }

    fn neighbors(&self, index: GridIndex) -> impl Iterator<Item = GridIndex> {
        [
            self.up(index),
            self.right(index),
            self.down(index),
            self.left(index),
        ]
        .into_iter()
        .flatten()
    }

    fn up(&self, index: GridIndex) -> Option<GridIndex> {
        index
            .as_usize()
            .checked_sub(self.width.get())
            .map(GridIndex)
    }

    fn right(&self, index: GridIndex) -> Option<GridIndex> {
        index
            .as_usize()
            .checked_add(1)
            .filter(|_| index.as_usize() % self.width.get() != self.width.get() - 1)
            .map(GridIndex)
    }

    fn down(&self, index: GridIndex) -> Option<GridIndex> {
        index
            .as_usize()
            .checked_add(self.width.get())
            .filter(|&added| added < self.len())
            .map(GridIndex)
    }

    fn left(&self, index: GridIndex) -> Option<GridIndex> {
        index
            .as_usize()
            .checked_sub(1)
            .filter(|_| !index.as_usize().is_multiple_of(self.width.get()))
            .map(GridIndex)
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
}
