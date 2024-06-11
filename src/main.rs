use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::accept_async;
use tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel(100);

    let serveur = "127.0.0.1:8800";
    let listener = TcpListener::bind(serveur).await.expect("Échec de la liaison");

    println!("Le serveur s'est lancé sur {}", serveur);

    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("Échec de l'acceptation");
            println!("Connexion acceptée");

            let (mut ecriture, mut lecture) = ws_stream.split();

            tokio::spawn(async move {
                while let Some(Ok(msg)) = lecture.next().await {
                    println!("Message reçu: {}", msg);
                    tx.send(msg.to_text().unwrap().to_string()).unwrap();
                }
            });

            while let Ok(msg) = rx.recv().await {
                if ecriture.send(Message::text(msg)).await.is_err() {
                    break;
                }
            }
        });
    }
}
