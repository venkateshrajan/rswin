use windows_sys::Win32::Storage::FileSystem::{FindFirstFileA, WIN32_FIND_DATAA};
use windows_sys::Win32::Foundation::{FILETIME, INVALID_HANDLE_VALUE};

fn main() {
    unsafe {
        let finddata: *mut WIN32_FIND_DATAA = &mut WIN32_FIND_DATAA{
            dwFileAttributes: 0,
            ftLastAccessTime: FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 },
            ftLastWriteTime: FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 },
            ftCreationTime: FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 },
            nFileSizeHigh: 0,
            nFileSizeLow: 0,
            dwReserved0: 0,
            dwReserved1: 0,
            cFileName: [0; 260],
            cAlternateFileName: [0; 14],
        };
        let foldername: *const u8 = "c:\\".as_bytes().as_ptr();
        if FindFirstFileA(foldername, finddata) == INVALID_HANDLE_VALUE
        {
            println!("Cannot find C:");
        }
    }
}
