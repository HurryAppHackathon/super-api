macro_rules! config {
    ($($name:ident $type:tt $( = $value: expr)?)* ) => {
        lazy_static! {
            $(
                pub static ref $name: $type = std::env::var(stringify!($name)).unwrap_or_else(|_| {
                    $( if true { return $value.to_string(); } )?
                    panic!("coudn't find {} in env, or you should set default value.", stringify!($name))
                }).parse::<$type>().unwrap();
            )+
        }
    };
}

config! {
    PORT u32 = 3000 // default port 3000
    SECRET String = "super-secret"
}
