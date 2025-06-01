fn main() {
    println!("Narayan Narayan");

    let res: bool = odd_even(45);

    if res {
        println!("even");
    } else {
        print!("odd")
    }
}

fn odd_even(n: i32) -> bool {
    n % 2 == 0
}
