use futures_util::{SinkExt, StreamExt, stream};
use tokio::{ net::{TcpListener, TcpStream}};
use tokio_tungstenite::{accept_async};


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").await.expect("System could not bind Address");
    println!("Awaiting Connection from Client");

    // == Loop forever 
    loop {
        match listener.accept().await{
            Ok((socket, addr)) => {
                println!("Socket recivied: {:?}", addr);
                tokio::task::spawn(handle(socket));
            },
            Err(_) => {}
        };
    }
}


// == Handle for Client
async fn handle (stream : TcpStream){
    // == build a ws from the stream

    match accept_async(stream).await{
        // == IF OK we split into streams and read data from the websocket
        Ok(ws) => {
            let (mut tx, mut rx) = ws.split();

            // == Loop for messages received
            while let Some(Ok(msg)) = rx.next().await{
                if msg.is_text() || msg.is_binary(){
                    let _ = tx.send(msg).await;
                }
            }
        },
        Err(e) => {println!("Error occured during socket accepting: {:?}", e)}
    }
}



