use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Read;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .read(true)
                .open("db.txt")
                .expect("cannot open file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    if config.query == "get" {
        get(&config.params, &file_contents);
    }

    if config.query == "set" {
        set(&config.params, &mut file_contents);
        
    }

    if config.query == "del" {
        del(&config.params, &file_contents);
    }

    if config.query == "print" {
        println!("{}", file_contents);
    }

    Ok(())
}

pub struct Config {
    query: String,
    params: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let query: String;
        let params: String;

        if args.len() == 2 {
            query = args[1].clone();
            params = "".to_string();
            
        } else {
            query = args[1].clone();
            params = args[2].clone();
        }

        Ok(Config { query, params })
    }
}

pub fn get<'a>(query: &str, contents: &'a str) -> Option<&'a str> {
    for line in contents.lines() {
        let kv_vec = line.split(":").collect::<Vec<_>>()[0];
        
        if kv_vec == query.split(":").collect::<Vec<_>>()[0] {
        
            println!("found matching kv_entry {:?}", line);
        
            return Some(line)
        }
    }

    println!("could not find matching kv_entry");

    None
}

pub fn set<'a>(query: &'a str, contents: &'a mut str) -> &'a str {
    let contains_kv = match get(&query, &contents) {
        Some(_) => true, 
        None => false
    };

    if !contains_kv {
        let mut db = OpenOptions::new()
                .append(true)
                .create(true)
                .open("db.txt")
                .expect("cannot open file");
        
        let mut query_w_newline = query.to_string();
        query_w_newline.push_str("\n");

        db.write(&query_w_newline.as_bytes())
                .expect("Unable to write to db file");

        println!("set kv_entry {:?}", query);

        return query
    }
    
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query.split(":").collect::<Vec<_>>()[0]) {

            results.push(line.replace(line, query));
        } else {
            results.push(line.to_string());
        }
    }

    let results_as_str = results.join("\n");

    let mut results_w_newline = results_as_str.to_string();
    results_w_newline.push_str("\n");

    let mut db = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open("db.txt")
                .expect("cannot open file");

    db.write(&results_w_newline.as_bytes())
                .expect("Unable to write to db file");

    println!("set kv_entry {:?}", query);

    query
}

pub fn del<'a>(query: &'a str, contents: &'a str) -> &'a str{
    let contains_kv = match get(&query, &contents) {
        Some(_) => true, 
        None => false
    };

    if !contains_kv {
        println!("No kv entry found for key {:?}", query);
        return query
    }

    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query.split(":").collect::<Vec<_>>()[0]) {

            continue;
        } else {
            results.push(line.to_string());
        }
    }

    let results_as_str = results.join("\n");

    let mut results_w_newline = results_as_str.to_string();
    results_w_newline.push_str("\n");

    let mut db = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open("db.txt")
                .expect("cannot open file");

    db.write(&results_w_newline.as_bytes())
                .expect("Unable to write to db file");

    println!("deleted kv entry {:?}...", query);

    query
}
