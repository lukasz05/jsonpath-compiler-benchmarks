extern "C" {
    fn free_result_buffer(result_buf: *mut u8);

    fn dom_canada_second_coord_component_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_canada_coord_476_1446_1_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_citm_seat_category_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_ast_nested_inner_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_ast_deepest_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_bestbuy_all_nodes_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_bestbuy_video_only_direct_nodes_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_bestbuy_video_only_descendant_nodes_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_google_map_routes_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_google_map_travel_modes_direct_nodes_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_google_map_travel_modes_descendant_nodes_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_inner_array_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_user_second_mention_index_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_twitter_metadata_direct_nodes_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_twitter_metadata_descendant_nodes_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_walmart_items_name_direct_nodes_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_walmart_items_name_descendant_nodes_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_all_first_index_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim1_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim2_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim3_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim4_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim5_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim6_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim7_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim8_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim9_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim10_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim11_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim12_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim13_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim14_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim15_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim16_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim17_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim18_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim19_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn dom_claim20_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

}


pub fn dom_canada_second_coord_component(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_canada_second_coord_component_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_canada_coord_476_1446_1(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_canada_coord_476_1446_1_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_citm_seat_category(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_citm_seat_category_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_ast_nested_inner(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_ast_nested_inner_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_ast_deepest(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_ast_deepest_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_bestbuy_all_nodes(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_bestbuy_all_nodes_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_bestbuy_video_only_direct_nodes(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_bestbuy_video_only_direct_nodes_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_bestbuy_video_only_descendant_nodes(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_bestbuy_video_only_descendant_nodes_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_google_map_routes(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_google_map_routes_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_google_map_travel_modes_direct_nodes(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_google_map_travel_modes_direct_nodes_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_google_map_travel_modes_descendant_nodes(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_google_map_travel_modes_descendant_nodes_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_inner_array(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_inner_array_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_user_second_mention_index(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_user_second_mention_index_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_twitter_metadata_direct_nodes(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_twitter_metadata_direct_nodes_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_twitter_metadata_descendant_nodes(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_twitter_metadata_descendant_nodes_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_walmart_items_name_direct_nodes(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_walmart_items_name_direct_nodes_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_walmart_items_name_descendant_nodes(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_walmart_items_name_descendant_nodes_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_all_first_index(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_all_first_index_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim1(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim1_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim2(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim2_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim3(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim3_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim4(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim4_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim5(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim5_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim6(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim6_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim7(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim7_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim8(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim8_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim9(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim9_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim10(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim10_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim11(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim11_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim12(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim12_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim13(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim13_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim14(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim14_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim15(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim15_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim16(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim16_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim17(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim17_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim18(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim18_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim19(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim19_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn dom_claim20(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = dom_claim20_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}
