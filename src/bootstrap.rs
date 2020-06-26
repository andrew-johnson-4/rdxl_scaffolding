use rdxl::{xtype,xrender};

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

fn show_hide_js(class_name:&str, id_name:&str) -> String {
   format!(r#"'
let cs = document.getElementsByClassName("{}");
for(var ci=0; ci<cs.length; ci++){{ cs[ci].style.display = "none"; }}
document.getElementById("{}").style.display = "initial";'"#, class_name, id_name)
}

xrender!(AlertPrimary, <div class="alert alert-primary alert-dismissible fade show" role="alert">
  {{ self.message }}
  <button type="button" class="close" "data-dismiss"="alert" "aria-label"="Close">
    <span "aria-hidden"="true">&times;</span>
  </button>
</div>);
xrender!(AlertSecondary, <div class="alert alert-secondary alert-dismissible fade show" role="alert">
  {{ self.message }}
  <button type="button" class="close" "data-dismiss"="alert" "aria-label"="Close">
    <span "aria-hidden"="true">&times;</span>
  </button>
</div>);
xrender!(AlertSuccess, <div class="alert alert-success alert-dismissible fade show" role="alert">
  {{ self.message }}
  <button type="button" class="close" "data-dismiss"="alert" "aria-label"="Close">
    <span "aria-hidden"="true">&times;</span>
  </button>
</div>);
xrender!(AlertDanger, <div class="alert alert-danger alert-dismissible fade show" role="alert">
  {{ self.message }}
  <button type="button" class="close" "data-dismiss"="alert" "aria-label"="Close">
    <span "aria-hidden"="true">&times;</span>
  </button>
</div>);
xrender!(AlertWarning, <div class="alert alert-warning alert-dismissible fade show" role="alert">
  {{ self.message }}
  <button type="button" class="close" "data-dismiss"="alert" "aria-label"="Close">
    <span "aria-hidden"="true">&times;</span>
  </button>
</div>);
xrender!(AlertInfo, <div class="alert alert-info alert-dismissible fade show" role="alert">
  {{ self.message }}
  <button type="button" class="close" "data-dismiss"="alert" "aria-label"="Close">
    <span "aria-hidden"="true">&times;</span>
  </button>
</div>);
xrender!(AlertLight, <div class="alert alert-light alert-dismissible fade show" role="alert">
  {{ self.message }}
  <button type="button" class="close" "data-dismiss"="alert" "aria-label"="Close">
    <span "aria-hidden"="true">&times;</span>
  </button>
</div>);
xrender!(AlertDark, <div class="alert alert-dark alert-dismissible fade show" role="alert">
  {{ self.message }}
  <button type="button" class="close" "data-dismiss"="alert" "aria-label"="Close">
    <span "aria-hidden"="true">&times;</span>
  </button>
</div>);

xrender!(Image, <img src={{ format!("'{}'", self.name) }} style="width:100%"/>);
xrender!(Card, <div class="card">
  {{ for im in self.children.iter() {{
    {{ if let CardChildren::Image(im) = im {{
      <img src={{ format!("'{}'", im.name) }} class="card-img-top"/>
    }} }}
  }} }}
  <div class="card-body">
    {{ for d in self.children.iter() {{
      {{ if let CardChildren::Display(d) = d {{
        {{ d }}
      }} }}
    }} }}
  </div>
</div>);

xrender!(Carousel, 
{{ let carousel_id = crate::util::unique_identifier("carousel"); }}
{{ let carousel_idh = format!("'#{}'", carousel_id); }}
<div id={{ carousel_id }} class="carousel slide" "data-ride"="carousel">
  <div class="carousel-inner">
    {{ for (i,cs) in self.children.iter().enumerate() {{
      {{ let CarouselChildren::CarouselSlide(cs) = cs; }}
      <div class={{ format!("'carousel-item{}'", if i==0 {" active"} else {""}) }}>
        {{ for cc in cs.children.iter() {{
          {{ if let CarouselSlideChildren::Image(ci) = cc {{
            <img src={{ format!("'{}'", ci.name) }} class="d-block w-100"/>
          }} else if let CarouselSlideChildren::Card(cc) = cc {{
            {{ cc }}
          }} }}
        }} }}
      </div>
    }} }}
  </div>
  <a class="carousel-control-prev" href={{ carousel_idh }} role="button" "data-slide"="prev">
    <span class="carousel-control-prev-icon" "aria-hidden"="true"></span>
    <span class="sr-only">Previous</span>
  </a>
  <a class="carousel-control-next" href={{ carousel_idh }} role="button" "data-slide"="next">
    <span class="carousel-control-next-icon" "aria-hidden"="true"></span>
    <span class="sr-only">Next</span>
  </a>
</div>);

xrender!(Table, <table class="table">
  <tbody>
    {{ for tr in self.children.iter() {{
      {{ let TableChildren::TableRow(tr) = tr; }}
      {{ tr }}
    }} }}
  </tbody>
</table>);
xrender!(TableRow, <tr>
  {{ for tc in self.children.iter() {{
    {{ let TableRowChildren::TableCell(tc) = tc; }}
    {{ tc }}
  }} }}
</tr>);
xrender!(TableCell, <td>
  {{ for d in self.children.iter() {{
    {{ let TableCellChildren::Display(d) = d; }}
    {{ d }}
  }} }}
</td>);

xrender!(List, <ul class="list-group">
  {{ for li in self.children.iter() {{
    {{ let ListChildren::ListItem(li) = li; }}
    {{ li }}
  }} }}
</ul>);
xrender!(ListItem, <li class="list-group-item">
  {{ for d in self.children.iter() {{
    {{ let ListItemChildren::Display(d) = d; }}
    {{ d }}
  }} }}
</li>);

xrender!(IndexTabs, <div style="padding-top:10px;">
  {{ let tabs_classname = crate::util::unique_identifier("index_tabs"); }}
  {{ for (tabi,tab) in self.children.iter().enumerate() {{
    {{ let IndexTabsChildren::IndexTab(tab) = tab }}
    {{ let tab_id = format!("{}_{}", tabs_classname, tabi); }}
    <div style="float:left; margin-right:3px; padding:4px; border:2px solid #EEEEEE; border-radius:5px 5px 0 0; cursor:pointer;"
         onclick={{ show_hide_js(&tabs_classname, &tab_id) }}>
         {{ tab.name }}
    </div>
  }} }}
  <div style="clear:both;"></div>
  <div style="padding:10px; background-color:#F0F0F0;">
    {{ for (tabi,tab) in self.children.iter().enumerate() {{
      {{ let tab_id = format!("{}_{}", tabs_classname, tabi); }}
      {{ let IndexTabsChildren::IndexTab(tab) = tab; }}
      <div id={{tab_id}} class={{tabs_classname}} style={{ if tabi==0 {{"''"}} else {{"'display:none'"}} }}>
        {{ for tc in tab.children.iter() {{
          {{ let IndexTabChildren::Display(d) = tc; }}
          {{ d }}
        }} }}
      </div>
    }} }}
  </div>
</div>);

