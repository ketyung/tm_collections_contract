pub mod models;
pub mod collections_manage;
pub mod collections_view;
pub mod ext;
pub mod ticket_mint;
mod tests;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env,ext_contract, BorshStorageKey, AccountId, Gas, PromiseError, Promise  };
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::{UnorderedMap};
use crate::models::{Collection, CollectionId, TicketType, Attribute, TicketTemplate};
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::{Token, TokenId};


const NEAR: u128 = 1000_000_000_000_000_000_000_000;

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    
    CollectionStorageKey,
}


// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    
    collections : UnorderedMap<CollectionId, Collection>,

    allowed_callers : Option<Vec<AccountId>>,

    ticket_mints_contract_id : Option<AccountId>, 

    date_updated : Option<u64>, 

}


// Define the default, which automatically initializes the contract
impl Default for Contract{

    fn default() -> Self{
        Self{
            collections : UnorderedMap::new(StorageKey::CollectionStorageKey),
            allowed_callers : None, ticket_mints_contract_id : None, 
            date_updated : Some(env::block_timestamp()),
        }
    }
}

#[near_bindgen]
impl Contract {

    #[allow(dead_code)]
    pub (crate) fn test_init() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        
        Self{ collections :  UnorderedMap::new(StorageKey::CollectionStorageKey),  
            allowed_callers : None, ticket_mints_contract_id : None,    
            date_updated : Some(env::block_timestamp())}
    }

}


#[near_bindgen]
impl Contract {

    #[init]
    #[private] // Public - but only callable by env::current_account_id()
    pub fn init(allowed_callers : Vec<AccountId>, ticket_mints_contract_id : AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        
        let s = Self{ collections :  UnorderedMap::new(StorageKey::CollectionStorageKey),  
            allowed_callers : Some(allowed_callers.clone()),    
            ticket_mints_contract_id : Some(ticket_mints_contract_id.clone()), 
            date_updated : Some(env::block_timestamp())};

        env::log_str(format!("Contract {} has been initialized with allowed callers {:?} & {}",
        env::current_account_id(),
        allowed_callers, ticket_mints_contract_id).as_str());

        return s ;
    }
}


