
#[ tokio::main ]
async fn main() -> app::Result< () >
{
  app::run_with_env().await
}
