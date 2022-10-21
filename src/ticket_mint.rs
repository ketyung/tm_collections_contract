use crate::*;
use crate::ext::*;

#[near_bindgen]
impl Contract {


    pub fn ticket_mint (&self, 
    mint_by : AccountId,     
    collection_id : CollectionId, 
    token_id : TokenId, ticket_image : String) -> bool {
        
        let coll =  self.collections.get(&collection_id);
        if coll.is_none () {
            return false;
        }

        let uw_coll = coll.unwrap();

        let token_meta = Self::create_token_metadata(
            format!("Ticket {}", token_id),
            uw_coll.title,Some(ticket_image), None);

        nft_contract::ext(uw_coll.contract_id.unwrap())
        .with_static_gas(Gas(5*TGAS))
        .nft_mint(token_id, mint_by, token_meta).as_return();

        

        true 

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