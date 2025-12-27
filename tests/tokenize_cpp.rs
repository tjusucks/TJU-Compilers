use rustcc::cpp::lexer::ffi;

/// Test the C++ tokenizer via FFI for the lexer defined in:
/// https://github.com/cbx6666/Compilers/blob/26218517fd8d68e2f61874c8ec1136a681035b0d/lexer-generator/generated_lexer.cpp
#[test]
fn tokenize_cpp() {
    let test_code = r#"int x = 123;
        float y = 45.67;
        if (x > 100) {
        return x + y;
        }
        // This is a comment.
        string name = "hello";"#;

    let tokens = ffi::tokenize(test_code);
    for (index, token) in tokens.iter().enumerate() {
        println!(
            "[{}] kind: {:?}, value: {:?}, line: {}, column: {}",
            index + 1,
            token.get_kind(),
            token.get_value(),
            token.get_line(),
            token.get_column()
        );
    }
}
