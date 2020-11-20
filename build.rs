fn main() {
    println!("cargo:rerun-if-file-changed=config.json");
    let config: Config = serde_json::from_str(include_str!("config.json")).unwrap();
    uneval::to_out_dir(config, "config/config_data.rs");
}