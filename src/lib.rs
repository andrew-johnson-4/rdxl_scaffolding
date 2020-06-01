use rdxl::xtype;
pub mod html;

xtype!(<!ProgressBar numerator:u64 denominator:u64 unit:String/>);
xtype!(<!IndexTabs><!IndexTab name:String/></IndexTabs>);

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

xtype!(<!Date year:u64 month:u64 day:u64/>);
xtype!(<!Time hour:u64 minute:u64 second:u64/>);
xtype!(<!DateTime date:Date time:Time/>);
