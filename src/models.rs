use crate::*;


#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Event{

    pub title : String, 

    pub symbol : String,

    pub description : Option<String>, 

    pub contract_id : Option<AccountId>,

    pub ticket_types : Option<Vec<TicketType>>,

    pub total_tickets : Option<u64>,
    
    pub tickets_sold : Option<u64>,

    pub attributes : Option<Vec<EventAttribute>>,

    pub owner : AccountId, 

    pub date_updated : Option<u64>, 

}


#[derive(Debug, Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum EventAttributeType {

    Date,

    StartTime,

    EndTime, 
}




#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EventAttribute{

    pub name : EventAttributeType,

    pub value : String, 
}



#[derive(BorshDeserialize, BorshSerialize)]
pub struct EventId {

    pub title : String, 

    pub symbol : String, 
    
    pub owner : AccountId, 

}


#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TicketType{

    pub ticket_type : String,

    // ticket price is stored as u32 
    // the stored value is always divided by 1000
    // e.g. 3.2 Near token is stored as 3200 
    pub price : u32, 
}



