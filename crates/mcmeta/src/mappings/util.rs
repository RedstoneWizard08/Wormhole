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
