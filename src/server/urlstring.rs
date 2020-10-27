#[derive(Clone, Debug)]
pub struct UrlString {
    value: String,
    has_query: bool,
}

impl UrlString {
    pub fn new(base: &str) -> Self {
        UrlString {
            value: base.into(),
            has_query: false,
        }
    }

    pub fn query<T: std::fmt::Display>(&mut self, name: &str, val: T) {
        self.separate();
        use std::fmt::Write;
        self.value
            .write_fmt(format_args!("{}={}", name, val))
            .expect("a Display implementation returned an error unexpectedly");
        self.has_query = true;
    }
    pub fn cond_query(&mut self, name: &str, cond: bool, val: &str) {
        self.separate();
        self.value.push_str(name);
        self.value.push('=');
        if !cond {
            self.value.push('!');
        }
        self.value.push_str(val);
    }

    fn separate(&mut self) {
        self.value.push(if self.has_query { '&' } else { '?' });
        self.has_query = true;
    }
}

impl Into<String> for UrlString {
    fn into(self) -> String {
        self.value
    }
}

impl AsRef<str> for UrlString {
    fn as_ref(&self) -> &str {
        &self.value
    }
}
