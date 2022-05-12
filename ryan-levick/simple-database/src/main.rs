use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1); // first argument is file path so skip 1st one
    let key = arguments.next().expect("key was not there");
    let value = arguments.next().unwrap();

    println!("Key is : {} and value is : {}", key, value);

    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("new-db.db", contents).unwrap();

    // expect() -> says compiler to crash on error.
    // try removing .expect(), program wont crash.
    let database = Database::new().expect("Database must crash");
}

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
        let contents = std::fs::read_to_string("new-db.db")?;

        //parse the string
        //populate the map
        let mut map = HashMap::new();
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No More keys.");
            let value = chunks.next().unwrap();

            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { map: map })
    }
}
