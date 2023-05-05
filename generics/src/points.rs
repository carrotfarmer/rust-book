struct Point<T, U> {
    x: T,
    y: U 
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let p1 = Point { x: 6, y: 9 };
    let p2 = Point { x: 6.0, y: 9.0 };
    let p3 = Point { x: 6, y: 9.0 };
}
