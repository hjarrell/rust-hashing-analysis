use std::collections::LinkedList;

fn num_of_bits(input: &u32) -> u32 {
    let mut copy: u32 = input.clone();
    let mut bits: u32 = 0;

    while copy != 0 {
        copy = copy >> 1;
        bits += 1;
    }

    bits
}

pub trait HashTable {
    fn get(&mut self, key: u32) -> Option<String>;
    fn load_factor(&self) -> f64;
    fn collisions(&self) -> u32;
    fn size(&self) -> u32;
    fn capacity(&self) -> u32;
    fn new(capacity: u32, hash_type: HashType) -> Self;
    fn put(&mut self, key: u32, value: String);
    // fn delete(&mut self, key: u32) -> String;
    fn hash(&self, key: u32) -> u32;
    fn is_full(&self) -> bool;
}

pub struct OAHashTable {
    capacity: u32,
    size: u32,
    collisions: u32,
    hash_type: HashType,
    entries: Vec<HashEntry>,
}

#[derive(Clone)]
pub enum HashEntry {
    Filled((u32, String)),
    Null,
    Deleted,
}

impl HashEntry {
    fn is_filled(&self) -> bool {
        match *self {
            HashEntry::Filled(_) => true,
            _ => false,
        }
    }

    fn is_null(&self) -> bool {
        match *self {
            HashEntry::Null => true,
            _ => false,
        }
    }

    fn is_deleted(&self) -> bool {
        match *self {
            HashEntry::Deleted => true,
            _ => false,
        }
    }

    fn unwrap(self) -> (u32, String) {
        match self {
            HashEntry::Filled(val) => val,
            _ => panic!("called HashEntry unwrap() on non Filled entry!")
        }
    }
}

pub enum HashType {
    KeyModTableSize,
    MidSquare,
}

impl HashTable for OAHashTable {
    /// Hash table with Open Adressing to handle collisions 

    fn new(capacity: u32, hash_type: HashType) -> Self {
        OAHashTable { capacity: capacity, size: 0, collisions: 0, entries: vec![HashEntry::Null; capacity as usize], hash_type: hash_type }
    }

    fn collisions(&self) -> u32 {
        self.collisions
    }

    fn load_factor(&self) -> f64 {
        (self.size as f64) / (self.capacity as f64)
    }

    fn size(&self) -> u32 {
        self.size
    }

    fn capacity(&self) -> u32 {
        self.capacity
    }

    fn is_full(&self) -> bool {
        self.size == (self.capacity-1)
    }

    fn hash(&self, key: u32) -> u32 {
        match &self.hash_type {
            HashType::MidSquare => {
                let mut hash: u32 = key*key;

                let max: f64 = ((self.capacity*self.capacity*9) as f64).log2().ceil();
                let table: f64 = ((self.capacity) as f64).log2().ceil();
                let diff: u32 = (max - table) as u32;
                hash = hash / ((diff/2).pow(2));
                hash = hash % self.capacity;
                return hash;
            },
            HashType::KeyModTableSize => {
                key % self.capacity
            },
        }
    }
    
    fn get(&mut self, key: u32) -> Option<String> {
        let mut hash: u32 = self.hash(key);

        while !self.entries[hash as usize].is_null() {
            let entry = self.entries[hash as usize].clone();
            if entry.is_filled() && entry.unwrap().0 == key {
                let entry = self.entries[hash as usize].clone();
                return Some(entry.unwrap().1.clone())
            } else {
                hash += 1;
                hash %= self.capacity;
            }
        }
        
        return None;
    }

    fn put(&mut self, key: u32, value: String) {
        let mut hash: u32 = self.hash(key);

        loop {
            match &self.entries[hash as usize] {
                HashEntry::Filled(v) => {
                    if v.0 == key {
                        break;
                    } else {
                        hash += 1;
                        hash %= self.capacity;
                        self.collisions += 1;
                    }
                },
                _ => {
                    self.size += 1;
                    break;
                },
            }
        }
        self.entries[hash as usize] = HashEntry::Filled((key, value));
    }

    // fn delete(&mut self, key: u32) -> String {

    // }
}

pub struct SCHashTable {
    capacity: u32,
    size: u32,
    collisions: u32,
    hash_type: HashType,
    entries: Vec<LinkedList<(u32, String)>>,
}

impl HashTable for SCHashTable {
    /// Hash Table with Seperate Chaining for collisions

    fn new(capacity: u32, hash_type: HashType) -> Self {
        SCHashTable { capacity: capacity, size: 0, collisions: 0, entries: vec![LinkedList::new(); capacity as usize], hash_type: hash_type }
    }

    fn collisions(&self) -> u32 {
        self.collisions
    }

    fn load_factor(&self) -> f64 {
        (self.size as f64) / (self.capacity as f64)
    }
    
    fn is_full(&self) -> bool {
        self.size == (self.capacity)
    }

    fn hash(&self, key: u32) -> u32 {
        match &self.hash_type {
            HashType::MidSquare => {
                let mut hash: u32 = key*key;

                let max: f64 = ((self.capacity*self.capacity*9) as f64).log2().ceil();
                let table: f64 = ((self.capacity) as f64).log2().ceil();
                let diff: u32 = (max - table) as u32;
                hash = hash / ((diff/2).pow(2));
                hash = hash % self.capacity;
                return hash;
            },
            HashType::KeyModTableSize => {
                key % self.capacity
            },
        }
    }

    fn size(&self) -> u32 {
        self.size
    }

    fn capacity(&self) -> u32 {
        self.capacity
    }

    fn put(&mut self, key: u32, value: String) {
        let hash: u32 = self.hash(key);

        let entry_list = &mut self.entries[hash as usize];
        if !entry_list.is_empty() {
            self.collisions += entry_list.len() as u32;
        }
        entry_list.push_back((key, value));

        self.size += 1;
    }

    fn get(&mut self, key: u32) -> Option<String> {
        let hash: u32 = self.hash(key);

        let entry_list = &self.entries[hash as usize];

        if entry_list.is_empty() {
            None
        } else {
            for node in entry_list.clone().iter() {
                if node.0 == key {
                    return Some(node.1.clone());
                }
            }
            None
        }
    }

}
