extern "C" {
    fn free_result_buffer(result_buf: *mut u8);

    fn canada_second_coord_component_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn canada_coord_476_1446_1_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn citm_seat_category_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn ast_nested_inner_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn ast_deepest_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn bestbuy_all_nodes_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn google_map_routes_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn google_map_travel_modes_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn inner_array_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn user_second_mention_index_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn retweet_count_58_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn retweet_count_gt_58_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn retweet_count_gte_1_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn twitter_text_abc_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn twitter_text_abc_user_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn twitter_text_exists_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn status_with_id_screen_name_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn status_with_id_screen_name_large_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn status_with_id_descendants_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn status_with_id_descendants_large_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn canada_multiple_subqueries_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn canada_consecutive_filter_segments_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn canada_interleaved_filter_segments_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim1_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim2_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim3_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim4_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim5_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim6_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim7_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim8_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim9_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim10_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim11_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim12_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim13_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim14_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim15_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim16_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim17_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim18_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim19_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim20_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

}


pub fn canada_second_coord_component(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = canada_second_coord_component_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn canada_coord_476_1446_1(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = canada_coord_476_1446_1_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn citm_seat_category(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = citm_seat_category_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn ast_nested_inner(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = ast_nested_inner_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn ast_deepest(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = ast_deepest_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn bestbuy_all_nodes(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = bestbuy_all_nodes_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn google_map_routes(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = google_map_routes_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn google_map_travel_modes(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = google_map_travel_modes_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn inner_array(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = inner_array_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn user_second_mention_index(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = user_second_mention_index_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn retweet_count_58(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = retweet_count_58_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn retweet_count_gt_58(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = retweet_count_gt_58_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn retweet_count_gte_1(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = retweet_count_gte_1_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn twitter_text_abc(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = twitter_text_abc_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn twitter_text_abc_user(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = twitter_text_abc_user_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn twitter_text_exists(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = twitter_text_exists_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn status_with_id_screen_name(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = status_with_id_screen_name_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn status_with_id_screen_name_large(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = status_with_id_screen_name_large_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn status_with_id_descendants(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = status_with_id_descendants_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn status_with_id_descendants_large(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = status_with_id_descendants_large_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn canada_multiple_subqueries(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = canada_multiple_subqueries_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn canada_consecutive_filter_segments(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = canada_consecutive_filter_segments_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn canada_interleaved_filter_segments(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = canada_interleaved_filter_segments_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim1(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim1_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim2(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim2_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim3(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim3_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim4(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim4_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim5(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim5_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim6(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim6_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim7(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim7_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim8(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim8_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim9(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim9_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim10(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim10_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim11(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim11_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim12(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim12_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim13(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim13_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim14(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim14_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim15(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim15_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim16(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim16_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim17(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim17_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim18(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim18_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim19(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim19_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim20(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim20_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}
