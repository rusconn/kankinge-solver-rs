use crate::object::Object;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum Cell {
    Wall,
    Road,
    Start,
    Object(&'static Object),
}
