use info::{LInfo, RInfo, info_client::InfoClient};

pub mod info {
  tonic::include_proto!("info");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut c = InfoClient::connect("http://[::1]:50501").await?;
    let req = tonic::Request::new(LInfo{i1 : "client_req1".into()});
    let resp = c.get_i(req).await?;
    println!("Got resp -> {}", resp.into_inner().r1);
    Ok(())
}