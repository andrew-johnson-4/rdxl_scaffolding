#![feature(proc_macro_hygiene)]
use std::fs::File;
use std::io::prelude::*;
use rdxl::xhtml;
use rdxl_scaffolding::*;

fn main() -> std::io::Result<()> {
   let mut f = File::create("docs/index.html")?;

   f.write_all(b"<!doctype html>")?;
   f.write_all(br#"<html lang="en">"#)?;
   f.write_all(b"<head>")?;
   f.write_all(br#"<meta charset="utf-8">"#)?;
   f.write_all(br#"<meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">"#)?;
   f.write_all(br#"<link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css" integrity="sha384-ggOyR0iXCbMQv3Xipma34MD+dH/1fQ784/j6cY/iJTQUOhcWr7x9JvoRxT2MZw1T" crossorigin="anonymous">"#)?;
   f.write_all(br#"<script src="https://code.jquery.com/jquery-3.5.1.slim.min.js" integrity="sha384-DfXdz2htPH0lsSSs5nCTpuj/zy4C+OGpamoFVy38MVBnE+IbbVYUew+OrCXaRkfj" crossorigin="anonymous"></script>"#)?;
   f.write_all(br#"<script src="https://cdn.jsdelivr.net/npm/popper.js@1.16.0/dist/umd/popper.min.js" integrity="sha384-Q6E9RHvbIyZFJoft+2mJbHaEWldlvI9IOYy5n3zV9zzTtmI3UksdQRVvoxMfooAo" crossorigin="anonymous"></script>"#)?;
   f.write_all(br#"<script src="https://stackpath.bootstrapcdn.com/bootstrap/4.5.0/js/bootstrap.min.js" integrity="sha384-OgVRvuATP1z7JjHLkuOU7Xw704+h835Lr+6QL9UvYjZE3Ipu6Tp75j7Bh/kR0JKI" crossorigin="anonymous"></script>"#)?;
   f.write_all(b"<title>Example HTML</title>")?;
   f.write_all(b"</head>")?;
   f.write_all(b"<body>")?;
   f.write_all(br#"<div class="container">"#)?;
   f.write_all(br#"<h2><a href="https://crates.io/crates/rdxl">Rdxl Templating Examples</a></h2>"#)?;

   f.write_all(xhtml!( <!IndexTabs>
     <!IndexTab name="Flexible Container Elements">
       <?>
         <h3>Flexible Container Elements</h3>
         <div class="row">
           <div class="col-sm-6">
             <b>Card:</b>
           </div>
           <div class="col-sm-6">
             <b>Carousel:</b>
           </div>
         </div>
         <div class="row">
           <div class="col-sm-3">
             <!Card>
               <!Image name="stanford_bunny.png"/>
               <?><p>The Stanford Bunny is cute!</p></?>
             </Card>
           </div>
           <div class="col-sm-3">
             <!Card>
               <!Image name="icann.png"/>
               <?>
                 <h4>ICANN Headquarters</h4>
                 <p>ICANN is a large organization</p>
               </?>
             </Card>
           </div>
           <div class="col-sm-6">
             <!Carousel>
               <!CarouselSlide><!Image name="stanford_bunny.png"/></CarouselSlide>
               <!CarouselSlide><!Image name="icann.png"/></CarouselSlide>
               <!CarouselSlide><!Image name="stanford_bunny.png"/></CarouselSlide>
               <!CarouselSlide><!Image name="stanford_bunny.png"/></CarouselSlide>
             </Carousel>
           </div>
         </div>
       </?>
     </IndexTab>
     <!IndexTab name="Inline Container Elements">
       <?>
         <h3>Inline Container Elements</h3>
         <p>Table: <!Table>
           <!TableRow>
             <!TableCell><?>A</?></TableCell>
             <!TableCell><?>B</?></TableCell>
             <!TableCell><?>C</?></TableCell>
           </TableRow>
           <!TableRow>
             <!TableCell><?>1</?></TableCell>
             <!TableCell><?>3</?></TableCell>
             <!TableCell><?>2</?></TableCell>
           </TableRow>
           <!TableRow>
             <!TableCell><?>Q</?></TableCell>
             <!TableCell><?>W</?></TableCell>
             <!TableCell><?>E</?></TableCell>
           </TableRow>
         </Table></p>
         <p>List: <!List>
           <!ListItem><?>AB</?></ListItem>
           <!ListItem><?>CD</?></ListItem>
           <!ListItem><?>EF</?></ListItem>
           <!ListItem><?>GH</?></ListItem>
         </List></p>
       </?>
     </IndexTab>
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
          <!Address addressee="John Dover">
            <!AddressLine1 value="12 Third Avenue"/>
            <!City name="Providence"/>
            <!State name="Rhode Island"/>
            <!PostalCode><!ZipCode code="02902"/></PostalCode>
          </Address>
        </AddressList></p>
       </?>
     </IndexTab>
     <!IndexTab name="Graph Elements">
       <?>
         <h3>Graph Elements</h3>
         <p>Bar Graph: <!BarGraph xunit="name" yunit="pairs of shoes">
           <!BarGraphItem x="Joeline" y=7/>
           <!BarGraphItem x="John" y=5/>
           <!BarGraphItem x="Jane" y=9/>
           <!BarGraphItem x="Jennifer" y=3/>
           <!BarGraphItem x="James" y=4/>
         </BarGraph></p>
         <p>Histogram: <!Histogram xunit="seconds" yunit="# of finishers">
           <!HistogramItem xmin=45 xmax=49 y=1/>
           <!HistogramItem xmin=50 xmax=54 y=3/>
           <!HistogramItem xmin=55 xmax=59 y=5/>
           <!HistogramItem xmin=60 xmax=64 y=7/>
           <!HistogramItem xmin=65 xmax=69 y=2/>
           <!HistogramItem xmin=70 xmax=74 y=6/>
           <!HistogramItem xmin=75 xmax=79 y=2/>
         </Histogram></p>
       </?>
     </IndexTab>
   </IndexTabs> ).as_bytes())?;

   f.write_all(b"</div>")?;
   f.write_all(b"</body>")?;
   f.write_all(b"</html>")?;

   Ok(())
}
