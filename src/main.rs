use crate::shapes::{area::Area, circle::Circle, collisions::Collidable, rect::Rectangle};

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
    let rect2 = Rectangle::default();
    let circle = Circle::default();
    let circle2 = Circle::default();

    rect.collide(&rect2);
    circle.collide(&circle2);

    rect.collide(&circle);
    circle.collide(&rect);

    println!("{}", rect)
}
