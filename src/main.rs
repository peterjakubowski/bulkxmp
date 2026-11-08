use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use xmpkit::XmpMeta;

fn main() -> Result<(), Box<dyn Error>> {
    let template_path: &Path = Path::new("bulk_metadata_template.xmp");
    let template_string = read_to_string(template_path)?;
    let xmp_meta: XmpMeta = XmpMeta::parse(&template_string)?;
    println!("{:?}", xmp_meta.all_properties());
    Ok(())
}
