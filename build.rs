use std::process::Command;

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    tonic_build::compile_protos("protos/orderbook.proto")?;
    Ok(())
}
