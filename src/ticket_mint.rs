use crate::*;
use crate::ext::*;
use near_sdk::json_types::Base64VecU8;
use near_sdk::PromiseError;

const MIN_STORAGE_COST_FOR_MINT : u128 = 10000_000_000_000_000_000_000;


//7600_000_000_000_000_000_000;

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn ticket_mint (&mut self, 
    collection_id : CollectionId, 
    token_id : TokenId, ticket_image : String,
    ticket_type : Option<TicketType>,
    extra  : Option<String>,
    ref_hash : Option<String>) {
        
        self.panic_if_its_not_allowed_caller();

        let coll =  self.collections.get(&collection_id.clone());
        if coll.is_none () {
            env::panic_str(format!("Collection {:?} not found",collection_id.clone()).as_str());
        }

        let uw_coll = coll.unwrap();

        if uw_coll.tickets_sold.unwrap_or(0) >= uw_coll.total_tickets.unwrap_or(0) {
            env::panic_str("Tickets are sold out!");
        } 

        let tprice = Self::obtain_ticket_price_in_near(uw_coll.ticket_types.clone(), ticket_type.clone());

        let min_attached_deposit = tprice + MIN_STORAGE_COST_FOR_MINT;

        if env::attached_deposit() < min_attached_deposit  {
            env::panic_str(format!("Attached deposit {} is less than ticket price {}",
            env::attached_deposit(),min_attached_deposit).as_str());
        }

        self.has_exceeded_max_ticket_per_wallet(uw_coll.attributes.clone(),
            uw_coll.contract_id.clone(),
            tprice, env::current_account_id());

        let token_meta = Self::create_token_metadata(
            format!("Ticket {}", token_id),
            uw_coll.title.clone(),Some(ticket_image), 
            ref_hash, 
            extra);

        nft_contract::ext(uw_coll.contract_id.clone().unwrap())
        .with_static_gas(Gas(5*TGAS))
        .with_attached_deposit(MIN_STORAGE_COST_FOR_MINT)
        .nft_mint(token_id.clone(), env::signer_account_id(), token_meta)
        .then( 
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(5*TGAS))
            .after_mint_callback(uw_coll, tprice,ticket_type.unwrap().ticket_type, 
            token_id, env::signer_account_id())
        );


    }
}

#[near_bindgen]
impl Contract {

    #[private] // Public - but only callable by env::current_account_id()
    pub fn after_mint_callback(&mut self, collection : Collection, 
        ticket_price : u128, ticket_type : String, 
        token_id : TokenId, mint_by : AccountId,
        #[callback_result] call_result: Result<Token, PromiseError> ){

        let mut m_collection = collection;

        if call_result.is_err() {

            // refund the ticket price to the minter/buyer on error
            Promise::new(mint_by.clone()).transfer(ticket_price).as_return();

            env::log_str(format!("Buyer/minter {} has been refunded with {}",mint_by, ticket_price).as_str());

            env::panic_str(format!("Error at after_mint_callback {:?}", call_result).as_str());
        }    

        env::log_str(format!("Minted token is {:?}", call_result).as_str());

        env::log_str(format!("Going to pay owner {} with {}", m_collection.owner.clone(),
        ticket_price).as_str());
        
        Promise::new(m_collection.owner.clone()).transfer(ticket_price).as_return();

        let collection_id = CollectionId {
            owner : m_collection.owner.clone(),
            symbol : m_collection.symbol.clone(),
            title : m_collection.title.clone(),
        };

        // record ticket sales
        if self.ticket_mints_contract_id.is_some() {

            ticket_mints_contract::ext(self.ticket_mints_contract_id.clone().unwrap())
            .with_static_gas(Gas(5*TGAS))
            .insert_ticket_mint(collection_id.clone(), token_id,mint_by,Some(ticket_price),
            Some(ticket_type)).as_return();
        }

        if m_collection.tickets_sold.is_some() {

            m_collection.tickets_sold = Some(m_collection.tickets_sold.unwrap() + 1);
        }
        else {

            m_collection.tickets_sold = Some(1);
        }

        self.collections.remove(&collection_id);

        self.collections.insert(&collection_id, &m_collection);
    
      
    }


}

