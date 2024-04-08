use rbatis::{crud, impl_select};
use crate::domain::entity::CopyWriting;
crud!(CopyWriting {});
impl_select!(CopyWriting{select_by_id(id:u64) => "`where id = #{id}`"});

pub struct CopyWritingMapper {}