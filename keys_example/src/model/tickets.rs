use crate::*;

use serde::{ Deserialize, Serialize };
use std::ops::
{
  RangeBounds,
};

#[ derive( Debug, Clone, Serialize ) ]
pub struct Ticket
{
  pub id : u64,
  pub title : String,
}

#[ derive( Debug, Clone, Deserialize ) ]
pub struct TicketPrototype
{
  pub title : String,
}

#[ derive( Debug, Clone, Default ) ]
pub struct Control
{
  store : Arc< RwLock< Vec< Option< Ticket > > > >,
}

impl Control
{
  pub async fn new() -> Result< Self >
  {
    Ok( Self::default() )
  }
}

// CRUD
impl Control
{

  pub async fn ticket_new( &self, proto : TicketPrototype ) -> Result< Ticket >
  {
    let mut store = self.store.write().unwrap();
    let id = store.len() as u64;
    let title = proto.title;
    let ticket = Ticket
    {
      id,
      title,
    };
    store.push( Some( ticket.clone() ) );

    Ok( ticket )
  }

  pub async fn ticket( &self, id : u64 ) -> Result< Ticket >
  {
    let store = self.store.read().unwrap();
    let ticket : Option< &Option< Ticket > > = store.get( id as usize );

    ticket
    .and_then( | t | t.as_ref() )
    .cloned()
    .ok_or( Error::TicketDoesNotExist( id ) )
  }

  pub async fn tickets_list< R >
  (
    &self,
    range : R
  )
  -> Result< Vec< Ticket > >
  where
    R : RangeBounds< u64 >,
  {
    let store = self.store.read().unwrap();
    let start = match range.start_bound()
    {
      std::ops::Bound::Included( &start ) => start,
      std::ops::Bound::Excluded( &start ) => start + 1,
      std::ops::Bound::Unbounded => 0,
    };
    let end = match range.end_bound()
    {
      std::ops::Bound::Included( &end ) => end + 1,
      std::ops::Bound::Excluded( &end ) => end,
      std::ops::Bound::Unbounded => store.len() as u64,
    };

    let tickets : Vec< Ticket > = store.iter()
    .skip( start as usize )
    .take( ( end - start ) as usize )
    .filter_map( |ticket| ticket.clone() )
    .collect();

    Ok( tickets )
  }

  pub async fn ticket_delete
  (
    &self,
    id : u64,
  )
  -> Result< Ticket >
  {
    let mut store = self.store.write().unwrap();
    let ticket = store.get_mut( id as usize ).and_then( | e | e.take() );
    ticket.ok_or( Error::TicketDoesNotExist( id ) )
  }

}
