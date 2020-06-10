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
