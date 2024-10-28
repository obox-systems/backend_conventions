use error_tools::untyped::Result;
use std::net::SocketAddr;
use serde_json::json;

#[ tokio::test ]
async fn hello_test() -> Result< () >
{

  let current_dir = std::env::current_dir()?.display().to_string();
  println!( "current_dir : {current_dir}" );
  dotenv::from_filename( "./key/-env.sh" ).ok();
  let server_addr : SocketAddr = std::env::var( "SERVER_ADDR" ).expect( "SERVER_ADDR must be set" ).parse()?;
  let full_url = format!( "http://{server_addr}" );
  println!( "full_url : {full_url}" );

  let hc = httpc_test::new_client( full_url )?;

  hc.do_get( "/hello_with_params?name=deb" ).await?.print().await?;
  hc.do_get( "/hello_with_path/deb" ).await?.print().await?;
  hc.do_get( "/file/aws.png" ).await?.print().await?;

  hc.do_post( "/api/login", json!({ "username" : "username", "password" : "password" }) ).await?.print().await?;

  hc.do_post( "/api/tickets", json!({ "title" : "Title1" }) ).await?.print().await?;
  hc.do_post( "/api/tickets", json!({ "title" : "Title2" }) ).await?.print().await?;
  hc.do_get( "/api/tickets" ).await?.print().await?;
  hc.do_get( "/api/tickets/1" ).await?.print().await?;
  hc.do_delete( "/api/tickets/1" ).await?.print().await?;
  hc.do_get( "/api/tickets/1" ).await?.print().await?;
  hc.do_get( "/api/tickets" ).await?.print().await?;

  Ok( () )
}
