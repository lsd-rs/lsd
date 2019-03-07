use std::ffi::{OsStr, OsString};
use std::io;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;
use std::ptr::null_mut;

use winapi::ctypes::c_void;
use winapi::shared::winerror;
use winapi::um::accctrl::TRUSTEE_W;
use winapi::um::winnt;

use super::{Owner, Permissions};

// All pointers should only be used with WinAPI calls
struct SecurityDescriptor {
    sd_ptr: *mut c_void,
    owner_sid_ptr: *mut c_void,
    group_sid_ptr: *mut c_void,
    dacl_ptr: *mut winapi::um::winnt::ACL,
}

// Only sd_ptr needs to be freed
// Other pointers point to memory inside sd_ptr
impl Drop for SecurityDescriptor {
    fn drop(&mut self) {
        unsafe {
            winapi::um::winbase::LocalFree(self.sd_ptr);
        }
    }
}

impl SecurityDescriptor {
    pub fn new(path: Vec<u16>) -> Result<Self, io::Error> {
        let mut security_descriptor = SecurityDescriptor {
            sd_ptr: null_mut(),
            owner_sid_ptr: null_mut(),
            group_sid_ptr: null_mut(),
            dacl_ptr: null_mut(),
        };

        // Pointers are only valid if their corresponding X_SECURITY_INFORMATION
        // flags are set
        let error_code = unsafe {
            winapi::um::aclapi::GetNamedSecurityInfoW(
                path.as_ptr(),
                winapi::um::accctrl::SE_FILE_OBJECT,
                winnt::OWNER_SECURITY_INFORMATION
                    | winnt::GROUP_SECURITY_INFORMATION
                    | winnt::DACL_SECURITY_INFORMATION,
                &mut security_descriptor.owner_sid_ptr,
                &mut security_descriptor.group_sid_ptr,
                &mut security_descriptor.dacl_ptr,
                null_mut(),
                &mut security_descriptor.sd_ptr,
            )
        };

        match error_code {
            winerror::ERROR_SUCCESS => Ok(security_descriptor),
            _ => Err(std::io::Error::from_raw_os_error(error_code as i32)),
        }
    }

    pub fn get_owner(&mut self) -> Result<String, std::io::Error> {
        get_account_info(self.owner_sid_ptr)
    }

    pub fn get_group(&mut self) -> Result<String, std::io::Error> {
        get_account_info(self.group_sid_ptr)
    }

    pub fn get_owner_access_mask(&mut self) -> Result<u32, io::Error> {
        self.get_access_mask(self.owner_sid_ptr)
    }

    pub fn get_group_access_mask(&mut self) -> Result<u32, io::Error> {
        self.get_access_mask(self.group_sid_ptr)
    }

    pub fn get_world_access_mask(&mut self) -> Result<u32, io::Error> {
        let world_sid = get_world_sid()?;

        self.get_access_mask(world_sid.as_ptr() as *mut c_void)
    }

    fn get_access_mask(&mut self, sid: *mut c_void) -> Result<u32, io::Error> {
        let mut trustee = trustee_from_sid(sid);

        get_acl_access_mask(self.dacl_ptr as *mut _, &mut trustee)
    }
}

