#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // first approach
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        areaWithParams(width1, height1)
    );

    // using tuples
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        areaWithTuples(rect1)
    );

    // using structures
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        areaWithStructure(&rect1)
    );

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
}

fn areaWithParams(width: u32, height: u32) -> u32 {
    width * height
}

fn areaWithTuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn areaWithStructure(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}