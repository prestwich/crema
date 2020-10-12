use std::ops::DerefMut;

use hex;
use lazy_static::lazy_static;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

use ethers::{core::types::Bytes, providers::ProviderError};

use crate::rpc::{Eip2537Rpc, Eip2539Rpc};
use eth_pairings::public_interface::{
    eip2537::{randgen as randgen2537, EIP2537Executor},
    eip2539::{randgen as randgen2539, EIP2539Executor},
    ApiError,
};

use tokio::sync::RwLock;

lazy_static! {
    // TODO
    static ref RNG: RwLock<XorShiftRng> = XorShiftRng::from_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]).into();
}

pub fn compare_pair_results(
    left: Result<Bytes, ProviderError>,
    right: Result<[u8; 32], ApiError>,
) -> bool {
    if let Ok(a) = left {
        if let Ok(b) = right {
            return b.as_ref() == a.as_ref();
        }
        dbg!("right error");
    }
    dbg!("left error");
    false
}

pub fn compare_g1_results(
    left: Result<Bytes, ProviderError>,
    right: Result<[u8; 128], ApiError>,
) -> bool {
    if let Ok(a) = left {
        if let Ok(b) = right {
            return b.as_ref() == a.as_ref();
        }
        dbg!("right error");
    }
    dbg!("left error");
    false
}

pub fn compare_g2_results(
    left: Result<Bytes, ProviderError>,
    right: Result<[u8; 256], ApiError>,
) -> bool {
    if let Ok(a) = left {
        if let Ok(b) = right {
            return b.as_ref() == a.as_ref();
        }
        dbg!("right error");
    }
    dbg!("left error");
    false
}

async fn write_random_scalar<W: std::io::Write>(mut w: W) {
    let encoding = {
        let mut rng = RNG.write().await;
        let (_, encoding) = randgen2537::make_random_scalar_with_encoding(rng.deref_mut());
        encoding
    };
    w.write(&encoding).unwrap();
}

async fn write_random_eip2537_g1<W: std::io::Write>(mut w: W) {
    let encoding = {
        let mut rng = RNG.write().await;
        let (_, encoding) = randgen2537::make_random_g1_with_encoding(rng.deref_mut());
        encoding
    };
    w.write(&encoding).unwrap();
}

async fn write_random_eip2537_g2<W: std::io::Write>(mut w: W) {
    let encoding = {
        let mut rng = RNG.write().await;
        let (_, encoding) = randgen2537::make_random_g2_with_encoding(rng.deref_mut());
        encoding
    };
    w.write(&encoding).unwrap();
}

async fn write_random_eip2539_g1<W: std::io::Write>(mut w: W) {
    let encoding = {
        let mut rng = RNG.write().await;
        let (_, encoding) = randgen2539::make_random_g1_with_encoding(rng.deref_mut());
        encoding
    };
    w.write(&encoding).unwrap();
}

async fn write_random_eip2539_g2<W: std::io::Write>(mut w: W) {
    let encoding = {
        let mut rng = RNG.write().await;
        let (_, encoding) = randgen2539::make_random_g2_with_encoding(rng.deref_mut());
        encoding
    };
    w.write(&encoding).unwrap();
}

pub async fn compare_eip2537_g1_add(input: &[u8]) -> bool {
    let left = Eip2537Rpc::g1_add(input).await;
    let right = EIP2537Executor::g1_add(input);
    compare_g1_results(left, right)
}

pub async fn rand_eip2537_g1_add() {
    let mut input = vec![];
    write_random_eip2537_g1(&mut input).await;
    write_random_eip2537_g1(&mut input).await;
    if !compare_eip2537_g1_add(&input).await {
        panic!(format!(
            "compare_eip2537_g1_add failed on input {}",
            hex::encode(&input)
        ))
    }
}

pub async fn compare_eip2537_g1_mul(input: &[u8]) -> bool {
    let left = Eip2537Rpc::g1_mul(input).await;
    let right = EIP2537Executor::g1_mul(input);
    compare_g1_results(left, right)
}

pub async fn rand_eip2537_g1_mul() {
    let mut input = vec![];
    write_random_eip2537_g1(&mut input).await;
    write_random_scalar(&mut input).await;
    if !compare_eip2537_g1_mul(&input).await {
        panic!(format!(
            "compare_eip2537_g1_mul failed on input {}",
            hex::encode(&input)
        ))
    }
}

