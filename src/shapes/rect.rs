use super::{area::Area, circle::Circle, collisions::Collidable};
use std::fmt::Display;

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        return self.x <= x && self.x + self.width >= x && self.y <= y && self.y + self.height >= y;
    }
}

impl Collidable<Rectangle> for Rectangle {
    fn collide(&self, other: &Rectangle) -> bool {
        for point in other {
            if self.contains_point(point) {
                return true;
            }
        }
        return false;
    }
}

impl Collidable<Circle> for Rectangle {
    fn collide(&self, other: &Circle) -> bool {
        return self.contains_point((other.x, other.y));
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        return Rectangle {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        };
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Rectangle({}, {}): {}x{}",
            self.x, self.y, self.width, self.height
        );
    }
}

pub struct RectIter {
    points: Vec<(f64, f64)>,
    idx: usize,
}

impl Iterator for RectIter {
    type Item = (f64, f64);

    // typescripty
    // fn next(&mut self) -> Option<Self::Item> {
    //     if self.idx >= self.points.len() {
    //         return None; // None is the signal to the outside world that you're done with the iterating
    //     }
    //     // 1. get current point
    //     let point = self.points[self.idx];
    //     // 2. increment index
    //     self.idx += 1;

    //     return Some(point);
    // }

    // rusty
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;

        return self.points.get(idx).map(|x| *x); // deref means you COPY the data
                                                 // *x only works if the underlying datatype has the Copy Trait
    }
}

// v2 even better refactor, this is the From trait, a built-in trait
impl From<&Rectangle> for RectIter {
    fn from(value: &Rectangle) -> Self {
        return RectIter {
            points: vec![
                (value.x, value.y),
                (value.x + value.width, value.y),
                (value.x, value.y + value.height),
                (value.x + value.width, value.y + value.height),
            ],
            idx: 0,
        };
    }
}

// v1 refactor to reduce duplication
// impl RectIter {
//     // can be called anything you want, not just new()
//     fn new(rect: &Rectangle) -> Self {
//         return RectIter {
//             points: vec![
//                 (rect.x, rect.y),
//                 (rect.x + rect.width, rect.y),
//                 (rect.x, rect.y + rect.height),
//                 (rect.x + rect.width, rect.y + rect.height),
//             ],
//             idx: 0,
//         };
//     }
// }

impl IntoIterator for Rectangle {
    type Item = (f64, f64);

    type IntoIter = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        // return RectIter::new(&self); // v1
        return (&self).into(); // v2
    }
}

impl IntoIterator for &Rectangle {
    type Item = (f64, f64);

    type IntoIter = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        // return RectIter::new(self); // v1
        return self.into(); // v2
    }
}
