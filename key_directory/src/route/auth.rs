use crate::*;

pub async fn require
(
  cookies : tower_cookies::Cookies,
  req : ax::Request,
  next : ax::Next,
)
-> Result< ax::Response >
{
  let auth_token = cookies.get( AUTH_TOKEN_NAME ).map( | c | c.value().to_string() );

  auth_token.ok_or( Error::AuthErrorNoAuthToken )?;

  Ok( next.run( req ).await )
}
