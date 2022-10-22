use crate::*;
use crate::ext::*;
use near_sdk::json_types::Base64VecU8;

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn ticket_mint (&mut self, 
    mint_by : AccountId,     
    collection_id : CollectionId, 
    token_id : TokenId, ticket_image : String,
    ticket_type : Option<TicketType>,
    ref_hash : Option<String>) {
        
        let coll =  self.collections.get(&collection_id.clone());
        if coll.is_none () {
            env::panic_str(format!("Collection {:?} not found",collection_id.clone()).as_str());
        }

        let uw_coll = coll.unwrap();

        let tprice = Self::obtain_ticket_price_in_near(uw_coll.ticket_types, ticket_type);

        if env::attached_deposit() < tprice {
            env::panic_str(format!("Attached deposit {} is less than ticket price {}",env::attached_deposit(),tprice).as_str());
        }

        let token_meta = Self::create_token_metadata(
            format!("Ticket {}", token_id),
            uw_coll.title,Some(ticket_image), 
            ref_hash, 
            None);

        nft_contract::ext(uw_coll.contract_id.unwrap())
        .with_static_gas(Gas(5*TGAS))
        .nft_mint(token_id.clone(), mint_by.clone(), token_meta)
        .then( 
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(1*TGAS))
            .after_mint_and_pay_owner_callback(collection_id, tprice, token_id, mint_by)
        );


    }
}

#[near_bindgen]
impl Contract {

    #[private] // Public - but only callable by env::current_account_id()
    pub fn after_mint_and_pay_owner_callback(&mut self, collection_id : CollectionId, 
        ticket_price : u128, token_id : TokenId, mint_by : AccountId ){
        
        Promise::new(collection_id.clone().owner).transfer(ticket_price).as_return();

        // record ticket sales
        if self.ticket_mints_contract_id.is_some() {

            ticket_mints_record::ext(self.ticket_mints_contract_id.clone().unwrap())
            .with_static_gas(Gas(5*TGAS))
            .insert_ticket_mint(collection_id, token_id,mint_by).as_return();
    
        }
      
    }
}

#[near_bindgen]
impl Contract {

    fn obtain_ticket_price_in_near(ticket_types : Option<Vec<TicketType>>, ticket_type : Option<TicketType>) -> u128{

        let mut ticket_price : u128 = 0;
        
        if ticket_type.is_some() {

            if ticket_types.is_some() {

                let tt = ticket_type.unwrap();
                if !ticket_types.unwrap().contains(&tt) {
                    env::panic_str(format!("Invalid ticket type {:?}",tt).as_str());
                }

                ticket_price =  (((tt.price as f64) / 1000.00) * (NEAR as f64)) as u128;

            }
            else {
                env::panic_str("Invalid ticket type!");
            }
        }
        else {


        }


        return ticket_price;
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
