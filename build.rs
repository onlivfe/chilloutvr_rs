fn main() {
	println!("cargo:rerun-if-env-changed=CFG_RELEASE_CHANNEL");
	if channel() == "nightly" {
		println!("cargo:rustc-cfg=nightly")
	}
}

fn channel() -> String {
	if let Ok(channel) = std::env::var("CFG_RELEASE_CHANNEL") {
		channel
	} else {
		"nightly".to_owned()
	}
}
