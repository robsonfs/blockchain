mod block;

use block::Block;

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
}
