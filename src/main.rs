use actix_files::Files;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use rustls::{
    pki_types::CertificateDer,
    ServerConfig,
};
use std::fs::File;
use std::io::BufReader;
use std::env;

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

    // println!("Starting HTTPS server at https://localhost:8443");

    HttpServer::new(|| {
        App::new().service(
            web::resource("/v1/env").route(web::get().to(get_environment))
        )
        .service(
            Files::new("/", "./webroot")
                .index_file("index.html")
                .use_last_modified(true),
        )
    })
    .bind_rustls_0_23(("0.0.0.0", 8443), config)?
    .run()
    .await
}

// Handler function to collect all OS environment variables and display them in HTML.
async fn get_environment(req: HttpRequest) -> impl Responder {
    // 1. Collect all environment variables
    let env_vars: Vec<(String, String)> = env::vars().collect();

    let header_keys: Vec<&actix_web::http::header::HeaderName> = req.headers().keys().collect();

    // 2. Start building the HTML content
    let mut html_content = String::from(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Environment Variables Dump</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background-color: #f4f7f9;
            padding: 20px;
            color: #333;
        }
        .container {
            max-width: 800px;
            margin: 40px auto;
            padding: 30px;
            background-color: #ffffff;
            border-radius: 10px;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
        }
        h1 {
            color: #0056b3;
            border-bottom: 2px solid #e1e1e1;
            padding-bottom: 10px;
            margin-bottom: 20px;
        }
        table {
            width: 100%;
            border-collapse: collapse;
            margin-top: 20px;
        }
        th, td {
            padding: 12px 15px;
            text-align: left;
            border-bottom: 1px solid #ddd;
        }
        th {
            background-color: #007bff;
            color: white;
            font-weight: 600;
        }
        tr:nth-child(even) {
            background-color: #f9f9f9; /* Zebra striping for readability */
        }
        code {
            background-color: #eee;
            padding: 2px 5px;
            border-radius: 3px;
            font-family: monospace;
        }
        /* Styling for long values */
        td:last-child {
            max-width: 90%;
            word-wrap: break-word;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🖥️ System Environment Variables Dump</h1>
        <p>This list shows all environment variables available to the running process.</p>
        <table>
            <thead>
                <tr>
                    <th>Variable Key</th>
                    <th>Value</th>
                </tr>
            </thead>
            <tbody>
    "#);

    // 3. Build the table rows
    for (key, value) in env_vars {
        // Escape HTML special characters just in case
        let key_html = key.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
        let value_html = value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");

        html_content.push_str(&format!(
            r#"
            <tr>
                <td><code>{}</code></td>
                <td>{}</td>
            </tr>
            "#,
            key_html,
            value_html
        ));
    }

    for key in header_keys {
        if let Some(value) = req.headers().get(key) {
            // Escape HTML special characters just in case
            let key_html = key.as_str().replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
            let value_html = value.to_str().unwrap_or("").replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");

            html_content.push_str(&format!(
                r#"
                <tr>
                    <td><code>{}</code></td>
                    <td>{}</td>
                </tr>
                "#,
                key_html,
                value_html
            ));
        }
    }

    // 4. Close the HTML tags
    html_content.push_str(r#"
            </tbody>
        </table>
    </div>
</body>
</html>
"#);

    // 5. Return the complete HTML string as an HTTP response
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content)
}