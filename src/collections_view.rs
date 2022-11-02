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


#[near_bindgen]
impl Contract {

    pub fn get_next_ticket_number (&mut self, collection_id : CollectionId,
    width : usize) -> Option<String> {

        let coll = self.collections.get(&collection_id);

        let mut next_ticket_no = String::from("01");

        if coll.is_some() {

            let mut uw_collection = coll.unwrap();

            if uw_collection.attributes.is_none() {
            
                let attbs : Vec<Attribute> = vec![Attribute{name : AttributeType::NextTicketNumber,
                value : "1".to_string()}];

                next_ticket_no = format!("{:0width$}", "1", width = width);
            
                uw_collection.attributes = Some(attbs);
            }
            else {

                let mut uw_attribs = uw_collection.attributes.unwrap();

                let attrb = Attribute{name : AttributeType::NextTicketNumber,
                    value : "1".to_string()};

                let index = uw_attribs.iter().position(|a| *a == attrb);
                if index.is_some() {

                    let mut a = uw_attribs[index.unwrap()].clone();
                    let mut current_no : u32 = a.value.parse::<u32>().expect("Failed to parse into interger");

                    current_no+=1;

                    a.value = format!("{}", current_no);
                  
                    next_ticket_no = format!("{:0width$}", a.value, width = width);
            
                    uw_attribs[index.unwrap()] = a;
                }
                else {

                    uw_attribs.push(attrb);
                    next_ticket_no = format!("{:0width$}", "1", width = width);
            
                }

                uw_collection.attributes = Some(uw_attribs);
          
            }

            self.collections.remove(&collection_id);
            self.collections.insert(&collection_id, &uw_collection);
    
            self.date_updated = Some(env::block_timestamp());
        

        }

        return Some(next_ticket_no);

    }
 
}

