fn main() {
    println!("Start!");
    let ans = sum_of_digits(934);
    println!("ans : {ans}");
    println!("finished!")
}

fn sum_of_digits(n: u32) -> u32 {
    let mut temp = n;
    let mut sum: u32 = 0;
    while temp > 0 {
        sum += temp % 10;
        temp /= 10;
    }
    sum
}

//fn fibonacci (mut n:u32){
//    let mut fib;
//    let mut a = 0;
//    let mut b = 1;
//    print!("0, 1");
//    while n > 0 {
//        fib = a + b;
//        a = b;
//        b = fib;
//
//    print!(" {},",fib);
//    n -=1;
//    }
//}

//fn factorial(num: u64) -> u64 {
//    if num == 0 {
//        return 1;
//    }
//    (1..=num).product()  //implicit return by ommiting the semicolon at the end;
//}

//fn gcd(mut a: i32, mut b: i32) -> i32 {
//    let mut remainder = 1;
//    let mut q = 0;
//    while remainder != 0 {
//        remainder = b % a;
//        q = b / a;
//        b = a;
//        a = remainder;
//    }
//    return b;
//}

//fn hcf(a:i32, b:i32) -> i32 {
//    let mut ans = 0;
//    for i in 1..= a/2 {
//        if a % i ==0  && b % i ==0 {
//            ans = i ;
//        }
//    }
//    return ans;
//}

//fn check_prime(num: i32) -> bool {   //prime or not.
//    for i in 2..=num / 2 {
//        if num % i == 0 {
//            return false;
//
//        }
//    }
//    return true;
//}

//fn even_sum(num: i32) -> i32 {       //sum of all even nums till given num.
//    let mut sum = 0;
//    for i in 0..=num {
//        if i % 2 == 0 {
//            sum += i;
//        }
//    }
//    return sum;
//}
