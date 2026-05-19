use scs_telemetry_shared_memory::{TelemetryMap, SHARED_MEMORY_NAME};
use crate::error::Error;

#[cfg(windows)]
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::Memory::{
        MapViewOfFile, OpenFileMappingA, UnmapViewOfFile,
        MEMORY_MAPPED_VIEW_ADDRESS, FILE_MAP_READ,
    },
};

pub struct TelemetryReader {
    #[cfg(windows)]
    map_handle: HANDLE,
    #[cfg(windows)]
    view: *const TelemetryMap,
    #[cfg(not(windows))]
    _phantom: (),
}

unsafe impl Send for TelemetryReader {}
unsafe impl Sync for TelemetryReader {}

impl TelemetryReader {
    pub fn open() -> Result<Self, Error> {
        #[cfg(windows)]
        {
            let name_bytes = SHARED_MEMORY_NAME.as_bytes();
            let mut name_buf = [0u8; 64];
            let len = name_bytes.len().min(63);
            name_buf[..len].copy_from_slice(&name_bytes[..len]);

            let handle: HANDLE = unsafe {
                OpenFileMappingA(FILE_MAP_READ, 0, name_buf.as_ptr())
            };
            if handle.is_null() {
                return Err(Error::NotFound);
            }
            let mapped: MEMORY_MAPPED_VIEW_ADDRESS = unsafe {
                MapViewOfFile(handle, FILE_MAP_READ, 0, 0, core::mem::size_of::<TelemetryMap>())
            };
            let view = mapped.Value as *const TelemetryMap;
            if view.is_null() {
                unsafe { CloseHandle(handle) };
                return Err(Error::MapFailed);
            }
            Ok(TelemetryReader { map_handle: handle, view })
        }
        #[cfg(not(windows))]
        Err(Error::Unsupported)
    }

    pub fn read(&self) -> TelemetryMap {
        #[cfg(windows)]
        unsafe {
            core::ptr::read_volatile(self.view)
        }
        #[cfg(not(windows))]
        TelemetryMap::new()
    }

    pub fn is_active(&self) -> bool {
        self.read().sdk_active != 0
    }
}

impl Drop for TelemetryReader {
    fn drop(&mut self) {
        #[cfg(windows)]
        unsafe {
            if !self.view.is_null() {
                UnmapViewOfFile(MEMORY_MAPPED_VIEW_ADDRESS { Value: self.view as *mut core::ffi::c_void });
            }
            if !self.map_handle.is_null() {
                CloseHandle(self.map_handle);
            }
        }
    }
}
