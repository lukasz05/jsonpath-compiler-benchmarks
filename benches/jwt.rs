use std::cmp::min;

use criterion::{Criterion, criterion_group, criterion_main};
use serde_json::Value;
use uuid::Uuid;
use rand::{SeedableRng};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use jsonpath_compiler_benchmarks::ondemand_queries_bindings::*;

struct JWTPayloadGenerator {
    custom_claim_count: usize,
}

impl JWTPayloadGenerator {
    pub fn new(custom_claim_count: usize) -> JWTPayloadGenerator {
        JWTPayloadGenerator {
            custom_claim_count
        }
    }

    pub fn generate(&self) -> String {
        let mut claims_ids: Vec<usize> = (1..self.custom_claim_count + 1).collect();
        let mut rng = StdRng::seed_from_u64(0);
        claims_ids.shuffle(&mut rng);
        let claims = claims_ids
            .map(|id| Self::generate_custom_claim(id))
            .collect::<Vec<String>>()
            .join(",");
        return "{".to_string() + &claims + "}";
    }

    fn generate_custom_claim(id: usize) -> String {
        let guid = Uuid::new_v4();
        format!("\"claim{id}\": \"{guid}\"")
    }
}

fn get_query_function_for_claim(claim_name: &str) -> fn(&[u8]) -> String {
    match claim_name {
        "claim1" => ondemand_claim1,
        "claim2" => ondemand_claim2,
        "claim3" => ondemand_claim3,
        "claim4" => ondemand_claim4,
        "claim5" => ondemand_claim5,
        "claim6" => ondemand_claim6,
        "claim7" => ondemand_claim7,
        "claim8" => ondemand_claim8,
        "claim9" => ondemand_claim9,
        "claim10" => ondemand_claim10,
        "claim11" => ondemand_claim11,
        "claim12" => ondemand_claim12,
        "claim13" => ondemand_claim13,
        "claim14" => ondemand_claim14,
        "claim15" => ondemand_claim15,
        "claim16" => ondemand_claim16,
        "claim17" => ondemand_claim17,
        "claim18" => ondemand_claim18,
        "claim19" => ondemand_claim19,
        "claim20" => ondemand_claim20,
        _ => unimplemented!()
    }
}

fn jsonpath_compiler_get_claim(padded_payload: &[u8], claim_names: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for claim_name in claim_names {
        let query_function = get_query_function_for_claim(claim_name);
        result.push(query_function(padded_payload))
    }
    result
}

fn serde_get_claim(payload_json: &str, claim_names: &[String]) -> Vec<String> {
    let parsed_json: Value = serde_json::from_str(payload_json).unwrap();
    let mut result = Vec::new();
    for claim_name in claim_names {
        result.push(parsed_json[claim_name].to_string())
    }
    result
}


fn benchmark_inner(c: &mut Criterion, claim_count: usize) {
    let mut group = c.benchmark_group(format!("{claim_count}_claims"));

    let json = JWTPayloadGenerator::new(claim_count).generate();
    let json_bytes = json.to_string().into_bytes();
    let padding = vec![0; 64];
    let padded_payload_bytes: Vec<u8> = json_bytes.into_iter().chain(padding).collect();

    for selected_claim_count in 1..min(claim_count + 1, 11) {
        let selected_claims: Vec<String> = (1..selected_claim_count + 1)
            .map(|i| format!("claim{i}"))
            .collect();
        group.bench_function(
            format!("serde_json_{selected_claim_count}"),
            |b| b.iter(|| serde_get_claim(&json, &selected_claims)),
        );
        group.bench_function(
            format!("jsonpath_compiler_{selected_claim_count}"),
            |b| b.iter(|| jsonpath_compiler_get_claim(&padded_payload_bytes, &selected_claims)),
        );
    }

    group.finish();
}

fn claims_10(c: &mut Criterion) {
    benchmark_inner(c, 10)
}

fn claims_20(c: &mut Criterion) {
    benchmark_inner(c, 20)
}

fn claims_30(c: &mut Criterion) {
    benchmark_inner(c, 30)
}

fn claims_40(c: &mut Criterion) {
    benchmark_inner(c, 40)
}

fn claims_50(c: &mut Criterion) {
    benchmark_inner(c, 50)
}

fn claims_100(c: &mut Criterion) {
    benchmark_inner(c, 100)
}

fn claims_150(c: &mut Criterion) {
    benchmark_inner(c, 150)
}

fn claims_200(c: &mut Criterion) {
    benchmark_inner(c, 200)
}

fn single_claim_growing_payload(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_claim_growing_payload".to_string());

    for payload_claim_count_size in (10..201).step_by(10) {
        let json = JWTPayloadGenerator::new(payload_claim_count_size).generate();
        let json_bytes = json.to_string().into_bytes();
        let padding = vec![0; 64];
        let padded_payload_bytes: Vec<u8> = json_bytes.into_iter().chain(padding).collect();
        let selected_claims: Vec<String> = vec!["claim1".to_string()];
        group.bench_function(
            format!("serde_json_{payload_claim_count_size}"),
            |b| b.iter(|| serde_get_claim(&json, &selected_claims)),
        );
        group.bench_function(
            format!("jsonpath_compiler_{payload_claim_count_size}"),
            |b| b.iter(|| jsonpath_compiler_get_claim(&padded_payload_bytes, &selected_claims)),
        );
    }

    group.finish();
}

criterion_group!(
    jwt_benches,
    claims_10,
    claims_20,
    claims_30,
    claims_40,
    claims_50,
    claims_100,
    claims_150,
    claims_200,
    single_claim_growing_payload
);
criterion_main!(jwt_benches);