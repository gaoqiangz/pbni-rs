use crate::primitive::PBString;
use libloading::os::windows::{Library, Symbol};

pub struct PBLibrary {
    pbvm: Library,
    pbsys: Library,
    pbshr: Library,
    version: u32
}

impl PBLibrary {
    pub unsafe fn load() -> Result<Self, PBLibraryError> {
        use std::{mem, ptr};
        use winapi::{
            shared::minwindef::{DWORD, HMODULE, MAX_PATH}, um::{
                fileapi::GetFullPathNameW, processthreadsapi::GetCurrentProcess, psapi::{EnumProcessModules, GetModuleFileNameExW}, winnt::{LPWSTR, WCHAR}
            }
        };

        let mut pbvm = None;
        let mut pbsys = None;
        let mut pbshr = None;
        let mut version = 0;

        let hProcess = GetCurrentProcess();
        let mut hMods: [HMODULE; 1024] = [ptr::null_mut(); 1024];
        let mut cbNeeded: DWORD = 0;
        if EnumProcessModules(
            hProcess,
            hMods.as_mut_ptr(),
            mem::size_of_val(&hMods) as DWORD,
            (&mut cbNeeded) as *mut _
        ) != 1
        {
            return Err("EnumProcessModules failed".into());
        }
        let cnt = (cbNeeded as usize / mem::size_of::<HMODULE>()).min(hMods.len());
        for idx in 0..cnt {
            let mut path: [WCHAR; MAX_PATH] = mem::zeroed();
            let mut full_name: [WCHAR; MAX_PATH] = mem::zeroed();
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
                if let Some((dig, _)) = file_name[4..].split_once(".") {
                    if let Ok(ver) = dig.parse() {
                        version = ver;
                        if version < 110 {
                            return Err("PBVM版本号不能低于11".into());
                        }
                    } else {
                        return Err("无效的PBVM版本号".into());
                    }
                }
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
                    pbshr,
                    version
                })
            },
            (None, _, _) => return Err("PBVM模块未找到".into()),
            (_, None, _) => return Err("PBSYS模块未找到".into()),
            (_, _, None) => return Err("PBSHR模块未找到".into())
        }
    }

    pub unsafe fn get<T>(&self, symbol: &[u8]) -> Result<Symbol<T>, PBLibraryError> {
        match self.pbvm.get(symbol).or_else(|_| self.pbsys.get(symbol)).or_else(|_| self.pbshr.get(symbol)) {
            Ok(sym) => Ok(sym),
            Err(e) => Err(PBLibraryError::FindSymbol(String::from_utf8_lossy(symbol).into_owned(), e))
        }
    }

    pub fn version(&self) -> u32 { self.version }
}

#[derive(Debug)]
pub enum PBLibraryError {
    LibLoading(libloading::Error),
    FindSymbol(String, libloading::Error),
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
            PBLibraryError::FindSymbol(sym, e) => write!(f, "symbol: {}, error: {}", sym, e),
            PBLibraryError::Other(e) => write!(f, "{e}")
        }
    }
}
