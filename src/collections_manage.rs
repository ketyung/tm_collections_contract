use crate::*;
use crate::ext::*;


const DEFAULT_USERS_CONTRACT_ID :  &'static str = "tm_users_contract.testnet";

#[near_bindgen]
impl Contract {

    fn get_users_contract_account(&self) -> AccountId {

        if self.users_contract_id.is_some() {

            return self.users_contract_id.clone().unwrap();
        }

        DEFAULT_USERS_CONTRACT_ID.parse().unwrap()
    }
}

#[near_bindgen]
impl Contract {

    pub fn create_collection (&mut self, 
        acc_id : AccountId,     
        title : String, 
        symbol : String,
        icon : Option<String>,
        base_uri : Option<String>,
        description : Option<String>,
        category : Option<String>,
        total_tickets : Option<u64>,
        tickets_sold : Option<u64>,
        ticket_types : Option<Vec<TicketType>>,
        attributes : Option<Vec<Attribute>>,
        template_type : Option<TicketTemplate>,
        contract_id : Option<AccountId>) {


        users_contract::ext(self.get_users_contract_account())
        .with_static_gas(Gas(5*TGAS))
        .has_user(&acc_id.clone().as_str().to_string())
        .then( 
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(5*TGAS))
            .create_collection_callback(acc_id, title, symbol, 
            icon, base_uri, description, category,
            total_tickets,tickets_sold, 
            ticket_types,  attributes,template_type, contract_id)
        );

    
    }

    

}

#[near_bindgen]
impl Contract {

    

    #[private] // Public - but only callable by env::current_account_id()
    pub fn create_collection_callback(&mut self,
        acc_id : AccountId,     
        title : String, 
        symbol : String,
        icon : Option<String>,
        base_uri : Option<String>,
        description : Option<String>,
        category : Option<String>,
        total_tickets : Option<u64>,
        tickets_sold : Option<u64>,
        ticket_types : Option<Vec<TicketType>>,
        attributes : Option<Vec<Attribute>>,
        template_type : Option<TicketTemplate>,
        contract_id : Option<AccountId>,
        #[callback_result] call_result: Result<bool, PromiseError>) {
        
        if call_result.is_err() {
            env::panic_str("The was an error calling the remote contract");
        }
    
        // Return the greeting
        let has_usr: bool = call_result.unwrap();
        
        if has_usr {
    
            self.internal_create_collection(env::signer_account_id(), 
            title, symbol, icon, 
            base_uri, description, category, 
            total_tickets,tickets_sold,
            ticket_types, attributes, 
            template_type, contract_id);
    
            self.date_updated = Some(env::block_timestamp());
    
        }
        else {
            env::panic_str(format!("User with '{}' does NOT exist",acc_id.as_str()).as_str());
        }

    }
}


#[near_bindgen]
impl Contract {

    pub (crate) fn internal_create_collection (&mut self, 
        acc_id : AccountId,     
        title : String, 
        symbol : String,
        icon : Option<String>,
        base_uri : Option<String>,

        description : Option<String>,
        category : Option<String>,
        total_tickets : Option<u64>,
        tickets_sold : Option<u64>,

        ticket_types : Option<Vec<TicketType>>,
        attributes : Option<Vec<Attribute>>,
        template_type : Option<TicketTemplate>,
        contract_id : Option<AccountId>) {
    
        
        let collection_id = CollectionId {
            owner : acc_id.clone(),
            symbol : symbol.clone(), 
            title : title.clone(),
        };

        if self.collections.get(&collection_id).is_some() {
            env::panic_str(format!("The collection {} for {} already exists",title,acc_id).as_str())
        }


        let collection = Collection {
            title : title,
            description : description,
            category : category, 
            icon : icon, 
            base_uri : base_uri, 
            total_tickets : total_tickets,
            tickets_sold : tickets_sold,
            symbol : symbol,
            owner : acc_id,
            contract_id : contract_id, 
            ticket_types : ticket_types, 
            attributes : attributes,
            ticket_template_type: template_type, 
            date_updated : Some(env::block_timestamp()),
        };


        self.collections.insert(&collection_id, &collection);
    }

       

 
}





#[near_bindgen]
impl Contract {

    // temporary 
    pub fn remove_all_collections(&mut self) {

        if env::signer_account_id() == env::current_account_id() {

            self.collections.clear();
        }
        else {

            env::panic_str(format!("{} is unauthorized for removal of all collections", env::signer_account_id()).as_str())
        }
    }
}
