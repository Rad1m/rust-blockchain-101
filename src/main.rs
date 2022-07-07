use blockchainlib::*;

fn main () {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;
    let block_number:u8 = 0;
    let mut block = Block::new(0, now(), vec![0;32], 1, block_number.to_string().to_owned(), difficulty);

    block.hash = block.hash();
    println!("{:?}", &block);

    
    block.mine();
    println!("{:?}", &block);
}
