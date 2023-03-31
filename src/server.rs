use tonic::{transport::Server, Request, Response, Status};
use info::{LInfo, RInfo, info_server::{Info, InfoServer}};

pub mod info {
  tonic::include_proto!("info");
}

pub struct InfoService {}

#[tonic::async_trait]
impl Info for InfoService {
    async fn get_i(&self,request:Request<info::LInfo> ,) ->  Result<Response<info::RInfo> ,tonic::Status> {
        Ok(Response::new(RInfo{r1 : format!("Response from server -> {}", request.into_inner().i1)}))
    }
}

#[tokio::main]
async fn main () -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50501".parse().unwrap();
    let iservice = InfoService{};
    Server::builder().add_service(InfoServer::new(iservice)).serve(addr).await?;
    Ok(())
}