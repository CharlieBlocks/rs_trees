mod tree_map;
mod dynamic_array;
mod static_tree;
mod counted_tree_map;
mod static_tree_planner;
use tree_map::TreeMap;
use static_tree_planner::StaticTreePlanner;

use std::time::Instant;

static BENCHMARK_IT: usize = 1_000_000;

fn main() {
    // Basic Tree
    let mut map: TreeMap<i32, &str> = TreeMap::new();
    map.insert(vec!["a", "b", "c"].as_slice(), 1);
    map.insert(vec!["a", "b", "d", "e", "f", "g", "h"].as_slice(), 2);
    map.insert(vec!["e", "f"].as_slice(), 3);


    // Static Tree
    let mut builder: StaticTreePlanner<i32, &str> = StaticTreePlanner::new();
    builder = builder
        .add(vec!["a", "b", "c"].as_slice(), 1)
        .add(vec!["a", "b", "d", "e", "f", "g", "h"].as_slice(), 2)
        .add(vec!["e", "f"].as_slice(), 3);
    let stree = builder.compile();


    // let lookup_index = vec!["a", "b", "d", "e", "f", "g", "h"];
    // let lookup_index = vec!["a", "b", "c"];
    let lookup_index = vec!["e", "f"];


    println!("Benchmarking...");
    println!("Basic map ({} iterations)", BENCHMARK_IT);
    let i1 = Instant::now();
    for _ in 0..BENCHMARK_IT {
        let _ = map.find(lookup_index.as_slice()).unwrap();
    }
    let t1 = i1.elapsed();

    println!("Static map ({} iterations)", BENCHMARK_IT);
    let i2 = Instant::now();
    for _ in 0..BENCHMARK_IT {
        let _ = stree.find::<i32, &str>(lookup_index.as_slice()).unwrap();
    }
    let t2 = i2.elapsed();


    println!("Results:");
    println!("\tBasic Map - Total: {}ms, Per: {}ns", t1.as_millis(), t1.as_nanos() / BENCHMARK_IT as u128);
    println!("\tStatic Map - Total: {}ms, Per: {}ns", t2.as_millis(), t2.as_nanos() / BENCHMARK_IT as u128);
    println!("\t{:.4}x", (t1.as_nanos() as f64 / BENCHMARK_IT as f64) / (t2.as_nanos() as f64 / BENCHMARK_IT as f64));

}
