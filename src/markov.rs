extern crate rand;

use rand::{Rng};
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

fn is_upper(c: &String)
  -> bool {
    let mut upper: bool = false;
    if c.is_empty() {
      return upper;
    }
    upper = c
      .chars()
      .next()
      .unwrap()
      .is_ascii_uppercase();
    upper
  }

fn weighted_char(map: HashMap<String, usize>)
  -> String {
    
    if map.is_empty() {
      return " ".to_string();
    }
  
    let mut choice = String::new();
    
    let mut sum = 0;
    
    for (_,value) in &map {
      sum += value;
    }
    
    let mut seed = rand::thread_rng()
      .gen_range(0,sum);
      
    'gen: for (key,value) in &map {
      if seed <= *value {
          choice = key.to_string();
          break 'gen;
      }
      seed -= value;
    }
    
    choice
  }

fn generate_text(length: usize, table: HashMap<String,HashMap<String, usize>>, order: usize)
  -> String {
    let mut result = String::new();
    
    // Build vector of keys to sample
    let keys: Vec<_> = table
      .keys()
      .cloned()
      .collect();

    // Get random first character from keys
    let mut first_char = String::new();
    
    while !is_upper(&first_char) {
    first_char = keys
       .choose(&mut rand::thread_rng())
       .unwrap()
       .to_string();
    }
    
    result.push_str(&first_char);
    
    let mut next_char;
    for _ in 0..(length / order) {
      match table.get(&first_char) {
        Some(map) => next_char = weighted_char(map.to_owned()),
        _ => next_char = keys
          .choose(&mut rand::thread_rng())
          .unwrap()
          .to_owned(),
      }

      first_char = next_char.to_owned();
      result.push_str(&next_char);
    }

    result
  }

fn generate_table(contents: &String, order: usize) 
  -> HashMap<String, HashMap<String,usize>>{
  
    let mut table = HashMap::new();
    
    // Read contents to associate 
    for x in 0..(contents.len() - order) {
      let seq: String = contents
        .chars()
        .skip(x)
        .take(order)
        .collect();
      if !table.contains_key(&seq.to_owned()) {
        table.insert(seq.clone(),HashMap::new());
      }
      let next: String = contents
        .chars()
        .skip(x + order)
        .take(order)
        .collect();
      table.entry(seq)
        .and_modify(|map|
          if let Some(value) = map.get_mut(&next) {
            *value += 1;
          } else {
            map.insert(next,1);
          }
        );
    }
    
    // Return resulting table as HashMap
    table
  }

fn read(filename: String)
  -> String {
    let mut contents = String::new();
    let mut fh = File::open(filename)
      .expect("Unable to open file.");
    fh.read_to_string(&mut contents)
      .expect("Could not read file.");
    contents
  }

fn main() {
  let args: Vec<String> = env::args()
    .skip(1)
    .collect();
  let contents = read(args[0].clone());
  let order = args[1]
    .parse::<usize>()
    .unwrap();
  let length = args[2]
    .parse::<usize>()
    .unwrap();
  let table = generate_table(&contents, order);
  let result = generate_text(length, table, order);
  println!("{}",result);
}
