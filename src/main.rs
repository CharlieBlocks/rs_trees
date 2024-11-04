mod tree_map;
mod dynamic_array;
mod static_tree;
mod counted_tree_map;
mod static_tree_planner;
use tree_map::*;
use static_tree_planner::StaticTreePlanner;

use std::time::Instant;

static BENCHMARK_IT: usize = 1_000_000;


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

    println!("Reading Values from static tree:");
    println!("\tabc -> {}", stree.find::<i32, String>(vec!["a".to_string(), "b".to_string(), "c".to_string()].as_slice()).unwrap());
    println!("\tabd -> {}", stree.find::<i32, String>(vec!["a".to_string(), "b".to_string(), "d".to_string()].as_slice()).unwrap_or(&-1));
    println!("\tef -> {}", stree.find::<i32, String>(vec!["e".to_string(), "f".to_string()].as_slice()).unwrap_or(&-1));



    println!("Benchmarking...");
    println!("Basic map ({} iterations)", BENCHMARK_IT);
    let i1 = Instant::now();
    for _ in 0..BENCHMARK_IT {
        let _ = map.find(vec!["a".to_string(), "b".to_string(), "c".to_string()].as_slice()).unwrap();
    }
    let t1 = i1.elapsed();

    println!("Static map ({} iterations)", BENCHMARK_IT);
    let i2 = Instant::now();
    for _ in 0..BENCHMARK_IT {
        let _ = stree.find::<i32, String>(vec!["a".to_string(), "b".to_string(), "c".to_string()].as_slice()).unwrap();
    }
    let t2 = i2.elapsed();


    println!("Results:");
    println!("\tBasic Map - Total: {}ms, Per: {}ns", t1.as_millis(), t1.as_nanos() / BENCHMARK_IT as u128);
    println!("\tStatic Map - Total: {}ms, Per: {}ns", t2.as_millis(), t2.as_nanos() / BENCHMARK_IT as u128);
    println!("\t{:.4}x", (t1.as_nanos() as f64 / BENCHMARK_IT as f64) / (t2.as_nanos() as f64 / BENCHMARK_IT as f64));

}
