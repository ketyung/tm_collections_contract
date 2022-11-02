use crate::*;

#[near_bindgen]
impl Contract {


    pub fn get_all_collections(&self,  offset : Option<usize>, limit : Option<usize>) -> Vec<Collection>{

        self.collections.values_as_vector().iter()
        .skip(offset.unwrap_or(0))
        .take(limit.unwrap_or(10))
        .collect::<Vec<Collection>>()

    }

    pub fn get_collections_of (&self, account_id : AccountId,
        offset : Option<usize>, limit : Option<usize>) -> Vec<Collection>{

        self.collections.values_as_vector().iter()
        .filter(|c| c.owner == account_id)
        .skip(offset.unwrap_or(0))
        .take(limit.unwrap_or(10))
        .collect::<Vec<Collection>>()
    }


    pub fn get_collections_by (&self, category : String,
        offset : Option<usize>, limit : Option<usize>) -> Vec<Collection>{

        self.collections.values_as_vector().iter()
        .filter(|c| c.category == Some(category.clone()))
        .skip(offset.unwrap_or(0))
        .take(limit.unwrap_or(10))
        .collect::<Vec<Collection>>()
    }


    pub fn get_collection (&self, collection_id : CollectionId) -> Option<Collection> {

        self.collections.get(&collection_id)
    }


    pub fn get_next_ticket_number (&self, collection_id : CollectionId, width : Option<usize>) -> Option<String> {

        let coll = self.collections.get(&collection_id);

        if coll.is_some() {

            let uw_collection = coll.unwrap();
            let attribs = uw_collection.attributes;

            if attribs.is_some() {

                let uw_attribs = attribs.unwrap();
                let attrb = Attribute{name : AttributeType::NextTicketNumber,
                    value : "1".to_string()};

                let index = uw_attribs.iter().position(|a| *a == attrb);
                if index.is_some() {

                    let a = uw_attribs[index.unwrap()].clone();
                   
                    return Some(Self::pad_left_with_zero(a.value.as_str(),width.unwrap_or(5)));
                }


            }

            
        }

        None 
    }

}




