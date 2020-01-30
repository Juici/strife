use std::env;

fn feature_enabled(feature: &str) -> bool {
    let var_name = format!("CARGO_FEATURE_{}", feature.to_uppercase().replace("-", "_"));
    env::var_os(var_name).is_some()
}

macro_rules! warn {
    ($s:literal) => {
        println!(concat!("cargo:warning=", $s));
    };
}

fn main() {
    if feature_enabled("rustls-tls") && feature_enabled("native-tls") {
        warn!("both `rustls-tls` and `native-tls` features are enabled, but only one can be used (using `rustls-tls`)");
    }
}
