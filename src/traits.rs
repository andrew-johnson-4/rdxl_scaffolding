trait ToMarkup<T> {
   fn to_markup(&self) -> T;
}

trait ToId<T> {
   fn to_id(&self) -> T;
}

trait ToClass<T> {
   fn to_class(&self) -> T;
}

trait ToStyle<T> {
   fn to_style(&self) -> T;
}

trait ToSrc<T> {
   fn to_src(&self) -> T;
}

trait ToHref<T> {
   fn to_href(&self) -> T;
}
