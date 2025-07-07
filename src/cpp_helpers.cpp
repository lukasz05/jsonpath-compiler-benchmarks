extern "C" {
    void free_result_buffer(char* result_buf) {
        delete result_buf;
    }
}