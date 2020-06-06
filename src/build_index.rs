#![feature(proc_macro_hygiene)]
use std::fs::File;
use std::io::prelude::*;
use rdxl::xhtml;
use rdxl_scaffolding::*;

fn main() -> std::io::Result<()> {
   let mut f = File::create("index.html")?;

   f.write_all(b"<html>")?;
   f.write_all(b"<body>")?;

   f.write_all(xhtml!( <!IndexTabs>
     <!IndexTab name="Input Elements">
       <?>
         <h3>Input Elements</h3>
         <!InputButton name="Button"/>
/*
xtype!(<!InputButtonGroup><?InputButton/></InputButtonGroup>);
xtype!(<!InputCheckbox name:String/>);
xtype!(<!InputColor name:String/>);
xtype!(<!InputDate name:String/>);
xtype!(<!InputDatetime name:String/>);
xtype!(<!InputEmail name:String/>);
xtype!(<!InputFile name:String/>);
xtype!(<!InputImage name:String/>);
xtype!(<!InputMonth name:String/>);
xtype!(<!InputNumber name:String/>);
xtype!(<!InputPassword name:String/>);
xtype!(<!InputRadio name:String><!InputRadioOption value:String/></InputRadio>);
xtype!(<!InputRange name:String min:u64 max:u64/>);
xtype!(<!InputSearch name:String/>);
xtype!(<!InputSubmit/>);
xtype!(<!InputTelephoneNumber name:String/>);
xtype!(<!InputText name:String/>);
xtype!(<!InputTime name:String/>);
xtype!(<!InputUrl name:String/>);
xtype!(<!InputWeek name:String/>);
*/
       </?>
     </IndexTab>
     <!IndexTab name="Tab B">
       <?>
         <h3>Tab B</h3>
       </?>
     </IndexTab>
     <!IndexTab name="Tab C">
       <?>
         <h3>Tab C</h3>
       </?>
     </IndexTab>
     <!IndexTab name="Tab D">
       <?>
         <h3>Tab D</h3>
       </?>
     </IndexTab>
   </IndexTabs> ).as_bytes())?;

   /*
   f.write_all(xhtml!( <!ProgressBar numerator=312 denominator=1532 unit="MB"/> ).as_bytes())?;
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
   */

   f.write_all(b"</body>")?;
   f.write_all(b"</html>")?;

   Ok(())
}
