use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::string::FromUtf8Error;

use memmap2::Mmap;
use thiserror::Error;

use crate::dom_queries_bindings::*;
use crate::framework::implementation::Implementation;
use crate::ondemand_eager_filters_queries_bindings::*;
use crate::ondemand_queries_bindings::*;

type QueryFunction = fn(&[u8]) -> String;

struct JsonPathCompilerCore<'a> {
    query_functions: HashMap<&'a str, QueryFunction>,
}

pub struct JsonPathCompilerResult(String);

impl JsonPathCompilerCore<'_> {
    fn new_ondemand() -> Result<Self, JsonPathCompilerError> {
        Ok(JsonPathCompilerCore {
            query_functions: HashMap::from([
                ("$.features[*].geometry.coordinates[*][*][1]", ondemand_canada_second_coord_component as QueryFunction),
                ("$..coordinates[476][1446][1]", ondemand_canada_coord_476_1446_1 as QueryFunction),
                ("$..seatCategoryId", ondemand_citm_seat_category as QueryFunction),
                ("$..inner..inner..type.qualType", ondemand_ast_nested_inner as QueryFunction),
                ("$..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*", ondemand_ast_deepest as QueryFunction),
                ("$..*", ondemand_bestbuy_all_nodes as QueryFunction),
                ("$.products[*].videoChapters", ondemand_bestbuy_video_only_direct_nodes as QueryFunction),
                ("$..videoChapters", ondemand_bestbuy_video_only_descendant_nodes as QueryFunction),
                ("$[*].routes[*].legs[*].steps[*].distance.text", ondemand_google_map_routes as QueryFunction),
                ("$[*].available_travel_modes", ondemand_google_map_travel_modes_direct_nodes as QueryFunction),
                ("$..available_travel_modes", ondemand_google_map_travel_modes_descendant_nodes as QueryFunction),
                ("$..inner[0]", ondemand_inner_array as QueryFunction),
                ("$..entities.user_mentions[1]", ondemand_user_second_mention_index as QueryFunction),
                ("$.statuses[?(@.retweet_count == 58)]", ondemand_retweet_count_58 as QueryFunction),
                ("$.statuses[?(@.retweet_count > 58)]", ondemand_retweet_count_gt_58 as QueryFunction),
                ("$.statuses[?(@.retweet_count >= 1)]", ondemand_retweet_count_gte_1 as QueryFunction),
                ("$..[?(@.text == \"abc\")]", ondemand_twitter_text_abc as QueryFunction),
                ("$..[?(@.text == \"abc\")].user", ondemand_twitter_text_abc_user as QueryFunction),
                ("$..[?(@.text)]", ondemand_twitter_text_exists as QueryFunction),
                ("$.search_metadata.count", ondemand_twitter_metadata_direct_nodes as QueryFunction),
                ("$..count", ondemand_twitter_metadata_descendant_nodes as QueryFunction),
                ("$.statuses[?(@.id == 505874873961308160)].entities.user_mentions[0].screen_name", ondemand_status_with_id_screen_name as QueryFunction),
                ("$[?(@.id == 787994505744097280)].entities.user_mentions[0].screen_name", ondemand_status_with_id_screen_name_large as QueryFunction),
                ("$.statuses[?(@.id == 505874873961308160)]..*", ondemand_status_with_id_descendants as QueryFunction),
                ("$[?(@.id == 787994505744097280)]..*", ondemand_status_with_id_descendants_large as QueryFunction),
                ("$[?(@[0].geometry.coordinates[0][13][1] && @[0].geometry.coordinates[48][20][1] && @[0].geometry.coordinates[96][12][1] && @[0].geometry.coordinates[144][22][1] && @[0].geometry.coordinates[192][32][1] && @[0].geometry.coordinates[240][18][1] && @[0].geometry.coordinates[288][19][1] && @[0].geometry.coordinates[336][54][1] && @[0].geometry.coordinates[384][18][1] && @[0].geometry.coordinates[432][71][1])]", ondemand_canada_multiple_subqueries as QueryFunction),
                ("$[?(@[0])][?(@.geometry)][?(@.coordinates)][?(@[479])][?(@[5275])][5275][1]", ondemand_canada_consecutive_filter_segments as QueryFunction),
                ("$[?(@[0])][0][?(@.coordinates)][\"coordinates\"][?(@[5275])][5275][1]", ondemand_canada_interleaved_filter_segments as QueryFunction),
                ("$.items[*].name", ondemand_walmart_items_name_direct_nodes as QueryFunction),
                ("$..items_name", ondemand_walmart_items_name_descendant_nodes as QueryFunction),
                ("$..[0]", ondemand_eager_filters_all_first_index as QueryFunction)
            ])
        })
    }

    fn new_ondemand_eager_filters() -> Result<Self, JsonPathCompilerError> {
        Ok(JsonPathCompilerCore {
            query_functions: HashMap::from([
                ("$.features[*].geometry.coordinates[*][*][1]", ondemand_eager_filters_canada_second_coord_component as QueryFunction),
                ("$..coordinates[476][1446][1]", ondemand_eager_filters_canada_coord_476_1446_1 as QueryFunction),
                ("$..seatCategoryId", ondemand_eager_filters_citm_seat_category as QueryFunction),
                ("$..inner..inner..type.qualType", ondemand_eager_filters_ast_nested_inner as QueryFunction),
                ("$..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*", ondemand_eager_filters_ast_deepest as QueryFunction),
                ("$..*", ondemand_eager_filters_bestbuy_all_nodes as QueryFunction),
                ("$.products[*].videoChapters", ondemand_eager_filters_bestbuy_video_only_direct_nodes as QueryFunction),
                ("$..videoChapters", ondemand_eager_filters_bestbuy_video_only_descendant_nodes as QueryFunction),
                ("$[*].routes[*].legs[*].steps[*].distance.text", ondemand_eager_filters_google_map_routes as QueryFunction),
                ("$[*].available_travel_modes", ondemand_eager_filters_google_map_travel_modes_direct_nodes as QueryFunction),
                ("$..available_travel_modes", ondemand_eager_filters_google_map_travel_modes_descendant_nodes as QueryFunction),
                ("$..inner[0]", ondemand_eager_filters_inner_array as QueryFunction),
                ("$..entities.user_mentions[1]", ondemand_eager_filters_user_second_mention_index as QueryFunction),
                ("$.statuses[?(@.retweet_count == 58)]", ondemand_eager_filters_retweet_count_58 as QueryFunction),
                ("$.statuses[?(@.retweet_count > 58)]", ondemand_eager_filters_retweet_count_gt_58 as QueryFunction),
                ("$.statuses[?(@.retweet_count >= 1)]", ondemand_eager_filters_retweet_count_gte_1 as QueryFunction),
                ("$..[?(@.text == \"abc\")]", ondemand_eager_filters_twitter_text_abc as QueryFunction),
                ("$..[?(@.text == \"abc\")].user", ondemand_eager_filters_twitter_text_abc_user as QueryFunction),
                ("$..[?(@.text)]", ondemand_eager_filters_twitter_text_exists as QueryFunction),
                ("$.search_metadata.count", ondemand_eager_filters_twitter_metadata_direct_nodes as QueryFunction),
                ("$..count", ondemand_eager_filters_twitter_metadata_descendant_nodes as QueryFunction),
                ("$.statuses[?(@.id == 505874873961308160)].entities.user_mentions[0].screen_name", ondemand_eager_filters_status_with_id_screen_name as QueryFunction),
                ("$[?(@.id == 787994505744097280)].entities.user_mentions[0].screen_name", ondemand_eager_filters_status_with_id_screen_name_large as QueryFunction),
                ("$.statuses[?(@.id == 505874873961308160)]..*", ondemand_eager_filters_status_with_id_descendants as QueryFunction),
                ("$[?(@.id == 787994505744097280)]..*", ondemand_eager_filters_status_with_id_descendants_large as QueryFunction),
                ("$[?(@[0].geometry.coordinates[0][13][1] && @[0].geometry.coordinates[48][20][1] && @[0].geometry.coordinates[96][12][1] && @[0].geometry.coordinates[144][22][1] && @[0].geometry.coordinates[192][32][1] && @[0].geometry.coordinates[240][18][1] && @[0].geometry.coordinates[288][19][1] && @[0].geometry.coordinates[336][54][1] && @[0].geometry.coordinates[384][18][1] && @[0].geometry.coordinates[432][71][1])]", ondemand_eager_filters_canada_multiple_subqueries as QueryFunction),
                ("$[?(@[0])][?(@.geometry)][?(@.coordinates)][?(@[479])][?(@[5275])][5275][1]", ondemand_eager_filters_canada_consecutive_filter_segments as QueryFunction),
                ("$[?(@[0])][0][?(@.coordinates)][\"coordinates\"][?(@[5275])][5275][1]", ondemand_eager_filters_canada_interleaved_filter_segments as QueryFunction),
                ("$.items[*].name", ondemand_eager_filters_walmart_items_name_direct_nodes as QueryFunction),
                ("$..items_name", ondemand_eager_filters_walmart_items_name_descendant_nodes as QueryFunction),
                ("$..[0]", ondemand_eager_filters_all_first_index as QueryFunction)
            ])
        })
    }

    fn new_dom() -> Result<Self, JsonPathCompilerError> {
        Ok(JsonPathCompilerCore {
            query_functions: HashMap::from([
                ("$.features[*].geometry.coordinates[*][*][1]", dom_canada_second_coord_component as QueryFunction),
                ("$..coordinates[476][1446][1]", dom_canada_coord_476_1446_1 as QueryFunction),
                ("$..seatCategoryId", dom_citm_seat_category as QueryFunction),
                ("$..inner..inner..type.qualType", dom_ast_nested_inner as QueryFunction),
                ("$..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*", dom_ast_deepest as QueryFunction),
                ("$..*", dom_bestbuy_all_nodes as QueryFunction),
                ("$.products[*].videoChapters", dom_bestbuy_video_only_direct_nodes as QueryFunction),
                ("$..videoChapters", dom_bestbuy_video_only_descendant_nodes as QueryFunction),
                ("$[*].routes[*].legs[*].steps[*].distance.text", dom_google_map_routes as QueryFunction),
                ("$[*].available_travel_modes", dom_google_map_travel_modes_direct_nodes as QueryFunction),
                ("$..available_travel_modes", dom_google_map_travel_modes_descendant_nodes as QueryFunction),
                ("$..inner[0]", dom_inner_array as QueryFunction),
                ("$..entities.user_mentions[1]", dom_user_second_mention_index as QueryFunction),
                ("$.search_metadata.count", dom_twitter_metadata_direct_nodes as QueryFunction),
                ("$..count", dom_twitter_metadata_descendant_nodes as QueryFunction),
                ("$.items[*].name", dom_walmart_items_name_direct_nodes as QueryFunction),
                ("$..items_name", dom_walmart_items_name_descendant_nodes as QueryFunction),
                ("$..[0]", dom_all_first_index as QueryFunction)
            ])
        })
    }

    fn compile_query(&self, query: &str) -> Result<QueryFunction, JsonPathCompilerError> {
        match self.query_functions.get(query) {
            Some(query_function) => {
                Ok(*query_function)
            }
            None => {
                Err(JsonPathCompilerError::UnrecognizedQuery(query.to_string()))
            }
        }
    }

    fn run<'a>(&self, query_function: &QueryFunction, file: &'a [u8]) -> Result<JsonPathCompilerResult, JsonPathCompilerError> {
        let res = query_function(file);
        Ok(JsonPathCompilerResult(res))
    }
}

