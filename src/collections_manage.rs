use crate::*;


#[near_bindgen]
impl Contract {

    pub (crate) fn panic_if_its_not_allowed_caller(&self) {

        let uw_allowed_callers = self.allowed_callers.clone().unwrap();

        if !uw_allowed_callers.contains(&env::predecessor_account_id()) {
            env::panic_str(format!("@{} Error : Caller {} is NOT allowed",
            env::current_account_id(),
            env::predecessor_account_id()).as_str());
        }
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

        // use allowed caller check
        self.panic_if_its_not_allowed_caller();

        self.internal_create_collection(
        acc_id, title, symbol, icon, 
        base_uri, description, category, 
        total_tickets,tickets_sold,
        ticket_types, attributes, 
        template_type, contract_id);

        self.date_updated = Some(env::block_timestamp());
    
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

    pub fn update_collection (&mut self, 
        collection_id : CollectionId,
        update_collection_data : crate::models::CollectionDataForUpdate) {

        self.panic_if_its_not_allowed_caller();

        let collection = self.collections.get(&collection_id);

        if collection.is_none() {
            env::panic_str(format!("The collection {} for {} does NOT exist",
            collection_id.title,collection_id.owner).as_str())
        }

        let mut uw_collection = collection.unwrap();

        // only update when the specified property is not none 
        if update_collection_data.icon.is_some() {
            uw_collection.icon = update_collection_data.icon;
        }

        if update_collection_data.description.is_some() {
            uw_collection.description = update_collection_data.description;
        }

        if update_collection_data.ticket_types.is_some() {
            uw_collection.ticket_types = update_collection_data.ticket_types;
        }

        if update_collection_data.total_tickets.is_some() {
            uw_collection.total_tickets = update_collection_data.total_tickets;
        }

        if update_collection_data.attributes.is_some() {

            if uw_collection.attributes.is_none() {

                uw_collection.attributes = update_collection_data.attributes;
            }
            else {

                let mut uw_attribs = uw_collection.attributes.clone().unwrap();
                let uw_upd_attribs = update_collection_data.attributes.unwrap();
                for attrb in uw_upd_attribs {

                    if !uw_attribs.contains(&attrb){
                        uw_attribs.push(attrb);
                    }
                    else {

                        let index = uw_attribs.iter().position(|a| *a == attrb).unwrap();
                        uw_attribs[index] = attrb;
                    }
                }

                uw_collection.attributes = Some(uw_attribs);

            }
        }

        if update_collection_data.ticket_template_type.is_some() {
            uw_collection.ticket_template_type = update_collection_data.ticket_template_type;
        }

        if update_collection_data.category.is_some() {
            uw_collection.category = update_collection_data.category;
        }

        uw_collection.date_updated = Some(env::block_timestamp());
        
        self.collections.remove(&collection_id);
        self.collections.insert(&collection_id, &uw_collection);

        self.date_updated = Some(env::block_timestamp());
    
    
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


    pub fn remove_collection_attributes(&mut self,  collection_id : CollectionId) {

        if env::signer_account_id() != env::current_account_id() {

            env::panic_str(format!("{} is unauthorized for removal of the collection's attributes", 
            env::signer_account_id()).as_str())
        }

        let coll = self.collections.get(&collection_id);

        if coll.is_some(){
            let mut uw_coll = coll.unwrap();

            uw_coll.attributes = None;

            self.collections.remove(&collection_id);
            self.collections.insert(&collection_id, &uw_coll);
    
            self.date_updated = Some(env::block_timestamp());
        
        }
    }
}

#[near_bindgen]
impl Contract {


    pub (crate) fn pad_left_with_zero(value : &str, width : usize) -> String {

        format!("{:0>1$}", value, width)
    }

    pub fn gen_next_ticket_number (&mut self, collection_id : CollectionId,
    width : Option<usize>) -> Option<String> {

        self.panic_if_its_not_allowed_caller();

        let coll = self.collections.get(&collection_id);

        let mut a_width : usize = 5 ;

        if width.is_some() {
            a_width = width.unwrap();
        }

        let mut next_ticket_no = String::from("01");

        if coll.is_some() {

            let mut uw_collection = coll.unwrap();

            if uw_collection.attributes.is_none() {
            
                let attbs : Vec<Attribute> = vec![Attribute{name : AttributeType::NextTicketNumber,
                value : "1".to_string()}];

                next_ticket_no = Self::pad_left_with_zero ("1", a_width);
            
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
                  
                    next_ticket_no = Self::pad_left_with_zero(a.value.as_str(), a_width);
            
                    uw_attribs[index.unwrap()] = a;
                }
                else {

                    uw_attribs.push(attrb);
                    next_ticket_no = Self::pad_left_with_zero("1", a_width);
            
                }

                uw_collection.attributes = Some(uw_attribs);
          
            }

            self.collections.remove(&collection_id);
            self.collections.insert(&collection_id, &uw_collection);
    
            self.date_updated = Some(env::block_timestamp());
        

        }

        env::log_str(format!("Next ticket no is {}", next_ticket_no).as_str());

        return Some(next_ticket_no);

    }
 
}