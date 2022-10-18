use crate::*;
use crate::ext::*;


const USERS_CONTRACT_ID :  &'static str = "tm_users_contract.testnet";

#[near_bindgen]
impl Contract {

    pub fn create_event (&mut self, 
        acc_id : AccountId,     
        title : String, 
        symbol : String,
        description : Option<String>,
        total_tickets : Option<u64>,
        tickets_sold : Option<u64>,
        ticket_types : Option<Vec<TicketType>>,
        attributes : Option<Vec<EventAttribute>>,
        contract_id : Option<AccountId>) {

        users_contract::ext(USERS_CONTRACT_ID.parse().unwrap())
        .with_static_gas(Gas(5*TGAS))
        .has_user(&acc_id.clone().as_str().to_string())
        .then( 
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(5*TGAS))
            .create_event_callback(acc_id, title, symbol, description,
            total_tickets,tickets_sold, ticket_types, attributes, contract_id)
        );

    
    }

    

}

#[near_bindgen]
impl Contract {

    

    #[private] // Public - but only callable by env::current_account_id()
    pub fn create_event_callback(&mut self,
        acc_id : AccountId,     
        title : String, 
        symbol : String,
        description : Option<String>,
        total_tickets : Option<u64>,
        tickets_sold : Option<u64>,
        ticket_types : Option<Vec<TicketType>>,
        attributes : Option<Vec<EventAttribute>>,
        contract_id : Option<AccountId>,
        #[callback_result] call_result: Result<bool, PromiseError>) {
        
        if call_result.is_err() {
            env::panic_str("The was an error calling the remote contract");
        }
    
        // Return the greeting
        let has_usr: bool = call_result.unwrap();
        
        if has_usr {
    
            self.internal_create_nft_collection(env::signer_account_id(), 
            title, symbol,description,  
            total_tickets,tickets_sold,
            ticket_types, attributes, contract_id);
    
            self.date_updated = Some(env::block_timestamp());
    
        }
        else {
            env::panic_str(format!("User with '{}' does NOT exist",acc_id.as_str()).as_str());
        }

    }
}


#[near_bindgen]
impl Contract {

    pub (crate) fn internal_create_nft_collection (&mut self, 
        acc_id : AccountId,     
        title : String, 
        symbol : String,
        description : Option<String>,
        total_tickets : Option<u64>,
        tickets_sold : Option<u64>,
        ticket_types : Option<Vec<TicketType>>,
        attributes : Option<Vec<EventAttribute>>,
        contract_id : Option<AccountId>) {
    
        
        let event_id = EventId {
            owner : acc_id.clone(),
            symbol : symbol.clone(), 
            title : title.clone(),
        };

        if self.events.get(&event_id).is_some() {
            env::panic_str(format!("The event {} for {} already exists",title,acc_id).as_str())
        }


        let event = Event {
            title : title,
            description : description,
            total_tickets : total_tickets,
            tickets_sold : tickets_sold,
            symbol : symbol,
            owner : acc_id,
            contract_id : contract_id, 
            ticket_types : ticket_types, 
            attributes : attributes,
            date_updated : Some(env::block_timestamp()),
        };


        self.events.insert(&event_id, &event);
    }

       

 
}





#[near_bindgen]
impl Contract {

    // temporary 
    pub fn remove_all_events(&mut self) {

        if env::signer_account_id() == env::current_account_id() {

            self.events.clear();
        }
        else {

            env::panic_str(format!("{} is unauthorized for removal of all events", env::signer_account_id()).as_str())
        }
    }
}
