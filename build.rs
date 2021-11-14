extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
	let outpath = PathBuf::from(env::var("OUT_DIR").unwrap());

	// Build library
	cc::Build::new()
		.file("src/ytnef.c")
		.warnings(false)
		.compile("libytnef.a");
	
	// Tell cargo/rustc to link against our new library
	println!("cargo:rustc-link-lib=static=ytnef");
	
	// Generate Rust bindings to C library
	bindgen::Builder::default()
		.header("stdio.h")
		.header("sys/types.h")
		.header("src/mapi.h")
		.header("src/mapidefs.h")
		.header("src/mapitags.h")
		.header("src/tnef-errors.h")
		.header("src/tnef-types.h")
		.header("src/ytnef.h")
		.allowlist_type("Attachment")
		.allowlist_type("MAPIProperty")
		.allowlist_type("MAPIPropertyTagList")
		.allowlist_type("MAPIProps")
		.allowlist_type("TNEFFileInfo")
		.allowlist_type("TNEFHandler")
		.allowlist_type("TNEFMemInfo")
		.allowlist_type("TNEFStruct")
		.allowlist_function("MAPIFindProperty")
		.allowlist_function("MAPIFindUserProp")
		.allowlist_function("MAPIPrint")
		.allowlist_function("MAPISysTimetoDTR")
		.allowlist_function("SwapDDWord")
		.allowlist_function("SwapDWord")
		.allowlist_function("SwapWord")
		.allowlist_function("TNEFCheckForSignature")
		.allowlist_function("TNEFFree")
		.allowlist_function("TNEFFreeAttachment")
		.allowlist_function("TNEFFreeMapProps")
		.allowlist_function("TNEFInitAttachment")
		.allowlist_function("TNEFInitMapi")
		.allowlist_function("TNEFInitialize")
		.allowlist_function("TNEFParse")
		.allowlist_function("TNEFParseFile")
		.allowlist_function("TNEFParseMemory")
		.allowlist_function("TNEFPrintDate")
		.generate()
		.expect("Failed to generate bindings")
		.write_to_file(outpath.join("bindings.rs"))
		.expect("Failed to write bindings");
}
