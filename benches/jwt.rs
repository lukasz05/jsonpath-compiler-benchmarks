use criterion::{Criterion, criterion_group, criterion_main};
use serde_json::Value;
use uuid::Uuid;
use rsonpath_benchmarks::bindings::{claim10, claim20, claim30, claim40, claim50, claim60, claim70, claim80, claim90, claim100, claim110, claim120, claim130, claim140, claim150, claim160, claim170, claim180, claim190, claim200, claim1, claim2, claim3};

struct JWTPayloadGenerator {
    custom_claim_count: usize
}

impl JWTPayloadGenerator {
    pub fn new(custom_claim_count: usize) -> JWTPayloadGenerator {
        JWTPayloadGenerator {
            custom_claim_count
        }
    }

    pub fn generate(&self) -> String {
        let claims = (1..self.custom_claim_count+1)
            .map(|id| Self::generate_custom_claim(id))
            .collect::<Vec<String>>()
            .join(",");
        return "{".to_string() + &claims + "}"
    }

    fn generate_custom_claim(id: usize) -> String {
        let guid = Uuid::new_v4();
        format!("\"claim{id}\": \"{guid}\"")
    }
}

fn get_query_function_for_claim(claim_name: &str) -> fn(&[u8]) -> String  {
    match claim_name {
        "claim1" => claim1,
        "claim2" => claim2,
        "claim3" => claim3,
        "claim10" => claim10,
        "claim20" => claim20,
        "claim30" => claim30,
        "claim40" => claim40,
        "claim50" => claim50,
        "claim60" => claim60,
        "claim70" => claim70,
        "claim80" => claim80,
        "claim90" => claim90,
        "claim100" => claim100,
        "claim110" => claim110,
        "claim120" => claim120,
        "claim130" => claim130,
        "claim140" => claim140,
        "claim150" => claim150,
        "claim160" => claim160,
        "claim170" => claim170,
        "claim180" => claim180,
        "claim190" => claim190,
        "claim200" => claim200,
        _ => unimplemented!()
    }
}

fn jsonpath_compiler_get_claim(padded_payload: &[u8], claim_names: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    for claim_name in claim_names {
        let query_function = get_query_function_for_claim(claim_name);
        result.push(query_function(padded_payload))
    }
    result
}

fn serde_get_claim(payload_json: &str, claim_names: &[&str]) -> Vec<String> {
    let parsed_json: Value = serde_json::from_str(payload_json).unwrap();
    let mut result = Vec::new();
    for claim_name in claim_names {
        result.push(parsed_json[claim_name].to_string())
    }
    result
}


fn benchmark_inner(c: &mut Criterion, name: &str, claim_count: usize, selected_claims: &[&str]) {
    let mut group = c.benchmark_group(name);

    let json = JWTPayloadGenerator::new(claim_count).generate();
    let json_bytes = json.to_string().into_bytes();
    let padding = vec![0; 64];
    let padded_payload_bytes: Vec<u8> = json_bytes.into_iter().chain(padding).collect();

    group.bench_function("serde-json", |b| b.iter(|| serde_get_claim(&json, selected_claims)));
    group.bench_function("jsonpath-compiler", |b| b.iter(|| jsonpath_compiler_get_claim(&padded_payload_bytes, selected_claims)));
    group.finish();
}

fn claim_count_10(c: &mut Criterion) {
    benchmark_inner(c, "claim_count_10", 10, &["claim10"])
}

fn claim_count_100(c: &mut Criterion) {
    benchmark_inner(c, "claim_count_100", 100, &["claim50"])
}

fn claim_count_200(c: &mut Criterion) {
    benchmark_inner(c, "claim_count_200", 200, &["claim100"])
}

fn claim_count_10_multiple_selected(c: &mut Criterion) {
    benchmark_inner(c, "claim_count_10_multiple_selected", 10, &["claim1", "claim2", "claim3"])
}

fn claim_count_20_multiple_selected(c: &mut Criterion) {
    benchmark_inner(c, "claim_count_20_multiple_selected", 20, &["claim1", "claim2", "claim3"])
}

fn claim_count_30_multiple_selected(c: &mut Criterion) {
    benchmark_inner(c, "claim_count_30_multiple_selected", 30, &["claim10", "claim20", "claim30"])
}

fn claim_count_200_multiple_selected(c: &mut Criterion) {
    benchmark_inner(c, "claim_count_200_multiple_selected", 200, &["claim10", "claim20", "claim30"])
}

criterion_group!(jwt_benches, claim_count_10, claim_count_100, claim_count_200, claim_count_10_multiple_selected, claim_count_20_multiple_selected, claim_count_30_multiple_selected, claim_count_200_multiple_selected);
criterion_main!(jwt_benches);
