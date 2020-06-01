#![feature(proc_macro_hygiene)]
use std::fs::File;
use std::io::prelude::*;
use rdxl::xhtml;
use rdxl_scaffolding::*;
//use rdxl_scaffolding::html::*;

fn main() -> std::io::Result<()> {
   let mut f = File::create("index.html")?;

   f.write_all(b"<html>")?;
   f.write_all(b"<body>")?;
   f.write_all(xhtml!( <!ProgressBar numerator=312 denominator=1532 unit="MB"/> ).as_bytes())?;
   f.write_all(b"</body>")?;
   f.write_all(b"</html>")?;

   Ok(())
}
