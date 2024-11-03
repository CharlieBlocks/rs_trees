mod tree_map;
mod dynamic_array;
mod static_tree;
mod counted_tree_map;
mod static_tree_planner;
use tree_map::*;
use static_tree_planner::StaticTreePlanner;
use static_tree::TreeBranch;

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
    println!("Testing static tree:");
    let mut builder: StaticTreePlanner<i32, String> = StaticTreePlanner::new();

    println!("\tInserting values into builder...");
    builder = builder.add(vec!["a".to_string(), "b".to_string(), "c".to_string()].as_slice(), 1)
        .add(vec!["a".to_string(), "b".to_string(), "d".to_string()].as_slice(), 2)
        .add(vec!["e".to_string(), "f".to_string()].as_slice(), 3);

    println!("\tCompiling static tree...");
    let stree = builder.compile();

    println!("Compiled Tree\n");

    println!("Implicit Nodes: {:?}, {:?}", stree.raw().get::<TreeBranch>(0), stree.raw().get::<TreeBranch>(8));


    println!("Reading Values from static tree:");
    println!("\tabc -> {}", stree.find::<i32, String>(vec!["a".to_string(), "b".to_string(), "c".to_string()].as_slice()).unwrap());
    println!("\tabd -> {}", stree.find::<i32, String>(vec!["a".to_string(), "b".to_string(), "d".to_string()].as_slice()).unwrap_or(&-1));
    println!("\tef -> {}", stree.find::<i32, String>(vec!["e".to_string(), "f".to_string()].as_slice()).unwrap_or(&-1));



}
