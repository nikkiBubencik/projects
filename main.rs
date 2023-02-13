use std::collections::HashMap;
use std::io::{BufRead};
use std::io;

fn main() {

    let mut fgroups: HashMap<String, Vec<String>> = HashMap::new();
    
    for line in io::stdin().lock().lines() {
        let l = line.unwrap();

        // line is only newline
        if l.is_empty() {
            eprintln!("Bad line");
            continue; 
        // starts with new line
        } else if l.starts_with("\n") {
            eprintln!("Bad line");
            continue; 
        // starts with whitespace
        }else if l.starts_with(char::is_whitespace) {
            eprintln!("Bad line");
            continue;
        // there are no nonwhite characters
        } else if l.trim().find(" ") == None {
            eprintln!("Bad line");
            continue;
        } 

        let index = l.find(char::is_whitespace).unwrap();
        let fingerprint = &l[..index];
        let name = &l[index..].trim_start();
        
        let words: Vec<_> = vec![fingerprint, name];
        match fgroups.get_mut(words[0]) {
            Some(c) => {
                c.push(words[1].to_string());
            }
            None => {
                fgroups.insert( words[0].to_string(), vec![words[1].to_string()]);
            }
        }
        
    } 

    let multinames: Vec<_> = fgroups.into_iter().filter(|(_, v)| v.len() > 1).collect();

    let mut counter = 0;
    for (_, name) in multinames.iter() {
        for i in name{
            println!("{}", i);
        }
        counter += 1;
        if counter < multinames.len(){
            println!();
        }
    }    
}
