fn main() {
    windows::build! {
        Windows::Win32::System::ProcessStatus::{K32EnumProcessModules},
        Windows::Win32::Foundation::{HANDLE, BOOL, HINSTANCE, PSTR, CloseHandle},
        Windows::Win32::System::Memory::{LocalAlloc, LOCAL_ALLOC_FLAGS, LocalFree},
        Windows::Win32::System::Threading::{OpenProcess, PROCESS_ACCESS_RIGHTS},
        Windows::Win32::UI::WindowsAndMessaging::{FindWindowA, IsWindow, GetWindowThreadProcessId},
        Windows::Win32::System::Diagnostics::Debug::WriteProcessMemory
    };
}
