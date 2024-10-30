mod tree_map;
use tree_map::*;

fn main() {
    let mut map: TreeMap<i32, String> = TreeMap::new();

    println!("Inserting values...");
    map.insert(vec!["a".to_string(), "b".to_string(), "c".to_string()].as_slice(), 1);
    map.insert(vec!["a".to_string(), "b".to_string(), "d".to_string()].as_slice(), 2);
    map.insert(vec!["e".to_string(), "f".to_string()].as_slice(), 3);

    println!("Reading values...");
    println!("abc -> {}", map.find(vec!["a".to_string(), "b".to_string(), "c".to_string()].as_slice()).unwrap_or(&-1));
    println!("abd -> {}", map.find(vec!["a".to_string(), "b".to_string(), "d".to_string()].as_slice()).unwrap_or(&-1));
    println!("ef -> {}", map.find(vec!["e".to_string(), "f".to_string()].as_slice()).unwrap_or(&-1));
}
