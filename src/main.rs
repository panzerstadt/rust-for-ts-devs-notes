mod shapes;

use shapes::{Area, Circle, Rectangle};

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
    let mut rect = Rectangle {
        x: 5.0,
        y: 5.0,
        width: 2.0,
        height: 2.0,
    };
    let circ = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };

    println!("{:?}", rect.area());
    println!("{:?}", circ.area());
    println!("{:?}", 4.2.area());

    rect.enlarge(2.0);

    println!("{:?}", rect.area());
}
