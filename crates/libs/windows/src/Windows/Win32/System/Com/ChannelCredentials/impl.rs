pub trait IChannelCredentials_Impl: Sized + super::IDispatch_Impl {
    fn SetWindowsCredential(&self, domain: &windows_core::BSTR, username: &windows_core::BSTR, password: &windows_core::BSTR, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetUserNameCredential(&self, username: &windows_core::BSTR, password: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetClientCertificateFromStore(&self, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR, findyype: &windows_core::BSTR, findvalue: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetClientCertificateFromStoreByName(&self, subjectname: &windows_core::BSTR, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetClientCertificateFromFile(&self, filename: &windows_core::BSTR, password: &windows_core::BSTR, keystorageflags: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDefaultServiceCertificateFromStore(&self, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR, findtype: &windows_core::BSTR, findvalue: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetDefaultServiceCertificateFromStoreByName(&self, subjectname: &windows_core::BSTR, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDefaultServiceCertificateFromFile(&self, filename: &windows_core::BSTR, password: &windows_core::BSTR, keystorageflags: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetServiceCertificateAuthentication(&self, storelocation: &windows_core::BSTR, revocationmode: &windows_core::BSTR, certificatevalidationmode: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetIssuedToken(&self, localissueraddres: &windows_core::BSTR, localissuerbindingtype: &windows_core::BSTR, localissuerbinding: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IChannelCredentials {}
impl IChannelCredentials_Vtbl {
    pub const fn new<Identity: IChannelCredentials_Impl, const OFFSET: isize>() -> IChannelCredentials_Vtbl {
        unsafe extern "system" fn SetWindowsCredential<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, domain: core::mem::MaybeUninit<windows_core::BSTR>, username: core::mem::MaybeUninit<windows_core::BSTR>, password: core::mem::MaybeUninit<windows_core::BSTR>, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetWindowsCredential(this, core::mem::transmute(&domain), core::mem::transmute(&username), core::mem::transmute(&password), core::mem::transmute_copy(&impersonationlevel), core::mem::transmute_copy(&allowntlm)).into()
        }
        unsafe extern "system" fn SetUserNameCredential<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: core::mem::MaybeUninit<windows_core::BSTR>, password: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetUserNameCredential(this, core::mem::transmute(&username), core::mem::transmute(&password)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromStore<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: core::mem::MaybeUninit<windows_core::BSTR>, storename: core::mem::MaybeUninit<windows_core::BSTR>, findyype: core::mem::MaybeUninit<windows_core::BSTR>, findvalue: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetClientCertificateFromStore(this, core::mem::transmute(&storelocation), core::mem::transmute(&storename), core::mem::transmute(&findyype), core::mem::transmute(&findvalue)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromStoreByName<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subjectname: core::mem::MaybeUninit<windows_core::BSTR>, storelocation: core::mem::MaybeUninit<windows_core::BSTR>, storename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetClientCertificateFromStoreByName(this, core::mem::transmute(&subjectname), core::mem::transmute(&storelocation), core::mem::transmute(&storename)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromFile<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: core::mem::MaybeUninit<windows_core::BSTR>, password: core::mem::MaybeUninit<windows_core::BSTR>, keystorageflags: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetClientCertificateFromFile(this, core::mem::transmute(&filename), core::mem::transmute(&password), core::mem::transmute(&keystorageflags)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStore<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: core::mem::MaybeUninit<windows_core::BSTR>, storename: core::mem::MaybeUninit<windows_core::BSTR>, findtype: core::mem::MaybeUninit<windows_core::BSTR>, findvalue: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetDefaultServiceCertificateFromStore(this, core::mem::transmute(&storelocation), core::mem::transmute(&storename), core::mem::transmute(&findtype), core::mem::transmute(&findvalue)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStoreByName<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subjectname: core::mem::MaybeUninit<windows_core::BSTR>, storelocation: core::mem::MaybeUninit<windows_core::BSTR>, storename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetDefaultServiceCertificateFromStoreByName(this, core::mem::transmute(&subjectname), core::mem::transmute(&storelocation), core::mem::transmute(&storename)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromFile<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: core::mem::MaybeUninit<windows_core::BSTR>, password: core::mem::MaybeUninit<windows_core::BSTR>, keystorageflags: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetDefaultServiceCertificateFromFile(this, core::mem::transmute(&filename), core::mem::transmute(&password), core::mem::transmute(&keystorageflags)).into()
        }
        unsafe extern "system" fn SetServiceCertificateAuthentication<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: core::mem::MaybeUninit<windows_core::BSTR>, revocationmode: core::mem::MaybeUninit<windows_core::BSTR>, certificatevalidationmode: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetServiceCertificateAuthentication(this, core::mem::transmute(&storelocation), core::mem::transmute(&revocationmode), core::mem::transmute(&certificatevalidationmode)).into()
        }
        unsafe extern "system" fn SetIssuedToken<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localissueraddres: core::mem::MaybeUninit<windows_core::BSTR>, localissuerbindingtype: core::mem::MaybeUninit<windows_core::BSTR>, localissuerbinding: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetIssuedToken(this, core::mem::transmute(&localissueraddres), core::mem::transmute(&localissuerbindingtype), core::mem::transmute(&localissuerbinding)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetWindowsCredential: SetWindowsCredential::<Identity, OFFSET>,
            SetUserNameCredential: SetUserNameCredential::<Identity, OFFSET>,
            SetClientCertificateFromStore: SetClientCertificateFromStore::<Identity, OFFSET>,
            SetClientCertificateFromStoreByName: SetClientCertificateFromStoreByName::<Identity, OFFSET>,
            SetClientCertificateFromFile: SetClientCertificateFromFile::<Identity, OFFSET>,
            SetDefaultServiceCertificateFromStore: SetDefaultServiceCertificateFromStore::<Identity, OFFSET>,
            SetDefaultServiceCertificateFromStoreByName: SetDefaultServiceCertificateFromStoreByName::<Identity, OFFSET>,
            SetDefaultServiceCertificateFromFile: SetDefaultServiceCertificateFromFile::<Identity, OFFSET>,
            SetServiceCertificateAuthentication: SetServiceCertificateAuthentication::<Identity, OFFSET>,
            SetIssuedToken: SetIssuedToken::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChannelCredentials as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
