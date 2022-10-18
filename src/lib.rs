pub mod models;
pub mod events_manage;
pub mod events_view;
pub mod ext;


use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env, BorshStorageKey, AccountId, Gas, PromiseError  };
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::{UnorderedMap};
use crate::models::{Event, EventId, TicketType, EventAttribute};


#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    EventStorageKey,
}


// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    
    events : UnorderedMap<EventId, Event>,

    date_updated : Option<u64>, 

}


// Define the default, which automatically initializes the contract
impl Default for Contract{

    fn default() -> Self{
        Self{
            events : UnorderedMap::new(StorageKey::EventStorageKey),
            date_updated : Some(env::block_timestamp()),
        }
    }
}

#[near_bindgen]
impl Contract {

    #[init]
    #[private] // Public - but only callable by env::current_account_id()
    pub fn init() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        
        Self{ events :  UnorderedMap::new(StorageKey::EventStorageKey),     
            date_updated : Some(env::block_timestamp())}
    }


}


