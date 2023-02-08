use std::path::{Path};
use std::str::FromStr;
use std::sync::Arc;
use certify::CertSigAlgo;

use log::*;
use third_wheel::{CertificateAuthority, mitm_layer, MitmProxy, ThirdWheel};
use third_wheel::hyper::{Body, Request, Uri};
use third_wheel::hyper::service::Service;

#[tokio::main]
async fn main() {
    env_logger::init();

    let username = Arc::new(std::env::var("JB_USER").expect("JB_USER isn't set! please set it to the username to mimic!"));

    if !Path::new("ca/cert").exists() || !Path::new("ca/key").exists() {
        let (cert_pem, key_pem) = certify::generate_ca(
            "US",
            "Huskitopian Bowling Society",
            "deeznuts.org",
            CertSigAlgo::EcDsa,
            None,
            None,
        ).expect("failed to generate ca cert/key");
        std::fs::create_dir_all("ca/").expect("failed to create \"ca/\" directory!");
        std::fs::write("ca/cert", cert_pem).expect("failed to write cert file!");
        std::fs::write("ca/key", key_pem).expect("failed to write key file!");
    }

    let ca = CertificateAuthority::load_from_pem_files(
        "ca/cert",
            "ca/key"
    ).expect("failed to load pem files from ./ca");

    let modifying_layer = mitm_layer(move |mut req: Request<Body>, mut tw: ThirdWheel| {
        let username = username.clone();
        let fut = async move {
            let mut content = req.uri().to_string();
            if let Some(replacement_range) = {
                if let Some(starting_index) =
                    search_for_string_and_return_ending_index("&username=", content.as_bytes()) {
                    let ending_index = find_next_from_index(b'&', starting_index, content.as_bytes())
                        .unwrap_or(content.len() + 1);
                    Some(starting_index + 1..ending_index)
                } else {
                    None
                }
            } {
                warn!("replacing {} in uri with {}", &content[replacement_range.clone()], username);
                content.replace_range(replacement_range, username.as_str());
                *req.uri_mut() = Uri::from_str(content.as_str()).unwrap();
                debug!("final uri: {}", content);
            };

            tw.call(req).await
        };

        Box::pin(fut)
    });

    let proxy = MitmProxy::builder(modifying_layer, ca).build();
    let host = std::env::var("JB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("JB_PORT").unwrap_or_else(|_| "6969".to_string());
    let (addr, proxy_fut) = proxy.bind(format!("{}:{}", host, port).parse().unwrap());
    println!("serving proxy on {}", addr);
    proxy_fut.await.unwrap();
}

fn search_for_string_and_return_ending_index(string: &str, input: &[u8]) -> Option<usize> {
    let mut cur = 0;
    for (i, c) in input.iter().enumerate() {
        if string.chars().nth(cur).unwrap() as u8 == c.to_ascii_lowercase() {
            cur += 1;
        } else {
            cur = 0;
        }
        if cur >= string.len() {
            return Some(i);
        }
    }
    None
}

fn find_next_from_index(byte: u8, index: usize, input: &[u8]) -> Option<usize> {
    (index..input.len()).find(|&i| input[i] == byte)
}