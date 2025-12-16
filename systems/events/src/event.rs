pub trait Event: Any + Send + Sync {}
impl<T: Any + Send + Sync> Event for T {}
