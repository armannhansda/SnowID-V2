use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::generator::SnowIdGenerator;


static GENERATOR: Lazy<Mutex<SnowIdGenerator>> = Lazy::new(||Mutex::new(SnowIdGenerator::new(1)));

pub fn generate_id() -> u64 {
  let mut generator = GENERATOR.lock().unwrap();
  generator.generate()
}