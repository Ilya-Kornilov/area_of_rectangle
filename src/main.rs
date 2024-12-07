struct Rectangle {
    width: u32,
    height: u32
}

fn main() {

    let width = 30;
    let height = 50;
    
    println!(
        " > 1: 
    >> the area of the rectangle is {} square pixels",
        area(width, height)
    );


    let rect = (width, height);

    println!(
        " > 2: 
    >> the area of the rectangle is {} square pixels",
        area_2(rect)
    );

    let rect_3 = Rectangle {
        width: 30, 
        height: 50
    };

    println!(
        " > 3: 
    >> the area of the rectangle is {} square pixels",
        area_3(&rect_3)
    );


}

fn area(width: u32, height: u32) -> u32
{
    width * height
}

fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}