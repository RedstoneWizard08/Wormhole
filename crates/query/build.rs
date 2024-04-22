fn main() {
    dotenvy::dotenv().unwrap();

    if let Ok(key) = std::env::var("CURSEFORGE_KEY") {
        println!("cargo::rustc-env=CURSEFORGE_KEY={}", key);
    }

    if let Ok(key) = std::env::var("MODRINTH_KEY") {
        println!("cargo::rustc-env=MODRINTH_KEY={}", key);
    }

    if let Ok(key) = std::env::var("NEXUS_API_KEY") {
        println!("cargo::rustc-env=NEXUS_API_KEY={}", key);
    }
}
