fn main() {
    color_eyre::install().unwrap();
    env_logger::init();
    pollster::block_on(game::run()).unwrap();
}
