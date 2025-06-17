
pub trait Area {
    type Output;
    fn area(&self) -> Self::Output;
}

pub struct Rectangle {
    pub length: u32,
    pub breadth: u32,
}

impl Area for Rectangle {
    type Output = u32;
    fn area(&self) -> Self::Output {
        &self.length * &self.breadth
    }
}

pub struct Circle {
   pub radius: f32,
}

impl Area for Circle {
    type Output = f32;
    fn area(&self) -> Self::Output {
        3.14 * (&self.radius * &self.radius)
    }
}

pub fn get_area<T: Area>(item: &T)
where
    T::Output: std::fmt::Debug,
{
    println!("area of given parameter is {:?}", item.area());
}