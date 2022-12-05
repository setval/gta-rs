use windows::Win32::{Foundation::HINSTANCE, System::LibraryLoader::FreeLibraryAndExitThread};

pub unsafe fn free(hmodule: HINSTANCE) {
    FreeLibraryAndExitThread(hmodule, 0);
}