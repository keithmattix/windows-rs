pub trait IDirect3DDxgiInterfaceAccess_Impl: Sized + windows_core::IUnknownImpl {
    fn GetInterface(&self, iid: *const windows_core::GUID, p: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirect3DDxgiInterfaceAccess {}
impl IDirect3DDxgiInterfaceAccess_Vtbl {
    pub const fn new<Identity: IDirect3DDxgiInterfaceAccess_Impl, const OFFSET: isize>() -> IDirect3DDxgiInterfaceAccess_Vtbl {
        unsafe extern "system" fn GetInterface<Identity: IDirect3DDxgiInterfaceAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDxgiInterfaceAccess_Impl::GetInterface(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&p)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetInterface: GetInterface::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DDxgiInterfaceAccess as windows_core::Interface>::IID
    }
}
