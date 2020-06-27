pub trait ToMarkup<T> {
   fn to_markup(&self) -> T;
}

pub trait ToId<T> {
   fn to_id(&self) -> T;
}

pub trait ToClass<T> {
   fn to_class(&self) -> T;
}

pub trait ToStyle<T> {
   fn to_style(&self) -> T;
}

pub trait ToSrc<T> {
   fn to_src(&self) -> T;
}

pub trait ToHref<T> {
   fn to_href(&self) -> T;
}

pub trait ToDomEvent<T> {
   fn to_domevent(&self) -> T;
}
impl<T> dyn ToDomEvent<T> {
   pub fn to_onabort(&self) -> T { self.to_domevent() }
   pub fn to_onafterprint(&self) -> T { self.to_domevent() }
   pub fn to_onanimationend(&self) -> T { self.to_domevent() }
   pub fn to_onanimationiteration(&self) -> T { self.to_domevent() }
   pub fn to_onanimationstart(&self) -> T { self.to_domevent() }
   pub fn to_onbeforeprint(&self) -> T { self.to_domevent() }
   pub fn to_onbeforeunload(&self) -> T { self.to_domevent() }
   pub fn to_onblur(&self) -> T { self.to_domevent() }
   pub fn to_oncanplay(&self) -> T { self.to_domevent() }
   pub fn to_oncanplaythrough(&self) -> T { self.to_domevent() }
   pub fn to_onchange(&self) -> T { self.to_domevent() }
   pub fn to_onclick(&self) -> T { self.to_domevent() }
   pub fn to_oncontextmenu(&self) -> T { self.to_domevent() }
   pub fn to_oncopy(&self) -> T { self.to_domevent() }
   pub fn to_oncut(&self) -> T { self.to_domevent() }
   pub fn to_ondblclick(&self) -> T { self.to_domevent() }
   pub fn to_ondrag(&self) -> T { self.to_domevent() }
   pub fn to_ondragend(&self) -> T { self.to_domevent() }
   pub fn to_ondragenter(&self) -> T { self.to_domevent() }
   pub fn to_ondragleave(&self) -> T { self.to_domevent() }
   pub fn to_ondragover(&self) -> T { self.to_domevent() }
   pub fn to_ondragstart(&self) -> T { self.to_domevent() }
   pub fn to_ondrop(&self) -> T { self.to_domevent() }
   pub fn to_ondurationchange(&self) -> T { self.to_domevent() }
   pub fn to_onended(&self) -> T { self.to_domevent() }
   pub fn to_onerror(&self) -> T { self.to_domevent() }
   pub fn to_onfocus(&self) -> T { self.to_domevent() }
   pub fn to_onfocusin(&self) -> T { self.to_domevent() }
   pub fn to_onfocusout(&self) -> T { self.to_domevent() }
   pub fn to_onfullscreenchange(&self) -> T { self.to_domevent() }
   pub fn to_onfullscreenerror(&self) -> T { self.to_domevent() }
   pub fn to_onhashchange(&self) -> T { self.to_domevent() }
   pub fn to_oninput(&self) -> T { self.to_domevent() }
   pub fn to_oninvalid(&self) -> T { self.to_domevent() }
   pub fn to_onkeydown(&self) -> T { self.to_domevent() }
   pub fn to_onkeypress(&self) -> T { self.to_domevent() }
   pub fn to_onkeyup(&self) -> T { self.to_domevent() }
   pub fn to_onload(&self) -> T { self.to_domevent() }
   pub fn to_onloadeddata(&self) -> T { self.to_domevent() }
   pub fn to_onloadedmetadata(&self) -> T { self.to_domevent() }
   pub fn to_onloadstart(&self) -> T { self.to_domevent() }
   pub fn to_onmessage(&self) -> T { self.to_domevent() }
   pub fn to_onmousedown(&self) -> T { self.to_domevent() }
   pub fn to_onmouseenter(&self) -> T { self.to_domevent() }
   pub fn to_onmouseleave(&self) -> T { self.to_domevent() }
   pub fn to_onmousemove(&self) -> T { self.to_domevent() }
   pub fn to_onmouseover(&self) -> T { self.to_domevent() }
   pub fn to_onmouseout(&self) -> T { self.to_domevent() }
   pub fn to_onmouseup(&self) -> T { self.to_domevent() }
   pub fn to_onmousewheel(&self) -> T { self.to_domevent() }
   pub fn to_onoffline(&self) -> T { self.to_domevent() }
   pub fn to_ononline(&self) -> T { self.to_domevent() }
   pub fn to_onopen(&self) -> T { self.to_domevent() }
   pub fn to_onpagehide(&self) -> T { self.to_domevent() }
   pub fn to_onpageshow(&self) -> T { self.to_domevent() }
   pub fn to_onpaste(&self) -> T { self.to_domevent() }
   pub fn to_onpause(&self) -> T { self.to_domevent() }
   pub fn to_onplay(&self) -> T { self.to_domevent() }
   pub fn to_onplaying(&self) -> T { self.to_domevent() }
   pub fn to_onpopstate(&self) -> T { self.to_domevent() }
   pub fn to_onprogress(&self) -> T { self.to_domevent() }
   pub fn to_onratechange(&self) -> T { self.to_domevent() }
   pub fn to_onresize(&self) -> T { self.to_domevent() }
   pub fn to_onreset(&self) -> T { self.to_domevent() }
   pub fn to_onscroll(&self) -> T { self.to_domevent() }
   pub fn to_onsearch(&self) -> T { self.to_domevent() }
   pub fn to_onseeked(&self) -> T { self.to_domevent() }
   pub fn to_onseeking(&self) -> T { self.to_domevent() }
   pub fn to_onselect(&self) -> T { self.to_domevent() }
   pub fn to_onshow(&self) -> T { self.to_domevent() }
   pub fn to_onstalled(&self) -> T { self.to_domevent() }
   pub fn to_onstorage(&self) -> T { self.to_domevent() }
   pub fn to_onsubmit(&self) -> T { self.to_domevent() }
   pub fn to_onsuspend(&self) -> T { self.to_domevent() }
   pub fn to_ontimeupdate(&self) -> T { self.to_domevent() }
   pub fn to_ontoggle(&self) -> T { self.to_domevent() }
   pub fn to_ontouchcancel(&self) -> T { self.to_domevent() }
   pub fn to_ontouchend(&self) -> T { self.to_domevent() }
   pub fn to_ontouchmove(&self) -> T { self.to_domevent() }
   pub fn to_ontouchstart(&self) -> T { self.to_domevent() }
   pub fn to_ontransitionend(&self) -> T { self.to_domevent() }
   pub fn to_onunload(&self) -> T { self.to_domevent() }
   pub fn to_onvolumechange(&self) -> T { self.to_domevent() }
   pub fn to_onwaiting(&self) -> T { self.to_domevent() }
   pub fn to_onwheel(&self) -> T { self.to_domevent() }
}
