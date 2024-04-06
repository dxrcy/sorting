/// Declare generator function, and define test and identifier for each algorithm
macro_rules! define_algorithms {
    ( $( $index:literal | $Name:ident => $name:ident ),* $(,)? ) => {
        /// Sorting algorithms
        pub mod sorts {
            $(
                mod $name;
                pub use $name::$name;
            )*
        }

        #[cfg(test)]
        mod algorithms {
            $(
                #[test]
                fn $name() {
                    crate::tests::test_algorithm(crate::sorts::$name);
                }
            )*
        }

        /// Sorting algorithm identifier
        #[derive(Clone, Copy, Debug, clap::ValueEnum)]
        pub enum Algorithm {
            $(
                $Name,
            )*
            Random,
        }

        impl std::fmt::Display for Algorithm {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use clap::ValueEnum;
                write!(f, "{}", self.to_possible_value().unwrap().get_name())
            }
        }

        impl Algorithm {
            /// Create generator function for choosen algorithm
            pub fn create(&self, list: ListRef) -> generator::LocalGenerator<'static, (), Compare> {
                use rand::Rng;
                match self {
                    $(
                        Algorithm::$Name => sorts::$name(list),
                    )*
                    Algorithm::Random => {
                        let mut rng = rand::thread_rng();
                        // Get index literal of last pattern
                        let count = *[ $( $index ),* ].last().unwrap();
                        // Get function from random index
                        match rng.gen_range(0..=count) {
                            $(
                                $index => sorts::$name(list),
                            )*
                            _ => unreachable!("macro is broken"),
                        }
                    }
                }
            }
        }
    };
}

macro_rules! algorithm {
    (
        $name:ident : | $list:ident, $scope:ident |
        { $($body:tt)* }
    ) => {
        pub fn $name(mut $list: crate::ListRef) -> generator::LocalGenerator<'static, (), crate::Compare> {
            generator::Gn::new_scoped_local(move |mut $scope| {
                yield_!($scope, None);
                {
                    let $list = $list.as_mut_slice();
                    $($body)*
                }
                yield_!($scope, None);
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
