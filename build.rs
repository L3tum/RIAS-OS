use std::{env, fs::File, io::Write, path::Path};
use std::collections::HashMap;

fn main() {
    let defaults: HashMap<&str, i32> = [
        ("TIME_INTERRUPT_PER_SECOND", 100)
    ].iter().cloned().collect();
    let mut hm = HashMap::new();
    let out_dir = env::var("OUT_DIR").expect("No out dir");
    let dest_path = Path::new(&out_dir).join("constants.rs");
    let mut f = File::create(&dest_path).expect("Could not create file");

    hm.insert("TIME_INTERRUPT_PER_SECOND", option_env!("TIME_INTERRUPT_PER_SECOND"));

    for kvp in hm {
        let var = kvp.1;
        let var = var.map_or(Ok(defaults[kvp.0]), str::parse).expect(&*format!("Missing env variable {}", kvp.0));
        write!(&mut f, "const {}: u32 = {};", kvp.0, var);
        println!("cargo:rerun-if-env-changed={}", kvp.0);
    }
}