// autogenerate String -> usize for struct fields within specified trait
// getter implementation: self.$field.parse::<usize>().unwrap()
// usage:
// struct Foo {
//     bar: String,
//     baz: String,
// }
// getter_usize! {
//     Foo,
//     FooGetters,
//     bar_get = bar,
//     baz_get = baz, // trailing comma is optional
// }
#[macro_export]
macro_rules! getter_usize {
    ($to:ty, $t:ident, $($getter:ident = $field:ident),*$(,)?) => {
        trait $t {
            $(fn $getter(&self) -> usize;)*
        }
        impl $t for $to {
            $(
                fn $getter(&self) -> usize {
                    self.$field.parse::<usize>().unwrap()
                }
            )*
        }
    };
}

// same as getter_usize
// getter implementation: self.$field != "0"
// for some reason last.fm wraps booleans into 0/1
#[macro_export]
macro_rules! getter_bool {
    ($to:ty, $t:ident, $($getter:ident = $field:ident),*$(,)?) => {
        trait $t {
            $(fn $getter(&self) -> bool;)*
        }
        impl $t for $to {
            $(
                fn $getter(&self) -> bool {
                    self.$field != "0"
                }
            )*
        }
    };
}

// autogenerate raw structure
// if not defined in struct, fields are String type by default
// * usage
// raw_gen!(Foo {bar: usize}, baz);
// //result:
// #[derive(Deserialize, Debug)]
// struct Foo {
//     bar: usize,
//     baz: String,
// }
#[macro_export]
macro_rules! raw_gen {
    ($struct:ident { $($def:tt)* }$(,)? $($field:ident),*$(,)?) => {
        #[derive(Deserialize, Debug)]
        struct $struct {
            $($def)*
            $($field: String,)*
        }
    }
}

// autogenerate Self {<fields>} structure
// * usage:
// from_raw! {
//     item, // bring identifier in scope
//     {
//         bar,
//         baz,
//     },
//     {
//         x = item.0,
//         y = item.1,
//     }
// }
// // result:
// Self {
//     bar,
//     baz,
//     x = item.0,
//     y = item.1,
// }
#[macro_export]
macro_rules! from_raw {
    ($scope:ident, {$($item:ident),*}, {$($def:ident = $val:expr),*}) => {
        $(let $def = $val;)*
        $(let $item = $scope.$item;)*
        Self {
            $($item,)*
            $($def,)*
        }
    }
}
