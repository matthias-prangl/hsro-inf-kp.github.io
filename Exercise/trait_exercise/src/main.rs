trait TwoDimensional {
    fn area(&self) -> f64;
    fn circumference(&self) -> f64;
}

struct Square {
    x: f64,
}

struct Rectangle {
    x: f64,
    y: f64,
}

struct Circle {
    r: f64,
}

impl TwoDimensional for Square {
    fn area(&self) -> f64 {
        self.x * self.x
    }

    fn circumference(&self) -> f64 {
        self.x * 4.0
    }
}

impl TwoDimensional for Rectangle {
    fn area(&self) -> f64 {
        self.x * self.y
    }

    fn circumference(&self) -> f64 {
        self.x * 2.0 + self.y * 2.0
    }
}

impl TwoDimensional for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.r).powf(2.0)
    }

    fn circumference(&self) -> f64 {
        2.0 * self.r * std::f64::consts::PI
    }
}

fn get_dimensions<T: TwoDimensional>(shape: &T) -> (f64, f64) {
    (shape.area(), shape.circumference())
}

impl Drop for Square {
    fn drop(&mut self) {
        println!("Square dropped!");
    }
}

fn main() {
    let dimensions_rec = get_dimensions(&Rectangle{ x: 2.0, y: 7.0});
    println!("{:?}", dimensions_rec);
    let v = vec![ Square{x: 1.0}, Square{x: 1.0}, Square{x: 1.0}, Square{x: 1.0}, Square{x: 1.0} ];
    for i in v {

    }
    println!("something");
}
