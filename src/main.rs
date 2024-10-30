mod tree_map;
use tree_map::*;

fn main() {
    println!("Testing basic tree:");
    let mut map: TreeMap<i32, String> = TreeMap::new();

    println!("\tInserting values...");
    map.insert(vec!["a".to_string(), "b".to_string(), "c".to_string()].as_slice(), 1);
    map.insert(vec!["a".to_string(), "b".to_string(), "d".to_string()].as_slice(), 2);
    map.insert(vec!["e".to_string(), "f".to_string()].as_slice(), 3);

    println!("\tReading values...");
    println!("\tabc -> {}", map.find(vec!["a".to_string(), "b".to_string(), "c".to_string()].as_slice()).unwrap_or(&-1));
    println!("\tabd -> {}", map.find(vec!["a".to_string(), "b".to_string(), "d".to_string()].as_slice()).unwrap_or(&-1));
    println!("\tef -> {}", map.find(vec!["e".to_string(), "f".to_string()].as_slice()).unwrap_or(&-1));


    // Static Tree
}
