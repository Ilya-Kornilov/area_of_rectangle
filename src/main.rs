#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // implementation methods always take "&self" as their 1st argument
    fn area_4(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    // associated functions do NOT take "&self" arguments
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
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

    println!();
    println!(" > rect_3: {:?}", rect_3);
    println!(" ... or > rect_3: 
    {:#?}", rect_3);

    println!(
        " > 4:
    >> the area of the rectangle is {} square pixels",
        rect_3.area_4()
    );

    let rect_4 = Rectangle {
        width: 20,
        height: 40
    };

    let rect_5 = Rectangle {
        width: 40,
        height: 50
    };

    println!();
    println!("rect_3 can hold rect_4: {}:  {:?} , {:?}", rect_3.can_hold(&rect_4), rect_3, rect_4);
    println!("rect_3 can hold rect_5: {}: {:?} , {:?}", rect_3.can_hold(&rect_5), rect_3, rect_5);
    
    println!();
    let rect_6 = Rectangle::square(25);
    println!(" > rect_6: {:?} => ⏹️", rect_6);

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