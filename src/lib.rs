pub mod data;
mod scope;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::scope::{ProjectScope, ScanConfig};
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_open_project() {
        let mut init = ProjectScope::init(r#"C:\Users\rm\Projects\Jabki\Assets"#, ScanConfig::default());
        assert!(init.is_ok())
    }
}
