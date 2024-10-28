use crate::*;

// #[ derive( Debug, Clone, ax::FromRef ) ]
// pub struct RouteState
// {
//   control : model::tickets::Control,
// }

pub fn router( control : model::tickets::Control ) -> ax::Router
{
  // let state = RouteState { control };
  ax::Router::new()
  .route( "/tickets", ax::get( tickets_list ).post( ticket_new ) )
  .route( "/tickets/:id", ax::get( ticket ).delete( ticket_delete ) )
  .with_state( control )
  .route_layer( ax::middleware::from_fn( route::auth::require ) )
}

#[ axum::debug_handler ]
pub async fn tickets_list
(
  ax::State( control ) : ax::State< model::tickets::Control >,
)
-> Result< ax::Json< Vec< model::tickets::Ticket > > >
{
  println!( " - {:<20} - {:<20}", "ENDPOINT", "tickets_list" );
  let tickets = control.tickets_list( .. ).await?;
  Ok( ax::Json( tickets ) )
}

#[ axum::debug_handler ]
pub async fn ticket_new
(
  ax::State( control ) : ax::State< model::tickets::Control >,
  ax::Json( prototype ) : ax::Json< model::tickets::TicketPrototype >,
)
-> Result< ax::Json< model::tickets::Ticket > >
{
  println!( " - {:<20} - {:<20}", "ENDPOINT", "ticket_new" );
  let ticket = control.ticket_new( prototype ).await?;
  Ok( ax::Json( ticket ) )
}

#[ axum::debug_handler ]
pub async fn ticket
(
  ax::State( control ) : ax::State< model::tickets::Control >,
  ax::Path( id ) : ax::Path< u64 >,
)
-> Result< ax::Json< model::tickets::Ticket > >
{
  println!( " - {:<20} - {:<20} - {:<20}", "ENDPOINT", "ticket", id );
  let ticket = control.ticket( id ).await?;
  Ok( ax::Json( ticket ) )
}

#[ axum::debug_handler ]
pub async fn ticket_delete
(
  ax::State( control ) : ax::State< model::tickets::Control >,
  ax::Path( id ) : ax::Path< u64 >,
)
-> Result< ax::Json< model::tickets::Ticket > >
{
  println!( " - {:<20} - {:<20} - {:<20}", "ENDPOINT", "ticket_delete", id );
  let ticket = control.ticket_delete( id ).await?;
  Ok( ax::Json( ticket ) )
}
