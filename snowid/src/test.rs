#[cfg(test)]
mod tests {
  use crate::generator::SnowIdGenerator;
  use crate::decode::decode;

  #[test]
  fn test_id_increasing() {
    let mut generate = SnowIdGenerator::new(1);

    let id1 = generate.generate();
    let id2 = generate.generate();

    assert!(id2 > id1);
  }

  #[test]
  fn test_decode() {
    let mut generate = SnowIdGenerator::new(1);
    let id = generate.generate();

    let (ts, machine_id, seq) = decode(id);
    

    assert!(ts>0);
    assert_eq!(machine, 1);
    assert!(seq>=0);
  }
}