fn main() {
    color_eyre::install().unwrap();
    env_logger::init();
    game::run().unwrap();
}
