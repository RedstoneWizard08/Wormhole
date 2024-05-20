// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use derive_builder::Builder;
use include_dir::Dir;

use crate::framework::Framework;

#[derive(Debug, Clone, PartialEq, Builder)]
#[builder(setter(into))]
pub struct GlueConfig {
    #[builder(setter(strip_option))]
    pub framework: Option<Framework>,

    #[builder(setter(custom))]
    pub base: Option<String>,

    pub dir: Option<Dir<'static>>,

    #[builder(setter(custom))]
    pub project: Option<String>,

    #[builder(setter(custom))]
    pub cmd: Vec<String>,
}

impl GlueConfig {
    pub fn builder() -> GlueConfigBuilder {
        GlueConfigBuilder::default()
    }
}

impl GlueConfigBuilder {
    pub fn base<T>(&mut self, value: T) -> &mut Self
    where
        T: AsRef<str>,
    {
        let new = self;
        new.base = Some(Some(value.as_ref().to_string()));
        new
    }

    pub fn project<T>(&mut self, value: T) -> &mut Self
    where
        T: AsRef<str>,
    {
        let new = self;
        new.project = Some(Some(value.as_ref().to_string()));
        new
    }

    pub fn cmd<T>(&mut self, value: T) -> &mut Self
    where
        T: AsRef<str>,
    {
        let new = self;
        let mut arr = new.cmd.clone().unwrap_or_default();

        arr.push(value.as_ref().to_string());
        new.cmd = Some(arr);
        new
    }

    pub fn arg<T>(&mut self, value: T) -> &mut Self
    where
        T: AsRef<str>,
    {
        self.cmd(value)
    }
}
