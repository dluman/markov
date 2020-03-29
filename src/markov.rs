extern crate mersenne_twister;
extern crate rand;

use mersenne_twister::MersenneTwister;
use rand::{Rng, SeedableRng};
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

fn weighted_char(table: HashMap<String, usize>)
  -> String {
    let mut choice = String::new();
    
    let mut sum = 0;
    
    for value in table.values() {
      sum += value;
    }
    
    let mut seed = rand::thread_rng()
      .gen_range(1,sum);
      
    'gen: for (key,value) in table {
      if seed <= value {
        choice = key.to_string();
        break 'gen;
      }
      seed -= value;
    }
    
    choice
  }

fn generate_text(length: usize, contents: &String, table: HashMap<String, usize>, order: usize)
  -> String {
    let mut result = String::new();
    
    // Build vector of keys to sample
    let keys: Vec<_> = table
      .keys()
      .cloned()
      .collect();

    // Get random first character sequence
    let mut first_char = keys
      .choose(&mut rand::thread_rng())
      .unwrap();
    
    result.to_owned().push_str(first_char);
    
    // Set up weighted characters
    let mut next_char = String::new();
    for _ in 0..(length / order) {
      match table.get(first_char) {
        Some(&dist) => next_char = weighted_char(table.clone()),
        _ => next_char = keys.choose(&mut rand::thread_rng()).unwrap().to_owned(),
      }
      first_char = &next_char;
      result.push_str(&next_char);
    }
    println!("{}",result);
    
    result
  }

fn generate_table(contents: &String, order: usize) 
  -> HashMap<String, HashMap<String,usize>>{
  
    let mut table = HashMap::new();
    
    // Read text once, store sequences in HashMap
    for x in (0..contents.len()) {
      let seq: String = contents
        .chars()
        .skip(x)
        .take(order)
        .collect();
      table.insert(seq.to_string(),HashMap::new());
    }
    
    // Read text again to associate 
    for x in (0..(contents.len() - order)) {
      let seq: String = contents
        .chars()
        .skip(x)
        .take(order)
        .collect();
      let next: String = contents
        .chars()
        .skip(x + order)
        .take(order)
        .collect();
      table.entry(seq)
        .and_modify(|map| if let Some(value) = map.get_mut(&next) { *value += 1; })
        .or_insert_with(HashMap::new)
        .insert(next,1);
    }
    println!("{:?}",table);
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
  //let result = generate_text(length, &contents, table, order);
  //println!("{:?}",table);
}
