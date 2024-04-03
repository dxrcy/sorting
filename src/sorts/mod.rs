macro_rules! algorithm {
    (
        $name:ident : | $list:ident, $scope:ident |
        { $($body:tt)* }
    ) => {
        use crate::{Compare, Value};

        /// # Safety
        ///
        /// Trust me.
        pub unsafe fn $name($list: *mut [Value]) -> impl Iterator<Item = Compare> {
            generator::Gn::new_scoped_local(move |mut $scope| {
                let $list = unsafe { &mut *$list };
                $($body)*
                generator::done()
            })
        }
    };
}

macro_rules! yield_ {
    ($scope:expr, None) => {
        unsafe { $scope.yield_unsafe(None) };
    };
    ($scope:expr, [$($x:expr),*]) => {
        unsafe { $scope.yield_unsafe(Some( [$($x),*] )) };
    };
}

// mod bubble;
// mod insertion;
mod selection;
// mod quick;

// pub use bubble::bubble;
// pub use insertion::insertion;
pub use selection::selection;
// pub use quick::quick;
