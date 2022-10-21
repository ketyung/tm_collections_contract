use crate::*;

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;

// Validator interface, for cross-contract calls
#[ext_contract(users_contract)]
trait UsersContract {
    fn has_user(&self, user_id : &String) -> bool;
}


// Validator interface, for cross-contract calls
#[ext_contract(nft_contract)]
trait NftContract {

    fn nft_mint (token_id: TokenId,receiver_id: AccountId,
    token_metadata: TokenMetadata) -> Token;

}

#[ext_contract(ticket_sales_record)]
trait TicketSalesRecord {

    fn insert_ticket_sale(collection_id : CollectionId, token_id : TokenId, mint_by : AccountId);

}