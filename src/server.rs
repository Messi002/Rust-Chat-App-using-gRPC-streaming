pub mod pb{
    tonic::include_proto!("chat");
}


use std::net::ToSocketAddrs;
use std::result;
use futures_util::future::ok;
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

#[tonic::async_trait]
impl pb::chat_service_server::ChatService for ChatServer{

    type ChatMessageStreamingStream = RepsonseStream;

    async fn chat_message_streaming(&self, request: Request<Streaming<ChatMessage>>)-> ChatResult<Self::ChatMessageStreamingStream>{
        let mut in_stream = request.into_inner();

        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move{
            while let Some(result) = in_stream.next().await  {
               match result {
                Ok(item) => {
                    println!("Received {:?} from {:?}", item.message, item.from);
                    tx.send(
                        Ok(ChatMessage{
                            message: format!("{:?} Server", item.message),
                            from : String::from("Server side")
                        })
                    ).await.unwrap();
                }

                Err() => {
                    println!("Error: {}", status);
                    break;
                }
               }

            }
        })
    }
}
fn  main() {
    
}