use rdxl::{xtype,xrender};

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

xrender!(InputButton, <input type="button" name={{ format!("'{}'",self.name) }} value={{ format!("'{}'",self.name) }}/>);
xrender!(InputCheckbox, <input type="checkbox" name={{ format!("'{}'",self.name) }}/>);
xrender!(InputButtonGroup, <span style="background-color:#CCCCCC;">
  {{ for bc in self.children.iter() {{
    {{ let InputButtonGroupChildren::InputButton(b) = bc; }}
    {{ b }}
  }} }}
</span>);
xrender!(InputRadio, <span style="background-color:#CCCCCC;">
  {{ for rc in self.children.iter() {{
    {{ let InputRadioChildren::InputRadioOption(r) = rc; }}
    <input type="radio" name={{ format!(r#""{}""#, self.name) }} value={{ format!(r#""{}""#, r.value) }}/>
  }} }}
</span>);
xrender!(InputText, <input type="text" name={{ format!("'{}'",self.name) }}/>);
xrender!(InputEmail, <input type="email" name={{ format!("'{}'",self.name) }}/>);
xrender!(InputSearch, <input type="search" name={{ format!("'{}'",self.name) }}/>);
xrender!(InputPassword, <input type="password" name={{ format!("'{}'",self.name) }}/>);
xrender!(InputNumber, <input type="number" name={{ format!("'{}'",self.name) }}/>);
xrender!(InputTelephoneNumber, <input type="tel" name={{ format!("'{}'",self.name) }}/>);
xrender!(InputUrl, <input type="url" name={{ format!("'{}'",self.name) }}/>);
xrender!(InputRange, <input type="range" name={{ format!("'{}'",self.name) }} min={{self.min}} max={{self.max}}/>);
xrender!(InputColor, <input type="color" name={{ format!("'{}'",self.name) }}/>);
xrender!(InputDate, <input type="date" name={{ format!("'{}'",self.name) }}/>);
xrender!(InputDatetime, <input type="datetime-local" name={{ format!("'{}'",self.name) }}/>);
xrender!(InputMonth, <input type="month" name={{ format!("'{}'",self.name) }}/>);
xrender!(InputWeek, <input type="week" name={{ format!("'{}'",self.name) }}/>);
xrender!(InputTime, <input type="time" name={{ format!("'{}'",self.name) }}/>);

xrender!(InputFile, <input type="file" name={{ format!("'{}'",self.name) }}/>);
xrender!(InputImage, <input type="image" name={{ format!("'{}'",self.name) }}/>);

xrender!(InputSubmit, <input type="submit"/>);
