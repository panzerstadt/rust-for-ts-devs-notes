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
    let mut rect = Rectangle::default();
    let circ = Circle::default();

    println!("{}", rect);
    println!("{:?}", rect.area());
    println!("{:?}", circ);
    println!("{:?}", circ.area());
    println!("{:?}", 4.2.area());

    rect.enlarge(2.0);

    println!("{:?}", rect.area());
}
