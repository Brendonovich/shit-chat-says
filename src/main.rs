mod prisma;

use axum::http::Method;
use rspc::Config;
use std::{error::Error, net::SocketAddr, path::PathBuf, sync::Arc};
use twitch_irc::{
    login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureTCPTransport,
    TwitchIRCClient,
};

type Router = rspc::Router<Arc<prisma::PrismaClient>>;

fn create_router() -> Router {
    Router::new()
        .query("allMessages", |db, _: ()| async move {
            Ok(db.message().find_many(vec![]).exec().await?)
        })
        .config(Config::new().export_ts_bindings(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./web/src/rspc/bindings.ts"),
        ))
        .build()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = Arc::new(prisma::new_client().await?);

    dotenv::dotenv().ok();

    let irc_task = tokio::spawn(run_irc(db.clone()));

    let router = create_router();

    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(tower_http::cors::Any);

    let app = axum::Router::new()
        .route(
            "/rspc/:id",
            Arc::new(router).axum_handler(move || db.clone()),
        )
        .layer(cors);


    let axum_task = tokio::spawn(async move {
        let mut addr = "[::]:4001".parse::<SocketAddr>().unwrap(); // This listens on IPv6 and IPv4
        addr.set_port(4001);
        println!("Listening on http://localhost:4001");
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("Error with HTTP server!");
    });

    tokio::join!(irc_task, axum_task);

    Ok(())
}

async fn run_irc(db: Arc<prisma::PrismaClient>) {
    let token = std::env::var("TWITCH_TOKEN").unwrap();

    let config = ClientConfig::new_simple(StaticLoginCredentials::new(
        "brendonovich".to_string(),
        Some(token),
    ));

    let (mut incoming_messages, twitch_client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    twitch_client.join("brendonovich".to_owned()).unwrap();

    while let Some(message) = incoming_messages.recv().await {
        dbg!(&message);
        if let ServerMessage::Privmsg(msg) = message {
            db.message()
                .create(msg.message_text, msg.sender.name, vec![])
                .exec()
                .await
                .unwrap();
        }
    }
}
