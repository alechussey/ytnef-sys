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
		.whitelist_type("Attachment")
		.whitelist_type("MAPIProperty")
		.whitelist_type("MAPIPropertyTagList")
		.whitelist_type("MAPIProps")
		.whitelist_type("TNEFFileInfo")
		.whitelist_type("TNEFHandler")
		.whitelist_type("TNEFMemInfo")
		.whitelist_type("TNEFStruct")
		.whitelist_function("MAPIFindProperty")
		.whitelist_function("MAPIFindUserProp")
		.whitelist_function("MAPIPrint")
		.whitelist_function("MAPISysTimetoDTR")
		.whitelist_function("SwapDDWord")
		.whitelist_function("SwapDWord")
		.whitelist_function("SwapWord")
		.whitelist_function("TNEFCheckForSignature")
		.whitelist_function("TNEFFree")
		.whitelist_function("TNEFFreeAttachment")
		.whitelist_function("TNEFFreeMapProps")
		.whitelist_function("TNEFInitAttachment")
		.whitelist_function("TNEFInitMapi")
		.whitelist_function("TNEFInitialize")
		.whitelist_function("TNEFParse")
		.whitelist_function("TNEFParseFile")
		.whitelist_function("TNEFParseMemory")
		.whitelist_function("TNEFPrintDate")
		.generate()
		.expect("Failed to generate bindings")
		.write_to_file(outpath.join("bindings.rs"))
		.expect("Failed to write bindings");
}
