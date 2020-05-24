use std::sync::Once;

pub fn init_env() {
    static ENV: Once = Once::new();
    ENV.call_once(|| {
        better_panic::install();
        dotenv::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/test/.env")).unwrap();
    })
}