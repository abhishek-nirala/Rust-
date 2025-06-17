#[derive(Debug)]
pub struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn get(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}

struct  Coordinates <T,U> {
    p : T,
    q : U,
}

impl<T,U> Coordinates<T,U> {
    fn new(p:T, q:U) -> Self {
        Self { p, q}
    }
    fn get(&self) -> (&T, &U) {
        (&self.p, &self.q)
    }
}
pub fn main_copy() {
    println!("__________g_struct_____________");
    let point = Point { x: 4, y: 6 };
    println!("point : {point:?}");
    let point = Point::new(4.4, 6.7);
    println!("point : {:?}", point.get());
    
    let coordinates = Coordinates::new(4,String::from("tofu")); //passing a i32 and f64 value
    println!("coordinates : {:?}",coordinates.get());


}
