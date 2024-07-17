use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
#[allow(unused)] 
use crate::errors::{
    Error, 
    Result
};

pub mod ticket {
    use super::*;

    #[allow(unused)]
    #[derive(Serialize, Debug, Clone)]
    pub struct Ticket {
        pub id: u64,
        pub title: String
    }

    #[allow(unused)]
    #[derive(Deserialize, Debug, Clone)]
    pub struct TicketForCreate {
        pub title: String
    }

    #[derive(Debug, Clone)]
    pub struct ModelConstroller {
        #[allow(unused)]
        ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>
    }

    impl ModelConstroller {
        #[allow(unused)]
        pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
            let mut store = self.ticket_store
                .lock()
                .unwrap();
            let id = store.len() as u64;
            let ticket = Ticket {
                id,
                title: ticket_fc.title,
            };
            store.push(Some(ticket.clone()));
            Ok(ticket)
        }
    }
}