pub fn get_file_data(path: &PathBuf) -> Result<(Owner, Permissions), io::Error> {
    // Assumptions:
    // - windows_path is a null-terminated WTF-16-encoded string
    let windows_path = buf_from_os(path.as_os_str());
    let mut security_descriptor = SecurityDescriptor::new(windows_path)?;

    let owner = security_descriptor.get_owner()?;
    let group = security_descriptor.get_group()?;

    let owner = Owner::new(owner, group);

    let owner_access_mask = security_descriptor.get_owner_access_mask()?;
    let group_access_mask = security_descriptor.get_group_access_mask()?;
    let world_access_mask = security_descriptor.get_world_access_mask()?;

    let has_bit = |field: u32, bit: u32| field & bit != 0;

    let permissions = Permissions {
        user_read: has_bit(owner_access_mask, winnt::FILE_GENERIC_READ),
        user_write: has_bit(owner_access_mask, winnt::FILE_GENERIC_WRITE),
        user_execute: has_bit(owner_access_mask, winnt::FILE_GENERIC_EXECUTE),

        group_read: has_bit(group_access_mask, winnt::FILE_GENERIC_READ),
        group_write: has_bit(group_access_mask, winnt::FILE_GENERIC_WRITE),
        group_execute: has_bit(group_access_mask, winnt::FILE_GENERIC_EXECUTE),

        other_read: has_bit(world_access_mask, winnt::FILE_GENERIC_READ),
        other_write: has_bit(world_access_mask, winnt::FILE_GENERIC_WRITE),
        other_execute: has_bit(world_access_mask, winnt::FILE_GENERIC_EXECUTE),

        sticky: false,
        setuid: false,
        setgid: false,
    };

    Ok((owner, permissions))
}

fn get_last_error() -> i32 {
    unsafe { winapi::um::errhandlingapi::GetLastError() as i32 }
}

fn get_world_sid() -> Result<Vec<u8>, io::Error> {
    // Get the size and allocate bytes for a 1-sub-authority SID
    // 1 sub-authority because the Windows World SID is always S-1-1-0, with
    // only a single sub-authority.
    //
    // Assumptions: None
    // "This function cannot fail"
    //     -- Windows Dev Center docs
    let mut world_sid_len: u32 = unsafe { winapi::um::securitybaseapi::GetSidLengthRequired(1) };
    let mut world_sid = vec![0u8; world_sid_len as usize];

    // Assumptions:
    // - world_sid_len is no larger than the number of bytes available at
    //   world_sid
    // - world_sid is appropriately aligned (if there are strange crashes this
    //   might be why)
    let result = unsafe {
        winapi::um::securitybaseapi::CreateWellKnownSid(
            winnt::WinWorldSid,
            null_mut(),
            world_sid.as_mut_ptr() as *mut _,
            &mut world_sid_len,
        )
    };

    match result {
        0 => Err(io::Error::from_raw_os_error(get_last_error())),
        _ => Ok(world_sid),
    }
}

/// Evaluate an ACL for a particular trustee and get its access rights
///
/// Assumptions:
/// - acl_ptr points to a valid ACL data structure
/// - trustee_ptr points to a valid trustee data structure
/// - Both remain valid through the function call (no long-term requirement)
fn get_acl_access_mask(
    acl_ptr: *mut c_void,
    trustee_ptr: *mut TRUSTEE_W,
) -> Result<u32, io::Error> {
    let mut access_mask = 0;

    // Assumptions:
    // - All function assumptions
    // - Result is not valid until return value is checked
    let err_code = unsafe {
        winapi::um::aclapi::GetEffectiveRightsFromAclW(
            acl_ptr as *mut _,
            trustee_ptr,
            &mut access_mask,
        )
    };

    match err_code {
        winerror::ERROR_SUCCESS => Ok(access_mask),
        _ => Err(io::Error::from_raw_os_error(err_code as i32)),
    }
}

/// Get a trustee buffer from a SID
///
/// Assumption: sid is valid, and the trustee is only valid as long as the SID
/// is
///
/// Note: winapi's TRUSTEE_W looks different from the one in the MS docs because
/// of some unusal pre-processor macros in the original .h file. The winapi
/// version is correct (MS's doc generator messed up)
fn trustee_from_sid(sid_ptr: *mut c_void) -> TRUSTEE_W {
    let mut trustee: TRUSTEE_W = unsafe { std::mem::zeroed() };

    unsafe {
        winapi::um::aclapi::BuildTrusteeWithSidW(&mut trustee, sid_ptr);
    }

    trustee
}

