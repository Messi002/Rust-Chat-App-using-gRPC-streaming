pub mod pb{
    tonic::include_proto!("chat");
}


use std::net::ToSocketAddrs;
use tokio::sync::mpsc;
use tokio_stream::{Stream, StreamExt};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Streaming, Request, Response, Status};
use tonic::transport::server;
use std::pin::Pin;
use pb::ChatMessage;



#[derive(Debug, Default)]
pub struct ChatServer {
    messages: mpsc::Sender<ChatMessage>,
}

type RepsonseStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, Status>> + Send>>;

type ChatResult<T> = Result<Response<T>, Status>;


fn  main() {
    
}