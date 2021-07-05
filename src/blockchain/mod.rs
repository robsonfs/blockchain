mod block;

use block::Block;
use digest::Digest;
use sha2::Sha256;

fn create_hash(msg: &str, mut hasher: Sha256) -> String {
    hasher.update(msg);
    format!("{:x}", hasher.finalize())
}

type ProofType = isize;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self { chain: Vec::new() }
    }

    pub fn create_block(&mut self, proof: i32, previous_hash: String) -> usize {
        let index = self.chain.len();
        let block = Block::new(index, Some(proof), Some(previous_hash));

        self.chain.insert(index, block);

        index
    }

    pub fn get_previous_block(&self) -> Option<Block> {
        let size = self.chain.len();
        if size == 0 {
            None
        } else {
            Some(self.chain[size - 1].clone())
        }
    }

    pub fn proof_of_work(&self, previous_proof: ProofType) -> usize {
        let mut new_proof: ProofType = 1;
        let mut check_proof = false;
        let hasher = Sha256::new();

        while !check_proof {
            let solution = {
                let s = (new_proof.pow(2) - previous_proof.pow(2)).to_string();
                create_hash(&s, hasher.clone())
            };

            if &solution[..4] == "0000" {
                check_proof = true;
            } else {
                new_proof += 1;
            }
        }
        new_proof as usize
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let new_blockchain = Blockchain::new();
        assert!(
            new_blockchain.chain.is_empty(),
            "A blockchain should start with empty chain"
        );
    }

    #[test]
    fn test_default() {
        let new_blockchain = Blockchain::default();
        assert!(
            new_blockchain.chain.is_empty(),
            "A default blockchain should start with empty chain"
        );
    }

    #[test]
    fn test_create_block() {
        let mut new_blockchain = Blockchain::new();
        let index = new_blockchain.create_block(1, String::from("0"));

        assert_eq!(0, index);
    }

    #[test]
    fn test_get_previous_block() {
        let mut new_blockchain = Blockchain::new();
        let _ = new_blockchain.create_block(1, String::from("0"));
        let _ = new_blockchain.create_block(10, String::from("previous_hash"));

        let previous_block = new_blockchain.get_previous_block().unwrap();

        assert_eq!(1, previous_block.index);
        assert_eq!(10, previous_block.proof);
        assert_eq!(String::from("previous_hash"), previous_block.previous_hash);
    }

    #[test]
    fn test_create_hash() {
        let msg = "Hello World!";
        let expected = "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069";
        assert_eq!(expected, create_hash(msg, Sha256::new()));
    }

    #[test]
    fn test_proof_of_work() {
        let blockchain = Blockchain::new();
        let proof = blockchain.proof_of_work(42);
        assert_eq!(1822, proof);
    }
}
