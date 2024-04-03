macro_rules! algorithm {
    (
        $name:ident : | $list:ident, $scope:ident |
        { $($body:tt)* }
    ) => {
        #[allow(unused_imports)]
        use crate::{Compare, SmartPointer, Value};

        pub fn $name(mut $list: SmartPointer) -> generator::LocalGenerator<'static, (), Compare> {
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

mod bubble;
mod insertion;
mod merge;
mod quick;
mod selection;

pub use bubble::bubble;
pub use insertion::insertion;
pub use merge::merge;
pub use quick::quick;
pub use selection::selection;
