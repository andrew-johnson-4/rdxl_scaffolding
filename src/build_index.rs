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
     <!IndexTab name="Input Elements">
       <?>
         <h2>Input Elements</h2>
       </?>
     </IndexTab>
     <!IndexTab name="Tab B">
       <?>
         <h2>Tab B</h2>
       </?>
     </IndexTab>
     <!IndexTab name="Tab C">
       <?>
         <h2>Tab C</h2>
       </?>
     </IndexTab>
     <!IndexTab name="Tab D">
       <?>
         <h2>Tab D</h2>
       </?>
     </IndexTab>
   </IndexTabs> ).as_bytes())?;
   f.write_all(b"<br/>")?;

   f.write_all(xhtml!( <!ContactList>
     <!Contact person=<!Person name=<!Name name="John Dover"/> /> >
       <!Title title="Doctor of Economics"/>
       <!Email email="john.dover@email.com"/>
       <!PhoneNumber number="1-234-567-8901"/>
       <!Website url="https://www.contact.com/johndover"/>
     </Contact>
     <!Contact person=<!Person name=<!Name name="Jane Doe"/> /> >
       <!PhoneNumber number="987.654.3210"/>
     </Contact>
   </ContactList> ).as_bytes())?;
   f.write_all(b"<br/>")?;

   f.write_all(b"</body>")?;
   f.write_all(b"</html>")?;

   Ok(())
}
