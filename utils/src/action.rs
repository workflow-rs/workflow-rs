use crate::imports::*;
use workflow_core::enums::Describe;

/// An enumerable command that can be presented to the user as an interactive
/// CLI selection and then executed against a shared `Context`.
pub trait Action<Context>: Describe + Clone + Copy + Eq {
    /// Error type returned when running the action fails.
    type Error;

    /// Prompts the user to pick a single action using the type's default caption.
    fn select() -> std::result::Result<Self, std::io::Error> {
        Self::select_with_prompt(Self::caption())
    }

    /// Prompts the user to pick a single action using the supplied prompt text.
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

    /// Prompts the user to pick multiple actions using the default caption,
    /// pre-selecting `values`.
    fn multiselect<S>(values: Vec<Self>) -> std::result::Result<Vec<Self>, std::io::Error> {
        Self::multiselect_with_prompt(Self::caption(), values)
    }

    /// Prompts the user to pick multiple actions using the supplied prompt text,
    /// pre-selecting `values`.
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

    /// Executes the selected action, mutating the shared `Context` as needed.
    fn run(&self, _ctx: &mut Context) -> std::result::Result<(), Self::Error>;
}
