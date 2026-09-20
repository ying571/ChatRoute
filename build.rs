fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        println!(
            "cargo:rustc-link-arg-bin=chatroute=/MANIFESTINPUT:packaging/windows/chatroute.exe.manifest"
        );
        println!("cargo:rustc-link-arg-bin=chatroute=/MANIFEST:EMBED");
        println!("cargo:rerun-if-changed=packaging/windows/chatroute.rc");
        println!("cargo:rerun-if-changed=packaging/icons/AppIcon.ico");
        embed_resource::compile("packaging/windows/chatroute.rc", embed_resource::NONE)
            .manifest_optional()
            .unwrap();
    }
}
