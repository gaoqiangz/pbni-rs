use crate::{
    pbx::{bindings::*, *}, prelude::*
};

/// 数组的引用
///
/// # Parameters
///
/// 数组函数的`dims`参数可指定不同维度的索引: `&[dim_1_index,dim_2_index,...]`
pub struct Array<'arr> {
    ptr: pbarray,
    info: ArrayInfo,
    is_object: bool,
    session: Session,
    _marker: PhantomData<&'arr ()>
}

impl<'arr> Array<'arr> {
    pub(crate) unsafe fn from_ptr(ptr: pbarray, is_object: bool, session: Session) -> Array<'arr> {
        let info = ArrayInfo::new(ptr, session.clone());
        Array {
            ptr,
            info,
            is_object,
            session,
            _marker: PhantomData
        }
    }
    pub(crate) fn as_ptr(&self) -> pbarray { self.ptr }
}