#[near_bindgen]
impl Contract {

    fn obtain_ticket_price_in_near(ticket_types : Option<Vec<TicketType>>, ticket_type : Option<TicketType>) -> u128{

        
        if ticket_type.is_some() {

            if ticket_types.is_some() {

                let tt = ticket_type.unwrap();
                if !ticket_types.unwrap().contains(&tt) {
                    env::panic_str(format!("Invalid ticket type {:?}",tt).as_str());
                }

                return  (((tt.price as f64) / 1000.00) * (NEAR as f64)) as u128;

            }
            else {
                env::panic_str("Invalid ticket type!");
            }
        }
        else {

            if ticket_types.is_some() {

                let uw_ticket_types = ticket_types.unwrap();

                let tt = uw_ticket_types.first().unwrap();

                return (((tt.price as f64) / 1000.00) * (NEAR as f64)) as u128;

            }
            else {

                env::panic_str("No tickets type defined!");
            }
        }


    }


    fn create_token_metadata(
        ticket_title : String, collection_title : String, 
        media : Option<String>, 
        ref_hash : Option<String>,
        extra : Option<String>) -> TokenMetadata{

        let mut reference_hash : Option<Base64VecU8> = None;

        if ref_hash.is_some(){

            reference_hash = Some(Base64VecU8::from(
                ref_hash.unwrap().as_bytes().to_vec()));
        }

        TokenMetadata {
            title: Some(ticket_title.clone()),
            description: Some(format!("{} of {}", ticket_title, collection_title)),
            media: media,
            media_hash: None,
            copies: Some(1u64),
            issued_at: None ,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: extra,
            reference: None,
            reference_hash: reference_hash,
        }
    }

}


#[near_bindgen]
impl Contract {


    fn has_exceeded_max_ticket_per_wallet(&mut self, 
        attributes :Option<Vec<Attribute>>,
        contract_id : Option<AccountId>,
        ticket_price : u128, mint_by : AccountId) {

       
        let mut max_ticket_pw_attrib = Attribute {
            name : AttributeType::MaxTicketPerWallet, value : "None".to_string()
        };

        if attributes.is_some() {

            let uw_attrbs = attributes.unwrap(); 
            let index =  uw_attrbs.iter().position(|a| *a == max_ticket_pw_attrib);

            if index.is_none() {
                return; 
            }

            let attrb = uw_attrbs.get(index.unwrap());
            max_ticket_pw_attrib.value = attrb.unwrap().clone().value;

        }    


        nft_contract::ext(contract_id.unwrap())
        .with_static_gas(Gas(5*TGAS))
        .nft_tokens_for_owner(env::signer_account_id(), None, None)
        .then( 
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(5*TGAS))
            .after_obtain_nft_count_callback(max_ticket_pw_attrib, ticket_price, mint_by)
        );

    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn after_obtain_nft_count_callback(&mut self, 
        max_ticket_per_wallet : Attribute,
        ticket_price : u128, mint_by : AccountId,
        #[callback_result] call_result: Result<Vec<Token>, PromiseError> ){

        if call_result.is_err() {

            // refund the ticket price to the minter/buyer on error
            Promise::new(mint_by.clone()).transfer(ticket_price).as_return();

            env::log_str(format!("Buyer/minter {} has been refunded with {}",mint_by, ticket_price).as_str());

            env::panic_str(format!("Error at after_obtain_nft_count_callback {:?}", call_result).as_str());
        }

        let res : Vec<Token> = call_result.unwrap();

        if res.len() >= max_ticket_per_wallet.value.parse::<usize>().unwrap() {

            env::panic_str("Has exceeed the number of tickets per wallet!");
        } 

    
    }


}
