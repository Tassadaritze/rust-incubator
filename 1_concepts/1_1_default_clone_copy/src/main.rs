#[derive(Copy, Clone, Default)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Polyline {
    // I'm assuming the "set" in "represents a non-empty set" is not meant literally
    points: Vec<Point>,
}

fn main() {
    let a = Point { x: -1, y: 1 };
    let b = a;
    let c = a;

    let polyline = Polyline {
        points: vec![a, b, c],
    };
    // let polyline_b = polyline;
    let polyline_b = polyline.clone();
    let polyline_c = polyline;
}
