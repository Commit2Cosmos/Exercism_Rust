#[macro_export]
macro_rules! hashmap {
    () => {
        crate::HashMap::new()
    };
    ( $( $key:expr => $value:expr ),+ $(,)? ) => {
        {
            let mut hashmap_temp = crate::HashMap::new();

            $(
                hashmap_temp.insert($key, $value);
            )*

            hashmap_temp
        }
    };
}
