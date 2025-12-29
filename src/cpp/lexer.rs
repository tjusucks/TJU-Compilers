#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("bridge.h");

        type BridgeToken;

        // Bind the C++ function.
        #[must_use]
        fn tokenize(input: &str) -> UniquePtr<CxxVector<BridgeToken>>;

        // Expose the C++ getters to Rust.
        fn get_kind(self: &BridgeToken) -> String;
        fn get_value(self: &BridgeToken) -> String;
        fn get_line(self: &BridgeToken) -> i32;
        fn get_column(self: &BridgeToken) -> i32;
    }
}
