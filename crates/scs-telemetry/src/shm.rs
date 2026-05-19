use core::mem::size_of;
use scs_telemetry_shared_memory::TelemetryMap;

#[cfg(windows)]
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::Memory::{
        CreateFileMappingA, MapViewOfFile, UnmapViewOfFile,
        MEMORY_MAPPED_VIEW_ADDRESS, FILE_MAP_WRITE, PAGE_READWRITE,
    },
};

static mut SHM_HANDLE: Option<ShmWriter> = None;

pub struct ShmWriter {
    #[cfg(windows)]
    map_handle: HANDLE,
    #[cfg(windows)]
    view: *mut TelemetryMap,
    #[cfg(not(windows))]
    _phantom: (),
}

unsafe impl Send for ShmWriter {}
unsafe impl Sync for ShmWriter {}

impl ShmWriter {
    pub fn create() -> Result<Self, ()> {
        #[cfg(windows)]
        {
            use scs_telemetry_shared_memory::SHARED_MEMORY_NAME;
            let name_bytes = SHARED_MEMORY_NAME.as_bytes();
            let mut name_buf = [0u8; 64];
            let len = name_bytes.len().min(63);
            name_buf[..len].copy_from_slice(&name_bytes[..len]);

            let size = size_of::<TelemetryMap>() as u32;
            let handle: HANDLE = unsafe {
                CreateFileMappingA(
                    windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE,
                    core::ptr::null(),
                    PAGE_READWRITE,
                    0,
                    size,
                    name_buf.as_ptr(),
                )
            };
            if handle.is_null() {
                return Err(());
            }
            let mapped: MEMORY_MAPPED_VIEW_ADDRESS = unsafe {
                MapViewOfFile(handle, FILE_MAP_WRITE, 0, 0, size as usize)
            };
            let view = mapped.Value as *mut TelemetryMap;
            if view.is_null() {
                unsafe { CloseHandle(handle) };
                return Err(());
            }
            unsafe { core::ptr::write(view, TelemetryMap::new()) };
            Ok(ShmWriter { map_handle: handle, view })
        }
        #[cfg(not(windows))]
        Err(())
    }

    pub fn map(&mut self) -> Option<&mut TelemetryMap> {
        #[cfg(windows)]
        if !self.view.is_null() {
            return Some(unsafe { &mut *self.view });
        }
        None
    }
}

impl Drop for ShmWriter {
    fn drop(&mut self) {
        #[cfg(windows)]
        unsafe {
            if !self.view.is_null() {
                UnmapViewOfFile(MEMORY_MAPPED_VIEW_ADDRESS { Value: self.view.cast() });
            }
            if !self.map_handle.is_null() {
                CloseHandle(self.map_handle);
            }
        }
    }
}

pub fn init() -> Result<(), ()> {
    let writer = ShmWriter::create()?;
    unsafe { SHM_HANDLE = Some(writer) };
    Ok(())
}

pub fn shutdown() {
    unsafe { SHM_HANDLE = None };
}

pub fn with_map<F: FnOnce(&mut TelemetryMap)>(f: F) {
    // SAFETY: plugin DLL is single-threaded (game calls init/shutdown/callbacks on one thread)
    let slot = &raw mut SHM_HANDLE;
    if let Some(ref mut w) = unsafe { (*slot).as_mut() } {
        if let Some(map) = w.map() {
            f(map);
        }
    }
}
