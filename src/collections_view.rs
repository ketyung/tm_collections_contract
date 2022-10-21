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
 
}
