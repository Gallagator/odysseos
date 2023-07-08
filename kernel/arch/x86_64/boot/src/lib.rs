#![no_std]

use kernel_boot_interface::{
    framebuf,
    hhdm::{self, BootHhdm},
    memmap, BootInfo,
};
use lazy_static::lazy_static;
use limine::{FramebufferRequest, HhdmRequest, MemmapRequest};

static MEMMAP_REQUEST: MemmapRequest = MemmapRequest::new(0);
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new(0);
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new(0);

lazy_static! {
    static ref BOOT_INFO: BootInfo = retrieve_boot_info();
}

pub fn arch_init() -> &'static BootInfo {
    return &BOOT_INFO;
}

fn retrieve_boot_info() -> BootInfo {
    let memmap = get_memmap();
    let hhdm = get_hhdm();
    let frame_buffer = get_framebuffer(&hhdm);

    BootInfo {
        memmap,
        frame_buffer,
        hhdm,
    }
}

fn get_memmap() -> memmap::Memmap {
    if let Some(memmap_response) = MEMMAP_REQUEST.get_response().get() {
        debug_assert!(memmap_response.entry_count <= memmap::MAX_MEM_REGIONS as u64);
        let mut memmap: memmap::Memmap = unsafe { core::mem::zeroed() };
        memmap.entry_count = memmap_response.entry_count as usize;

        memmap_response
            .memmap()
            .iter()
            .map(|entry| convert_memmap_entry(entry))
            .enumerate()
            .for_each(|(i, e)| memmap.entries[i] = e);
        memmap
    } else {
        panic!("No memmap response from limine.");
    }
}

fn get_framebuffer(hhdm: &BootHhdm) -> framebuf::BootFrameBuf {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
        let framebuffer = &framebuffer_response.framebuffers()[0];
        framebuf::BootFrameBuf {
            phys_address: framebuffer.address.as_ptr().unwrap() as usize - hhdm.base,
            width: framebuffer.width,
            height: framebuffer.height,
            pitch: framebuffer.pitch,
            bpp: framebuffer.bpp,
        }
    } else {
        panic!("No framebuffer response from limine.");
    }
}

fn get_hhdm() -> hhdm::BootHhdm {
    let hhdm_response = HHDM_REQUEST
        .get_response()
        .get()
        .expect("No hhdm response from limine.");
    hhdm::BootHhdm {
        base: hhdm_response.offset as usize,
    }
}

fn convert_memmap_entry(entry: &limine::MemmapEntry) -> memmap::MemmapEntry {
    let typ = match entry.typ {
        limine::MemoryMapEntryType::Usable => memmap::MemType::Usable,
        limine::MemoryMapEntryType::Reserved => memmap::MemType::Reserved,
        limine::MemoryMapEntryType::AcpiReclaimable => memmap::MemType::AcpiReclaimable,
        limine::MemoryMapEntryType::AcpiNvs => memmap::MemType::AcpiNvs,
        limine::MemoryMapEntryType::BadMemory => memmap::MemType::Reserved,
        limine::MemoryMapEntryType::BootloaderReclaimable => memmap::MemType::BootloaderReclaimable,
        limine::MemoryMapEntryType::KernelAndModules => memmap::MemType::Reserved,
        limine::MemoryMapEntryType::Framebuffer => memmap::MemType::Reserved,
    };

    memmap::MemmapEntry {
        base: entry.base as usize,
        len: entry.len as usize,
        typ,
    }
}
