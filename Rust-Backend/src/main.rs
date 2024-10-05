use std::error::Error;
pub mod traning_example;
pub mod data_preprocess;
// use std::env;
pub mod sol_connect;
// use std::iter::repeat_with;
#[tokio::main]
async  fn main() -> Result<(), Box<dyn Error>>{
    // let current_dir = env::current_dir()?;
    // println!("Current working directory: {:?}", current_dir);
    let (mut x_value, mut y_value): (Vec<f64>, Vec<f64>) = (Vec::new(), Vec::new());
    match traning_example::read_csv("src/study.csv") {
        Ok((x, y)) => {
            // println!("{:?}", (x, y));
            x_value = x;
            y_value = y
        },
        Err(e) => {
            eprintln!("Error reading CSV: {}", e);
            
        }
    }
    let min_chunk_size = 2;
    let max_chunk_size = 10;

    let chunks = data_preprocess::split_data_randomly(x_value, y_value, min_chunk_size, max_chunk_size);

    for (i, (chunk_x, chunk_y)) in chunks.iter().enumerate() {
        println!("Chunk {}: x = {:?}, y = {:?}", i + 1, chunk_x, chunk_y);
        let mut model = traning_example::LinearRegression::new();

        // Ensure chunks are of sufficient size
        if chunk_x.len() < 2 {
            eprintln!("Chunk {} has too few data points for linear regression", i + 1);
            continue;
        }

        // Train the model
        model.fit(chunk_x, chunk_y);
        // Print the model parameters
        println!("Chunk {}: Slope = {:.4}, Intercept = {:.4}", i + 1, model.slope, model.intercept);

        sol_connect::save_data_to_solana(model.slope, model.intercept);
    }
    Ok(())
        
}