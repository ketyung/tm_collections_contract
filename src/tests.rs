#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    // use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, AccountId};
    use crate::*;
    use crate::models::{AttributeType, CollectionDataForUpdate};
    
    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    // cargo test test_create_collections -- --show-output
    #[test]
    fn test_create_collections() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());

        let mut _contract = Contract::test_init();

        let acc_id0 = accounts(0);
            
        _contract.internal_create_collection(acc_id0.clone(),
        "Test Collection 01".to_string(), 
        "TC01".to_string(), Some("http://img.io/img/kslsj3".to_string()), None, 
        Some("This is an collection for selling 5000 NFT tickets".to_string()),
        Some("Event Ticket".to_string()),
        None, None, None, Some(vec![Attribute{
            name : AttributeType::StartDate,
            value : "2022-10-20".to_string(),
        }, Attribute{
            name : AttributeType::EndDate,
            value : "2022-10-21".to_string(),
        }]), None, None );


        _contract.internal_create_collection(acc_id0.clone(),
        "Test Collection 02".to_string(), 
        "TC02".to_string(), None, None, 
        Some("This is an collection for selling 250 NFT tickets".to_string()),
        Some("Concert Ticket".to_string()),
        None, None, None, Some(vec![Attribute{name : AttributeType::MaxTicketPerWallet,
            value : "1".to_string()}]), None, None );

        
        let collections = _contract.get_collections_of(acc_id0.clone(), None, None);

        for (pos, e) in collections.iter().enumerate() {
            println!("{} - Collection {:?} : {:?}, icon:{:?} attribs: {:?}", (pos + 1), e.title, 
            e.description.clone().unwrap_or("None".to_string()), 
            e.icon.clone().unwrap_or("No.icon".to_string()),
            e.attributes);
        }


        println!("\n");
    
        let collections = _contract.get_collections_by("Concert Ticket".to_string(), None, None);

        println!("Collection of Concert Ticket:");
        for (pos, e) in collections.iter().enumerate() {
            println!("{} - Collection By Cat {:?} : {:?}, icon:{:?} attribs: {:?}", (pos + 1), e.title, 
            e.description.clone().unwrap_or("None".to_string()), 
            e.icon.clone().unwrap_or("No.icon".to_string()),
            e.attributes);
        }

        let update_collection_id = CollectionId {
            owner : acc_id0.clone(),
            title : "Test Collection 02".to_string(), 
            symbol: "TC02".to_string()

        };

        _contract.update_collection(update_collection_id.clone(),
            CollectionDataForUpdate {

                icon : None, 
                description: Some("The Test Collection 02 which contains 500 NFTs".to_string()),
                base_uri : None,
                total_tickets : Some(500),
                attributes : None,
                ticket_template_type : None,
                category : None,
                ticket_types : Some(vec![TicketType {
                    ticket_type : "Standard".to_string(),
                    price : 23500,
                    color_code : Some("#236".to_string()), 
                },TicketType {
                    ticket_type : "Premium".to_string(),
                    price : 33500,
                    color_code : Some("#950".to_string()), 
                }]),
            }
        );


        let col = _contract.get_collection(update_collection_id).unwrap();

        println!("\nObtained.coll::title:{}, descr: {:?}, ticket_types :{:?}", col.title,
        col.description, col.ticket_types);

        testing_env!(context.is_view(true).build());


    }



  
 
}