//! Testing code for shared purposes

use std::path::PathBuf;
use once_cell::sync::Lazy;
use usv::style::Style;

#[allow(dead_code)]
pub static BENCHES_DIR: Lazy<PathBuf> = Lazy::new(||
    [env!("CARGO_MANIFEST_DIR"), "benches"].iter().collect::<PathBuf>()
);

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_example(c: &mut Criterion){
    let path = BENCHES_DIR.join("example.xls");
    
    let style = Style::default();
    let mut group = c.benchmark_group(
        &format!("benchmark group path: {:?}", &path)
    );
    group.sample_size(10);
    group.bench_function("xlsx_to_usv", |b| b.iter(|| xlsx_to_usv::xlsx_file_to_usv(&path, &style)));
    group.finish();
}

criterion_group!(benches, bench_example);
criterion_main!(benches);
