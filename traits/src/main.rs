mod cal_area;
use cal_area::Area;

use crate::cal_area::get_area; // Bringing the Area trait into scope.
// use cal_area::Rectangle;     //this is also correct but it is advised to import only Parent module and use it like in line 8.

fn main() {
    println!("Start!");

    let rect = cal_area::Rectangle {
        length : 4,
        breadth : 6
    };
    println!("Area of rectangle is {}",rect.area());

    let area_of_circle = cal_area::Circle{
        radius : 5.0
    };
    get_area(&rect);
    println!("Area of circle is {}",area_of_circle.area());

    println!("End!");
}
