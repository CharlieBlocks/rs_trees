mod tree_map;
mod dynamic_array;
mod static_tree;
mod counted_tree_map;
mod static_tree_planner;
use tree_map::*;
use static_tree_planner::StaticTreePlanner;

use std::time::Instant;
use std::hash::Hash;

static BENCHMARK_IT: usize = 1_000_000;

fn get_hash(s: &str) -> i32 {
    let mut acc = 1;
    for c in s.chars() {
        acc = acc * c as i32;
    }
    return acc;
}

fn hash_vec(arr: &[&str]) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::with_capacity(arr.len());
    for v in arr {
        out.push(get_hash(v));
    };
    return out;
}

fn main() {
    // Basic Tree
    let mut map: TreeMap<i32, &str> = TreeMap::new();
    map.insert(vec!["a", "b", "c"].as_slice(), 1);
    map.insert(vec!["a", "b", "d"].as_slice(), 2);
    map.insert(vec!["e", "f"].as_slice(), 3);


    // Static Tree
    let mut builder: StaticTreePlanner<i32, i32> = StaticTreePlanner::new();
    builder = builder.add(hash_vec(vec!["a", "b", "c"].as_slice()).as_slice(), 1)
        .add(hash_vec(vec!["a", "b", "d"].as_slice()).as_slice(), 2)
        .add(hash_vec(vec!["e", "f"].as_slice()).as_slice(), 3);
    let stree = builder.compile();


    let lookup_index = vec!["a", "b", "c"];
    let hashed_index = hash_vec(lookup_index.as_slice());

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
