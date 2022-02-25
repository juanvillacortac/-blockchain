mod blockchain;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::blockchain::*;
    #[test]
    fn test_genesis_block() {
        //create blockchain
        let p2p_bc: Vec<Block> = vec![Block::genesis()];
        assert_eq!(p2p_bc[0].number, 1);
        assert_eq!(p2p_bc[0].transaction_list[0].details, "Genesis block");
    }
    #[test]
    fn test_new_block() {
        let mut p2p_bc: Vec<Block> = vec![Block::genesis()];
        let new_txn = Transaction {
            id: String::from("1"),
            timestamp: 0,
            details: String::from("Testing a new transaction"),
        };
        let mut new_block = Block::new(vec![new_txn], &p2p_bc[p2p_bc.len() - 1]);
        Block::mine_block(&mut new_block, &PREFIX);
        p2p_bc.push(new_block);
        assert_eq!(p2p_bc.len(), 2);
        assert_eq!(
            p2p_bc[1].transaction_list[0].details,
            "Testing a new transaction"
        );
    }
}
