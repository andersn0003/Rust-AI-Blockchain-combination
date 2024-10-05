use rand::seq::SliceRandom;
use rand::Rng;
// use std::error::Error;

pub fn split_data_randomly(
    x: Vec<f64>, 
    y: Vec<f64>, 
    min_chunk_size: usize, 
    max_chunk_size: usize
) -> Vec<(Vec<f64>, Vec<f64>)> {
    let mut rng = rand::thread_rng();
    
    // Combine x and y into a tuple vector and shuffle
    let mut data: Vec<(f64, f64)> = x.into_iter().zip(y.into_iter()).collect();
    data.shuffle(&mut rng);
    
    let mut chunks = Vec::new();
    let mut remaining_data = data;

    while !remaining_data.is_empty() {
        // Determine the size of the next chunk, bounded by the remaining data size
        let chunk_size = rng.gen_range(min_chunk_size..=max_chunk_size)
            .min(remaining_data.len());

        // Split off a chunk of the data
        let chunk: Vec<(f64, f64)> = remaining_data.drain(0..chunk_size).collect();

        // Separate the chunk into x and y again
        let (chunk_x, chunk_y): (Vec<f64>, Vec<f64>) = chunk.into_iter().unzip();
        chunks.push((chunk_x, chunk_y));
    }

    chunks
}