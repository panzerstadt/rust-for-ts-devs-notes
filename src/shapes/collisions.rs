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

pub struct PointIter {
    points: Vec<(f64, f64)>,
    idx: usize,
}

impl Iterator for PointIter {
    type Item = (f64, f64);

    // rusty
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;

        return self.points.get(idx).map(|x| *x); // deref means you COPY the data
                                                 // *x only works if the underlying datatype has the Copy Trait
    }
}

// can't implement foreign traits for foreign types
// impl From<Vec<(f64, f64)>> for f64 {
impl From<Vec<(f64, f64)>> for PointIter {
    fn from(points: Vec<(f64, f64)>) -> Self {
        return PointIter { points, idx: 0 };
    }
}

pub trait Points {
    fn points(&self) -> PointIter;
}

pub trait Contains {
    fn contains_point(&self, point: (f64, f64)) -> bool;
}

// for any T that implements Points and Contains,
// we will implement Collidable for them
// T is other, V is self
impl<Other, Me> Collidable<Other> for Me
where
    Other: Points,
    Me: Contains,
{
    fn collide(&self, other: &Other) -> bool {
        for point in other.points() {
            if self.contains_point(point) {
                return true;
            }
        }
        return false;
    }
}
