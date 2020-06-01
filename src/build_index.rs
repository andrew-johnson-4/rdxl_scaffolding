#![feature(proc_macro_hygiene)]
use std::fs::File;
use std::io::prelude::*;
use rdxl::xhtml;
use rdxl_scaffolding::*;

fn main() -> std::io::Result<()> {
   let mut f = File::create("index.html")?;

   f.write_all(b"<html>")?;
   f.write_all(b"<body>")?;

   f.write_all(xhtml!( <!ProgressBar numerator=312 denominator=1532 unit="MB"/> ).as_bytes())?;
   f.write_all(b"<br/>")?;

   f.write_all(xhtml!( <!IndexTabs>
     <!IndexTab name="Tab A"/>
     <!IndexTab name="Tab B"/>
     <!IndexTab name="Tab C"/>
     <!IndexTab name="Tab D"/>
   </IndexTabs> ).as_bytes())?;
   f.write_all(b"<br/>")?;

   f.write_all(xhtml!( <!ContactList>
     <!Contact person=<!Person name=<!Name name="John Dover"/> /> >
       <!Title title="Doctor of Economics"/>
       <!Email email="john.dover@email.com"/>
       <!PhoneNumber number="1-234-567-8901"/>
       <!Website url="https://www.contact.com/johndover"/>
     </Contact>
   </ContactList> ).as_bytes())?;
   f.write_all(b"<br/>")?;

   f.write_all(b"</body>")?;
   f.write_all(b"</html>")?;

   Ok(())
}
