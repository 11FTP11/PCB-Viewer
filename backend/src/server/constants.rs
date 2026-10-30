use std::sync::LazyLock;
use std::env;

pub const JWT_SECRET_KEY: &'static [u8] = b"test_key";
pub static MASTER_PASSWORD: LazyLock<String> = LazyLock::new(
    || env::var("MASTER_PASSWORD")
        .expect("MASTER_PASSWORD required in \".env\" file")
);
