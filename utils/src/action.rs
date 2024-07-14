use crate::imports::*;
use workflow_core::enums::Describe;

pub trait Action<Context>: Describe + Clone + Copy + Eq {
    type Error;

    fn select() -> std::result::Result<Self, std::io::Error> {
        Self::select_with_prompt(Self::caption())
    }

    fn select_with_prompt<S>(prompt: S) -> std::result::Result<Self, std::io::Error>
    where
        S: Display,
    {
        let mut selector = cliclack::select(prompt.to_string());
        for action in Self::iter() {
            selector = selector.item(*action, action.describe(), action.rustdoc());
        }

        selector.interact()
    }

    fn multiselect<S>(values: Vec<Self>) -> std::result::Result<Vec<Self>, std::io::Error> {
        Self::multiselect_with_prompt(Self::caption(), values)
    }

    fn multiselect_with_prompt<S>(
        prompt: S,
        values: Vec<Self>,
    ) -> std::result::Result<Vec<Self>, std::io::Error>
    where
        S: Display,
    {
        let mut selector = cliclack::multiselect(prompt.to_string()).initial_values(values);
        for option in Self::into_iter() {
            selector = selector.item(option, option.describe(), option.rustdoc());
        }

        selector.interact()
    }

    fn run(&self, _ctx: &mut Context) -> std::result::Result<(), Self::Error>;
}
