use rocket::tokio::runtime;
use setsail;

fn main() {
    _ = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(setsail::bootstrap().launch());
}
