Rdxl Scaffolding
================

[![Crates.IO](https://img.shields.io/crates/v/rdxl_scaffolding.svg)](https://crates.rs/crates/rdxl_scaffolding)

Prefabricated HTML components

```rust
xhtml!(<!ProgressBar numerator=12 denominator=32 unit="MB"/>)
```

This UI scaffolding is meant to abstract over many possible display logics. Whether it be a site redesign,
or an entirely separate platform, rdxl! is meant to provide only a thin logical abstraction rather than an
opinionated UI framework. The concept of "Minimal Common Abstraction" will be the unifying design concept
herewithin.

A common concept in Rdxl is the "View as data, not Display" idiom. This pattern is a small extension of the MVC pattern.
In any Model/View/Controller codebase there may be multiple Views, for Desktop vs Mobile for instance; however, it is
desirable that the Models and Controllers need not change significantly for these separate views to be performed. To
encourage this consolidation of models and controllers, Rdxl Scaffolding defines the base of each view as a simple XML
document. The user visible UI is generated from these XML common components. This is helpful whether you exchange data
from the server as XML or JSON, as long as these objects are serializable.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in rdxl_scaffolding by you,
shall be dual licensed under the MIT and Apache 2.0 license without any additional terms or conditions.
