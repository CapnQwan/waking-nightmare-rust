use crate::Core;

pub trait Plugin {
    fn register(core: &mut Core);
}
