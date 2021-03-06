extern crate rand;
extern crate plotlib;

use rand::prelude::*;

use plotlib::scatter::Scatter;
use plotlib::scatter;
use plotlib::style::{Marker, Point};
use plotlib::view::View;
use plotlib::page::Page;

mod hashtable;
use self::hashtable::HashTable as HashTable;
use self::hashtable::OAHashTable as OAHashTable;
use self::hashtable::SCHashTable as SCHashTable;
use self::hashtable::HashType as HashType;

fn generate_result(mut hash_table: impl HashTable, rng: &mut ThreadRng) -> Vec<(f64, f64)> {
    let mut results: Vec<(f64, f64)> = Vec::new();
    while hash_table.load_factor() != 1.0 {
        let key = rng.gen_range(0, hash_table.capacity() * 3);
        if hash_table.load_factor() != 1.0 {
            hash_table.put(key, key.to_string());
        }
        if !results.iter().any(|t| t.0 == hash_table.load_factor()) {
            results.push((hash_table.load_factor(), hash_table.collisions() as f64));
        }     
    }
    results
}

fn make_graph(results_size_1: Vec<(f64, f64)>, results_size_2: Vec<(f64, f64)>, results_size_3: Vec<(f64, f64)>, y_range: f64, file_name: String) {
    let scatter_size_1 = Scatter::from_vec(&results_size_1)
            .style(scatter::Style::new()
            .marker(Marker::Circle)
            .colour("#DD3355"));


    let scatter_size_2 = Scatter::from_vec(&results_size_2)
            .style(scatter::Style::new()
            .marker(Marker::Cross)
            .colour("#35C788"));

    let scatter_size_3 = Scatter::from_vec(&results_size_3)
            .style(scatter::Style::new()
            .marker(Marker::Square)
            .colour("#35C788"));

    let v = View::new()
        .add(&scatter_size_1)
        .add(&scatter_size_2)
        .add(&scatter_size_3)
        .x_range(0., 1.)
        .y_range(0., y_range)
        .x_label("Load Factor")
        .y_label("Number of Collisions");

    Page::single(&v).save(file_name);
}


fn main() {
    const EXPERIMENT_1_SIZE: u32 = 10;
    const EXPERIMENT_2_SIZE: u32 = 50;
    const EXPERIMENT_3_SIZE: u32 = 1000;
    let mut rng: ThreadRng = thread_rng();

    // Run Key % Table experiments with size 1
    let oa_keymod_1: OAHashTable = HashTable::new(EXPERIMENT_1_SIZE, HashType::KeyModTableSize);
    let sc_keymod_1: SCHashTable = HashTable::new(EXPERIMENT_1_SIZE, HashType::KeyModTableSize);
    let result_oa_keymod_1 = generate_result(oa_keymod_1, &mut rng);
    let result_sc_keymod_1 = generate_result(sc_keymod_1, &mut rng);
    
    // Run Midsquare experiments with size 1
    let oa_midsquare_1: OAHashTable = HashTable::new(EXPERIMENT_1_SIZE, HashType::MidSquare);
    let sc_midsquare_1: SCHashTable = HashTable::new(EXPERIMENT_1_SIZE, HashType::MidSquare);
    let result_oa_midsquare_1 = generate_result(oa_midsquare_1, &mut rng);
    let result_sc_midsquare_1 = generate_result(sc_midsquare_1, &mut rng);


    let oa_keymod_2: OAHashTable = HashTable::new(EXPERIMENT_2_SIZE, HashType::KeyModTableSize);
    let sc_keymod_2: SCHashTable = HashTable::new(EXPERIMENT_2_SIZE, HashType::KeyModTableSize);
    let result_oa_keymod_2 = generate_result(oa_keymod_2, &mut rng);
    let result_sc_keymod_2 = generate_result(sc_keymod_2, &mut rng);

    let oa_midsquare_2: OAHashTable = HashTable::new(EXPERIMENT_2_SIZE, HashType::MidSquare);
    let sc_midsquare_2: SCHashTable = HashTable::new(EXPERIMENT_2_SIZE, HashType::MidSquare);
    let result_oa_midsquare_2 = generate_result(oa_midsquare_2, &mut rng);
    let result_sc_midsquare_2 = generate_result(sc_midsquare_2, &mut rng);



    let oa_keymod_3: OAHashTable = HashTable::new(EXPERIMENT_3_SIZE, HashType::KeyModTableSize);
    let sc_keymod_3: SCHashTable = HashTable::new(EXPERIMENT_3_SIZE, HashType::KeyModTableSize);
    let result_oa_keymod_3 = generate_result(oa_keymod_3, &mut rng);
    let result_sc_keymod_3 = generate_result(sc_keymod_3, &mut rng);

    let oa_midsquare_3: OAHashTable = HashTable::new(EXPERIMENT_3_SIZE, HashType::MidSquare);
    let sc_midsquare_3: SCHashTable = HashTable::new(EXPERIMENT_3_SIZE, HashType::MidSquare);
    let result_oa_midsquare_3 = generate_result(oa_midsquare_3, &mut rng);
    let result_sc_midsquare_3 = generate_result(sc_midsquare_3, &mut rng);

    make_graph(result_oa_keymod_1, result_oa_keymod_2, result_oa_keymod_3, EXPERIMENT_3_SIZE as f64, String::from("OpenAddress_Keymod.svg"));
    make_graph(result_oa_midsquare_1, result_oa_midsquare_2, result_oa_midsquare_3, EXPERIMENT_3_SIZE as f64, String::from("OpenAddress_MidSquare.svg"));
    make_graph(result_sc_keymod_1, result_sc_keymod_2, result_sc_keymod_3, EXPERIMENT_3_SIZE as f64, String::from("SeperateChaining_Keymod.svg"));
    make_graph(result_sc_midsquare_1, result_sc_midsquare_2, result_sc_midsquare_3, EXPERIMENT_3_SIZE as f64, String::from("SeperateChaining_MidSquare.svg"));
}
