use rdxl::xrender;
use crate::*;
use std::time::SystemTime;

fn unique_identifier() -> String {
   let e = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
   if let Ok(e) = e {
      format!("_uid_{}_{}", e.as_secs(), e.subsec_nanos())
   } else {
      format!("_uid_error")
   }
}

xrender!(ProgressBar, <div style="position:relative; height:30px; width:300px; background-color:#CCCCCC;">
  <div style={{ format!("\"position:absolute; top:0; left:0; height:30px; width:{}px; background-color:#999999;\"", self.numerator*300/self.denominator ) }}></div>
  <div style="line-height:30px; width:300px; text-align:center; font-family:sans;">{{ self.numerator }} / {{ self.denominator }} {{ self.unit }}</div>
</div>);

xrender!(IndexTabs, <div>
  {{ let index_tab_unique_classname = unique_identifier(); }}
  {{ for tab in self.children.iter() {{
    {{ if let IndexTabsChildren::IndexTab(tab) = tab {{
      <div style="float:left; margin-left:5px; background-color:#CCCCCC;"
           onclick="document.getElementsByClassName();">{{ tab.name }}</div>
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
