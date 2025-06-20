fn main() {
    println!("Start!");
    let  v1 = vec![1,2,3,4,5,6,7,8];
    let mut iter = v1.iter();
    // for i in iter {
    //     // println!("{i}");
    //     *i +=1;
    // // }
    
    while let Some(val) = iter.next() {
        println!("val : {}",val+1);
    }



    println!("{:?}",v1);
    println!("End!");
}
