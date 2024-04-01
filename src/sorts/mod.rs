macro_rules! yield_ {
    ($scope:expr, $list:expr, None) => {
        $scope.yield_(SortState {
            list: $list.clone(),
            just_compared: None,
        })
    };
    ($scope:expr, $list:expr, [$($x:expr),*]) => {
        $scope.yield_(SortState {
            list: $list.clone(),
            just_compared: Some([ $($x),* ]),
        })
    };
}

mod bubble;
mod insertion;
mod selection;
mod quick;

pub use bubble::bubble;
pub use insertion::insertion;
pub use selection::selection;
pub use quick::quick;
