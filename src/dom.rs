use rdxl::{xtype,xrender};

xtype!(
   <!Script async_:bool={{false}} charset:String={{"UTF-8".to_string()}} defer:bool={{false}}
            src:String={{"".to_string()}} type_:String={{"application/javascript".to_string()}}>
     <?/>
   </Script>
);
xrender!(Script,<script {{if self.async_ {{async}}}} charset={{self.charset}}
                        {{if self.defer {{defer}}}}
                        src={{self.src}} type={{self.type_}}>
   {{ for cd in self.children.iter() {{
      {{ let ScriptChildren::Display(cd) = cd; }}
      {{cd}}
   }} }}
</script>);

xtype!(
  <!Html lang:String={{"en".to_string()}} doctype:String={{"html".to_string()}} xmlns:String={{"".to_string()}}>
    <?Script/>
  </Html>
);
