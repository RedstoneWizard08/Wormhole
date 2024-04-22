fn main() {
    dotenvy::dotenv().unwrap();

    if let Ok(key) = std::env::var("MSA_CLIENT_SECRET") {
        println!("cargo::rustc-env=MSA_CLIENT_SECRET={}", key);
    }
}
