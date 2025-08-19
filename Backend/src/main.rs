use actix_cors::Cors;
use actix_web::{post, web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use openssl::symm::{encrypt, Cipher};
use sha2::{Sha256, Digest};
use base64::{engine::general_purpose, Engine as _};
use hex;
use dotenv::dotenv;
use std::env;


#[derive(Deserialize)]
struct PaymentRequest {
    order_number: String,
    amount: f64,
    success_url: String,
    failure_url: String,
}

#[derive(Serialize)]
struct PaymentResponse {
    post_url: String,
    me_id: String,
    merchant_request: String,
    hash: String,
}

/// AES-256-CBC encryption with PKCS7 padding (handled by OpenSSL), then Base64 encode
fn encrypt_aes256(plaintext: &str, key: &[u8], iv: &[u8]) -> String {
    let cipher = Cipher::aes_256_cbc();
    let ciphertext = encrypt(cipher, key, Some(iv), plaintext.as_bytes())
        .expect("Encryption failed");
    general_purpose::STANDARD.encode(ciphertext)
}

/// Base64-decode key
fn decode_base64(key: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(key.trim()).expect("Invalid base64 key")
}

/// Format amount : integer if whole, else two decimals
fn format_amount(amount: f64) -> String {
    if amount.fract() == 0.0 {
        format!("{:.0}", amount)
    } else {
        format!("{:.2}", amount)
    }
}

/// Generate encrypted merchant_request
fn generate_merchant_request(
    me_id: &str,
    payload: &PaymentRequest,
    key_bytes: &[u8],
    iv: &[u8],
) -> String {
    let amount_formatted = format_amount(payload.amount);

    // Build each section 
    let txn_details = format!(
        "yagout|{}|{}|{}|ETH|ETB|SALE|{}|{}|WEB",
        me_id, payload.order_number, amount_formatted, payload.success_url, payload.failure_url
    );
    let pg_details = "|||";
    let card_details = "||||";
    let cust_details = "||||Y"; 
    let bill_details = "||||";
    let ship_details = "||||||";
    let item_details = "||";
    let upi_details = "";
    let other_details = "||||";

    let all_sections = vec![
        txn_details,
        pg_details.to_string(),
        card_details.to_string(),
        cust_details.to_string(),
        bill_details.to_string(),
        ship_details.to_string(),
        item_details.to_string(),
        upi_details.to_string(),
        other_details.to_string(),
    ].join("~");

    encrypt_aes256(&all_sections, key_bytes, iv)
}

/// Generate encrypted SHA-256 hash
fn generate_encrypted_hash(
    me_id: &str,
    order_number: &str,
    amount: f64,
    key_bytes: &[u8],
    iv: &[u8],
) -> String {
    let amount_str = format_amount(amount);
    let hash_input = format!("{}~{}~{}~ETH~ETB", me_id, order_number, amount_str);
    
    let mut hasher = Sha256::new();
    hasher.update(hash_input.as_bytes());
    let hash_result = hasher.finalize();
    let hash_hex = hex::encode(hash_result);

    encrypt_aes256(&hash_hex, key_bytes, iv)
}

#[post("/create_payment")]
async fn create_payment(
    payload: web::Json<PaymentRequest>,
    data: web::Data<(String, Vec<u8>, [u8;16])>
) -> impl Responder {
    let (me_id, key_bytes, iv) = data.get_ref();

    let merchant_request_encrypted = generate_merchant_request(me_id, &payload, key_bytes, iv);
    let hash_encrypted = generate_encrypted_hash(
        me_id,
        &payload.order_number,
        payload.amount,
        key_bytes,
        iv,
    );

    let response = PaymentResponse {
        post_url: "https://uatcheckout.yagoutpay.com/ms-transaction-core-1-0/paymentRedirection/checksumGatewayPage".to_string(),
        me_id: me_id.clone(),
        merchant_request: merchant_request_encrypted,
        hash: hash_encrypted,
    };

    HttpResponse::Ok().json(response)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let me_id = env::var("MERCHANT_ID").expect("MERCHANT_ID must be set in .env");
    let key_b64 = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set in .env");

    let key_bytes = decode_base64(&key_b64);
    let iv: [u8; 16] = *b"0123456789abcdef";

    let bind_address = "127.0.0.1:8000";
    println!("Server running at http://{}/", bind_address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new((me_id.clone(), key_bytes.clone(), iv)))
            .wrap(cors)
            .service(create_payment)
    })
    .bind(bind_address)?
    .run()
    .await
}
