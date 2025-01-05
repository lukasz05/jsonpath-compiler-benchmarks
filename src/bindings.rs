extern "C" {
    fn free_result_buffer(result_buf: *mut u8);

    fn canada_second_coord_component_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn canada_coord_476_1446_1_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn citm_seat_category_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn ast_nested_inner_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn ast_deepest_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn bestbuy_all_nodes_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn google_map_routes_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn inner_array_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn user_second_mention_index_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim1_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim2_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim3_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim10_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim20_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim30_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim40_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim50_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim60_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim70_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim80_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim90_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim100_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim110_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim120_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim130_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim140_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim150_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim160_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim170_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim180_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim190_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

    fn claim200_binding(padded_input: *const u8, input_length: usize, result_length: *mut usize) -> *mut u8;

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

pub fn claim30(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim30_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim40(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim40_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim50(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim50_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim60(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim60_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim70(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim70_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim80(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim80_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim90(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim90_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim100(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim100_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim110(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim110_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim120(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim120_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim130(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim130_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim140(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim140_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim150(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim150_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim160(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim160_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim170(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim170_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim180(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim180_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim190(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim190_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}

pub fn claim200(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 = claim200_binding(input_ptr, padded_input.len(), &mut result_length);
        let result_slice = std::slice::from_raw_parts(result_ptr, result_length);
        let result_str = String::from_utf8_unchecked(result_slice.to_vec());
        free_result_buffer(result_ptr);
        result_str
    }
}
