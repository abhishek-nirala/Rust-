// use std::io::Split;

// pub fn summary_ques_two() {
/*
2. Convert strings to pig latin. the first consonant of each word is moved
   to the end of the word and 'ay' is added, so first becomes 'irst-fay'. Words that starts
   with a vowel have hay added to the end instead therefore 'apple' becomes 'apple-hay'. Keep
   in mind the details about UTF-8 encoding!
 */

// enum Types {
//     Vowel(String),
//     Consonant(String)
// }
// let vowel = ['a', 'e', 'i', 'o', 'u'];
// let word = String::from("first");
// let mut vec = Vec::new();
// for i in word.chars() {
//     println!("{i}");
//     vec.push(i);
// }
// for i in vowel {
//     if vec[0] == i {
//         println!("{word}-hay");
//         break;
//     }
// }
// let pig_latin = &vec[1..];
// println!("{:?}-{}ay",pig_latin.join(), vec[0]);
// println!("{vowel:?}, {word}");

// match word {
//     word
// }
// }
fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

fn to_pig_latin(word: &str) -> String {
    let mut chars = word.chars();
    // println!("chars : {:?}",chars);
    if let Some(first) = chars.next() {
        if is_vowel(first) {
            format!("{}-hay", word)
        } else {
            let rest: String = chars.collect();
            format!("{}-{}ay", rest, first)
        }
    } else {
        String::new()
    }
}

pub fn main_entry() {
    let sentence = "apple first";
    let pig_latin: Vec<String> = sentence.split_whitespace().map(to_pig_latin).collect();

    println!("{}", pig_latin.join(" "));
}
