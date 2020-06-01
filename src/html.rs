use std::fmt;
use rdxl::xrender;
use crate::*;

xrender!(ProgressBar, <div style="position:relative; height:30px; width:300px; background-color:#CCCCCC;">
  <div style={{ format!("\"position:absolute; top:0; left:0; height:30px; width:{}px; background-color:#999999;\"", self.numerator*300/self.denominator ) }}></div>
  <div style="line-height:30px; width:300px; text-align:center; font-family:sans;">{{ self.numerator }} / {{ self.denominator }} {{ self.unit }}</div>
</div>);

xrender!(IndexTabs, <div>
  {{ for tab in self.children.iter() {{
    {{ if let IndexTabsChildren::IndexTab(tab) = tab {{
      <div style="float:left; margin-left:5px; background-color:#CCCCCC;">{{ tab.name }}</div>
    }} }}
  }} }}
</div>);
