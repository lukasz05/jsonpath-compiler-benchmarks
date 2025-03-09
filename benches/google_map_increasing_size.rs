use criterion::{Criterion};
use jsonpath_compiler_benchmarks::{benchsets, dataset};
use jsonpath_compiler_benchmarks::framework::{BenchmarkError, Benchset};


pub fn google_map_increasing_size_routes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    for i in 1..11 {
        let benchset = Benchset::new(
            format!("google_map::increasing_size::routes::{i}"),
            dataset::pison_google_map_sized(i)
        )?
        .do_not_measure_file_load_time()
        .add_all_capable_targets("$[*].routes[*].legs[*].steps[*].distance.text")?
        .finish();

        benchset.run(c);
    }
    Ok(())
}

pub fn google_map_increasing_size_travel_modes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    for i in 1..11 {
        let benchset = Benchset::new(
            format!("google_map::increasing_size::travel_modes::{i}"),
            dataset::pison_google_map_sized(i)
        )?
            .do_not_measure_file_load_time()
            .add_all_capable_targets("$[*].available_travel_modes")?
            .finish();

        benchset.run(c);
    }
    Ok(())
}

benchsets!(
    google_map_increasing_size,
    //google_map_increasing_size_routes
    google_map_increasing_size_travel_modes
);