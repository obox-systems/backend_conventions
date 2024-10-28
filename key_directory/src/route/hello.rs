use crate::*;

pub fn router() -> ax::Router
{

  ax::Router::new()
  .route
  (
    "/hello_with_params",
    ax::get( hello_with_params ),
  )
  .route
  (
    "/hello_with_path/:name",
    ax::get( hello_with_path ),
  )

}

#[ derive( Debug, Deserialize ) ]
pub struct HelloParams
{
  name : Option< String >,
}

// /hello?name=John
async fn hello_with_params( ax::Query( params ) : ax::Query< HelloParams > ) -> impl axum::response::IntoResponse
{
  println!( " - {:<20} - {:<20} - {params:?}", "ENDPOINT", "hello_with_params" );
  let name = params.name.as_deref().unwrap_or( "there" );
  axum::response::Html( format!( "Hello {name}!" ) )
}

// /hello/John
async fn hello_with_path( ax::Path( name ) : ax::Path< String > ) -> impl axum::response::IntoResponse
{
  println!( " - {:<20} - {:<20} - {name:?}", "ENDPOINT", "hello_with_params" );
  axum::response::Html( format!( "Hello {name}!" ) )
}
