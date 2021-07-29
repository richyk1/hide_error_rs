use bindings::{
    Windows::Win32::Foundation::{CloseHandle, BOOL, HANDLE, HINSTANCE},
    Windows::Win32::System::Diagnostics::Debug::WriteProcessMemory,
    Windows::Win32::System::Memory::{LocalAlloc, LocalFree, LPTR},
    Windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS},
    Windows::Win32::UI::WindowsAndMessaging::{FindWindowA, GetWindowThreadProcessId, IsWindow},
    Windows::Win32::{Foundation::PSTR, System::ProcessStatus::K32EnumProcessModules},
};

use log::error;

fn get_process_base_address(process_handle: HANDLE) -> usize {
    let mut base_address = 0;
    let module_array_bytes;
    let mut bytes_required: u32 = 0;

    unsafe {
        let result: BOOL =
            K32EnumProcessModules(process_handle, std::ptr::null_mut(), 0, &mut bytes_required);
        if result == true {
            module_array_bytes = LocalAlloc(LPTR, bytes_required as usize);

            let result = K32EnumProcessModules(
                process_handle,
                module_array_bytes as *mut HINSTANCE,
                bytes_required,
                &mut bytes_required,
            );
            if result == true {
                base_address = *(module_array_bytes as *mut usize);
            }

            LocalFree(module_array_bytes);
        }
    }
    return base_address;
}

fn main() {
    env_logger::init();

    let window_name = "Europa Universalis IV";

    loop {
        let window;
        unsafe {
            window = FindWindowA(PSTR::NULL, window_name);
            if IsWindow(window) == true {
                error!("{:?}", window);
                error!("Found window");
                let mut proc_id: u32 = 0;

                GetWindowThreadProcessId(window, &mut proc_id);
                error!("ProcID: {:?}", proc_id);

                let handle = OpenProcess(PROCESS_ALL_ACCESS, false, proc_id);
                error!("Handle: {:?}", handle);

                let base_addr = get_process_base_address(handle);
                let arr: [u8; 6] = [0x90, 0x90, 0x90, 0x90, 0x90, 0x90];
                WriteProcessMemory(
                    handle,
                    (base_addr + 0x7BF10D) as *mut std::ffi::c_void,
                    arr.as_ptr() as *const std::ffi::c_void,
                    arr.len(),
                    std::ptr::null_mut(),
                ); // FF 15 ? ? ? ? FF 15 ? ? ? ? 85 C0

                error!("Base address: {:#04x}", base_addr);

                CloseHandle(handle);
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
