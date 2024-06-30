pub mod elems;

pub mod parser;

// mod parse;
// pub use parse::*;

#[cfg(test)]
mod tests {
    pub fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
}
