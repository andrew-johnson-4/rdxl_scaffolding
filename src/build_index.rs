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
         <p>Checkbox: <!InputCheckbox name="Checkbox"/></p>
         <p>Button Group: <!InputButtonGroup>
           <!InputButton name="GroupButton1"/>
           <!InputButton name="GroupButton2"/>
         </InputButtonGroup></p>
         <p>Text Input: <!InputText name="Text"/></p>
         <p>Email Input: <!InputEmail name="Email"/></p>
         <p>Search Input: <!InputSearch name="Search"/></p>
         <p>Password Input: <!InputPassword name="Password"/></p>
         <p>Number Input: <!InputNumber name="Number"/></p>
         <p>Telephone Input: <!InputTelephoneNumber name="Telephone"/></p>
         <p>Url Input: <!InputUrl name="Url"/></p>
         <p>Range Input: <!InputRange name="Range" min=4 max=11/></p>

         <p>Color Input: <!InputColor name="Color"/></p>
         <p>Date Input: <!InputDate name="Date"/></p>
         <p>Datetime Input: <!InputDatetime name="Datetime"/></p>
         <p>Month Input: <!InputMonth name="Month"/></p>
         <p>Week Input: <!InputWeek name="Week"/></p>
         <p>Time Input: <!InputTime name="Time"/></p>

         <p>File Input: <!InputFile name="File"/></p>
         <p>Image Input: <!InputImage name="Image"/></p>

         <p>Radio Input: <!InputRadio name="Radio">
           <!InputRadioOption value="A"/>
           <!InputRadioOption value="B"/>
           <!InputRadioOption value="C"/>
         </InputRadio></p>

         <p>Submit Input: <!InputSubmit/></p>
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
