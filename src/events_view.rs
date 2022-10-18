use crate::*;

#[near_bindgen]
impl Contract {


    pub fn get_all_events(&self,  offset : Option<usize>, limit : Option<usize>) -> Vec<Event>{

        self.events.values_as_vector().iter()
        .skip(offset.unwrap_or(0))
        .take(limit.unwrap_or(10))
        .collect::<Vec<Event>>()

    }

    pub fn get_events_of (&self, account_id : AccountId,
        offset : Option<usize>, limit : Option<usize>) -> Vec<Event>{

        self.events.values_as_vector().iter()
        .filter(|c| c.owner == account_id)
        .skip(offset.unwrap_or(0))
        .take(limit.unwrap_or(10))
        .collect::<Vec<Event>>()
    }

 
}