impl Display for JsonPathCompilerResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

#[derive(Error, Debug)]
pub enum JsonPathCompilerError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    Utf8Error(#[from] FromUtf8Error),
    #[error("Unrecognized query: '{0}'")]
    UnrecognizedQuery(String),
}

pub struct JsonPathCompilerOndemand<'a> {
    core: JsonPathCompilerCore<'a>,
}

impl Implementation for JsonPathCompilerOndemand<'_> {
    type Query = QueryFunction;

    type File = Vec<u8>;

    type Error = JsonPathCompilerError;

    type Result<'a> = JsonPathCompilerResult;

    fn id() -> &'static str {
        "jsonpath_compiler_ondemand"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(JsonPathCompilerOndemand {
            core: JsonPathCompilerCore::new_ondemand()?
        })
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::read_to_string(file_path)?;
        let input = file.into_bytes();
        let padding = vec![0; 64];
        let padded_input = input.into_iter().chain(padding).collect();
        Ok(padded_input)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        self.core.compile_query(query)
    }

    fn run<'a>(&self, query: &'a Self::Query, file: &'a Self::File) -> Result<Self::Result<'a>, Self::Error> {
        self.core.run(query, file)
    }
}

pub struct JsonPathCompilerOndemandMmap<'a> {
    core: JsonPathCompilerCore<'a>,
}

