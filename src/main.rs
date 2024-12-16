mod application;
mod frontend;
mod storage;
mod tf_serde;

fn main() {
    application::init();
    application::run();
}
