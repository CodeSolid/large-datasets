use std::time::{Instant};
use std::fs::File;
use polars::prelude::*;

fn read_lazy() {
    println!("Reading using lazy read");
    let now = Instant::now();
    let df = LazyCsvReader::new("../data/train.csv")
        .has_header(true)
        .finish();    
    println!("{:?}", df.expect("Fetch failed").fetch(5));
    println!("{} milliseconds elapsed.", now.elapsed().as_millis());
}

fn read_normal() {
    let now = Instant::now();
    println!("Opening file for non-lazy read");
    let file = File::open("../data/train.csv").expect("could not open file");
    println!("Reading file using CsvReader");
    let df = CsvReader::new(file)
    .infer_schema(None)
    .has_header(true)
    .finish();
    println!("Dataframe read");
    println!("{} milliseconds elapsed.", now.elapsed().as_millis());
    let iterator = (0..5).into_iter(); // .map(|idx| Some(idx));
    let rows = df.expect("Rows not available").take_iter(iterator);   
    println!("{:?}", rows); 
}

fn read_normal_faster_maybe() {
    let now = Instant::now();
    println!("Dataframe read");
    let df = CsvReader::from_path("../data/train.csv").expect("Read failed")    
    .with_chunk_size(2000)
    .has_header(true)
    .finish();    
    println!("{} milliseconds elapsed.", now.elapsed().as_millis());
    let iterator = (0..5).into_iter(); // .map(|idx| Some(idx));
    let rows = df.expect("Rows not available").take_iter(iterator);   
    println!("{:?}", rows); 
}



fn main() {
    //read_lazy();
    //read_normal();
    read_normal_faster_maybe();
}
