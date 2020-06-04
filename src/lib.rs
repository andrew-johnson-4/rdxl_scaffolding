//! # Progress Bar component
//!
//! ```
//! # use rdxl_scaffolding::*;
//! # fn main() {
//! ProgressBar { numerator:33, denominator:50, unit:"MB".to_string(), children:vec![] }
//! # ;}
//! ```

use rdxl::xtype;

#[cfg(feature = "debug_html")]
pub mod html;

xtype!(<!IndexTabs><!IndexTab name:String><?/></IndexTab></IndexTabs>);
xtype!(<!Table><!TableRow><!TableCell><?/></TableCell></TableRow></Table>);
xtype!(<!List><!ListItem><?/></ListItem></List>);
xtype!(<!BarGraph xunit:String yunit:String><!BarGraphItem x:String y:f64/></BarGraph>);
xtype!(<!Histogram xunit:String yunit:String><!HistogramItem xmin:f64 xmax:f64 y:f64/></Histogram>);
xtype!(<!PieChart><!PieChartItem xtag:String y:f64/></PieChart>);
xtype!(<!LineGraph xunit:String yunit:String><!LineGraphLine f:String/></LineGraph>);
xtype!(<!ProgressBar numerator:u64 denominator:u64 unit:String/>);


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
