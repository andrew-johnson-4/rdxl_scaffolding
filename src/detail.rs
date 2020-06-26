use rdxl::{xtype,xrender};

xtype!(
  /** BreadCrumb renders a series of crumbs leading to the current location path */
  <!BreadCrumb active:String><!BreadCrumbItem name:String><?/></BreadCrumbItem></BreadCrumb>
);

xtype!(
  /** ProgressBar renders a progress bar */
  <!ProgressBar numerator:u64 denominator:u64 unit:String/>
);

xrender!(ProgressBar, <div style="position:relative; height:30px; width:300px; background-color:#CCCCCC;">
  <div style={{ format!("\"position:absolute; top:0; left:0; height:30px; width:{}px; background-color:#999999;\"", self.numerator*300/self.denominator ) }}></div>
  <div style="line-height:30px; width:300px; text-align:center; font-family:sans;">{{ self.numerator }} / {{ self.denominator }} {{ self.unit }}</div>
</div>);
