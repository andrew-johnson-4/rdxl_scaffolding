xrender!(ProgressBar, <div style="position:relative; height:30px; width:300px; background-color:#CCCCCC;">
  <div style={{ format!("\"position:absolute; top:0; left:0; height:30px; width:{}px; background-color:#999999;\"", self.numerator*300/self.denominator ) }}></div>
  <div style="line-height:30px; width:300px; text-align:center; font-family:sans;">{{ self.numerator }} / {{ self.denominator }} {{ self.unit }}</div>
</div>);

xrender!(City, {{self.name}});
xrender!(State, {{self.name}});
xrender!(Country, {{self.name}});
xrender!(ZipCode, {{self.code}});
xrender!(PostalCode, {{ for pc in self.children.iter() {{
   {{ let PostalCodeChildren::ZipCode(z) = pc; }}
   {{ z }}
}} }});

xrender!(AddressList, <ul>
  {{ for AddressListChildren::Address(a) in self.children.iter() {{ <li>{{a}}</li> }} }}
</ul>);

xrender!(Address, <span>
   <b>{{ self.addressee }}</b><br/>
   {{ for ch in self.children.iter() {{
      {{ if let AddressChildren::AddressLine1(l) = ch {{
        {{ l.value }}<br/>
      }} }}
   }} }}
   {{ for ch in self.children.iter() {{
      {{ if let AddressChildren::AddressLine2(l) = ch {{
        {{ l.value }}<br/>
      }} }}
   }} }}
   {{ for ch in self.children.iter() {{
      {{ if let AddressChildren::City(c) = ch {{
        {{ c }},
      }} }}
   }} }}
   {{ for ch in self.children.iter() {{
      {{ if let AddressChildren::State(s) = ch {{
        {{ s }},
      }} }}
   }} }}
   {{ for ch in self.children.iter() {{
      {{ if let AddressChildren::Country(c) = ch {{
        {{ c }}
      }} }}
   }} }}
   {{ for ch in self.children.iter() {{
      {{ if let AddressChildren::PostalCode(p) = ch {{
        {{ p }}
      }} }}
   }} }}
</span>);

xrender!(ContactList, <ul>
  {{ for ContactListChildren::Contact(c) in self.children.iter() {{ <li>{{c}}</li> }} }}
</ul>);

xrender!(Contact, <span>
   <b>{{ self.name }}</b>:
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

