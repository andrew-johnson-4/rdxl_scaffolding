use rdxl::xrender;
use crate::*;
use std::time::SystemTime;

xrender!(InputButton, <input type="button" name={{ format!("'{}'",self.name) }} value={{ format!("'{}'",self.name) }}/>);
/*
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
*/

fn unique_identifier(prefix: &str) -> String {
   let e = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
   if let Ok(e) = e {
      format!("_uid_{}_{}_{}", prefix, e.as_secs(), e.subsec_nanos())
   } else {
      format!("_uid_{}_error", prefix)
   }
}

fn show_hide_js(class_name:&str, id_name:&str) -> String {
   format!(r#"'
let cs = document.getElementsByClassName("{}");
for(var ci=0; ci<cs.length; ci++){{ cs[ci].style.display = "none"; }}
document.getElementById("{}").style.display = "initial";'"#, class_name, id_name)
}

xrender!(ProgressBar, <div style="position:relative; height:30px; width:300px; background-color:#CCCCCC;">
  <div style={{ format!("\"position:absolute; top:0; left:0; height:30px; width:{}px; background-color:#999999;\"", self.numerator*300/self.denominator ) }}></div>
  <div style="line-height:30px; width:300px; text-align:center; font-family:sans;">{{ self.numerator }} / {{ self.denominator }} {{ self.unit }}</div>
</div>);

xrender!(IndexTabs, <div>
  {{ let tabs_classname = unique_identifier("index_tabs"); }}
  {{ for (tabi,tab) in self.children.iter().enumerate() {{
    {{ if let IndexTabsChildren::IndexTab(tab) = tab {{
      {{ let tab_id = format!("{}_{}", tabs_classname, tabi); }}
      <div style="float:left; margin-left:5px; padding:4px; cursor:pointer; background-color:#CCCCCC;"
           onclick={{ show_hide_js(&tabs_classname, &tab_id) }}>
           {{ tab.name }}
      </div>
    }} }}
  }} }}
  <div style="clear:both;"></div>
  {{ for (tabi,tab) in self.children.iter().enumerate() {{
    {{ let tab_id = format!("{}_{}", tabs_classname, tabi); }}
    {{ if let IndexTabsChildren::IndexTab(tab) = tab {{
      <div id={{tab_id}} class={{tabs_classname}} style={{ if tabi==0 {{"''"}} else {{"'display:none'"}} }}>
        {{ for tc in tab.children.iter() {{
          {{ if let IndexTabChildren::Display(d) = tc {{
            {{ d }}
          }} }}
        }} }}
      </div>
    }} }}  
  }} }}
</div>);

xrender!(ContactList, <ul>
  {{ for ContactListChildren::Contact(c) in self.children.iter() {{ <li>{{c}}</li> }} }}
</ul>);

xrender!(Contact, <span>
   <b>{{ self.person.name.name }}</b>:
   {{ for ch in self.children.iter() {{
      {{ if let ContactChildren::Title(t) = ch {{
        {{ t.title }},
      }} }}
   }} }}
   {{ for ch in self.children.iter() {{
      {{ if let ContactChildren::Email(e) = ch {{
        e-mail: {{ e.email }}
      }} }}
   }} }}
   {{ for ch in self.children.iter() {{
      {{ if let ContactChildren::PhoneNumber(p) = ch {{
        phone: {{ p.number }}
      }} }}
   }} }}
   {{ for ch in self.children.iter() {{
      {{ if let ContactChildren::Website(w) = ch {{
        website: {{ w.url }}
      }} }}
   }} }}
</span>);
