/*
Summary -> Ques 1. Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/

use std::collections::HashMap;

pub fn median_mod() {
    println!("--------------------------inside midian_mode--------------------------");
    let mut v: Vec<u8> = vec![2, 45,2,56,3,32, 4, 23, 5, 76, 3, 56, 3, 65];
    v.sort();
    let len_of_v = v.len();
    let median_of_v = len_of_v / 2;
    println!("median of v : {}", v[median_of_v]);
    // println!("v : {:?}", v);

    let mut map: HashMap<u8, u8> = HashMap::new();

    for item in &v {
        let count = map.entry(*item).or_insert(0);
        *count += 1;
    }
    println!("mode of v : {map:?}");
}
