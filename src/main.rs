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
    let mut server_cert_file = BufReader::new(File::open("/home/opc/hyperspire/ssl/key.pem").unwrap());
    let mut chain_cert_file = BufReader::new(File::open("/home/opc/hyperspire/ssl/cert.pem").unwrap());
    let mut key_file = BufReader::new(File::open("/home/opc/hyperspire/ssl/hyperspire.key").unwrap());

    let mut cert_chain: Vec<CertificateDer> = rustls_pemfile::certs(&mut server_cert_file)
        .map(|result| result.unwrap())
        .collect();
    let mut intermediates: Vec<CertificateDer> = rustls_pemfile::certs(&mut chain_cert_file)
        .map(|result| result.unwrap())
        .collect();
    cert_chain.append(&mut intermediates);

    let key = rustls_pemfile::private_key(&mut key_file).unwrap().unwrap();

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .unwrap();

    println!("Starting HTTPS server at https://localhost:443");

    HttpServer::new(|| {
        App::new().service(
            Files::new("/", "./webroot")
                .index_file("index.html")
                .use_last_modified(true),
        )
    })
    .bind_rustls_0_23(("0.0.0.0", 443), config)?
    .run()
    .await
}
