
use std::
{
  net::SocketAddr,
  sync::{ Arc, RwLock },
};

pub use serde::{ Deserialize };
pub use error_tools::thiserror;

pub mod route;
pub mod model;

pub mod ax
{
  pub use axum::
  {
    *,
    extract::*,
    routing::*,
    response::*,
    middleware::{ Next },
  };
}

use error_tools::typed::Error;

#[ derive( Debug, Error ) ]
pub enum Error
{

  #[ error( "IO error" )]
  IoError( #[ from ] std::io::Error ),

  #[ error( "{0:?}" ) ]
  AddrParseError( #[ from ] std::net::AddrParseError ),

  #[ error( "Login error" ) ]
  LoginFail,

  #[ error( "Authorization error: not auth token" ) ]
  AuthErrorNoAuthToken,

  #[ error( "Ticket {0} does not exist" ) ]
  TicketDoesNotExist( u64 ),

}

impl ax::IntoResponse for Error
{
  fn into_response( self ) -> ax::Response
  {
    println!( " - {:<20} - {self:<20}", "INTO_RESPONSE" );
    ( ax::http::StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR" ).into_response()
  }
}

pub type Result< R > = core::result::Result< R, Error >;

pub const AUTH_TOKEN_NAME : &str = "auth-token";

// use sqlx::
// {
//   Row,
// };

pub struct Config
{
  pub database_url : String,
  pub server_addr : SocketAddr,
}

impl Config
{

  pub async fn run( &self ) -> Result< () >
  {

    let router = ax::Router::new()
    .merge( route::hello::router() )
    .merge( route::login::router() )
    .nest( "/api", route::tickets::router( model::tickets::Control::default() ) )
    .layer( ax::middleware::map_response( response_mapper ) )
    .layer( tower_cookies::CookieManagerLayer::new() )
    .fallback_service( static_routes() )
    ;

    println!( "Listenning on {}", self.server_addr );
    let listener = tokio::net::TcpListener::bind( self.server_addr ).await.unwrap();
    ax::serve( listener, router ).await?;

    Ok( () )
  }

}

pub async fn response_mapper( r : ax::Response ) -> ax::Response
{
  println!( " - {:<20} - {:<20}", "MAPPER", "MAIN" );
  println!();
  r
}

pub async fn run_with_env() -> Result< () >
{

  dotenv::from_filename( "./key/-env.sh" ).ok();
  let database_url = std::env::var( "DATABASE_URL" ).expect( "DATABASE_URL must be set" );
  let server_addr : SocketAddr = std::env::var( "SERVER_ADDR" ).expect( "SERVER_ADDR must be set" ).parse()?;
  let config = Config { database_url, server_addr };

  config.run().await
}

pub fn static_routes() -> ax::Router
{
  ax::Router::new()
  .nest_service( "/file", ax::get_service( tower_http::services::ServeDir::new( "./file" ) ) )
}
