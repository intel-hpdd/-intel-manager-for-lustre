/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl<T> ::std::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
pub const LUSTRE_MAXFSNAME: u32 = 8;
pub const OBD_MAX_FIDS_IN_ARRAY: u32 = 4096;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __u32 = ::std::os::raw::c_uint;
pub type __u64 = ::std::os::raw::c_ulonglong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __time_t = ::std::os::raw::c_long;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[test]
fn bindgen_test_layout_timespec() {
    assert_eq!(
        ::std::mem::size_of::<timespec>(),
        16usize,
        concat!("Size of: ", stringify!(timespec))
    );
    assert_eq!(
        ::std::mem::align_of::<timespec>(),
        8usize,
        concat!("Alignment of ", stringify!(timespec))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::std::os::raw::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __unused: [__syscall_slong_t; 3usize],
}
#[test]
fn bindgen_test_layout_stat() {
    assert_eq!(
        ::std::mem::size_of::<stat>(),
        144usize,
        concat!("Size of: ", stringify!(stat))
    );
    assert_eq!(
        ::std::mem::align_of::<stat>(),
        8usize,
        concat!("Alignment of ", stringify!(stat))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct lu_fid {
    pub f_seq: __u64,
    pub f_oid: __u32,
    pub f_ver: __u32,
}
#[test]
fn bindgen_test_layout_lu_fid() {
    assert_eq!(
        ::std::mem::size_of::<lu_fid>(),
        16usize,
        concat!("Size of: ", stringify!(lu_fid))
    );
    assert_eq!(
        ::std::mem::align_of::<lu_fid>(),
        8usize,
        concat!("Alignment of ", stringify!(lu_fid))
    );
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ost_id {
    pub __bindgen_anon_1: ost_id__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ost_id__bindgen_ty_1 {
    pub oi: ost_id__bindgen_ty_1__bindgen_ty_1,
    pub oi_fid: lu_fid,
    _bindgen_union_align: [u64; 2usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ost_id__bindgen_ty_1__bindgen_ty_1 {
    pub oi_id: __u64,
    pub oi_seq: __u64,
}
#[test]
fn bindgen_test_layout_ost_id__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<ost_id__bindgen_ty_1__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(ost_id__bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<ost_id__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(ost_id__bindgen_ty_1__bindgen_ty_1)
        )
    );
}
#[test]
fn bindgen_test_layout_ost_id__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<ost_id__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(ost_id__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<ost_id__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(ost_id__bindgen_ty_1))
    );
}
impl Default for ost_id__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_ost_id() {
    assert_eq!(
        ::std::mem::size_of::<ost_id>(),
        16usize,
        concat!("Size of: ", stringify!(ost_id))
    );
    assert_eq!(
        ::std::mem::align_of::<ost_id>(),
        8usize,
        concat!("Alignment of ", stringify!(ost_id))
    );
}
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct lov_user_ost_data_v1 {
    pub l_ost_oi: ost_id,
    pub l_ost_gen: __u32,
    pub l_ost_idx: __u32,
}
#[test]
fn bindgen_test_layout_lov_user_ost_data_v1() {
    assert_eq!(
        ::std::mem::size_of::<lov_user_ost_data_v1>(),
        24usize,
        concat!("Size of: ", stringify!(lov_user_ost_data_v1))
    );
    assert_eq!(
        ::std::mem::align_of::<lov_user_ost_data_v1>(),
        1usize,
        concat!("Alignment of ", stringify!(lov_user_ost_data_v1))
    );
}
#[repr(C, packed)]
#[derive(Default)]
pub struct lov_user_md_v1 {
    pub lmm_magic: __u32,
    pub lmm_pattern: __u32,
    pub lmm_oi: ost_id,
    pub lmm_stripe_size: __u32,
    pub lmm_stripe_count: __u16,
    pub __bindgen_anon_1: lov_user_md_v1__bindgen_ty_1,
    pub lmm_objects: __IncompleteArrayField<lov_user_ost_data_v1>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union lov_user_md_v1__bindgen_ty_1 {
    pub lmm_stripe_offset: __u16,
    pub lmm_layout_gen: __u16,
    _bindgen_union_align: u16,
}
#[test]
fn bindgen_test_layout_lov_user_md_v1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<lov_user_md_v1__bindgen_ty_1>(),
        2usize,
        concat!("Size of: ", stringify!(lov_user_md_v1__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<lov_user_md_v1__bindgen_ty_1>(),
        2usize,
        concat!("Alignment of ", stringify!(lov_user_md_v1__bindgen_ty_1))
    );
}
impl Default for lov_user_md_v1__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_lov_user_md_v1() {
    assert_eq!(
        ::std::mem::size_of::<lov_user_md_v1>(),
        32usize,
        concat!("Size of: ", stringify!(lov_user_md_v1))
    );
    assert_eq!(
        ::std::mem::align_of::<lov_user_md_v1>(),
        1usize,
        concat!("Alignment of ", stringify!(lov_user_md_v1))
    );
}
#[repr(C, packed)]
pub struct lov_user_mds_data_v1 {
    pub lmd_st: lstat_t,
    pub lmd_lmm: lov_user_md_v1,
}
#[test]
fn bindgen_test_layout_lov_user_mds_data_v1() {
    assert_eq!(
        ::std::mem::size_of::<lov_user_mds_data_v1>(),
        176usize,
        concat!("Size of: ", stringify!(lov_user_mds_data_v1))
    );
    assert_eq!(
        ::std::mem::align_of::<lov_user_mds_data_v1>(),
        1usize,
        concat!("Alignment of ", stringify!(lov_user_mds_data_v1))
    );
}
#[repr(C)]
#[derive(Default, Clone)]
pub struct fid_array {
    pub fa_nr: __u32,
    pub fa_padding0: __u32,
    pub fa_padding1: __u64,
    pub fa_fids: __IncompleteArrayField<lu_fid>,
}
#[test]
fn bindgen_test_layout_fid_array() {
    assert_eq!(
        ::std::mem::size_of::<fid_array>(),
        16usize,
        concat!("Size of: ", stringify!(fid_array))
    );
    assert_eq!(
        ::std::mem::align_of::<fid_array>(),
        8usize,
        concat!("Alignment of ", stringify!(fid_array))
    );
}
