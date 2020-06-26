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

xtype!(<!Link crossorigin:String href:String hreflang:String media:String referrerpolicy:String rel:String sizes:String title:String type_:String/>);
xrender!(Link,<link {{if self.crossorigin.len()>0 {{crossorigin={{self.crossorigin}}}}}}
                    {{if self.href.len()>0 {{href={{self.href}}}}}}
                    {{if self.hreflang.len()>0 {{hreflang={{self.hreflang}}}}}}
                    {{if self.media.len()>0 {{media={{self.media}}}}}}
                    {{if self.referrerpolicy.len()>0 {{referrerpolicy={{self.referrerpolicy}}}}}}
                    {{if self.rel.len()>0 {{rel={{self.rel}}}}}}
                    {{if self.sizes.len()>0 {{sizes={{self.sizes}}}}}}
                    {{if self.title.len()>0 {{title={{self.title}}}}}}
                    {{if self.type_.len()>0 {{"type"={{self.type_}}}}}}/>
);

xtype!(<!Meta charset:String content:String http_equiv:String name:String/>);
xrender!(Meta,<meta {{if self.charset.len()>0 {{charset={{self.charset}}}}}}
                    {{if self.content.len()>0 {{content={{self.content}}}}}}
                    {{if self.http_equiv.len()>0 {{"http-equiv"={{self.http_equiv}}}}}}
                    {{if self.name.len()>0 {{name={{self.name}}}}}}/>
);

xtype!(<!Base href:String target:String/>);
xrender!(Base,<base {{if self.href.len()>0 {{href={{self.href}}}}}}
                    {{if self.target.len()>0 {{target={{self.target}}}}}}/>
);

xtype!(
  <!Html lang:String={{"en".to_string()}} doctype:String={{"html".to_string()}} xmlns:String={{"".to_string()}}>
    <?Title/>
    <?Script/>
    <?Style/>
    <?Link/>
    <?Base/>
    <?/>
  </Html>
);
xrender!(Html,{{ format!("<DOCTYPE {}>", self.doctype) }}
<html>
  <head>
    {{ for c in self.children.iter() {{
      {{ if let HtmlChildren::Title(cd) = c {{ {{cd}} }}
         else if let HtmlChildren::Script(cd) = c {{ {{cd}} }}
         else if let HtmlChildren::Style(cd) = c {{ {{cd}} }}
         else if let HtmlChildren::Link(cd) = c {{ {{cd}} }}
         else if let HtmlChildren::Base(cd) = c {{ {{cd}} }}
      }}
    }} }}
  </head>
  <body>
    {{ for c in self.children.iter() {{
      {{ if let HtmlChildren::Display(cd) = c {{
        {{ cd }}
      }} }}
    }} }}
  </body>
</html>);
