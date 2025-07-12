use jsonpath_compiler_benchmarks::prelude::*;

pub fn canada_second_coord_component(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset: jsonpath_compiler_benchmarks::framework::ConfiguredBenchset =
        Benchset::new("canada::second_coord_component", dataset::nativejson_canada())?
            .do_not_measure_file_load_time()
            .add_all_targets("$.features[*].geometry.coordinates[*][*][1]")?
            .finish();

    benchset.run(c);

    Ok(())
}

pub fn canada_coord_476_1446_1(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset: jsonpath_compiler_benchmarks::framework::ConfiguredBenchset =
        Benchset::new("canada::coord_476_1446_1", dataset::nativejson_canada())?
            .do_not_measure_file_load_time()
            .add_all_targets("$..coordinates[476][1446][1]")?
            .finish();

    benchset.run(c);

    Ok(())
}

pub fn citm_seat_category(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset: jsonpath_compiler_benchmarks::framework::ConfiguredBenchset =
        Benchset::new("citm::seatCategoryId", dataset::nativejson_citm())?
            .do_not_measure_file_load_time()
            .add_all_targets("$..seatCategoryId")?
            .finish();

    benchset.run(c);

    Ok(())
}

pub fn ast_nested_inner(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("ast::nested_inner", dataset::ast())?
        .do_not_measure_file_load_time()
        .add_all_targets("$..inner..inner..type.qualType")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn ast_deepest(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let query = "$..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*";
    let benchset = Benchset::new("ast::deepest", dataset::ast())?
        .do_not_measure_file_load_time()
        .add_all_targets_except_jsonpath_rust(query)?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_products_video_only_direct_nodes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("bestbuy::products_video_only", dataset::pison_bestbuy_short())?
        .do_not_measure_file_load_time()
        .add_all_targets("$.products[*].videoChapters")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_products_video_only_descendant_nodes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("bestbuy::products_video_only_descendants", dataset::pison_bestbuy_short())?
        .do_not_measure_file_load_time()
        .add_all_targets("$..videoChapters")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_all_nodes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("bestbuy::all_nodes", dataset::pison_bestbuy_short())?
        .do_not_measure_file_load_time()
        .add_all_targets("$..*")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn google_map_routes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("google_map::routes", dataset::pison_google_map_short())?
        .do_not_measure_file_load_time()
        .add_all_targets("$[*].routes[*].legs[*].steps[*].distance.text")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn google_map_travel_modes_direct_nodes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("google_map::travel_modes", dataset::pison_google_map_short())?
        .do_not_measure_file_load_time()
        .add_all_targets("$[*].available_travel_modes")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn google_map_travel_modes_descendant_nodes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("google_map::travel_modes_descendants", dataset::pison_google_map_short())?
        .do_not_measure_file_load_time()
        .add_all_targets("$..available_travel_modes")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn walmart_items_name_direct_nodes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("walmart::items_name", dataset::pison_walmart_short())?
        .do_not_measure_file_load_time()
        .add_all_targets("$.items[*].name")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn walmart_items_name_descendant_nodes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("walmart::items_name_descendants", dataset::pison_walmart_short())?
        .do_not_measure_file_load_time()
        .add_all_targets("$..items_name")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn twitter_metadata_direct_nodes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("twitter::metadata", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_all_targets("$.search_metadata.count")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn twitter_metadata_direct_descendant_nodes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("twitter::metadata_descendants", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_all_targets("$..count")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn inner_array(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("inner_array", dataset::ast())?
        .do_not_measure_file_load_time()
        .add_all_targets("$..inner[0]")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn user_second_mention_index(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("user_mentions_indices", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_all_targets("$..entities.user_mentions[1]")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn all_first_index(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let query = "$..[0]";
    let benchset = Benchset::new("all_first_index", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_all_targets_except_jsonpath_rust(query)?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(
    main_benches,
    canada_second_coord_component,
    canada_coord_476_1446_1,
    citm_seat_category,
    ast_nested_inner,
    ast_deepest,
    bestbuy_products_video_only_direct_nodes,
    bestbuy_products_video_only_descendant_nodes,
    bestbuy_all_nodes,
    google_map_routes,
    google_map_travel_modes_direct_nodes,
    google_map_travel_modes_descendant_nodes,
    inner_array,
    user_second_mention_index,
    walmart_items_name_direct_nodes,
    walmart_items_name_descendant_nodes,
    twitter_metadata_direct_nodes,
    twitter_metadata_direct_descendant_nodes,
    all_first_index
);
