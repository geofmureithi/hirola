use crate::component::Props;

impl Props for i32 {}
impl<T> Props for Option<T> where T: Props {}