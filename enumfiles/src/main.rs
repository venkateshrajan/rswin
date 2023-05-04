use windows_sys::Win32::Storage::FileSystem::{FindFirstFileA, FindNextFileA, WIN32_FIND_DATAA};
use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE, GetLastError};
use windows_sys::Win32::System::Memory::*;

fn enumerate_files(foldername: &str) {
    unsafe {
        let buffer = LocalAlloc(LPTR, std::mem::size_of::<WIN32_FIND_DATAA>());
        let finddata: *mut WIN32_FIND_DATAA = buffer as *mut WIN32_FIND_DATAA;
        let cfoldername = std::ffi::CString::new((String::from(foldername) + "\\*").as_str()).unwrap();
        let hfind = FindFirstFileA(cfoldername.as_c_str().as_ptr() as *const u8, finddata);
        if hfind != INVALID_HANDLE_VALUE {
            println!("Items under {}", foldername);
        } else {
            let error = GetLastError();
            panic!("Invalid input {}, Error {:#x}:{}", foldername, error, error);
        }
        LocalFree(buffer);

        while hfind != INVALID_HANDLE_VALUE { 
            // Todo: Consider zeroing the memory instead of reallocation
            let buffer1 = LocalAlloc(LPTR, std::mem::size_of::<WIN32_FIND_DATAA>());
            let finddata1: *mut WIN32_FIND_DATAA = buffer1 as *mut WIN32_FIND_DATAA;

            if FindNextFileA(hfind, finddata1) == 0 { break; }
            println!("{}", String::from_utf8_lossy(&(*finddata1).cFileName));

            LocalFree(buffer1);
        }

        CloseHandle(hfind);

        println!();
    }
}

fn main() {
    enumerate_files("D:\\Workspace");
    enumerate_files("D:");
}

