# tonic_catch

catch any tonic await error convert into Status

for issue https://github.com/hyperium/tonic/discussions/716#discussioncomment-991404


use example:

```rust
use crate::var::evt::Evt;
use async_std::channel::Sender;
use tonic_catch::{tonic_catch, Result, Error};
use log::info;
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response};
pub mod proto {
  tonic::include_proto!("proto");
}

use proto::rmw_server::{Rmw, RmwServer};
use proto::Url;

pub struct RmwSrv {
  sender: Sender<Evt>,
}

#[tonic_catch]
impl Rmw for RmwSrv {
  async fn head(&self, request: Request<Url>) -> Result<()> {
    let addr = match request.remote_addr() {
      Some(addr) => addr.to_string(),
      None => String::new(),
    };

    let url = request.into_inner();
    println!("{:?} {}/{}", addr, url.addr, url.path);

    self
      .sender
      .send(Evt::Head(url.addr.parse()?, url.path))
      .await?;

    Ok(Response::new(()))
  }
}

pub async fn run(addr: SocketAddr, sender: Sender<Evt>) -> anyhow::Result<()> {
  let rmw = RmwServer::new(RmwSrv { sender });

  info!("grpc://{}", addr);

  Server::builder()
    .accept_http1(true)
    .add_service(tonic_web::enable(rmw))
    .serve(addr)
    .await?;

  Ok(())
}
```




## About

This project is part of the **[rmw.link](//rmw.link)** Code Project .

## 关于

本项目隶属于**人民网络([rmw.link](//rmw.link))** 代码计划。

![人民网络](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)
