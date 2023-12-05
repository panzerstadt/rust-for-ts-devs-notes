use std::{fmt::Display, str::FromStr};

use crate::shapes::{circle::Circle, collisions::Collidable, rect::Rectangle};
use anyhow::Result;
use shapes::collisions::{Contains, Points};

mod shapes;

trait Enlarge {
    fn enlarge(&mut self, multiple: f64);
}

impl Enlarge for Rectangle {
    fn enlarge(&mut self, multiple: f64) {
        self.height *= multiple;
        self.width *= multiple;
    }
}

#[derive(Debug)]
enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (shape, data) = s.split_once(" ").unwrap_or(("", ""));

        match shape {
            "rect" => return Ok(Shape::Rectangle(data.parse()?)),
            "circle" => return Ok(Shape::Circle(data.parse()?)),
            _ => return Err(anyhow::anyhow!("bad shape!")),
        }
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Circle(c) => write!(f, "{}", c),
            Shape::Rectangle(r) => write!(f, "{}", r),
        }
    }
}

impl Points for &Shape {
    fn points(&self) -> shapes::collisions::PointIter {
        match self {
            Shape::Circle(c) => return c.points(),
            Shape::Rectangle(r) => return r.points(),
        }
    }
}

impl Contains for &Shape {
    fn contains_point(&self, point: (f64, f64)) -> bool {
        match self {
            Shape::Circle(c) => c.contains_point(point),
            Shape::Rectangle(r) => r.contains_point(point),
        }
    }
}

fn main() -> Result<()> {
    // read file
    let file = std::fs::read_to_string("shapes.file")?;

    // read each line and use split_one
    let shapes = file
        .lines()
        .filter_map(|x| x.parse::<Shape>().ok())
        .collect::<Vec<_>>();

    // gawd
    shapes
        .iter()
        .skip(1)
        .zip(shapes.iter().take(shapes.len() - 1))
        .filter(|(a, b)| a.collide(b))
        .for_each(|(a, b)| println!("{} collides with {}", a, b));

    return Ok(());
}

/*
next steps
- cli, implement ls
- websockets, learn about async rust
- wasm, leptos (like solid.js)
*/
