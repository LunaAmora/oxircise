// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        #[cfg(wasm)]
        web_sys::console::log_1(&format!( $( $t )* ).into());

        #[cfg(not(wasm))]
        println!($( $t )*);
    }
}
