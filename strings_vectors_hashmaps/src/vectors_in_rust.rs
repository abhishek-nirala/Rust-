
pub fn vectors() {
    let mut vec = Vec::new();

    vec.push(45);
    vec.push(43);
    println!("vec : {:?}", vec);

    let mut v = vec![2, 3, 5, 3, 5, 5, 4, 6, 4, 6];

    v.push(45);
    println!("v : {:?}", v);

    let third = &v[2]; //index method to get an element;

    println!("third : {}", third);

    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{i}");
    }

    let fourth = v.get(3); //get() method which returns Option<&T>;        
    match fourth {
        Some(i) => println!("fourth : {}", i),
        None => println!("invalid! index out of bound"),
    }

    enum MixedData {
        Integer(u8),
        Text(String),
    }

    let mut data_collection: Vec<MixedData> = Vec::new();

    data_collection.push(MixedData::Integer(32));
    data_collection.push(MixedData::Text(String::from("tofu ")));
    data_collection.push(MixedData::Integer(23));
    data_collection.push(MixedData::Text(String::from("titanic")));
    data_collection.push(MixedData::Integer(76));

    for item in data_collection {
        match item {
            MixedData::Integer(item) => {
                println!("Text {item}");
            },
            MixedData::Text(item) => {
                println!("String {item}");
            }
        }
    }
}
