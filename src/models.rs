use crate::*;

#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Collection{

    pub title : String, 

    pub symbol : String,

    pub description : Option<String>, 

    // the icon/logo of the collection
    pub icon : Option<String>,

    pub base_uri : Option<String>,
    
    pub contract_id : Option<AccountId>,

    pub ticket_types : Option<Vec<TicketType>>,

    pub total_tickets : Option<u64>,
    
    pub tickets_sold : Option<u64>,

    pub attributes : Option<Vec<Attribute>>,

    pub ticket_template_type : Option<TicketTemplate>,

    pub category : Option<String>,

    pub owner : AccountId, 

    pub date_updated : Option<u64>, 

}


#[derive(Debug, Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum TicketTemplateType {

    Fixed,

    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TicketTemplate {

    pub value : String, 

    pub template_type : TicketTemplateType,

}


#[derive(Debug, Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum AttributeType {

    StartDate,

    EndDate, 

    StartTime,

    EndTime, 

    MaxTicketPerWallet,
}



#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Attribute{

    pub name : AttributeType,

    pub value : String, 
}



#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug,Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CollectionId {

    pub title : String, 

    pub symbol : String, 
    
    pub owner : AccountId, 

}

#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TicketType{

    pub ticket_type : String,

    // ticket price is stored as u32 
    // the stored value is always divided by 1000
    // e.g. 3.2 Near token is stored as 3200 
    pub price : u32, 

    pub color_code : Option<String>,
}

impl PartialEq for TicketType {

    fn eq(&self, other: &Self) -> bool {
        self.ticket_type == other.ticket_type  
    }
}
