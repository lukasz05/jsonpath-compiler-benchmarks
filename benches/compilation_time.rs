use criterion::{criterion_group, criterion_main};
use jsonpath_compiler::compiler::StandaloneProgGeneratingCompiler;
use jsonpath_compiler::targets::simdjson::ondemand::OnDemandCodeStandaloneProgGenerator;
use jsonpath_compiler_benchmarks::prelude::*;

pub fn ast_nested_inner(c: &mut Criterion)  {
    c.bench_function("ast_nested_inner", |b| b.iter(|| {
        compile_query("$..inner..inner..type.qualType");
    }));
}

pub fn ast_deepest(c: &mut Criterion) {
    c.bench_function("ast_deepest", |b| b.iter(|| {
        compile_query("$..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*");
    }));
}

fn compile_query(query: &str) {
    let compiler = StandaloneProgGeneratingCompiler::new();
    compiler.compile::<OnDemandCodeStandaloneProgGenerator>(query, "").unwrap();
}

criterion_group!(
    compilation_time_benches,
    ast_deepest,
    ast_nested_inner,
);
criterion_main!(compilation_time_benches);
