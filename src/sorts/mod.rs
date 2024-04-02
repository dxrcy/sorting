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
