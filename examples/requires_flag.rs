#![feature(proc_macro_hygiene)]
use rdxl::xhtml;
use rdxl_scaffolding::*;

fn main() {
   let _ = ProgressBar { numerator:1, denominator:2, unit:"MB".to_string(), children:vec![] };
   let _ = xhtml!(<div></div>);
   //println!("{}", xhtml!( <!ProgressBar numerator=312 denominator=1532 unit="MB"/> ) );
}
