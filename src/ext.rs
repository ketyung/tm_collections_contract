use crate::*;

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;


// Validator interface, for cross-contract calls
#[ext_contract(nft_contract)]
trait NftContract {

    fn nft_mint (token_id: TokenId,receiver_id: AccountId,
    token_metadata: TokenMetadata) -> Token;

    fn nft_tokens_for_owner(account_id: AccountId,
        from_index: Option<near_sdk::json_types::U128>,
        limit: Option<u64>) -> Vec<Token>;

}

#[ext_contract(ticket_mints_contract)]
trait TicketMintsRecord {

    fn insert_ticket_mint(collection_id : CollectionId, token_id : TokenId,
        mint_by : AccountId, price : Option<u128>, ticket_type : Option<String>);

}