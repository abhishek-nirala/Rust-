enum Ranks {
    First,
    Second,
    Third,
    Top(u32),
    Fail,
}

fn main() {
    println!("Start!");

    rank(&Ranks::Top(489));
    rank(&Ranks::Fail);
    rank(&Ranks::First);
    rank(&Ranks::Second);
    rank(&Ranks::Third);

    let x = Some(5);
    let _six = plus_one(x);
    let none = plus_one(None);
    println!("none : {:#?}", none);

    //if let syntax contorl_flow using if let.
    //

    println!("End!");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn rank(ranks: &Ranks) {
    match ranks {
        Ranks::First => println!("marks above 300 but less than 450"),
        Ranks::Second => println!("marks above 250 but less than 300"),
        Ranks::Third => println!("marks above 200 but less than 250"),
        Ranks::Top(marks) =>{
            println!("marks above 450 and the marks is {marks}");
        } 
        Ranks::Fail => println!("less than 30 in any subject"),
    }
    println!("ranks!");
}
