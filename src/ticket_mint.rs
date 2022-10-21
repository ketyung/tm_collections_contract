use crate::*;
use crate::ext::*;

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn ticket_mint (&mut self, 
    mint_by : AccountId,     
    collection_id : CollectionId, 
    token_id : TokenId, ticket_image : String, 
    ticket_type : Option<TicketType>) {
        
        let coll =  self.collections.get(&collection_id);
        if coll.is_none () {
            env::panic_str(format!("Collection {:?} not found",collection_id).as_str());
        }

        let uw_coll = coll.unwrap();

        let tprice = Self::obtain_ticket_price_in_near(uw_coll.ticket_types, ticket_type);

        if env::attached_deposit() < tprice {
            env::panic_str(format!("Attached deposit {} is less than ticket price {}",env::attached_deposit(),tprice).as_str());
        }
        
        let token_meta = Self::create_token_metadata(
            format!("Ticket {}", token_id),
            uw_coll.title,Some(ticket_image), None);

        nft_contract::ext(uw_coll.contract_id.unwrap())
        .with_static_gas(Gas(5*TGAS))
        .nft_mint(token_id, mint_by, token_meta).as_return();

    }


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


        return ticket_price;
    }


    fn create_token_metadata(ticket_title : String, collection_title : String, 
        media : Option<String>, extra : Option<String>) -> TokenMetadata{

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
            reference_hash: None,
        }
    }

}