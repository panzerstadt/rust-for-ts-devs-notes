pub trait Collidable<T> {
    fn collide(&self, other: &T) -> bool;

    // this is a trait, but it can have an implementation!
    // this is because if you have collide(), you can use them already
    // you can also provide a default implementation
    // so long as the user implements collide(), we can already provide collides() to the user
    fn collides(&self, others: &[T]) -> bool {
        for other in others {
            if self.collide(other) {
                return true;
            }
        }
        return false;
    }
}
