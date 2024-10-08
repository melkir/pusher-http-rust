use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client},
    rt::TokioExecutor,
};
use rustls::RootCertStore;

use crate::PusherBuilder;

type Connector = HttpsConnector<HttpConnector>;

fn default_client() -> Client<Connector, String> {
    let root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.into(),
    };
    let client_config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = HttpsConnectorBuilder::new()
        .with_tls_config(client_config)
        .https_only()
        .enable_http1()
        .build();
    Client::builder(TokioExecutor::new()).build(connector)
}

impl PusherBuilder<Connector> {
    /// Initializes the client that makes requests to the HTTP API, authenticates
    /// private- or presence-channels, and validates webhooks.
    ///
    /// This returns a `PusherBuilder`, on which to set configuration options
    /// before calling `finalize()`.
    ///
    /// **Example:**
    ///
    ///
    /// ```
    /// # use pusher::PusherBuilder;
    /// let pusher = PusherBuilder::new("id", "key", "secret").host("foo.bar.com").finalize();
    /// ```
    pub fn new_rustls(app_id: &str, key: &str, secret: &str) -> PusherBuilder<Connector> {
        PusherBuilder::new_with_client(default_client(), app_id, key, secret)
    }

    /// Initializes a client from a Pusher URL.
    ///
    /// This returns a `PusherBuilder`, on which to set configuration options
    /// before calling `finalize()`.
    ///
    /// **Example:**
    ///
    ///
    /// ```
    /// # use pusher::PusherBuilder;
    /// PusherBuilder::from_url("http://key:secret@api.host.com/apps/id").finalize();
    /// ```
    pub fn from_url_rustls(url: &str) -> PusherBuilder<Connector> {
        PusherBuilder::from_url_with_client(default_client(), url)
    }

    /// Initializes a client from an environment variable Pusher URL.
    ///
    /// This returns a `PusherBuilder`, on which to set configuration options
    /// before calling `finalize()`.
    ///
    ///
    /// **Example:**
    ///
    /// ```ignore
    /// # use pusher::Pusher;
    /// Pusher::from_env("PUSHER_URL").finalize();
    /// ```
    pub fn from_env_rustls(key: &str) -> PusherBuilder<Connector> {
        PusherBuilder::from_env_with_client(default_client(), key)
    }
}
