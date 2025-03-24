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

pub fn twitter_text_abc_user(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("twitter_text_abc_user", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_all_targets_supporting_filters("$..[?(@.text == \"abc\")].user")?
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

pub fn status_with_id_screen_name(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("status_with_id_screen_name", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_all_targets_supporting_filters("$.statuses[?(@.id == 505874873961308160)].entities.user_mentions[0].screen_name")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn status_with_id_screen_name_large(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("status_with_id_screen_name_large", dataset::pison_twitter_large())?
        .do_not_measure_file_load_time()
        .add_all_targets_supporting_filters_except_jsurfer("$[?(@.id == 787994505744097280)].entities.user_mentions[0].screen_name")?
        .finish();

    benchset.run(c);

    Ok(())
}


pub fn status_with_id_descendants(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("status_with_id_descendants", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_all_targets_supporting_filters("$.statuses[?(@.id == 505874873961308160)]..*")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn status_with_id_descendants_large(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("status_with_id_descendants_large", dataset::pison_twitter_large())?
        .do_not_measure_file_load_time()
        .add_all_targets_supporting_filters_except_jsurfer("$[?(@.id == 787994505744097280)]..*")?
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

pub fn canada_consecutive_filter_segments(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("canada_consecutive_filter_segments", dataset::nativejson_canada())?
        .do_not_measure_file_load_time()
        .add_all_targets_supporting_filters("$[?(@[0])][?(@.geometry)][?(@.coordinates)][?(@[479])][?(@[5275])][5275][1]")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn canada_interleaved_filter_segments(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("canada_interleaved_filter_segments", dataset::nativejson_canada())?
        .do_not_measure_file_load_time()
        .add_all_targets_supporting_filters("$[?(@[0])][0][?(@.coordinates)][\"coordinates\"][?(@[5275])[5275][1]")?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(
    filters,
    retweet_count_58,
    retweet_count_gt_58,
    retweet_count_gte_1,
    twitter_text_abc,
    twitter_text_abc_user,
    twitter_text_exists,
    status_with_id_screen_name,
    status_with_id_screen_name_large,
    status_with_id_descendants,
    status_with_id_descendants_large,
    canada_multiple_subqueries,
    canada_consecutive_filter_segments,
    canada_interleaved_filter_segments
);