impl Implementation for JsonPathCompilerOndemandMmap<'_> {
    type Query = QueryFunction;

    type File = Mmap;

    type Error = JsonPathCompilerError;

    type Result<'a> = JsonPathCompilerResult;

    fn id() -> &'static str {
        "jsonpath_compiler_ondemand_mmap"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(JsonPathCompilerOndemandMmap {
            core: JsonPathCompilerCore::new_ondemand()?
        })
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::File::open(file_path)?;
        unsafe {
            let mapped_file = Mmap::map(&file)?;
            Ok(mapped_file)
        }
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        self.core.compile_query(query)
    }

    fn run<'a>(&self, query: &'a Self::Query, file: &'a Self::File) -> Result<Self::Result<'a>, Self::Error> {
        self.core.run(query, file)
    }
}

pub struct JsonPathCompilerOndemandEagerFilters<'a> {
    core: JsonPathCompilerCore<'a>,
}

impl Implementation for JsonPathCompilerOndemandEagerFilters<'_> {
    type Query = QueryFunction;

    type File = Vec<u8>;

    type Error = JsonPathCompilerError;

    type Result<'a> = JsonPathCompilerResult;

    fn id() -> &'static str {
        "jsonpath_compiler_ondemand_eager_filters"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(JsonPathCompilerOndemandEagerFilters {
            core: JsonPathCompilerCore::new_ondemand_eager_filters()?
        })
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::read_to_string(file_path)?;
        let input = file.into_bytes();
        let padding = vec![0; 64];
        let padded_input = input.into_iter().chain(padding).collect();
        Ok(padded_input)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        self.core.compile_query(query)
    }

    fn run<'a>(&self, query: &'a Self::Query, file: &'a Self::File) -> Result<Self::Result<'a>, Self::Error> {
        self.core.run(query, file)
    }
}

pub struct JsonPathCompilerDom<'a> {
    core: JsonPathCompilerCore<'a>,
}

impl Implementation for JsonPathCompilerDom<'_> {
    type Query = QueryFunction;

    type File = Vec<u8>;

    type Error = JsonPathCompilerError;

    type Result<'a> = JsonPathCompilerResult;

    fn id() -> &'static str {
        "jsonpath_compiler_dom"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(JsonPathCompilerDom {
            core: JsonPathCompilerCore::new_dom()?
        })
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::read_to_string(file_path)?;
        let input = file.into_bytes();
        let padding = vec![0; 64];
        let padded_input = input.into_iter().chain(padding).collect();
        Ok(padded_input)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        self.core.compile_query(query)
    }

    fn run<'a>(&self, query: &'a Self::Query, file: &'a Self::File) -> Result<Self::Result<'a>, Self::Error> {
        self.core.run(query, file)
    }
}