use crate::pbstr::PBString;
use libloading::os::windows::{Library, Symbol};

pub struct PBLibrary {
    pbvm: Library,
    pbsys: Library,
    pbshr: Library
}

impl PBLibrary {
    pub unsafe fn load() -> Result<Self, PBLibraryError> {
        use std::{
            mem::{size_of, size_of_val}, ptr
        };
        use winapi::{
            shared::minwindef::{DWORD, HMODULE, MAX_PATH}, um::{
                fileapi::GetFullPathNameW, processthreadsapi::GetCurrentProcess, psapi::{EnumProcessModules, GetModuleFileNameExW}, winnt::{LPWSTR, WCHAR}
            }
        };

        let mut pbvm = None;
        let mut pbsys = None;
        let mut pbshr = None;

        let hProcess = GetCurrentProcess();
        let mut hMods: [HMODULE; 1024] = [ptr::null_mut(); 1024];
        let mut cbNeeded: DWORD = 0;
        if EnumProcessModules(
            hProcess,
            hMods.as_mut_ptr(),
            size_of_val(&hMods) as DWORD,
            (&mut cbNeeded) as *mut _
        ) != 1
        {
            return Err("EnumProcessModules failed".into());
        }
        let cnt = (cbNeeded as usize / size_of::<HMODULE>()).min(hMods.len());
        for idx in 0..cnt {
            let mut path: [WCHAR; MAX_PATH] = [0; MAX_PATH];
            let mut full_name: [WCHAR; MAX_PATH] = [0; MAX_PATH];
            let mut file_name: LPWSTR = ptr::null_mut();
            GetModuleFileNameExW(hProcess, hMods[idx], (&mut path) as *mut _, MAX_PATH as DWORD);
            GetFullPathNameW(
                (&path) as *const _,
                MAX_PATH as DWORD,
                (&mut full_name) as *mut _,
                (&mut file_name) as *mut _
            );
            let file_name = PBString::from_ptr_str(file_name).to_string_lossy();
            if file_name[0..4].eq_ignore_ascii_case("PBVM") {
                pbvm = Some(Library::open_already_loaded(file_name)?);
            } else if file_name[0..5].eq_ignore_ascii_case("PBSYS") {
                pbsys = Some(Library::open_already_loaded(file_name)?);
            } else if file_name[0..5].eq_ignore_ascii_case("PBSHR") {
                pbshr = Some(Library::open_already_loaded(file_name)?);
            }
        }

        match (pbvm, pbsys, pbshr) {
            (Some(pbvm), Some(pbsys), Some(pbshr)) => {
                Ok(Self {
                    pbvm,
                    pbsys,
                    pbshr
                })
            },
            (None, _, _) => return Err("PBVM DLL NotFound".into()),
            (_, None, _) => return Err("PBSYS DLL NotFound".into()),
            (_, _, None) => return Err("PBSHR DLL NotFound".into())
        }
    }

    pub unsafe fn get<T>(&self, symbol: &[u8]) -> Result<Symbol<T>, PBLibraryError> {
        match self.pbvm.get(symbol).or_else(|_| self.pbsys.get(symbol)).or_else(|_| self.pbshr.get(symbol)) {
            Ok(sym) => Ok(sym),
            Err(e) => Err(e.into())
        }
    }
}

#[derive(Debug)]
pub enum PBLibraryError {
    LibLoading(libloading::Error),
    Other(String)
}

impl From<libloading::Error> for PBLibraryError {
    fn from(e: libloading::Error) -> Self { PBLibraryError::LibLoading(e) }
}

impl From<String> for PBLibraryError {
    fn from(e: String) -> Self { PBLibraryError::Other(e) }
}

impl<'a> From<&'a str> for PBLibraryError {
    fn from(e: &'a str) -> Self { PBLibraryError::Other(e.to_owned()) }
}

impl std::error::Error for PBLibraryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            PBLibraryError::LibLoading(ref source) => source.source(),
            _ => None
        }
    }
}

impl std::fmt::Display for PBLibraryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PBLibraryError::LibLoading(e) => e.fmt(f),
            PBLibraryError::Other(e) => write!(f, "{e}")
        }
    }
}
