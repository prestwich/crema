use tokio;
// use tokio::sync::RwLock;

mod comp;
mod rpc;

const ROUNDS: u32 = 100;

async fn fuzz_round() {
    comp::rand_eip2537_g1_add().await;
    comp::rand_eip2537_g1_mul().await;
    comp::rand_eip2537_g1_multiexp().await;
    comp::rand_eip2537_g2_add().await;
    comp::rand_eip2537_g2_mul().await;
    comp::rand_eip2537_g2_multiexp().await;
    // comp::rand_eip2537_pairing().await;
    // comp::rand_eip2537_map_to_g1().await;
    // comp::rand_eip2537_map_to_g2().await;

    comp::rand_eip2539_g1_add().await;
    comp::rand_eip2539_g1_mul().await;
    comp::rand_eip2539_g1_multiexp().await;
    comp::rand_eip2539_g2_add().await;
    comp::rand_eip2539_g2_mul().await;
    comp::rand_eip2539_g2_multiexp().await;
    // comp::rand_eip2539_pairing().await;
}

#[tokio::main]
async fn main() {
    dbg!(rpc::run_precompile(4, &[1, 2, 3, 4]).await.unwrap());

    for _ in 0..ROUNDS {
    // loop {
        fuzz_round().await;
    }
}
