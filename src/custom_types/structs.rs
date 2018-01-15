
struct Nil;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { 
        p1: Point { x: x1, y: y1 }, 
        p2: Point { x: x2, y: y2 }
        } = rect;
    (x2 - x1) * (y2 - y1)
}

// fn square(point: Point, num: f32) -> Rectangle {
// }

pub fn run(){
    let point: Point = Point { x: 0.3, y:0.4};

    println!("point coordinates: ({}, {})", point.x, point.y);

    let Point { x: my_x, y:my_y} = point;

    let _rectangle = Rectangle{
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // e.g. 1
    let rect = Rectangle {
        p1: Point {x: 1.0, y: 2.0},
        p2: Point {x: 3.0, y: 5.0},
    };
    println!("area is {}", rect_area(rect));

}