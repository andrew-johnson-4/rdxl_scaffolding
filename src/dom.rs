use rdxl::{xtype,xrender};

xtype!(
   <!Script async_:bool={{false}} charset:String={{"UTF-8".to_string()}} defer:bool={{false}}
            src:String={{"".to_string()}} type_:String={{"application/javascript".to_string()}}>
     <?/>
   </Script>
);
xrender!(Script,<script {{if self.async_ {{async}}}}
                        {{if self.defer {{defer}}}}
                        {{if self.charset.len()>0 {{charset={{self.charset}}}}}}
                        {{if self.src.len()>0 {{src={{self.src}}}}}}
                        {{if self.type_.len()>0 {{"type"={{self.type_}}}}}}>
   {{ for cd in self.children.iter() {{
      {{ let ScriptChildren::Display(cd) = cd; }}
      {{cd}}
   }} }}
</script>);

xtype!(
   <!Style media:String type_:String={{"text/css".to_string()}}>
     <?/>
   </Style>
);
xrender!(Style,<style {{if self.media.len()>0 {{media={{self.media}}}}}}
                      {{if self.type_.len()>0 {{"type"={{self.type_}}}}}}>
   {{ for cd in self.children.iter() {{
      {{ let StyleChildren::Display(cd) = cd; }}
      {{cd}}
   }} }}
</style>);

xtype!(<!Title><?/></Title>);
xrender!(Title,{{ for c in self.children.iter() {{
   {{ let TitleChildren::Display(cd) = c; }}
   {{ cd }}
}}}});

xtype!(
  <!Html lang:String={{"en".to_string()}} doctype:String={{"html".to_string()}} xmlns:String={{"".to_string()}}>
    <?Title/>
    <?Script/>
    <?Style/>
  </Html>
);
