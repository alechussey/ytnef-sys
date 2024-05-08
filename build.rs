extern crate cc;

fn main() {
	// Build library
	cc::Build::new()
		.file("src/ytnef.c")
		.warnings(false)
		.compile("libytnef.a");
	
	// Tell cargo/rustc to link against our new library
	println!("cargo:rustc-link-lib=static=ytnef");
}
