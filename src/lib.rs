use rocket::{fairing::Fairing, Rocket};

/// Attach a fairing iff `condition == true`
pub trait ConditionalAttach {
    fn attach_if(self, condition: bool, fairing: impl Fairing) -> Self;
}

pub mod prelude {
    pub use crate::ConditionalAttach;
}

impl ConditionalAttach for Rocket {
    #[inline]
    fn attach_if(self, condition: bool, fairing: impl Fairing) -> Self {
        if condition {
            self.attach(fairing)
        } else {
            self
        }
    }
}

