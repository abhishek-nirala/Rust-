use std::collections::HashMap;

pub fn hashmaps() {
    println!("------------------in hash_maps-------------------------------");

    let mut scores: HashMap<String, u8> = HashMap::new();
    scores.insert(String::from("blue"), 43);
    scores.insert(String::from("yellow"), 23);

    scores.insert(String::from("pink"), 20);
    scores.insert(String::from("violet"), 32);
    // scores.insert(String::from("violet"), 65);  //it will override 32 & value will be 65;

    // println!("check entry : {:?}",scores.entry("green".to_string())); //checks if the value is present or not in the hash table.
    println!(
        "check entry : {:?}",
        scores.entry("green".to_string()).or_insert(46)
    );

    for (key, value) in &scores {
        println!("{key} : {value}");
    }
    println!("scores : {scores:?}");


    //counting occurrence of all the words.
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

//demonstrating the use of deference `*` operator which is similar to c/cpp where '*' is used to access the value at a given reference(address). Here also as can be seen the 'y' isn't marked as 'mut' but still able to change the value of the 'x' variable which 'y' is storing the reference of variable 'x'. The 'y' is still holding the mutable reference of the variable 'x', it only changed the value at which the reference is pointing to.
    // let mut x = 45;
    // let y = &mut x;
    // *y+=1;
    // println!("y : {}",y);
}
