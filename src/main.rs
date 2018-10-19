extern crate rand;

use std::fs::File;
use std::io::prelude::*;

use rand::prelude::*;

mod hashtable;
use self::hashtable::HashTable as HashTable;
use self::hashtable::OAHashTable as OAHashTable;
use self::hashtable::SCHashTable as SCHashTable;
use self::hashtable::HashType as HashType;


fn generate_results(mut oa_table: OAHashTable, mut sc_table: SCHashTable, file_prefix: String, rng: &mut ThreadRng) -> std::io::Result<()> {
    let mut file = File::create(file_prefix + ".csv")?;

    while oa_table.load_factor() != 1.0 || sc_table.load_factor() != 1.0 {
        let key = rng.gen_range(0, oa_table.capacity() * 3);
        if oa_table.load_factor() != 1.0 {
            oa_table.put(key, key.to_string());
        }
        if sc_table.load_factor() != 1.0 {
            sc_table.put(key, key.to_string());
        }
        writeln!(file, "{},{},{},{}", oa_table.load_factor(), oa_table.collisions(), sc_table.load_factor(), sc_table.collisions())?;
    }
    Ok(())
}

fn main() {
    let mut rng: ThreadRng = thread_rng();

    let mut table_size: u32 = 10;
    let oa_keymod_1: OAHashTable = HashTable::new(table_size, HashType::KeyModTableSize);
    let sc_keymod_1: SCHashTable = HashTable::new(table_size, HashType::KeyModTableSize);
    generate_results(oa_keymod_1, sc_keymod_1, String::from("keymod_size_10"), &mut rng).expect("Error saving file.");

    let oa_midsquare_1: OAHashTable = HashTable::new(table_size, HashType::MidSquare);
    let sc_midquare_1: SCHashTable = HashTable::new(table_size, HashType::MidSquare);
    generate_results(oa_midsquare_1, sc_midquare_1, String::from("midsquare_10"), &mut rng).expect("Error saving file.");


    table_size = 25;

    let oa_keymod_2: OAHashTable = HashTable::new(table_size, HashType::KeyModTableSize);
    let sc_keymod_2: SCHashTable = HashTable::new(table_size, HashType::KeyModTableSize);
    generate_results(oa_keymod_2, sc_keymod_2, String::from("keymod_size_25"), &mut rng).expect("Error saving file.");

    let oa_midsquare_2: OAHashTable = HashTable::new(table_size, HashType::MidSquare);
    let sc_midquare_2: SCHashTable = HashTable::new(table_size, HashType::MidSquare);
    generate_results(oa_midsquare_2, sc_midquare_2, String::from("midsquare_25"), &mut rng).expect("Error saving file.");


    table_size = 40;

    let oa_keymod_3: OAHashTable = HashTable::new(table_size, HashType::KeyModTableSize);
    let sc_keymod_3: SCHashTable = HashTable::new(table_size, HashType::KeyModTableSize);
    generate_results(oa_keymod_3, sc_keymod_3, String::from("keymod_size_40"), &mut rng).expect("Error saving file.");

    let oa_midsquare_3: OAHashTable = HashTable::new(table_size, HashType::MidSquare);
    let sc_midquare_3: SCHashTable = HashTable::new(table_size, HashType::MidSquare);
    generate_results(oa_midsquare_3, sc_midquare_3, String::from("midsquare_40"), &mut rng).expect("Error saving file.");
}
