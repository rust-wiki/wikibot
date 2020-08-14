/// Macro for creating a `HashMap<String, _>`.
///
/// The keys are written without quotes. The values are
/// automatically converted with `Into::into()`.
///
/// ## Example
/// ```
/// # use std::collections::HashMap;
/// let _: HashMap<String, String> = map!(hello: "world", foo: "bar");
/// ```
macro_rules! map {
    ( $($key:ident : $value:expr),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert(String::from(stringify!($key)), Into::into($value));
            )*
            map
        }
    };
}

#[test]
fn test_map_macro() {
    use std::collections::HashMap;

    let map: HashMap<String, String> = map!(hello: "world", foo: "bar");
    assert_eq!(
        map.into_iter().collect::<Vec<_>>(),
        vec![
            (String::from("hello"), String::from("world")),
            (String::from("foo"), String::from("bar")),
        ]
    );
}
