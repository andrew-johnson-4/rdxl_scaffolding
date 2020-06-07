#![feature(proc_macro_hygiene)]
use std::fs::File;
use std::io::prelude::*;
use rdxl::xhtml;
use rdxl_scaffolding::*;

fn main() -> std::io::Result<()> {
   let mut f = File::create("examples/index.html")?;

   f.write_all(b"<!doctype html>")?;
   f.write_all(br#"<html lang="en">"#)?;
   f.write_all(b"<head>")?;
   f.write_all(br#"<meta charset="utf-8">"#)?;
   f.write_all(br#"<meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">"#)?;
   f.write_all(br#"<link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css" integrity="sha384-ggOyR0iXCbMQv3Xipma34MD+dH/1fQ784/j6cY/iJTQUOhcWr7x9JvoRxT2MZw1T" crossorigin="anonymous">"#)?;
   f.write_all(b"<title>Example HTML</title>")?;
   f.write_all(b"</head>")?;
   f.write_all(b"<body>")?;
   f.write_all(br#"<div class="container">"#)?;

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
     <!IndexTab name="Miscellaneous Elements">
       <?>
         <h3>Miscellaneous Elements</h3>
         <p>Progress Bar: <!ProgressBar numerator=312 denominator=1532 unit="MB"/></p>
       </?>
     </IndexTab>
     <!IndexTab name="Dataset Elements">
       <?>
         <h3>Dataset Elements</h3>
         <p>Contact List: <!ContactList>
           <!Contact person=<!Person name=<!Name name="John Dover"/> /> >
             <!Title title="Doctor of Economics"/>
             <!Email email="john.dover@email.com"/>
             <!PhoneNumber number="1-234-567-8901"/>
             <!Website url="https://www.contact.com/johndover"/>
           </Contact>
           <!Contact person=<!Person name=<!Name name="Jane Doe"/> /> >
             <!PhoneNumber number="987.654.3210"/>
          </Contact>
        </ContactList></p>
        <p>Address List: <!AddressList>
          <!Address addressee="Jane Doe">
            <!AddressLine1 value="222 Second Street"/>
            <!AddressLine2 value="Apartment B"/>
            <!City name="Dover"/>
            <!State name="Massachusetts"/>
            <!PostalCode><!ZipCode code="02030"/></PostalCode>
          </Address>
        </AddressList></p>
       </?>
     </IndexTab>
     <!IndexTab name="Tab D">
       <?>
         <h3>Tab D</h3>
       </?>
     </IndexTab>
   </IndexTabs> ).as_bytes())?;

   f.write_all(b"</div>")?;
   f.write_all(b"</body>")?;
   f.write_all(b"</html>")?;

   Ok(())
}
