//! # Progress Bar component
//!
//! ```
//! # use rdxl_scaffolding::*;
//! # fn main() {
//! ProgressBar { numerator:33, denominator:50, unit:"MB".to_string(), children:vec![] }
//! # ;}
//! ```

use rdxl::xtype;
pub mod html;

xtype!(<!IndexTabs><!IndexTab name:String><?/></IndexTab></IndexTabs>);
xtype!(<!Collapse><!CollapseHead name:String><?/></CollapseHead><!CollapseBody name:String><?/></CollapseBody></Collapse>);
xtype!(<!DropDown><!DropDownHead name:String><?/></DropDownHead><!DropDownBody name:String><?/></DropDownBody></DropDown>);
xtype!(<!Table><!TableRow><!TableCell><?/></TableCell></TableRow></Table>);
xtype!(<!List><!ListItem><?/></ListItem></List>);
xtype!(<!Card><?Image/><?/></Card>);
xtype!(<!Carousel><!CarouselSlide><?Image/><?/></CarouselSlide></Carousel>);

xtype!(<!BarGraph xunit:String yunit:String><!BarGraphItem x:String y:f64/></BarGraph>);
xtype!(<!Histogram xunit:String yunit:String><!HistogramItem xmin:f64 xmax:f64 y:f64/></Histogram>);
xtype!(<!PieChart><!PieChartItem xtag:String y:f64/></PieChart>);
xtype!(<!LineGraph xunit:String yunit:String><!LineGraphLine f:String/></LineGraph>);

xtype!(<!Alert message:String><?/></Alert>);
xtype!(<!Tooltip message:String><?/></Tooltip>);
xtype!(<!BreadCrumb active:String><!BreadCrumbItem name:String><?/></BreadCrumbItem></BreadCrumb>);

xtype!(<!InputButton name:String/>);
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

xtype!(<!ProgressBar numerator:u64 denominator:u64 unit:String/>);
xtype!(<!Image name:String/>);
xtype!(<!Icon name:String/>);
xtype!(<!Badge name:String/>);

xtype!(<!Person name:Name/>);
xtype!(<!Name name:String/>);
xtype!(<!Title title:String/>);
xtype!(<!Email email:String/>);
xtype!(<!PhoneNumber number:String/>);
xtype!(<!Website url:String/>);
xtype!(<!Contact person:Person>
   <?Title/>
   <?Email/>
   <?PhoneNumber/>
   <?Website/>
</Contact>);
xtype!(<!ContactList><?Contact/></ContactList>);

xtype!(<!Date year:u64 month:u64 day:u64/>);
xtype!(<!Time hour:u64 minute:u64 second:u64/>);
xtype!(<!DateTime date:Date time:Time/>);
