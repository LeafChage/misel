#[derive(Clone, Debug)]
pub enum HowToHandle {
    /// abcd until b return (a, cd)
    Ignore,

    /// abcd until b return (a, bcd)
    Leave,

    /// abcd until b return (ab, cd)
    Include,
}