pub async fn compare_eip2537_g1_multiexp(input: &[u8]) -> bool {
    let left = Eip2537Rpc::g1_multiexp(input).await;
    let right = EIP2537Executor::g1_multiexp(input);
    compare_g1_results(left, right)
}

pub async fn rand_eip2537_g1_multiexp() {
    let mut input = vec![];
    // TODO: multiples
    write_random_eip2537_g1(&mut input).await;
    write_random_scalar(&mut input).await;

    if !compare_eip2537_g1_multiexp(&input).await {
        panic!(format!(
            "compare_eip2537_g1_multiexp failed on input {}",
            hex::encode(&input)
        ))
    }
}

pub async fn compare_eip2537_g2_add(input: &[u8]) -> bool {
    let left = Eip2537Rpc::g2_add(input).await;
    let right = EIP2537Executor::g2_add(input);
    compare_g2_results(left, right)
}

pub async fn rand_eip2537_g2_add() {
    let mut input = vec![];
    write_random_eip2537_g2(&mut input).await;
    write_random_eip2537_g2(&mut input).await;
    if !compare_eip2537_g2_add(&input).await {
        panic!(format!(
            "compare_eip2537_g2_add failed on input {}",
            hex::encode(&input)
        ))
    }
}

pub async fn compare_eip2537_g2_mul(input: &[u8]) -> bool {
    let left = Eip2537Rpc::g2_mul(input).await;
    let right = EIP2537Executor::g2_mul(input);
    compare_g2_results(left, right)
}

pub async fn rand_eip2537_g2_mul() {
    let mut input = vec![];
    write_random_eip2537_g2(&mut input).await;
    write_random_scalar(&mut input).await;
    if !compare_eip2537_g2_mul(&input).await {
        panic!(format!(
            "compare_eip2537_g2_mul failed on input {}",
            hex::encode(&input)
        ))
    }
}

pub async fn compare_eip2537_g2_multiexp(input: &[u8]) -> bool {
    let left = Eip2537Rpc::g2_multiexp(input).await;
    let right = EIP2537Executor::g2_multiexp(input);
    compare_g2_results(left, right)
}

pub async fn rand_eip2537_g2_multiexp() {
    let mut input = vec![];
    // TODO: multiples
    write_random_eip2537_g2(&mut input).await;
    write_random_scalar(&mut input).await;
    if !compare_eip2537_g2_multiexp(&input).await {
        panic!(format!(
            "compare_eip2537_g2_multiexp failed on input {}",
            hex::encode(&input)
        ))
    }
}

// TODO

// pub async fn compare_eip2537_pairing(input: &[u8]) -> bool {
//     let left = Eip2537Rpc::pairing(input).await;
//     let right = EIP2537Executor::pair(input);
//     compare_pair_results(left, right)
// }

// pub async fn rand_eip2537_pairing() {
//     let mut input = vec![];
//     if !compare_eip2537_pairing(&input).await {
//         panic!(format!("compare_eip2537_pairing failed on input {}", hex::encode(&input)))
//     }
// }

// pub async fn compare_eip2537_map_to_g1(input: &[u8]) -> bool {
//     let left = Eip2537Rpc::map_to_g1(input).await;
//     let right = EIP2537Executor::map_fp_to_g1(input);
//     compare_g1_results(left, right)
// }

// pub async fn rand_eip2537_map_to_g1() {
//     let mut input = vec![];
//     if !compare_eip2537_map_to_g1(&input).await {
//         panic!(format!("compare_eip2537_map_to_g1 failed on input {}", hex::encode(&input)))
//     }
// }

// pub async fn compare_eip2537_map_to_g2(input: &[u8]) -> bool {
//     let left = Eip2537Rpc::map_to_g2(input).await;
//     let right = EIP2537Executor::map_fp2_to_g2(input);
//     compare_g2_results(left, right)
// }

// pub async fn rand_eip2537_map_to_g2() {
//     let mut input = vec![];
//     if !compare_eip2537_map_to_g2(&input).await {
//         panic!(format!("compare_eip2537_map_to_g2 failed on input {}", hex::encode(&input)))
//     }
// }

