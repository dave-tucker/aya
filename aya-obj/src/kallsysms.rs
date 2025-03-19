//! /proc/kallsyms bindings

use std::{borrow::ToOwned, string::String};

use thiserror::Error;

use crate::util::HashMap;

/// A symbol from /proc/kallsyms.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Symbol {
    address: u64,
    visible: bool,
    name: String,
    module_name: Option<String>,
}

impl Symbol {
    /// Get the symbol address.
    pub fn address(&self) -> u64 {
        self.address
    }

    /// Whether the symbol is visible to kernel modules.
    pub fn visible(&self) -> bool {
        self.visible
    }

    /// Get the symbol name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the module name where the symbol is defined.
    pub fn module_name(&self) -> Option<&str> {
        self.module_name.as_deref()
    }
}

/// A collection of symbols from /proc/kallsyms.
pub struct KAllSyms {
    syms: HashMap<String, Symbol>,
}

/// Errors that can occur when parsing /proc/kallsyms.
#[derive(Debug, Error)]
pub enum KAllSymsError {
    /// Missing address.
    #[error("missing address.")]
    MissingAddress,
    /// Missing visibility.
    #[error("missing visibility.")]
    MissingVisibility,
    /// Missing name.
    #[error("missing name.")]
    MissingName,
    /// Missing module name.
    #[error("unexpected extra fields.")]
    UnexpectedExtraFields,
    /// Io error.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// ParseInt error.
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

impl KAllSyms {
    /// Create a new KAllSyms reading from /proc/kallsyms.
    pub fn new() -> Result<Self, KAllSymsError> {
        let f = std::fs::read_to_string("/proc/kallsyms")?;
        Self::try_from(f.as_str())
    }

    /// Get a symbol by name.
    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.syms.get(name)
    }
}

impl TryFrom<&str> for KAllSyms {
    type Error = KAllSymsError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut syms = HashMap::new();
        for line in value.lines() {
            let mut parts = line.split_whitespace();
            let address = parts.next().ok_or(KAllSymsError::MissingAddress)?;
            let address = u64::from_str_radix(address, 16)?;
            let visible = parts
                .next()
                .ok_or(KAllSymsError::MissingVisibility)?
                .chars()
                .nth(0)
                .is_some_and(|c| c.is_uppercase());
            let name = parts.next().ok_or(KAllSymsError::MissingName)?;
            let module_name = parts
                .next()
                .and_then(|s| s.strip_prefix(r"["))
                .and_then(|s| s.strip_suffix(r"]"));

            if parts.next().is_some() {
                return Err(KAllSymsError::UnexpectedExtraFields);
            }
            syms.insert(
                name.to_owned(),
                Symbol {
                    address,
                    visible,
                    name: name.to_owned(),
                    module_name: module_name.map(ToOwned::to_owned),
                },
            );
        }

        Ok(Self { syms })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kallsyms() {
        // sudo cat /proc/kallsyms | sed -n '0~10000p'
        let s = r#"ffffffff9211ae70 T __pfx_queue_delayed_work_on
ffffffff9224e0c0 T __pfx___audit_openat2_how
ffffffff923ab280 t perf_trace_mm_vmscan_throttled
ffffffff9252bda0 T __block_write_full_folio
ffffffff926d7480 t log_new_dir_dentries
ffffffff9285f590 T __traceiter_iocost_iocg_activate
ffffffff9298ccf0 T ZSTD_customMalloc
ffffffff92abad80 t hsu_dma_chan_start
ffffffff92bf2030 T driver_attach
ffffffff92d38020 T usb_serial_generic_get_icount
ffffffff92e7abe0 T skb_cow_data_for_xdp
ffffffff930379c0 t addrconf_get_prefix_route
ffffffff931a2821 t sw842_decompress.cold.0
ffffffff93609bc0 d _entry.24
ffffffff93739a80 d configuration_table
ffffffff93828940 d _entry.12
ffffffff93fbcff0 d _entry_ptr.50
ffffffff93fd0c88 r __ksymtab___ip_mc_dec_group
ffffffff93fee130 r __ksymtab_devlink_port_attrs_set
ffffffff949f19c0 d ftrace_event_fields_kernel_stack
ffffffff94b56f80 D __SCK__tp_func_ata_qc_prep
ffffffff94c3b920 d __bpf_trace_tp_map_flock_lock_inode
ffffffff95571eb8 d __event_vector_update
ffffffff95931d20 B security_hook_active_bprm_creds_for_exec_0
ffffffffc61a1710 t cleanup_module       [ip_tables]
ffffffffc5f8c4c0 t __pfx___traceiter_ib_mad_handle_opa_smi      [ib_core]
ffffffffc5d30760 t __pfx_nft_counter_do_dump    [nf_tables]
ffffffffc59eb680 r _entry.404   [nvidia_uvm]
ffffffffc5a35540 r __func__.0   [nvidia_uvm]
ffffffffc5860f40 d _entry_ptr.573       [nvidia_uvm]
ffffffffc1174830 t _nv025081rm  [nvidia]
ffffffffc18a63f0 t _nv005272rm  [nvidia]
ffffffffc0e3ce00 t _nv038425rm  [nvidia]
ffffffffc0f54ba0 t _nv018808rm  [nvidia]
ffffffffc1942810 t _nv000912rm  [nvidia]
ffffffffc1677b00 t _nv027710rm  [nvidia]
ffffffffc0c5b7c0 t snd_usb_clock_find_source    [snd_usb_audio]
ffffffffc09a7420 b global_uncached      [ttm]
ffffffffc07357b8 r nlm_grace_period_max [lockd]
ffffffffc04e427f r __kstrtab_nvme_unquiesce_io_queues   [nvme_core]"#;
        let syms = KAllSyms::try_from(s).unwrap();
        assert_eq!(syms.syms.len(), 40);

        let sym = syms.get("__pfx_nft_counter_do_dump").unwrap();
        assert_eq!(sym.address(), 0xffffffffc5d30760);
        assert!(!sym.visible());
        assert_eq!(sym.name(), "__pfx_nft_counter_do_dump");
        assert_eq!(sym.module_name(), Some("nf_tables"));
    }
}
