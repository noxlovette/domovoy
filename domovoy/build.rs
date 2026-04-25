fn main() {
    dotenvy::dotenv().ok();
    println!(
        "cargo:rustc-env=YANDEX_CLIENT_ID={}",
        std::env::var("YANDEX_CLIENT_ID").expect("YANDEX_CLIENT_ID not set")
    );
    println!(
        "cargo:rustc-env=YANDEX_CLIENT_SECRET={}",
        std::env::var("YANDEX_CLIENT_SECRET").expect("YANDEX_CLIENT_SECRET not set")
    );
}