pub async fn compare_eip2539_g1_add(input: &[u8]) -> bool {
    let left = Eip2539Rpc::g1_add(input).await;
    let right = EIP2539Executor::g1_add(input);
    compare_g1_results(left, right)
}

pub async fn rand_eip2539_g1_add() {
    let mut input = vec![];
    write_random_eip2539_g1(&mut input).await;
    write_random_eip2539_g1(&mut input).await;
    if !compare_eip2539_g1_add(&input).await {
        panic!(format!(
            "compare_eip2539_g1_add failed on input {}",
            hex::encode(&input)
        ))
    }
}

pub async fn compare_eip2539_g1_mul(input: &[u8]) -> bool {
    let left = Eip2539Rpc::g1_mul(input).await;
    let right = EIP2539Executor::g1_mul(input);
    compare_g1_results(left, right)
}

pub async fn rand_eip2539_g1_mul() {
    let mut input = vec![];
    write_random_eip2539_g1(&mut input).await;
    write_random_scalar(&mut input).await;
    if !compare_eip2539_g1_mul(&input).await {
        panic!(format!(
            "compare_eip2539_g1_mul failed on input {}",
            hex::encode(&input)
        ))
    }
}

pub async fn compare_eip2539_g1_multiexp(input: &[u8]) -> bool {
    let left = Eip2539Rpc::g1_multiexp(input).await;
    let right = EIP2539Executor::g1_multiexp(input);
    compare_g1_results(left, right)
}

pub async fn rand_eip2539_g1_multiexp() {
    let mut input = vec![];
    // TODO: multiples
    write_random_eip2539_g1(&mut input).await;
    write_random_scalar(&mut input).await;

    if !compare_eip2539_g1_multiexp(&input).await {
        panic!(format!(
            "compare_eip2539_g1_multiexp failed on input {}",
            hex::encode(&input)
        ))
    }
}

pub async fn compare_eip2539_g2_add(input: &[u8]) -> bool {
    let left = Eip2539Rpc::g2_add(input).await;
    let right = EIP2539Executor::g2_add(input);
    compare_g2_results(left, right)
}

pub async fn rand_eip2539_g2_add() {
    let mut input = vec![];
    write_random_eip2539_g2(&mut input).await;
    write_random_eip2539_g2(&mut input).await;
    if !compare_eip2539_g2_add(&input).await {
        panic!(format!(
            "compare_eip2539_g2_add failed on input {}",
            hex::encode(&input)
        ))
    }
}

pub async fn compare_eip2539_g2_mul(input: &[u8]) -> bool {
    let left = Eip2539Rpc::g2_mul(input).await;
    let right = EIP2539Executor::g2_mul(input);
    compare_g2_results(left, right)
}

pub async fn rand_eip2539_g2_mul() {
    let mut input = vec![];
    write_random_eip2539_g2(&mut input).await;
    write_random_scalar(&mut input).await;
    if !compare_eip2539_g2_mul(&input).await {
        panic!(format!(
            "compare_eip2539_g2_mul failed on input {}",
            hex::encode(&input)
        ))
    }
}

pub async fn compare_eip2539_g2_multiexp(input: &[u8]) -> bool {
    let left = Eip2539Rpc::g2_multiexp(input).await;
    let right = EIP2539Executor::g2_multiexp(input);
    compare_g2_results(left, right)
}

pub async fn rand_eip2539_g2_multiexp() {
    let mut input = vec![];
    // TODO: multiples
    write_random_eip2539_g2(&mut input).await;
    write_random_scalar(&mut input).await;
    if !compare_eip2539_g2_multiexp(&input).await {
        panic!(format!(
            "compare_eip2539_g2_multiexp failed on input {}",
            hex::encode(&input)
        ))
    }
}
// TODO

// pub async fn compare_eip2539_pairing(input: &[u8]) -> bool {
//     let left = Eip2539Rpc::pairing(input).await;
//     let right = EIP2539Executor::pair(input);
//     compare_pair_results(left, right)
// }

// pub async fn rand_eip2539_pairing() {
//     let mut input = vec![];
//     if !compare_eip2539_pairing(&input).await {
//         panic!(format!("compare_eip2539_pairing failed on input {}", hex::encode(&input)))
//     }
// }
