use chromiumoxide_pdl::build::Generator;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
fn main() -> Result<(), Box<dyn Error>> {
    let js_proto = PathBuf::from("./devtools-protocol/pdl/js_protocol.pdl");

    let browser_proto = PathBuf::from("./devtools-protocol/pdl/browser_protocol.pdl");

    Generator::default()
        .out_dir(PathBuf::from("./cdp"))
        .experimental(true)
        .deprecated(true)
        .compile_pdls(&[js_proto, browser_proto])
        .unwrap();

    Ok(())
}