/// Get a username and domain name from a SID
///
/// Returns null-terminated Vec's, one for the name and one for the domain.
fn lookup_account_sid(sid: *mut c_void) -> Result<(Vec<u16>, Vec<u16>), std::io::Error> {
    const BUF_SIZE: u32 = 256;
    let mut name_size = BUF_SIZE;
    let mut domain_size = BUF_SIZE;

    loop {
        let mut name: Vec<u16> = vec![0; name_size as usize];
        let mut domain: Vec<u16> = vec![0; domain_size as usize];

        let old_name_size = name_size;
        let old_domain_size = domain_size;

        let mut sid_name_use = 0;

        // Assumptions:
        // - after call, name_size and domain_size accurately reflect the sizes
        let result = unsafe {
            winapi::um::winbase::LookupAccountSidW(
                null_mut(),
                sid,
                name.as_mut_ptr(),
                &mut name_size,
                domain.as_mut_ptr(),
                &mut domain_size,
                &mut sid_name_use,
            )
        };

        if result != 0 {
            return Ok((name, domain));
        } else if name_size != old_name_size || domain_size != old_domain_size {
            // Need bigger buffers
            // name_size and domain_size are already set, just loop
            continue;
        } else {
            // Some other failure
            return Err(io::Error::from_raw_os_error(get_last_error()));
        }
    }
}

fn get_account_info(sid: *mut c_void) -> Result<String, std::io::Error> {
    let (name, domain) = lookup_account_sid(sid)?;
    let name = os_from_buf(&name);
    let domain = os_from_buf(&domain);

    Ok(format!("{}\\{}", domain.to_string_lossy(), name.to_string_lossy()))
}

/// Create an `OsString` from a NUL-terminated buffer
///
/// Decodes the WTF-16 encoded buffer until it hits a NUL (code point 0).
/// Everything after and including that code point is not included.
fn os_from_buf(buf: &[u16]) -> OsString {
    OsString::from_wide(
        &buf.iter()
            .cloned()
            .take_while(|&n| n != 0)
            .collect::<Vec<u16>>(),
    )
}

/// Create a WTF-16-encoded NUL-terminated buffer from an `OsStr`.
///
/// Decodes the `OsStr`, then appends a NUL.
fn buf_from_os(os: &OsStr) -> Vec<u16> {
    let mut buf: Vec<u16> = os.encode_wide().collect();
    buf.push(0);
    buf
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_wtf16_behavior() {
        let basic_os = OsString::from("TeSt");
        let basic_buf = vec![0x54, 0x65, 0x53, 0x74, 0x00];
        let basic_buf_nuls = vec![0x54, 0x65, 0x53, 0x74, 0x00, 0x00, 0x00, 0x00];

        assert_eq!(os_from_buf(&basic_buf), basic_os);
        assert_eq!(buf_from_os(&basic_os), basic_buf);
        assert_eq!(os_from_buf(&basic_buf_nuls), basic_os);

        let unicode_os = OsString::from("💩");
        let unicode_buf = vec![0xd83d, 0xdca9, 0x0];
        let unicode_buf_nuls = vec![0xd83d, 0xdca9, 0x0, 0x0, 0x0, 0x0, 0x0];

        assert_eq!(os_from_buf(&unicode_buf), unicode_os);
        assert_eq!(buf_from_os(&unicode_os), unicode_buf);
        assert_eq!(os_from_buf(&unicode_buf_nuls), unicode_os);
    }

    #[test]
    fn every_wtf16_codepair_roundtrip() {
        for lsb in 0..256u16 {
            let mut vec: Vec<u16> = Vec::with_capacity(257);

            for msb in 0..=256u16 {
                let val = msb << 8 | lsb;

                if val != 0 {
                    vec.push(val)
                }
            }

            vec.push(0);

            let os = os_from_buf(&vec);
            let new_vec = buf_from_os(&os);

            assert_eq!(&vec, &new_vec);
        }
    }

    #[test]
    fn get_file_data_success() {
        let path = PathBuf::from("Cargo.toml");

        match get_file_data(&path) {
            Ok(_) => assert!(true),
            _ => assert!(false),
        }
    }
}
