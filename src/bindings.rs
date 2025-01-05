extern "C" {
    fn free_result_buffer(result_buf: *mut u8);

    fn canada_second_coord_component_binding(padded_input: * const u8, input_length: usize, result_length: * mut usize) -> * mut u8;

    fn canada_coord_476_1446_1_binding(padded_input: * const u8, input_length: usize, result_length: * mut usize) -> * mut u8;

    fn citm_seat_category_binding(padded_input: * const u8, input_length: usize, result_length: * mut usize) -> * mut u8;

    fn ast_nested_inner_binding(padded_input: * const u8, input_length: usize, result_length: * mut usize) -> * mut u8;

    fn ast_deepest_binding(padded_input: * const u8, input_length: usize, result_length: * mut usize) -> * mut u8;

    fn bestbuy_all_nodes_binding(padded_input: * const u8, input_length: usize, result_length: * mut usize) -> * mut u8;

    fn google_map_routes_binding(padded_input: * const u8, input_length: usize, result_length: * mut usize) -> * mut u8;

    fn inner_array_binding(padded_input: * const u8, input_length: usize, result_length: * mut usize) -> * mut u8;

    fn user_second_mention_index_binding(padded_input: * const u8, input_length: usize, result_length: * mut usize) -> * mut u8;

}


pub fn canada_second_coord_component(padded_input: &[u8]) -> String {
    let input_ptr = padded_input.as_ptr();
    let mut result_length: usize = 0;
    unsafe {
        let result_ptr: *mut u8 =  canada_second_coord_component_binding(input_ptr, padded_input.len(), &mut result_length);
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
