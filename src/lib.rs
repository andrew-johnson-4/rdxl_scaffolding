use rdxl::xtype;
pub mod html;

xtype!(
  /** IndexTabs encapsulates multiple panels of content with tabs to select
      an active panel */
  <!IndexTabs><!IndexTab name:String><?/></IndexTab></IndexTabs>
);
xtype!(
  /** Collapse encapsulates a single panel of content that slides up or down 
      as inline content */
  <!Collapse><!CollapseHead name:String><?/></CollapseHead><!CollapseBody name:String><?/></CollapseBody></Collapse>
);
xtype!(
  /** Dropdown implements a dropdown menu */
  <!DropDown><!DropDownHead name:String><?/></DropDownHead><!DropDownBody name:String><?/></DropDownBody></DropDown>
);
xtype!(
  /** Table encapsulates tabular data and elements */
  <!Table><!TableRow><!TableCell><?/></TableCell></TableRow></Table>
);
xtype!(
  /** List encapsulates list data and elements */
  <!List><!ListItem><?/></ListItem></List>
);
xtype!(
  /** Card renders an image with associated with elements */
  <!Card><?Image/><?/></Card>
);
xtype!(
  /** Carousel renders a revolving carousel of images or cards */
  <!Carousel><!CarouselSlide><?Image/><?Card/></CarouselSlide></Carousel>
);

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
  /** Tooltip renders a tip message or element at the cursor */
  <!Tooltip message:String><?/></Tooltip>
);
xtype!(
  /** BreadCrumb renders a series of crumbs leading to the current location path */
  <!BreadCrumb active:String><!BreadCrumbItem name:String><?/></BreadCrumbItem></BreadCrumb>
);

xtype!(
  /** InputButton renders a button element */
  <!InputButton name:String/>
);
xtype!(
  /** InputButtonGroup renders a group of button elements */
  <!InputButtonGroup><?InputButton/></InputButtonGroup>
);
xtype!(
  /** InputCheckbox renders a checkbox element */
  <!InputCheckbox name:String/>
);
xtype!(
  /** InputColor renders a color picker element */
  <!InputColor name:String/>
);
xtype!(
  /** InputDate renders a date input element */
  <!InputDate name:String/>
);
xtype!(
  /** InputDatetime renders a datetime input element */
  <!InputDatetime name:String/>
);
xtype!(
  /** InputEmail renders an email input element */
  <!InputEmail name:String/>
);
xtype!(
  /** InputFile renders a file input element */
  <!InputFile name:String/>
);
xtype!(
  /** InputImage renders an image input element */
  <!InputImage name:String/>
);
xtype!(
  /** InputMonth renders a month input element */
  <!InputMonth name:String/>
);
xtype!(
  /** InputNumber renders a number input element */
  <!InputNumber name:String/>
);
xtype!(
  /** InputPassword renders a password input element */
  <!InputPassword name:String/>
);
xtype!(
  /** InputRadio renders a group of radio input elements */
  <!InputRadio name:String><!InputRadioOption value:String/></InputRadio>
);
xtype!(
  /** InputRange renders a range slider element */
  <!InputRange name:String min:u64 max:u64/>
);
xtype!(
  /** InputSearch renders a search input element */
  <!InputSearch name:String/>
);
xtype!(
  /** InputSubmit renders a form submit element */
  <!InputSubmit/>
);
xtype!(
  /** InputTelephoneNumber renders a telephone input element */
  <!InputTelephoneNumber name:String/>
);
xtype!(
  /** InputText renders a text input element */
  <!InputText name:String/>
);
xtype!(
  /** InputTime renders a time input element */
  <!InputTime name:String/>
);
xtype!(
  /** InputUrl renders a url input element */
  <!InputUrl name:String/>
);
xtype!(
  /** InputWeek renders a week input element */
  <!InputWeek name:String/>
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
  <!Person name:Name/>
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
  <!Contact person:Person>
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
xtype!(
  /** DateTime encapsulates the concept of a dated day and a time of day */
  <!DateTime date:Date time:Time/>
);
