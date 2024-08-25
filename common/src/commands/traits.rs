/// Common trait each and every command in the zeus project should have to enforce consistency
/// maintainability of the porject
pub(super) trait Cmd {
    fn validate(&self) -> bool;
    // fn run(&self);
}
