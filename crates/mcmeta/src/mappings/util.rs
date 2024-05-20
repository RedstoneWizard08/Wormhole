// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use anyhow::{anyhow, Result};

pub struct Consumable<T> {
    inner: Vec<T>,
}

impl<T> Consumable<T> {
    pub fn next(&mut self) -> Result<T> {
        if !self.inner.is_empty() {
            Ok(self.inner.remove(0))
        } else {
            Err(anyhow!("Tried to access a None value!"))
        }
    }

    pub fn peek(&self) -> Result<&T> {
        self.inner
            .first()
            .ok_or(anyhow!("Tried to access a None value!"))
    }
}

pub trait IntoConsumable<T> {
    fn into_consumable(self) -> Consumable<T>;
}

impl<T, I: Iterator<Item = T>> IntoConsumable<T> for I {
    fn into_consumable(self) -> Consumable<T> {
        Consumable {
            inner: self.collect::<Vec<T>>(),
        }
    }
}

pub trait TrimExt {
    fn trim_start_count(&mut self, pat: char) -> usize;
    fn start_count(&self, pat: char) -> usize;
}

impl TrimExt for String {
    fn trim_start_count(&mut self, pat: char) -> usize {
        let mut count = 0;

        while self.starts_with(pat) {
            self.remove(0);
            count += 1;
        }

        count
    }

    fn start_count(&self, pat: char) -> usize {
        let mut count = 0;

        while self.chars().nth(count) == Some(pat) {
            count += 1;
        }

        count
    }
}
