use blockchainlib::*;

fn main () {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let block_number:u8 = 0;
    let mut block = Block::new(0, now(), vec![0;32], 1, block_number.to_string().to_owned(), difficulty);
    
    block.mine();
    println!("Mined genesis block {:?}", &block);

    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    println!("Verify: {}", &blockchain.verify());
    
    for i in 1..=10 {
        let mut block = Block::new(i, now(), last_hash, 1, "New Block".to_owned(), difficulty);
        
        block.mine();
        println!("Mined block {:?}", &block);
        
        last_hash = block.hash.clone();
        
        blockchain.blocks.push(block);
        println!("Verify: {}", &blockchain.verify());
    }
}
