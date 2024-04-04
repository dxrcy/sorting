macro_rules! algorithm {
    (
        $name:ident : | $list:ident, $scope:ident |
        { $($body:tt)* }
    ) => {
        #[allow(unused_imports)]
        use crate::{Compare, ListRef, Value};

        pub fn $name(mut $list: ListRef) -> generator::LocalGenerator<'static, (), Compare> {
            generator::Gn::new_scoped_local(move |mut $scope| {
                let $list = $list.as_mut_slice();
                $($body)*
                generator::done()
            })
        }
    };
}

macro_rules! yield_ {
    ($scope:expr, None $(,)?) => {
        unsafe { $scope.yield_unsafe(None) };
    };
    ($scope:expr, [$($x:expr),*] $(,)?) => {
        unsafe { $scope.yield_unsafe(Some( [$($x),*] )) };
    };
}

// Wrapper for subslicing a `Slice` or `SliceMut` to preserve original whole slice
macro_rules! slice {
    ( & $list:ident, $start:expr, $end:expr ) => {{
        let start = $list.start();
        Slice::new($list.get_whole(), start + $start, start + $end)
    }};
    ( &mut $list:ident, $start:expr, $end:expr ) => {{
        let start = $list.start();
        SliceMut::new($list.get_whole_mut(), start + $start, start + $end)
    }};
}

macro_rules! define_algorithms {
    ( $( $name:ident ),* $(,)? ) => {
        $(
            mod $name;
            pub use $name::$name;
        )*
    };
}

define_algorithms! {
    bubble,
    insertion,
    merge,
    quick,
    selection,
}
