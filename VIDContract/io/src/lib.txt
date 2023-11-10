#![no_std]
use gstd::{ prelude::*, ActorId };
use gmeta::{In, InOut, Metadata};

#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct VidConection {
    pub nft_conection : ActorId,
}

#[derive(Debug,Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TokenMetadata {
    pub owner: ActorId,
    pub name: String,
    pub description: String,
    pub media_url: String,
    pub attrib_url: String,
}

#[derive(Debug,Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Actions {
    TokeCreate(u64, TokenMetadata)
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum NFTAction{
    Mint {
        transaction_id: u64,
        token_metadata: TokenMetadata
    }
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Event {
    SuccessfulCreate,
    SuccessfulDestroy,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Status {
    Ok,
    Err,
    Balance(u128),
    PermitId(u128),
}

pub struct NftMetadata;


impl Metadata for NftMetadata {
    type Init = In<VidConection>;
    type Handle = InOut<Actions, Event>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Vec<(ActorId, u128)>;
}