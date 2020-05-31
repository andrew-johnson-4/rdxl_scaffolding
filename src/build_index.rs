#![feature(proc_macro_hygiene)]
use std::fs::File;
use std::io::prelude::*;
use rdxl::xhtml;
use rdxl_scaffolding::*;

fn main() -> std::io::Result<()> {
   let mut f = File::create("index.html")?;

   f.write_all(b"<html>")?;
   f.write_all(b"<body>")?;
   f.write_all(xhtml!( <br/> ).as_bytes())?;
   f.write_all(b"</body>")?;
   f.write_all(b"</html>")?;

   Ok(())
}
