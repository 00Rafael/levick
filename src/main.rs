use std::{iter::Skip, fmt::format};
use std::collections::HashMap;
use std::path::Path;
fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);
    let contents= format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).unwrap();
    assert!(Path::new("kv.db").exists());
    let database = Database::new().expect("Creating db failed");
}

struct Database{  
    map: HashMap<String,String>,

}

impl Database {
    fn new()-> Result<Database,std::io::Error> {
        //read the kv.db file
    
            let mut map= HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?; 
       for line in contents.lines(){
          let mut chunks = line.splitn(2,'\t');
          let key = chunks.next().expect("No key!!");
          let value = chunks.next().expect("No value!!");
           map.insert(key.to_owned(), value.to_owned());
        }
       Ok( Database { 
            map: HashMap::new(),
         })
    }
}