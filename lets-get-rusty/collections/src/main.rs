#![allow(unused_doc_comments)]
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
fn main() {
    println!("Hello, world!");
    let a: [i32; 4] = [1, 2, 3, 4];

    // can grow in size
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);

    //create and initialize vec with values -> vec macro
    {
        let v2 = vec![1, 2, 3];
    } //scope of v2 will d ropped here -> heap

    let mut v3 = vec![1, 2, 3, 4];
    let third = &v3[3];
    //run below line -> gets runtime error index out of bounds.
    //runtime becaz size does not know while compile time.
    // incase of arrays we have to fix these type of error other program will not be compiled.
    // -> compile time error..array size known at compile time.
    // let tenty = &v3[20];

    //cannot borrow `v3` as mutable because it is also borrowed as immutable -> line 17
    //mutable borrow occurs here
    // #[allow(dead_code)]
    /**
     * Reason :
     * below line tries to add 7 to the vector v3. To do this, we need to allocate more
     * memory to the vector to add this new value. To do this we need to move all the elements
     * to the new memory locations. When this happens, the variable above 'third' might be
     * pointing to some other location. So....
     */
    //v3.push(7); //-> UNCOMMENT THIS TO CHECK ERROR
    println!("third : {}", third);

    //if we want our program to run even if there is no element or want to handle
    //index out of bounds, we can do like :
    match v.get(20) {
        //v.get returns an Option
        //this will not throw runtime error
        Some(twenty) => println!("20th element is : {}", twenty),
        None => println!("No 20th element"),
    }

    //Iterate
    let mut itr = vec![1, 2, 3, 4, 5];

    //mutable ref to itr
    for i in &mut itr {
        *i += 50;
    }

    //immutable ref to itr
    for i in &itr {
        println!("Index : {}", i);
    }

    // accessing vec element two ways : by index, and get method
    let v11 = vec![1, 2, 3, 4, 5];
    let does_not_exist = &v[100]; // code will panic and crashes
    let does_not_exist = v.get(100); // returns an option, err can be handled in None Case

    //following code wont work
    let mut v9 = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v9.push(6);
    println!("The first element is: {}", first); // at this line 'first' might be moved to new memory location

    //vectors only store one type of data, what if we want vec to store diff types of data ?
    // solution -> store enums inside vec
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    // now row vec can store value of type int, float, text
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // strings are store as a collection of UTF-8 encoded bytes
    let s1 = String::new();
    let s2 = "slics_string";
    let s3 = s1.to_string();
    let mut s4 = String::from("krishna");

    //appending to string
    s4.push_str("sai");
    s4.push('s');

    let hello = String::from("Hello ");
    let world = String::from("World");
    //hello ownership moved to s_mix, world -> copied
    let s_mix = hello + &world;
    // println!("Cant print hello : {}", hello);

    // can be cancatenated usinf 'format' macro
    //format macro doesnt take ownership of things.
    // let s_mix = format!("{}{}", hello, world);

    //we cannot access chars in string using index. caz, string is store as bytes. Diff languages has diff length for chars
    //ex

    let hello1 = String::from("krishna");

    //in bytes
    //[123, 324, 325, 545, 566, 544, ...] //like this

    println!("-----bytes------");
    //to access in bytes
    for i in hello1.bytes() {
        println!("{}", i);
    }

    //scalar values
    //[<has diff length>]
    println!("-----scalar------");

    //to access scalar values
    for i in hello1.chars() {
        println!("{}", i);
    }
    println!("-----grapheme------");

    //grapheme clusters
    //[<has diff length>]

    //crate is needed to iterate thorugh grapheme clusters -> unicode-segmentation
    for i in hello1.graphemes(true) {
        println!("{}", i);
    }
    //so we cannot chars like -> hello1[0]

    //HAshMap

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    //ownership of blue and yellow is moved...can be referenced -> will be covered in lifetimes
    scores.insert(blue, 10);
    scores.insert(yellow, 20);

    // println!("Cant print : {}", blue);

    //iterate
    for (key, value) in &scores {
        println!("Key : {} and value : {}", key, value);
    }

    //getting value
    // returns an option -> caz compiler dont know if value is available or not -> heap, runtime
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Blue team score is : {}", score.unwrap());

    //updating hashmap
    let mut fruits: HashMap<String, i32> = HashMap::new();

    fruits.insert(String::from("apple"), 10);
    fruits.insert(String::from("apple"), 20); //overrides above banana

    //to update if there is no entry
    fruits.entry(String::from("banana")).or_insert(10);
    fruits.entry(String::from("banana")).or_insert(20); // at this line banana is there so, this line will not update banana

    for (key, value) in &fruits {
        println!("Fruit : {} and Quantity : {}", key, value);
    }

    //example
    // count how many times a word appears in a string and update the count as value and word as key in hashmap

    let text = "hello world wonderful world";

    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }

    println!("{:?}", map);
}
