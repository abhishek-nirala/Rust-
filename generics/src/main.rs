mod g_structs;

fn main() {
    println!("Start!");

    let v = vec!['a','g','b','z','5','A','0','!'];

    // let mut largest = &v[0];

    // for item in &v {
    //     if item > largest {
    //         largest = item;
    //     }
    // }

    // println!("largest : {largest}");

    let ans = largest_item(&v);

    println!("ans_char : {ans}");

    let v = vec![3,2,5,6,4,6,65,4,6,3];
    let ans = largest_item(&v);
    println!("ans_int: {ans}");
    

    g_structs::main_copy();
    
    println!("End!");
}


fn largest_item<T: std::cmp::PartialOrd>(items:&[T]) -> &T{
    let mut largest = &items[0];

    for item in items {
        if item > largest {
            largest = item;
        }
    };

    // println!("{items:?}");
    largest
}
