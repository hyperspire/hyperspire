use actix_files::Files;
use actix_web::{App, HttpServer};
use rustls::{
    pki_types::CertificateDer,
    ServerConfig,
};
use std::fs::File;
use std::io::BufReader;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut cert_file = BufReader::new(File::open("cert.pem").unwrap());
    let mut key_file = BufReader::new(File::open("key.pem").unwrap());

    let cert_chain: Vec<CertificateDer> = rustls_pemfile::certs(&mut cert_file)
        .map(|result| result.unwrap())
        .collect();
    let key = rustls_pemfile::private_key(&mut key_file).unwrap().unwrap();

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .unwrap();

    println!("Starting HTTPS server at https://localhost:8443");

    HttpServer::new(|| {
        App::new().service(
            Files::new("/", "./webroot")
                .index_file("index.html")
                .use_last_modified(true),
        )
    })
    .bind_rustls_0_23(("0.0.0.0", 8443), config)?
    .run()
    .await
}
