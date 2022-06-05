use s2n_quic::Server;
use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cert = fs::read_to_string("/tmp/cert.pem")?;
    let key = fs::read_to_string("/tmp/key.pem")?;
    println!("{} -- {}", cert, key);
    let mut server = Server::builder()
        .with_tls((cert.as_str(), key.as_str()))?
        .with_io("127.0.0.1:4433")?
        .start()?;

    while let Some(mut connection) = server.accept().await {
        // spawn a new task for the connection
        tokio::spawn(async move {
            while let Ok(Some(mut stream)) = connection.accept_bidirectional_stream().await {
                // spawn a new task for the stream
                tokio::spawn(async move {
                    // echo any data back to the stream
                    while let Ok(Some(data)) = stream.receive().await {
                        println!("got some data {:?}", data);
                        stream.send(data).await.expect("stream should be open");
                    }
                });
            }
        });
    }

    Ok(())
}
