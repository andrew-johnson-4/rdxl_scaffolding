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
  /** Alert renders an alert message or element at the top of the page */
  <!Alert message:String><?/></Alert>
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

xtype!(<!City name:String/>);
xtype!(<!State name:String/>);
xtype!(<!Country name:String/>);
xtype!(<!ZipCode code:String/>);
xtype!(<!PostalCode><?ZipCode/></PostalCode>);

xtype!(<!Address addressee:String>
   <!AddressLine1 value:String/>
   <!AddressLine2 value:String/>
   <?City/>
   <?State/>
   <?PostalCode/>
   <?Country/>
</Address>);
xtype!(<!AddressList><?Address/></AddressList>);

xtype!(<!Date year:u64 month:u64 day:u64/>);
xtype!(<!Time hour:u64 minute:u64 second:u64/>);
xtype!(<!DateTime date:Date time:Time/>);
