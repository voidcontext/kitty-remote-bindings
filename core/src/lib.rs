use std::path::PathBuf;

pub trait ToArg {
    fn to_arg(&self) -> Vec<String>;
}

impl ToArg for String {
    fn to_arg(&self) -> Vec<String> {
        vec![self.clone()]
    }
}

impl ToArg for PathBuf {
    fn to_arg(&self) -> Vec<String> {
        vec![self.to_string_lossy().to_string()]
    }
}

impl<T: ToArg> ToArg for &T {
    fn to_arg(&self) -> Vec<String> {
        (*self).to_arg()
    }
}

impl<T: ToArg> ToArg for Vec<T> {
    fn to_arg(&self) -> Vec<String> {
        self.iter().flat_map(ToArg::to_arg).collect()
    }
}
