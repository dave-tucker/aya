//! Raw tracepoints.
use std::ffi::CString;

use crate::{
    generated::bpf_prog_type::BPF_PROG_TYPE_RAW_TRACEPOINT,
    programs::{load_program, utils::attach_raw_tracepoint, LinkRef, ProgramData, ProgramError},
    util::{get_pinned_path, PinnedObject},
};

use super::FdLink;

/// A program that can be attached at a pre-defined kernel trace point, but also
/// has an access to kernel internal arguments of trace points, which
/// differentiates them from traditional tracepoint eBPF programs.
///
/// The kernel provides a set of pre-defined trace points that eBPF programs can
/// be attached to. See`/sys/kernel/debug/tracing/events` for a list of which
/// events can be traced.
///
/// # Minimum kernel version
///
/// The minimum kernel version required to use this feature is 4.17.
///
/// # Examples
///
/// ```no_run
/// # let mut bpf = Bpf::load_file("ebpf_programs.o")?;
/// use aya::{Bpf, programs::RawTracePoint};
/// use std::convert::TryInto;
///
/// let program: &mut RawTracePoint = bpf.program_mut("sys_enter").unwrap().try_into()?;
/// program.load()?;
/// program.attach("sys_enter")?;
/// # Ok::<(), aya::BpfError>(())
/// ```
#[derive(Debug)]
#[doc(alias = "BPF_PROG_TYPE_RAW_TRACEPOINT")]
pub struct RawTracePoint {
    pub(crate) data: ProgramData,
}

impl RawTracePoint {
    /// Loads the program inside the kernel.
    ///
    /// See also [`Program::load`](crate::programs::Program::load).
    pub fn load(&mut self) -> Result<(), ProgramError> {
        load_program(BPF_PROG_TYPE_RAW_TRACEPOINT, &mut self.data)
    }

    /// Attaches the program to the given tracepoint.
    pub fn attach(&mut self, tp_name: &str) -> Result<LinkRef, ProgramError> {
        let pin_path = get_pinned_path(
            &self.data.pin_path,
            PinnedObject::Link {
                prog_name: self.data.name.clone(),
                info: tp_name.to_string(),
            },
        );
        let tp_name_c = CString::new(tp_name).unwrap();
        let fd = attach_raw_tracepoint(&mut self.data, Some(&tp_name_c))?;
        Ok(self.data.link(FdLink {
            fd: Some(fd),
            pin_path,
        }))
    }
}
