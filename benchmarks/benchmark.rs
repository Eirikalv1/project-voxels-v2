use criterion::{criterion_group, criterion_main, Criterion};
use project_voxels_v2::engine_loop::run;

fn _main2() {
    std::env::set_var("RUST_LOG", "error");
    pretty_env_logger::init();

    pollster::block_on(run());
}

pub fn criterion_benchmark(_: &mut Criterion) {}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
