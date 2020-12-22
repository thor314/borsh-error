#[allow(unused_imports)]
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{UnorderedMap, UnorderedSet, Vector},
    env, near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId, Balance, Promise, PromiseOrValue,
};
use std::collections::HashMap;
type Percentage = f64;
type UniqueId = String;
type TokenId = u32;
type StoreId = String;
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Marketplace {
    pub(crate) ft: UnorderedMap<UniqueId, Token>,
    pub(crate) my_vector: Vector<Offer>,
}
impl Default for Marketplace {
    fn default() -> Self {
        env::panic(b"cannot default init marketplace");
    }
}
pub fn gen_unique_id(store_id: StoreId, token_id: TokenId) -> UniqueId {
    store_id + "." + &token_id.to_string()
}
#[near_bindgen]
impl Marketplace {
    pub fn new_token(
        &mut self,
        owner_id: AccountId,
        token_id: TokenId,
        royalties: Option<(Percentage, HashMap<AccountId, Percentage>)>,
    ) {
        let unique_id = gen_unique_id(env::predecessor_account_id(), token_id);
        let token = Token::new(owner_id, env::predecessor_account_id(), token_id, royalties);
        self.ft.insert(&unique_id, &token);
    }
    //
    //
    // BIG BAD: Each of these trigger an error on build.sh (not cargo build)
    // the trait `Serialize` is not implemented for `Token`
    pub fn get_token(&self, unique_id: &UniqueId) -> Token {
        self.ft
            .get(unique_id)
            .unwrap_or_else(|| env::panic(b"could not find that token"))
    }
    // the trait `Serialize` is not implemented for `Offer`
    pub fn get_offer_at_index(&self, unique_id: &UniqueId, index: u64) -> Option<Offer> {
        self.ft
            .get(unique_id)
            .unwrap_or_else(|| env::panic(b"could not find that token"))
            .get_offer_at_index(index)
    }

    // the trait `Serialize` is not implemented for `Royalty`
    pub fn get_royalties(&self, unique_id: &UniqueId) -> Option<Royalty> {
        self.get_token(unique_id).royalties
    }
    // I expected this to error out, but since it operates at the contract
    // interface level, it appears to be fine
    pub fn lookup_within_vector(&self, index: u64) -> Option<Offer> {
        self.my_vector.get(index)
    }
}

// Add Serialize, Deserialize because we can't tell the wasm
// compiler to use Borsh
//
// Convert the Vector in offer_history to a Vec, because we can't use Borsh
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
    pub(crate) owner_id: AccountId,
    pub(crate) store_id: StoreId,
    pub(crate) token_id: TokenId,
    pub(crate) royalties: Option<Royalty>,
    offer_history: Vec<Offer>, // this has to become a Vec
}
impl Token {
    pub(crate) fn new(
        owner_id: AccountId,
        store_id: StoreId,
        token_id: TokenId,
        royalties: Option<(Percentage, HashMap<AccountId, Percentage>)>,
    ) -> Self {
        let royalties = royalties.map(|(p, map)| Royalty::new(p, map));
        Self {
            owner_id,
            store_id,
            token_id,
            royalties,
            offer_history: vec![],
        }
    }
    pub(crate) fn get_offer_at_index(&self, i: u64) -> Option<Offer> {
        self.offer_history.get(i as usize).map(|o| *o)
    }
}

// Add Serialize, Deserialize because we can't tell the wasm
// compiler to use Borsh
//
// Convert the UnorderedMap in Royalty to a Hashmap, because we can't use Borsh
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Royalty {
    percentage: Percentage,
    split_between: HashMap<AccountId, Percentage>,
}
impl Royalty {
    pub(crate) fn new(
        percentage: Percentage,
        split_between: HashMap<AccountId, Percentage>,
    ) -> Self {
        let mut umap: HashMap<AccountId, Percentage> = HashMap::new();
        split_between.iter().for_each(|(acctid, p)| {
            umap.insert(acctid.to_string(), *p);
        });
        Self {
            percentage,
            split_between: umap,
        }
    }
}

// add Clone, Copy so we can pass around references to Offers
//
// add Serialize, Deserialize because we can't tell the wasm
// compiler to use Borsh
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub struct Offer {
    price: Balance,
}
