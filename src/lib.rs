pub mod bootstrap;
pub mod dom;
pub mod form;
pub mod util;

/*
xtype!(
  /** BarGraph renders enclosed data as a bar graph */
  <!BarGraph xunit:String yunit:String><!BarGraphItem x:String y:u64/></BarGraph>
);
xtype!(
  /** Histogram renders enclosed data as a histogram */
  <!Histogram xunit:String yunit:String><!HistogramItem xmin:u64 xmax:u64 y:u64/></Histogram>
);
xtype!(
  /** PieChart renders enclosed data as a pie chart */
  <!PieChart><!PieChartItem xtag:String y:u64/></PieChart>
);
xtype!(
  /** LineGraph renders enclosed data as a line graph */
  <!LineGraph xunit:String yunit:String><!LineGraphLine f:String/></LineGraph>
);

xtype!(
  /** AlertPrimary renders an alert message or element at the top of the page */
  <!AlertPrimary message:String><?/></AlertPrimary>
);
xtype!(
  /** AlertSecondary renders an alert message or element at the top of the page */
  <!AlertSecondary message:String><?/></AlertSecondary>
);
xtype!(
  /** AlertSuccess renders an alert message or element at the top of the page */
  <!AlertSuccess message:String><?/></AlertSuccess>
);
xtype!(
  /** AlertDanger renders an alert message or element at the top of the page */
  <!AlertDanger message:String><?/></AlertDanger>
);
xtype!(
  /** AlertWarning renders an alert message or element at the top of the page */
  <!AlertWarning message:String><?/></AlertWarning>
);
xtype!(
  /** AlertInfo renders an alert message or element at the top of the page */
  <!AlertInfo message:String><?/></AlertInfo>
);
xtype!(
  /** AlertLight renders an alert message or element at the top of the page */
  <!AlertLight message:String><?/></AlertLight>
);
xtype!(
  /** AlertDark renders an alert message or element at the top of the page */
  <!AlertDark message:String><?/></AlertDark>
);

xtype!(
  /** BreadCrumb renders a series of crumbs leading to the current location path */
  <!BreadCrumb active:String><!BreadCrumbItem name:String><?/></BreadCrumbItem></BreadCrumb>
);

xtype!(
  /** ProgressBar renders a progress bar */
  <!ProgressBar numerator:u64 denominator:u64 unit:String/>
);
xtype!(
  /** Image renders an inline image */
  <!Image name:String/>
);
xtype!(
  /** Icon renders an inline icon */
  <!Icon name:String/>
);
xtype!(
  /** Badge renders an inline badge */
  <!Badge name:String/>
);

xtype!(
  /** Person encapsulates the concept of personhood */
  <!Person name:String/>
);
xtype!(
  /** Name encapsulates the concept of the name of something */
  <!Name name:String/>
);
xtype!(
  /** Title encapsulates the concept of the title of something */
  <!Title title:String/>
);
xtype!(
  /** Email associates an email address with something */
  <!Email email:String/>
);
xtype!(
  /** PhoneNumber associates a phone number with something */
  <!PhoneNumber number:String/>
);
xtype!(
  /** Website associates a website with something */
  <!Website url:String/>
);
xtype!(
  /** Contact aggregates the contact information of a person */
  <!Contact name:String>
    <?Title/>
    <?Email/>
    <?PhoneNumber/>
    <?Website/>
 </Contact>
);
xtype!(
  /** ContactList aggregates contact information of people */
  <!ContactList><?Contact/></ContactList>
);

xtype!(
  /** City encapsulates the concept of a city */
  <!City name:String/>
);
xtype!(
  /** State encapsulates the concept of a state */
  <!State name:String/>
);
xtype!(
  /** Country encapsulates the concept of a country */
  <!Country name:String/>
);
xtype!(
  /** ZipCode encapsulates the concept of a zip code */
  <!ZipCode code:String/>
);
xtype!(
  /** PostalCode encapsulates the concept of a postal code */
  <!PostalCode><?ZipCode/></PostalCode>
);

xtype!(
  /** Address encapsulates the concept of an address */
  <!Address addressee:String>
    <!AddressLine1 value:String/>
    <!AddressLine2 value:String/>
    <?City/>
    <?State/>
    <?PostalCode/>
    <?Country/>
 </Address>
);
xtype!(
  /** AddressList encapsulates a list of addresses */
  <!AddressList><?Address/></AddressList>
);

xtype!(
  /** Date encapsulates the concept of a dated day */
  <!Date year:u64 month:u64 day:u64/>
);
xtype!(
  /** Time encapsulates the concept of a time of day */
  <!Time hour:u64 minute:u64 second:u64/>
);
*/
