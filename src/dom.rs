use rdxl::{xtype};

xtype!(
   <!Script async_:bool={{false}} charset:String={{"UTF-8".to_string()}} defer:bool={{false}}
            src:String={{"".to_string()}} type_:String={{"application/javascript".to_string()}}>
     <?/>
   </Script>
);

xtype!(
  <!Html lang:String={{"en".to_string()}} doctype:String={{"html".to_string()}} xmlns:String={{"".to_string()}}>
    <?Script/>
  </Html>
);
