#![cfg_attr(feature = "nightly-docs", feature(doc_cfg))]

pub mod error;
pub mod platform;
pub mod window;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
