use std::collections::HashMap;
pub type SizedImages = HashMap<String, String>;
pub type Opts<'a> = Option<&'a [(&'a str, &'a str)]>;
