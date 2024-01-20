pub mod transaction;

#[derive(Clone, Copy)]
pub struct DefaultContext;

/// Dependency Inversion Factory
pub trait Injects<'a, Ty: Sized + 'a>: Sized + 'a {

    fn inject(&'a self) -> Ty;

}