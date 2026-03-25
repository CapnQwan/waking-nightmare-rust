pub trait Event: Send + Sync {}
impl<T:  Send + Sync> Event for T {}
