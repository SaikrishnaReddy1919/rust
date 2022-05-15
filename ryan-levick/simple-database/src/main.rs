use std::collections::HashMap;

fn main() {
    // first argument is file path to executable file so skip 1st one
    let mut arguments = std::env::args().skip(1); 
    let key = arguments.next().expect("key was not there");
    let value = arguments.next().unwrap(); 

    // expect() -> says compiler to crash on error.
    // try removing .expect(), program wont crash.
    let mut database = Database::new().expect("Database must crash");

    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
}

#[derive(Debug)]
struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read from file

        // let contents = match std::fs::read_to_string("new-db.db") {
        //     Ok(c) => c,
        //     Err(error) => return Err(error)
        // };

        // above lines in single line :
        //if there is error in below line err will be returned
        let contents = std::fs::read_to_string("kv.db")?;

        //parse the string
        //populate the map
        let mut map = HashMap::new();
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No More keys.");
            let value = chunks.next().unwrap();

            //.to_string() can also be used, but not efficient check to_string vs to_owned.
            // .clone() wont work
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { map })
    }

    fn insert(&mut self, key:String, value:String) {
        self.map.insert(key, value);
    }

}

// what if we want to call flush after inserting automatically.
// That's where drop comes in...if we impl Drop trait for Database, we can use drop inside Drop and 
// do whatever we want to do(the code inside flush method) inside drop method.

impl Drop for Database {
    fn drop(&mut self) {
        let mut contents = String::new();

        //try removing & to self.map
        for (key, value) in &self.map {
            // method 1 - not efficient
            //reason : push_str is taking red to kvpair...what happens to the created memory after after it is used in push_str ?
            // let kvpair = format!("{}\t{}\n", key, value);
            // contents.push_str(&kvpair);

            //method - 2 : we can also do same without creating new memory for kvpair
            // efficient
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');

            
        }
        // _ means -> keep the result in _ and ignore it.
        let _ = std::fs::write("kv.db", contents);
    }
}