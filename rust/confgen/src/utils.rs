macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = HashMap::new();
         $( map.insert($key.to_string(), $val.to_string()); )*
         map
    }}
}
