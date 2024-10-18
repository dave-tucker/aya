//! Hash map types.
use std::os::fd::AsFd as _;

use crate::{
    errors::MapError,
    sys::{bpf_map_delete_elem, bpf_map_update_elem},
    Pod,
};

#[allow(clippy::module_inception)]
mod hash_map;
mod per_cpu_hash_map;

pub use hash_map::*;
pub use per_cpu_hash_map::*;

use super::MapData;

pub(crate) fn insert<K: Pod, V: Pod>(
    map: &MapData,
    key: &K,
    value: &V,
    flags: u64,
) -> Result<(), MapError> {
    let fd = map.fd().as_fd();
    bpf_map_update_elem(fd, Some(key), value, flags)?;
    Ok(())
}

pub(crate) fn remove<K: Pod>(map: &MapData, key: &K) -> Result<(), MapError> {
    let fd = map.fd().as_fd();
    bpf_map_delete_elem(fd, key).map(|_| ()).map_err(Into::into)
}
