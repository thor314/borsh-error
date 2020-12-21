This repo contains an example of an error I'm encountering, having to do
serialization. To reproduce the error, run `./build.sh`. To eliminate the error,
comment out the function `get_token` in `lib.rs`.

The error appears to occur on any instance of attempting to access data stored
within an UnorderedMap data structure.

Examples:
```rust
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Marketplace {
    pub(crate) ft: UnorderedMap<UniqueId, Token>,
    pub(crate) my_vector: Vector<Offer>,
}
pub struct Token {
    pub(crate) owner_id: AccountId,
    pub(crate) store_id: StoreId,
    pub(crate) token_id: TokenId,
    pub(crate) royalties: Option<Royalty>,
    offer_history: Vector<Offer>,
}
struct Offer{}
impl Marketplace{
    // BIG BAD: Each of these trigger an error on build.sh (not cargo build)
    // the trait `Serialize` is not implemented for `Token`
    pub fn get_token(&self, unique_id: &UniqueId) -> Token {
        self.ft
            .get(unique_id)
            .unwrap_or_else(|| env::panic(b"could not find that token"))
    }
    // the trait `Serialize` is not implemented for `Offer`
    pub fn get_offer_at_index(&self, unique_id: &UniqueId, index: u64) -> Option<Offer> {
        self.get_token(unique_id).get_offer_at_index(index)
    }
    // the trait `Serialize` is not implemented for `Royalty`
    pub fn get_royalties(&self, unique_id: &UniqueId) -> Option<Royalty> {
        self.get_token(unique_id).royalties
    }
}
```

Possible Fix: For all helper structs, use exclusively Serde-serialization
compatible data structures. Convert to Borsh-serializeable data structures only
at the contract interface level (ie, in this example Token is a helper, and
Market is the contract interface).
