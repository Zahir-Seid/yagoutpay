use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, Result};
use aes::Aes256;
use aes::cipher::{KeyIvInit, BlockEncryptMut};
use cbc::Encryptor;
use block_padding::Pkcs7;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use base64::{engine::general_purpose, Engine as _};
use utoipa::OpenApi;
use utoipa::ToSchema;
use utoipa_swagger_ui::SwaggerUi;


type Aes256CbcEnc = Encryptor<Aes256>;

#[derive(Clone)]
struct AppConfig {
    merchant_id: String,
    encryption_key: Vec<u8>,
    post_url: String,
    iv: [u8; 16],
}

#[derive(Deserialize, ToSchema)]
struct PaymentRequest {
    order_number: String,
    amount: f64,
    success_url: String,
    failure_url: String,
    email: String,
    mobile_no: String,
}

#[derive(Serialize, ToSchema)]
struct PaymentResponse {
    post_url: String,
    me_id: String,
    merchant_request: String,
    hash: String,
}

fn encrypt_aes_cbc(plaintext: &str, key: &[u8; 32], iv: &[u8; 16]) -> Vec<u8> {
    let cipher = Aes256CbcEnc::new_from_slices(key, iv).expect("invalid key/iv length");

    // create a mutable buffer from the plaintext
    let mut buf = plaintext.as_bytes().to_vec();

    // pad & encrypt in place
    let ciphertext = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buf, plaintext.len())
        .expect("encryption failed");

    // return as Vec<u8>
    ciphertext.to_vec()
}

fn aes_encrypt(plaintext: &str, key: &[u8], iv: &[u8]) -> String {
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&key[..32]);

    let mut iv_array = [0u8; 16];
    iv_array.copy_from_slice(&iv[..16]);

    let encrypted = encrypt_aes_cbc(plaintext, &key_array, &iv_array);
    general_purpose::STANDARD.encode(&encrypted)
}

#[utoipa::path(
    post,
    path = "/create_payment",
    request_body = PaymentRequest,
    responses(
        (status = 200, description = "Payment created successfully", body = PaymentResponse)
    )
)]
#[post("/create_payment")]
async fn create_payment(
    payload: web::Json<PaymentRequest>,
    config: web::Data<AppConfig>,
) -> Result<impl Responder> {
    let amount_formatted = format!("{:.2}", payload.amount);

    let txn_details = format!(
        "yagout|{}|{}|{}|ETH|ETB|SALE|{}|{}|WEB",
        config.merchant_id,
        payload.order_number,
        amount_formatted,
        payload.success_url,
        payload.failure_url
    );

    let pg_details = "||";
    let card_details = "||||";
    let cust_details = format!("||||{}|{}|0|N", payload.email, payload.mobile_no);
    let bill_details = "||||";
    let ship_details = "|||||";
    let item_details = "||";
    let upi_details = "";
    let other_details = "";

    let all_values = format!(
        "{}~{}~{}~{}~{}~{}~{}~{}~{}",
        txn_details,
        pg_details,
        card_details,
        cust_details,
        bill_details,
        ship_details,
        item_details,
        upi_details,
        other_details
    );

    let merchant_request = aes_encrypt(&all_values, &config.encryption_key, &config.iv);

    let hash_input = format!(
        "{}~{}~{}~ETH~ETB",
        config.merchant_id,
        payload.order_number,
        amount_formatted
    );
    let mut hasher = Sha256::new();
    hasher.update(hash_input);
    let hash_hex = format!("{:x}", hasher.finalize());
    let hash_encrypted = aes_encrypt(&hash_hex, &config.encryption_key, &config.iv);

    let response = PaymentResponse {
        post_url: config.post_url.clone(),
        me_id: config.merchant_id.clone(),
        merchant_request,
        hash: hash_encrypted,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[derive(OpenApi)]
#[openapi(
    paths(create_payment),
    components(schemas(PaymentRequest, PaymentResponse)),
    tags(
        (name = "payment", description = "Payment API")
    )
)]
struct ApiDoc;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig {
        merchant_id: "202508080001".to_string(),
        encryption_key: general_purpose::STANDARD
            .decode("IG3CNW5uNrUO2mU2htUOWb9rgXCF7XMAXmL63d7wNZo=")
            .unwrap(),
        post_url: "https://uatcheckout.yagoutpay.com/ms-transaction-core-1-0/paymentRedirection/checksumGatewayPage".to_string(),
        iv: *b"0123456789abcdef",
    };

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .service(create_payment)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", openapi.clone())
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

