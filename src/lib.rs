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
    // BIG BAD: COMMENT THIS OUT TO FIX build.sh
    //
    //
    // pub fn get_token(&self, unique_id: &UniqueId) -> Token {
    //     self.ft
    //         .get(&unique_id)
    //         .unwrap_or_else(|| env::panic(b"could not find that token"))
    // }
}
#[derive(BorshSerialize, BorshDeserialize)] // dependency: insert into UMap
pub struct Token {
    pub(crate) owner_id: AccountId,
    pub(crate) store_id: StoreId,
    pub(crate) token_id: TokenId,
    pub(crate) royalties: Option<Royalty>,
    offer_history: Vector<Offer>,
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
            offer_history: Vector::new(b"t".to_vec()),
        }
    }
}
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Royalty {
    percentage: Percentage,
    split_between: UnorderedMap<AccountId, Percentage>,
}
impl Royalty {
    pub(crate) fn new(
        percentage: Percentage,
        split_between: HashMap<AccountId, Percentage>,
    ) -> Self {
        let mut umap: UnorderedMap<AccountId, Percentage> = UnorderedMap::new(b"uf".to_vec());
        split_between.iter().for_each(|(acctid, p)| {
            umap.insert(&acctid.to_string(), &p);
        });
        Self {
            percentage,
            split_between: umap,
        }
    }
}
//#[derive(BorshSerialize, BorshDeserialize)] // unneccessary for example
pub struct Offer {}
