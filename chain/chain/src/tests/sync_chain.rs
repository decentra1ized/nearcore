use crate::test_utils::setup;
use crate::Block;
use near_logger_utils::init_test_logger;
use near_primitives::merkle::PartialMerkleTree;

#[test]
fn chain_sync_headers() {
    init_test_logger();
    let (mut chain, _, bls_signer) = setup();
    assert_eq!(chain.header_head().unwrap().height, 0);
    let mut blocks = vec![chain.get_block(&chain.genesis().hash().clone()).unwrap().clone()];
    let mut block_merkle_tree = PartialMerkleTree::default();
    for i in 0..4 {
        blocks.push(Block::empty_with_block_merkle_tree(
            &blocks[i],
            &*bls_signer,
            &mut block_merkle_tree,
        ));
    }
    chain
        .sync_block_headers(
            blocks.drain(1..).map(|block| block.header().clone()).collect(),
            &mut |_| panic!("Unexpected"),
        )
        .unwrap();
    assert_eq!(chain.header_head().unwrap().height, 4);
}
