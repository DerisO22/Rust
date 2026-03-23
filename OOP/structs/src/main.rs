struct Rectangle {
    width: f64,
    height: f64
}

trait ShapeSummary {
    fn calculateArea(&self) -> f64;
}

impl Rectangle  {
    fn new() -> Rectangle {
        Rectangle {
            width: 10.0,
            height: 10.0
        }
    }
}

impl ShapeSummary for Rectangle {
    fn calculateArea(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let rec: Rectangle = Rectangle::new();

    println!("Width: {}, Height: {}, area: {:.2}", &rec.width, &rec.height, rec.calculateArea());
}