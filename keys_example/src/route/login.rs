use crate::*;

#[ derive( Debug, Deserialize ) ]
pub struct LoginData
{
  username : String,
  password : String,
}

pub fn router() -> ax::Router
{
  ax::Router::new().route( "/api/login", ax::post( login ) )
}

#[ axum::debug_handler ]
pub async fn login
(
  cookies : tower_cookies::Cookies,
  data : ax::Json< LoginData >,
) -> Result< ax::Json< serde_json::Value > >
{
  println!( " - {:<20} - {:<20} - {data:?}", "ENDPOINT", "login" );

  if data.username != "username" || data.password != "password"
  {
    return Err( Error::LoginFail )
  }

  cookies.add( tower_cookies::Cookie::new( AUTH_TOKEN_NAME, "usrx.exp.sign" ) );

  let body = ax::Json
  (
    serde_json::json!
    ({
      "result" : { "success" : true },
    })
  );

  Ok( body )
}
