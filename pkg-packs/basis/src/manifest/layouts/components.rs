use crate::paths::AutoPath;
use derive_more::From;
use ordered_float::OrderedFloat;
use rill_protocol::io::provider::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, From)]
pub enum Container {
    Empty,
    Align(Align),
    Row(Row),
    Column(Column),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, From)]
pub struct Align {
    pub alignment: Alignment,
    pub child: Element,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, From)]
pub struct Row {
    pub children: Vec<Element>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, From)]
pub struct Column {
    pub children: Vec<Element>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Alignment {
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
}

impl Alignment {
    pub const fn new(x: f64, y: f64) -> Self {
        Self {
            x: OrderedFloat(x),
            y: OrderedFloat(y),
        }
    }
}

impl Alignment {
    pub const BOTTOM_CENTER: Self = Self::new(0.0, 1.0);
    pub const BOTTOM_LEFT: Self = Self::new(-1.0, 1.0);
    pub const BOTTOM_RIGHT: Self = Self::new(1.0, 1.0);
    pub const CENTER: Self = Self::new(0.0, 0.0);
    pub const CENTER_LEFT: Self = Self::new(-1.0, 0.0);
    pub const CENTER_RIGHT: Self = Self::new(1.0, 0.0);
    pub const TOP_CENTER: Self = Self::new(0.0, -1.0);
    pub const TOP_LEFT: Self = Self::new(-1.0, -1.0);
    pub const TOP_RIGHT: Self = Self::new(1.0, -1.0);
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, From)]
pub enum Element {
    Container(Box<Container>),
    Label(Label),
    Flow(Flow),
}

impl<T: Into<Container>> From<T> for Element {
    fn from(container: T) -> Self {
        Self::Container(Box::new(container.into()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, From)]
pub struct Label {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, From)]
pub struct Flow {
    pub path: Path,
}

impl Flow {
    pub fn new(path: impl Into<AutoPath>) -> Self {
        Self {
            path: path.into().into(),
        }
    }
}
