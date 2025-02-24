use jsonpath_compiler_benchmarks::prelude::*;

pub fn retweet_count_58(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("retweet_count_58", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_all_targets_supporting_filters("$.statuses[?(@.retweet_count == 58)]")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn retweet_count_gt_58(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("retweet_count_gt_58", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_all_targets_supporting_filters("$.statuses[?(@.retweet_count > 58)]")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn retweet_count_gte_1(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("retweet_count_gte_1", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_all_targets_supporting_filters("$.statuses[?(@.retweet_count >= 1)]")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn twitter_text_abc(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("twitter_text_abc", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_all_targets_supporting_filters("$..[?(@.text == \"abc\")]")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn twitter_text_exists(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("twitter_text_exists", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_all_targets_supporting_filters("$..[?(@.text)]")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn canada_multiple_subqueries(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("canada_multiple_subqueries", dataset::nativejson_canada())?
        .do_not_measure_file_load_time()
        .add_all_targets_supporting_filters("$[?(@[0].geometry.coordinates[0][13][1] && @[0].geometry.coordinates[48][20][1] && @[0].geometry.coordinates[96][12][1] && @[0].geometry.coordinates[144][22][1] && @[0].geometry.coordinates[192][32][1] && @[0].geometry.coordinates[240][18][1] && @[0].geometry.coordinates[288][19][1] && @[0].geometry.coordinates[336][54][1] && @[0].geometry.coordinates[384][18][1] && @[0].geometry.coordinates[432][71][1])]")?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(
    filter_benches,
    retweet_count_58,
    retweet_count_gt_58,
    retweet_count_gte_1,
    twitter_text_abc,
    twitter_text_exists,
    canada_multiple_subqueries
);