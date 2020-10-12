use lazy_static::lazy_static;
use std::convert::TryFrom;

use ethers::core::types::{Address, Bytes, TransactionRequest};
use ethers::providers::{Http, Middleware, Provider, ProviderError};

lazy_static! {
    static ref PROVIDER: Provider<Http> = Provider::<Http>::try_from("http://localhost:8545")
        .expect("could not instantiate HTTP Provider");
}

pub async fn run_precompile(addr: u8, data: &[u8]) -> Result<Bytes, ProviderError> {
    let tx_req = TransactionRequest::new()
        .to(Address::from_low_u64_be(addr as u64))
        .data(data.to_vec());
    let fut = { PROVIDER.call(&tx_req, None) };

    fut.await
}

#[repr(u8)]
enum Eip2537Addrs {
    G1Add = 10,
    G1Mul = 11,
    G1MultiExp = 12,
    G2Add = 13,
    G2Mul = 14,
    G2MultiExp = 15,
    Pairing = 16,
    MapToG1 = 17,
    MapToG2 = 18,
}

pub struct Eip2537Rpc;

impl Eip2537Rpc {
    pub async fn g1_add(input: &[u8]) -> Result<Bytes, ProviderError> {
        run_precompile(Eip2537Addrs::G1Add as u8, input).await
    }

    pub async fn g1_mul(input: &[u8]) -> Result<Bytes, ProviderError> {
        run_precompile(Eip2537Addrs::G1Mul as u8, input).await
    }

    pub async fn g1_multiexp(input: &[u8]) -> Result<Bytes, ProviderError> {
        run_precompile(Eip2537Addrs::G1MultiExp as u8, input).await
    }

    pub async fn g2_add(input: &[u8]) -> Result<Bytes, ProviderError> {
        run_precompile(Eip2537Addrs::G2Add as u8, input).await
    }

    pub async fn g2_mul(input: &[u8]) -> Result<Bytes, ProviderError> {
        run_precompile(Eip2537Addrs::G2Mul as u8, input).await
    }

    pub async fn g2_multiexp(input: &[u8]) -> Result<Bytes, ProviderError> {
        run_precompile(Eip2537Addrs::G2MultiExp as u8, input).await
    }

    pub async fn pairing(input: &[u8]) -> Result<Bytes, ProviderError> {
        run_precompile(Eip2537Addrs::Pairing as u8, input).await
    }

    pub async fn map_to_g1(input: &[u8]) -> Result<Bytes, ProviderError> {
        run_precompile(Eip2537Addrs::MapToG1 as u8, input).await
    }

    pub async fn map_to_g2(input: &[u8]) -> Result<Bytes, ProviderError> {
        run_precompile(Eip2537Addrs::MapToG2 as u8, input).await
    }
}

#[repr(u8)]
enum Eip2539Addrs {
    G1Add = 19,
    G1Mul = 20,
    G1MultiExp = 21,
    G2Add = 22,
    G2Mul = 23,
    G2MultiExp = 24,
    Pairing = 25,
}

pub struct Eip2539Rpc;

impl Eip2539Rpc {
    pub async fn g1_add(input: &[u8]) -> Result<Bytes, ProviderError> {
        run_precompile(Eip2539Addrs::G1Add as u8, input).await
    }

    pub async fn g1_mul(input: &[u8]) -> Result<Bytes, ProviderError> {
        run_precompile(Eip2539Addrs::G1Mul as u8, input).await
    }

    pub async fn g1_multiexp(input: &[u8]) -> Result<Bytes, ProviderError> {
        run_precompile(Eip2539Addrs::G1MultiExp as u8, input).await
    }

    pub async fn g2_add(input: &[u8]) -> Result<Bytes, ProviderError> {
        run_precompile(Eip2539Addrs::G2Add as u8, input).await
    }

    pub async fn g2_mul(input: &[u8]) -> Result<Bytes, ProviderError> {
        run_precompile(Eip2539Addrs::G2Mul as u8, input).await
    }

    pub async fn g2_multiexp(input: &[u8]) -> Result<Bytes, ProviderError> {
        run_precompile(Eip2539Addrs::G2MultiExp as u8, input).await
    }

    pub async fn pairing(input: &[u8]) -> Result<Bytes, ProviderError> {
        run_precompile(Eip2539Addrs::Pairing as u8, input).await
    }
}
