mod restparted;

use actix_http::{
	header::HeaderValue, Error, HttpService, Method, Request, Response, ResponseBuilder,
	StatusCode,
};
use actix_server::Server;
use futures_util::StreamExt;
use restparted::parted::command::run_command;
use rustls::ServerConfig;
use serde_json::{json, Value};
use std::{io, str, time::Duration};

#[actix_rt::main]
async fn main() -> io::Result<()> {
	env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

	restparted::initialize();

	let appname = restparted::config::Config::APPNAME.to_string();

	let config = restparted::CONFIG.lock().unwrap();
	let address: String = config.address.clone();
	let port: u16 = config.port;
	let workers: usize = config.max_worker;

	// info!("https://{address}:{port}");

	Server::build()
		.bind(appname, (address, port), || {
			HttpService::build()
				.client_request_timeout(Duration::from_secs(1000))
				.finish(|req: Request| async move {
					let content_type =
						("content-type", HeaderValue::from_static("application/json"));

					let body: Value;
					let status_code: StatusCode;

					loop {
						if req.method() != Method::POST {
							status_code = StatusCode::BAD_REQUEST;
							body = json!({
								"message": "Invalid request.",
							});
							break;
						}

						let (_, mut g) = req.into_parts();
						let mut buf = Vec::new();
						while let Some(Ok(chunk)) = g.next().await {
							buf.extend_from_slice(&chunk);
						}

						let body_json = serde_json::from_str(str::from_utf8(&buf).unwrap());
						if body_json.is_err() {
							status_code = StatusCode::NOT_ACCEPTABLE;
							body = json!({
								"message": "Invalid payload.",
							});
							break;
						}

						let cmd_output = run_command(body_json.unwrap());
						if cmd_output.is_err() {
							status_code = StatusCode::INTERNAL_SERVER_ERROR;
							body = json!({
								"message": "Invalid command",
								"error": cmd_output.err().unwrap().to_string(),
							});
							break;
						}

						status_code = StatusCode::OK;
						body = cmd_output.unwrap();
						break;
					}

					let mut res: ResponseBuilder = Response::build(status_code);
					res.insert_header(content_type);
					Ok::<_, Error>(res.body(body.to_string()))
				})
				.rustls_0_23(tls_config())
		})?
		.workers(workers)
		.run()
		.await
}

fn tls_config() -> ServerConfig {
	let rcgen::CertifiedKey { cert, key_pair } =
		rcgen::generate_simple_self_signed(["localhost".to_owned()]).unwrap();
	let cert_file = cert.pem();
	let key_file = key_pair.serialize_pem();

	let cert_file = &mut io::BufReader::new(cert_file.as_bytes());
	let key_file = &mut io::BufReader::new(key_file.as_bytes());

	let cert_chain = rustls_pemfile::certs(cert_file)
		.collect::<Result<Vec<_>, _>>()
		.unwrap();
	let mut keys = rustls_pemfile::pkcs8_private_keys(key_file)
		.collect::<Result<Vec<_>, _>>()
		.unwrap();

	let mut config = rustls::ServerConfig::builder()
		.with_no_client_auth()
		.with_single_cert(
			cert_chain,
			rustls::pki_types::PrivateKeyDer::Pkcs8(keys.remove(0)),
		)
		.unwrap();

	const H1_ALPN: &[u8] = b"http/1.1";
	const H2_ALPN: &[u8] = b"h2";

	config.alpn_protocols.push(H2_ALPN.to_vec());
	config.alpn_protocols.push(H1_ALPN.to_vec());

	config
}
