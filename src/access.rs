//! Accessor macro to get a value form a MySQL Row
#[macro_export]
macro_rules! mysql_row_try_get {
    ($var: tt, $index: expr) => {
        $var.get_opt($index).unwrap()
    };
}
