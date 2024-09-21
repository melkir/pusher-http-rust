extern crate hyper;
extern crate pusher;
extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate url;

use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Error, Request, Response};
use hyper_util::rt::TokioIo;
use pusher::PusherBuilder;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use url::form_urlencoded::parse;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await.unwrap();

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await.unwrap();

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(authenticate))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn authenticate(req: Request<Incoming>) -> Result<Response<String>, Error> {
    let pusher = PusherBuilder::from_url("http://key:secret@api.host.com/apps/id").finalize();
    let body = req.into_body().collect().await?.to_bytes();
    let params = parse(body.as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>();
    let channel_name = params.get("channel_name").unwrap();
    let socket_id = params.get("socket_id").unwrap();

    let mut member_data = HashMap::new();
    member_data.insert("twitter", "jamiepatel");
    let member = pusher::Member {
        user_id: "4",
        user_info: Some(member_data),
    };
    let auth_signature = serde_json::to_string(
        &pusher
            .authenticate_presence_channel(channel_name, socket_id, &member)
            .unwrap(),
    )
    .unwrap();

    Ok(Response::new(auth_signature))
}
