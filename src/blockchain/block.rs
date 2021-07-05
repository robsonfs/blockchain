use super::create_hash;
use chrono::{SecondsFormat, Utc};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: usize,
    pub timestamp: String,
    pub proof: i32,
    pub previous_hash: String,
}

impl Block {
    pub fn new(index: usize, proof: Option<i32>, previous_hash: Option<String>) -> Self {
        let proof = proof.unwrap_or(1);
        let previous_hash = previous_hash.unwrap_or_else(|| String::from("0"));

        Self {
            index,
            timestamp: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, false),
            proof,
            previous_hash,
        }
    }

    pub fn get_hash(&self) -> String {
        create_hash(&self.to_string())
    }
}

impl fmt::Display for Block {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = format!(
            "Index: {}, Timestamp: {}, Proof: {}, PreviousHash: {}",
            &self.index, &self.timestamp, &self.proof, &self.previous_hash
        );
        fmt.write_str(&s)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_genesis_block() {
        let genesis = Block::new(1, None, None);

        assert_eq!(genesis.index, 1);
    }

    #[test]
    fn test_proof_genesis_block() {
        let genesis = Block::new(1, None, None);

        assert_eq!(genesis.proof, 1);
    }

    #[test]
    fn test_previous_hash_genesis_block() {
        let genesis = Block::new(1, None, None);

        assert_eq!(genesis.previous_hash, String::from("0"));
    }

    #[test]
    fn test_get_hash() {
        let mut genesis = Block::new(1, None, None);
        genesis.timestamp = String::from("Fixedtimestamp");

        assert_eq!(
            genesis.get_hash(),
            String::from("c8937d0f547422f8d469e6b75c754ed15b51b2b8525f12cd7d1276ce5a1ab899")
        );
    }
}
