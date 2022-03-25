use rand::prelude::*;
use rand::distributions::Standard;
use optimal_application::*;
// use std::time::{Instant, Duration};



fn main() {
    const M: usize = 10;

    let mkt = rand_scm(M);
    println!("mkt = {:?}", mkt);

    println!("\nApp order, list version");
    let (xs, vs) = application_order_vec(&mkt);
    println!("xs = {:?}", xs); println!("vs = {:?}", vs);

    println!("\nApp order, heap version");
    let (xs, vs) = application_order_heap(&mkt);
    println!("xs = {:?}", xs); println!("vs = {:?}", vs);

    println!("\n");

    let mkt = rand_vcm(M);
    println!("mkt = {:?}", mkt);

    println!("\nExact DP");
    let (xs, v) = optimal_portfolio_dynamic(&mkt);
    println!("xs = {:?}", xs); println!("v = {}", v);

    println!("\nFPTAS, ε = 0.5");
    let (xs, v) = optimal_portfolio_fptas(&mkt, 0.5);
    println!("xs = {:?}", xs); println!("ṽ = {}", v);

    println!("\nFPTAS, ε = 0.05");
    let (xs, v) = optimal_portfolio_fptas(&mkt, 0.05);
    println!("xs = {:?}", xs); println!("ṽ = {}", v);
}


fn rand_exp() -> f64 {
    let mut rng = thread_rng();
    let r: f64 = Standard.sample(&mut rng);
    return -r.ln();
}

fn rand_uniform() -> f64 {
    let mut rng = thread_rng();
    return Standard.sample(&mut rng);
}


fn rand_vcm(m: usize) -> VariedCostsMarket {
    let mut t: Vec<usize> = (0..m).map(|_| (10.0 * rand_exp() + 1.0) as usize).collect();
    t.sort();
    let f: Vec<f64> = t.iter().map(|&t| 1.0 / (t as f64 + 10.0 * rand_uniform())).collect();
    let g: Vec<usize> = (0..m).map(|_| (5.0 + 6.0 * rand_uniform()) as usize).collect();
    let h_big: usize = g.iter().fold(0, |acc, &gi| acc + gi) / 2;
    
    return VariedCostsMarket::new(&f, &t, &g, h_big);
}


fn rand_scm(m: usize) -> SameCostsMarket {
    let mut t: Vec<usize> = (0..m).map(|_| (10.0 * rand_exp() + 1.0) as usize).collect();
    t.sort();
    let f: Vec<f64> = t.iter().map(|&t| 1.0 / (t as f64 + 10.0 * rand_uniform())).collect();
    let h: usize = m / 2;
    
    return SameCostsMarket::new(&f, &t, h);
}
