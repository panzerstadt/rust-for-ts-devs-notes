use crate::shapes::{area::Area, circle::Circle, rect::Rectangle};

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

fn main() {
    let rect = Rectangle::default();

    for point in &rect {}

    println!("{}", rect)
}
