
/// Model allows 2-way binding eg between a signal and an input
pub struct Model<Node, T: 'static>(Mutable<T>, PhantomData<Node>);

impl<T: Display + FromStr + Clone + 'static> Mixin<Identity> for Model<HtmlInputElement, T>
where
    <T as FromStr>::Err: Debug,
{
    fn mixin(&self, node: &Dom) {
        let input = {
            let node = node.node().as_ref().clone();
            node.dyn_into::<HtmlInputElement>().unwrap()
        };
        let signal = self.0.clone();
        node.effect(
            signal
                .signal_ref(|value| {
                    input.set_value(&format!("{}", value));
                })
                .to_future(),
        );
        let handler = Box::new(move |e: Event| {
            let input = e
                .current_target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            let new_value = input.value().parse().unwrap();
            signal.set(new_value);
        });

        node.event("input", handler);
    }
}

/// Two way binding for input and signals
pub mod model {
    use super::*;
    /// Bind a [HtmlInputElement] to a [Mutable<T>]
    pub fn input<T>(s: &Mutable<T>) -> Model<HtmlInputElement, T> {
        Model(s.clone(), PhantomData)
    }
}
