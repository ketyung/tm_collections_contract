#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    // use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, AccountId};
    use crate::*;
    
    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    // cargo test test_create_events -- --show-output
    #[test]
    fn test_create_events() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());

        let mut _contract = Contract::init();

        let acc_id0 = accounts(0);
            
        _contract.internal_create_event(acc_id0.clone(),
        "Test Event 01".to_string(), 
        "TC01".to_string(),
        Some("This is an event for selling 5000 NFT tickets".to_string()),
        None, None, None, None, None, None );


        _contract.internal_create_event(acc_id0.clone(),
        "Test Event 02".to_string(), 
        "TC02".to_string(),
        Some("This is an event for selling 250 NFT tickets".to_string()),
        None, None, None, None, None, None );


        let events = _contract.get_events_of(acc_id0, None, None);

        for (pos, e) in events.iter().enumerate() {
            println!("{} - Event {:?} : {:?}", (pos + 1), e.title, 
            e.description.clone().unwrap_or("None".to_string()));
        }

        testing_env!(context.is_view(true).build());

    }



  
 
}