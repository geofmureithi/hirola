//! Definition of `cloned!` macro. Proc-macros are defined in the separate `hirola-macros` crate.

/// Utility macro for cloning all the arguments and expanding the expression.
///
/// Temporary workaround for [Rust RFC #2407](https://github.com/rust-lang/rfcs/issues/2407).
///
/// # Example
/// ```
/// use hirola_core::prelude::*;
///
/// let state = Signal::new(0);
///
/// create_effect(cloned!((state) => move || {
///    state.get();
/// }));
///
/// // state still accessible outside of the effect
/// let _ = state.get();
/// ```
#[macro_export]
macro_rules! cloned {
    (($($arg:ident),*) => $e:expr) => {{
        // clone all the args
        $( let $arg = ::std::clone::Clone::clone(&$arg); )*

        $e
    }};
}


/// Macro for reactive classes 
/// 
/// # Example
/// ```rust, no_run
/// use hirola::prelude::*;
/// 
/// fn your_page(_app: &HirolaApp) -> Dom {
///     let bool1 = Signal::new(false);
///     let bool2 = Signal::new(true);
///     
///     let classes = classes! {
///     /* [class name] => [variable cloning] [expression that returns a boolean], */
///     "bg-white" => (bool1, bool2) *bool1.get() && *bool2.get(),
///     /*static class*/
///     "dark" => () true
///     };
/// 
///     html! {
///         <div class=classes.get().to_string()>
///             /* rest of codes */
///         </div>
///     }
/// }
/// ```
/// 
/// # Note
/// Overlapping classes will create unexpected behaviour, for instance:
/// ```rust, no_run
/// use hirola::prelude::*;
/// 
/// fn your_page(_app: HirolaApp) -> Dom {
///     let classes = classes! {
///         "bg-white" => () true
///     };
/// 
///     html! {
///         <div class=classes.get().to_string() class="bg-black"></div>
///     }
/// }
/// ```
/// 
#[macro_export]
macro_rules! classes {
    ($($class:expr => ($($signal:ident), *) $condition:expr), *) => {
        {
            let all_classes = Signal::new(String::new());
            $(  
                {   
                    create_effect({
                        let all_classes = all_classes.clone();
                        //Clone all the args
                        $(
                            let $signal = $signal.clone();
                        )*
                        move || {
                            let res = $condition;
                
                            if res {
                                let class = $class.trim();
                                let current_classes = all_classes.get().to_string();
                                let mut current_classes = current_classes.split_ascii_whitespace().collect::<Vec<&str>>();
                                if !current_classes.iter().any(|&c| c == class) {
                                    current_classes.push(&class);
                                    all_classes.set(current_classes.join(" ").to_string());
                                }
                            } else {
                                let class = $class.trim();
                                let current_classes = all_classes.get().to_string();
                                let mut current_classes = current_classes.split_ascii_whitespace().collect::<Vec<&str>>();
                                if let Some(index) = current_classes.iter().position(|&c| c == class) {
                                    current_classes.remove(index);
                                    all_classes.set(current_classes.join(" ").to_string());
                                }
                            }
                        }
                    });
                }
            )*

            all_classes.clone()
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn cloned() {
        let state = Signal::new(0);

        let _x = cloned!((state) => state);

        // state still accessible because it was cloned instead of moved
        let _ = state.get();
    }

    #[test]
    fn cloned_closure() {
        let state = Signal::new(0);

        create_effect(cloned!((state) => move || {
            state.get();
        }));

        // state still accessible outside of the effect
        let _ = state.get();
    }

    #[test]
    fn classes() {
        let bool1 = Signal::new(true);
        let bool2 = Signal::new(false);

        let classes = classes! {
            "bg-white" => (bool1) *bool1.get(),
            "dark" => (bool2) *bool2.get(),
            "grey" => (bool1, bool2) *bool1.get() && *bool2.get(),
            "my-2" => () true,
            "my-1" => () false
        };

        //classes should have 2 items when bool2 is false
        assert_eq!(2, classes.get().to_string().split_ascii_whitespace().collect::<Vec<&str>>().len());
        
        //classes should have 4 items once bool2 is true
        bool2.set(true);
        assert_eq!(4, classes.get().to_string().split_ascii_whitespace().collect::<Vec<&str>>().len());
    }
}
