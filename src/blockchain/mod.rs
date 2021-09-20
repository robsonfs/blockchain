mod block;

use block::Block;
use digest::Digest;
use sha2::Sha256;

fn create_hash(msg: &str) -> String {
    let mut hasher = Sha256::default();
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

        while !check_proof {
            let solution = {
                let s = (new_proof.pow(2) - previous_proof.pow(2)).to_string();
                create_hash(&s)
            };

            if &solution[..4] == "0000" {
                check_proof = true;
            } else {
                new_proof += 1;
            }
        }
        new_proof as usize
    }

    pub fn is_chain_valid(&self) -> bool {
        match self.chain.len() {
            0 => true,
            1 => {
                let block = self.chain[0].clone();
                (block.proof == 1) && (&block.previous_hash == "0")
            }
            _ => {
                let mut result = true;
                for block in self.chain[1..].iter() {
                    let previous_block = self.chain[block.index - 1].clone();
                    let previous_hash = previous_block.get_hash();
                    if previous_hash != block.previous_hash {
                        result = false;
                        break;
                    }
                    let solution = {
                        let s = (block.proof.pow(2) - previous_block.proof.pow(2)).to_string();
                        create_hash(&s)
                    };
                    if &solution[..4] != "0000" {
                        result = false;
                        break;
                    }
                }
                result
            }
        }
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
        assert_eq!(expected, create_hash(msg));
    }

    #[test]
    fn test_proof_of_work() {
        let blockchain = Blockchain::new();
        let proof = blockchain.proof_of_work(42);
        assert_eq!(1822, proof);
    }

    #[test]
    fn test_is_chain_valid_empty_chain() {
        let blockchain = Blockchain::default();
        assert!(
            blockchain.is_chain_valid(),
            "An empty chain must always be valid"
        );
    }

    #[test]
    fn test_is_chain_valid_unit_chain_valid_cases() {
        let mut valid_blockchain = Blockchain::default();
        let _ = valid_blockchain.create_block(1, String::from("0"));
        assert!(
            valid_blockchain.is_chain_valid(),
            "An unity chain is valid if proof=1 and previous_hash=String::from(\"0\")"
        );
    }

    #[test]
    fn test_is_chain_valid_unit_chain_invalid_cases() {
        let mut bchain_1 = Blockchain::default();
        let mut bchain_2 = Blockchain::default();
        let mut bchain_3 = Blockchain::default();

        bchain_1.create_block(42, String::from("0"));
        bchain_2.create_block(1, String::from("42"));
        bchain_3.create_block(42, String::from("42"));

        let bchains = vec![bchain_1, bchain_2, bchain_3];

        for bchain in bchains.iter() {
            assert!(!bchain.is_chain_valid());
        }
    }
}
