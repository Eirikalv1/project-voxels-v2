use project_voxels_v2::engine_loop::run;

fn main() {
    std::env::set_var("RUST_LOG", "error");
    pretty_env_logger::init();

    pollster::block_on(run());
}
