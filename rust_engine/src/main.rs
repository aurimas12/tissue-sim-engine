use std::fs;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;
use std::time::Instant;
fn main() {
    let input_path = "/Users/aurimas/Desktop/project/tissue-sim-engine/voxel_data.json";
    let output_path = "/Users/aurimas/Desktop/project/tissue-sim-engine/result.json";

    // Bandome nuskaityti JSON failą
    // let file = File::open(input_path).expect("❌ Nepavyko atidaryti voxel_data.json");
  
    println!("🚀 Difuzijos benchmarkas Rust'e startuoja...");
    let now = Instant::now();

    // Nustatymai
    let steps = 1000; // iteracijų kiekis
    let diffusion_rate = 0.1_f32;

    // Nuskaitome voxel_data.json (2D masyvas float)
    let file = File::open(input_path).expect("❌ Nepavyko atidaryti voxel_data.json");
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<f32>> = serde_json::from_reader(reader).expect("❌ JSON formatas netinkamas");

    let rows = grid.len();
    let cols = grid[0].len();

    for _ in 0..steps {
        let mut next_grid = grid.clone();
        for i in 1..rows - 1 {
            for j in 1..cols - 1 {
                let c = grid[i][j];
                let sum_neighbors = grid[i - 1][j] + grid[i + 1][j] + grid[i][j - 1] + grid[i][j + 1];
                next_grid[i][j] = c + diffusion_rate * (sum_neighbors - 4.0 * c);
            }
        }
        grid = next_grid;
    }

    // Trukmė
    let elapsed = now.elapsed();
    println!("✅ Difuzija baigta per: {:.2?} ({} iteracijų)", elapsed, steps);

    // Rezultato išsaugojimas
    let output = File::create("result.json").expect("❌ Nepavyko sukurti result.json");
    serde_json::to_writer_pretty(output, &grid).expect("❌ Nepavyko įrašyti JSON");
    println!("📄 Rezultatas įrašytas į result.json");
}

// Reikia pridėti Cargo.toml:
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
