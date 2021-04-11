extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::fmt;
use hex;
use std::i64;

fn main() {
    let mut filter = new_bloom_filter(128);
    println!("{}", &filter);

    filter.insert("hello");

    println!("{}", &filter);
}


struct BloomFilter {
    field: Vec<bool>,
    n: usize // length of field
}

impl BloomFilter {
    fn insert(&mut self, s: &str) -> () {
        let idx = self.hash(s);
        self.field[idx] = true;
    }

    fn hash(&self, s: &str) -> usize {
        let mut hasher = Md5::new();
        hasher.input(s.as_bytes());
        let mut buff = [0; 16];
        hasher.result(&mut buff);
        let num = u128::from_be_bytes(buff);
        num.checked_rem(self.n as u128).unwrap() as usize
    }
}

impl fmt::Display for BloomFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // use debug writer for vec
        write!(f, "{:?}", self.field)
    }
}

fn new_bloom_filter(field_size: usize) -> BloomFilter {
    BloomFilter{field: vec![false; field_size], n: field_size}
}

