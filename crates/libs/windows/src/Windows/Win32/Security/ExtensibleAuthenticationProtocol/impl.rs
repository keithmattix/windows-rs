pub trait IAccountingProviderConfig_Impl: Sized + windows_core::IUnknownImpl {
    fn Initialize(&self, pszmachinename: &windows_core::PCWSTR) -> windows_core::Result<usize>;
    fn Uninitialize(&self, uconnectionparam: usize) -> windows_core::Result<()>;
    fn Configure(&self, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> windows_core::Result<()>;
    fn Activate(&self, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> windows_core::Result<()>;
    fn Deactivate(&self, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAccountingProviderConfig {}
impl IAccountingProviderConfig_Vtbl {
    pub const fn new<Identity: IAccountingProviderConfig_Impl, const OFFSET: isize>() -> IAccountingProviderConfig_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IAccountingProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmachinename: windows_core::PCWSTR, puconnectionparam: *mut usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccountingProviderConfig_Impl::Initialize(this, core::mem::transmute(&pszmachinename)) {
                Ok(ok__) => {
                    puconnectionparam.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninitialize<Identity: IAccountingProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uconnectionparam: usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccountingProviderConfig_Impl::Uninitialize(this, core::mem::transmute_copy(&uconnectionparam)).into()
        }
        unsafe extern "system" fn Configure<Identity: IAccountingProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccountingProviderConfig_Impl::Configure(this, core::mem::transmute_copy(&uconnectionparam), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ureserved1), core::mem::transmute_copy(&ureserved2)).into()
        }
        unsafe extern "system" fn Activate<Identity: IAccountingProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccountingProviderConfig_Impl::Activate(this, core::mem::transmute_copy(&uconnectionparam), core::mem::transmute_copy(&ureserved1), core::mem::transmute_copy(&ureserved2)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: IAccountingProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccountingProviderConfig_Impl::Deactivate(this, core::mem::transmute_copy(&uconnectionparam), core::mem::transmute_copy(&ureserved1), core::mem::transmute_copy(&ureserved2)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Uninitialize: Uninitialize::<Identity, OFFSET>,
            Configure: Configure::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccountingProviderConfig as windows_core::Interface>::IID
    }
}
pub trait IAuthenticationProviderConfig_Impl: Sized + windows_core::IUnknownImpl {
    fn Initialize(&self, pszmachinename: &windows_core::PCWSTR) -> windows_core::Result<usize>;
    fn Uninitialize(&self, uconnectionparam: usize) -> windows_core::Result<()>;
    fn Configure(&self, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> windows_core::Result<()>;
    fn Activate(&self, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> windows_core::Result<()>;
    fn Deactivate(&self, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAuthenticationProviderConfig {}
impl IAuthenticationProviderConfig_Vtbl {
    pub const fn new<Identity: IAuthenticationProviderConfig_Impl, const OFFSET: isize>() -> IAuthenticationProviderConfig_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IAuthenticationProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmachinename: windows_core::PCWSTR, puconnectionparam: *mut usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAuthenticationProviderConfig_Impl::Initialize(this, core::mem::transmute(&pszmachinename)) {
                Ok(ok__) => {
                    puconnectionparam.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninitialize<Identity: IAuthenticationProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uconnectionparam: usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAuthenticationProviderConfig_Impl::Uninitialize(this, core::mem::transmute_copy(&uconnectionparam)).into()
        }
        unsafe extern "system" fn Configure<Identity: IAuthenticationProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAuthenticationProviderConfig_Impl::Configure(this, core::mem::transmute_copy(&uconnectionparam), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ureserved1), core::mem::transmute_copy(&ureserved2)).into()
        }
        unsafe extern "system" fn Activate<Identity: IAuthenticationProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAuthenticationProviderConfig_Impl::Activate(this, core::mem::transmute_copy(&uconnectionparam), core::mem::transmute_copy(&ureserved1), core::mem::transmute_copy(&ureserved2)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: IAuthenticationProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAuthenticationProviderConfig_Impl::Deactivate(this, core::mem::transmute_copy(&uconnectionparam), core::mem::transmute_copy(&ureserved1), core::mem::transmute_copy(&ureserved2)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Uninitialize: Uninitialize::<Identity, OFFSET>,
            Configure: Configure::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAuthenticationProviderConfig as windows_core::Interface>::IID
    }
}
pub trait IEAPProviderConfig_Impl: Sized + windows_core::IUnknownImpl {
    fn Initialize(&self, pszmachinename: &windows_core::PCWSTR, dweaptypeid: u32) -> windows_core::Result<usize>;
    fn Uninitialize(&self, dweaptypeid: u32, uconnectionparam: usize) -> windows_core::Result<()>;
    fn ServerInvokeConfigUI(&self, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, ureserved1: usize, ureserved2: usize) -> windows_core::Result<()>;
    fn RouterInvokeConfigUI(&self, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, ppconnectiondataout: *mut *mut u8, pdwsizeofconnectiondataout: *mut u32) -> windows_core::Result<()>;
    fn RouterInvokeCredentialsUI(&self, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, puserdatain: *const u8, dwsizeofuserdatain: u32, ppuserdataout: *mut *mut u8, pdwsizeofuserdataout: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEAPProviderConfig {}
impl IEAPProviderConfig_Vtbl {
    pub const fn new<Identity: IEAPProviderConfig_Impl, const OFFSET: isize>() -> IEAPProviderConfig_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IEAPProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmachinename: windows_core::PCWSTR, dweaptypeid: u32, puconnectionparam: *mut usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEAPProviderConfig_Impl::Initialize(this, core::mem::transmute(&pszmachinename), core::mem::transmute_copy(&dweaptypeid)) {
                Ok(ok__) => {
                    puconnectionparam.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninitialize<Identity: IEAPProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEAPProviderConfig_Impl::Uninitialize(this, core::mem::transmute_copy(&dweaptypeid), core::mem::transmute_copy(&uconnectionparam)).into()
        }
        unsafe extern "system" fn ServerInvokeConfigUI<Identity: IEAPProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, ureserved1: usize, ureserved2: usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEAPProviderConfig_Impl::ServerInvokeConfigUI(this, core::mem::transmute_copy(&dweaptypeid), core::mem::transmute_copy(&uconnectionparam), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&ureserved1), core::mem::transmute_copy(&ureserved2)).into()
        }
        unsafe extern "system" fn RouterInvokeConfigUI<Identity: IEAPProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, ppconnectiondataout: *mut *mut u8, pdwsizeofconnectiondataout: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEAPProviderConfig_Impl::RouterInvokeConfigUI(this, core::mem::transmute_copy(&dweaptypeid), core::mem::transmute_copy(&uconnectionparam), core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pconnectiondatain), core::mem::transmute_copy(&dwsizeofconnectiondatain), core::mem::transmute_copy(&ppconnectiondataout), core::mem::transmute_copy(&pdwsizeofconnectiondataout)).into()
        }
        unsafe extern "system" fn RouterInvokeCredentialsUI<Identity: IEAPProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, puserdatain: *const u8, dwsizeofuserdatain: u32, ppuserdataout: *mut *mut u8, pdwsizeofuserdataout: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEAPProviderConfig_Impl::RouterInvokeCredentialsUI(this, core::mem::transmute_copy(&dweaptypeid), core::mem::transmute_copy(&uconnectionparam), core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pconnectiondatain), core::mem::transmute_copy(&dwsizeofconnectiondatain), core::mem::transmute_copy(&puserdatain), core::mem::transmute_copy(&dwsizeofuserdatain), core::mem::transmute_copy(&ppuserdataout), core::mem::transmute_copy(&pdwsizeofuserdataout)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Uninitialize: Uninitialize::<Identity, OFFSET>,
            ServerInvokeConfigUI: ServerInvokeConfigUI::<Identity, OFFSET>,
            RouterInvokeConfigUI: RouterInvokeConfigUI::<Identity, OFFSET>,
            RouterInvokeCredentialsUI: RouterInvokeCredentialsUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEAPProviderConfig as windows_core::Interface>::IID
    }
}
pub trait IEAPProviderConfig2_Impl: Sized + IEAPProviderConfig_Impl {
    fn ServerInvokeConfigUI2(&self, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> windows_core::Result<()>;
    fn GetGlobalConfig(&self, dweaptypeid: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEAPProviderConfig2 {}
impl IEAPProviderConfig2_Vtbl {
    pub const fn new<Identity: IEAPProviderConfig2_Impl, const OFFSET: isize>() -> IEAPProviderConfig2_Vtbl {
        unsafe extern "system" fn ServerInvokeConfigUI2<Identity: IEAPProviderConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEAPProviderConfig2_Impl::ServerInvokeConfigUI2(this, core::mem::transmute_copy(&dweaptypeid), core::mem::transmute_copy(&uconnectionparam), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&pconfigdatain), core::mem::transmute_copy(&dwsizeofconfigdatain), core::mem::transmute_copy(&ppconfigdataout), core::mem::transmute_copy(&pdwsizeofconfigdataout)).into()
        }
        unsafe extern "system" fn GetGlobalConfig<Identity: IEAPProviderConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweaptypeid: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEAPProviderConfig2_Impl::GetGlobalConfig(this, core::mem::transmute_copy(&dweaptypeid), core::mem::transmute_copy(&ppconfigdataout), core::mem::transmute_copy(&pdwsizeofconfigdataout)).into()
        }
        Self {
            base__: IEAPProviderConfig_Vtbl::new::<Identity, OFFSET>(),
            ServerInvokeConfigUI2: ServerInvokeConfigUI2::<Identity, OFFSET>,
            GetGlobalConfig: GetGlobalConfig::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEAPProviderConfig2 as windows_core::Interface>::IID || iid == &<IEAPProviderConfig as windows_core::Interface>::IID
    }
}
pub trait IEAPProviderConfig3_Impl: Sized + IEAPProviderConfig2_Impl {
    fn ServerInvokeCertificateConfigUI(&self, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32, ureserved: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEAPProviderConfig3 {}
impl IEAPProviderConfig3_Vtbl {
    pub const fn new<Identity: IEAPProviderConfig3_Impl, const OFFSET: isize>() -> IEAPProviderConfig3_Vtbl {
        unsafe extern "system" fn ServerInvokeCertificateConfigUI<Identity: IEAPProviderConfig3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32, ureserved: usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEAPProviderConfig3_Impl::ServerInvokeCertificateConfigUI(this, core::mem::transmute_copy(&dweaptypeid), core::mem::transmute_copy(&uconnectionparam), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&pconfigdatain), core::mem::transmute_copy(&dwsizeofconfigdatain), core::mem::transmute_copy(&ppconfigdataout), core::mem::transmute_copy(&pdwsizeofconfigdataout), core::mem::transmute_copy(&ureserved)).into()
        }
        Self {
            base__: IEAPProviderConfig2_Vtbl::new::<Identity, OFFSET>(),
            ServerInvokeCertificateConfigUI: ServerInvokeCertificateConfigUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEAPProviderConfig3 as windows_core::Interface>::IID || iid == &<IEAPProviderConfig as windows_core::Interface>::IID || iid == &<IEAPProviderConfig2 as windows_core::Interface>::IID
    }
}
pub trait IRouterProtocolConfig_Impl: Sized + windows_core::IUnknownImpl {
    fn AddProtocol(&self, pszmachinename: &windows_core::PCWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: Option<&windows_core::IUnknown>, ureserved1: usize) -> windows_core::Result<()>;
    fn RemoveProtocol(&self, pszmachinename: &windows_core::PCWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: Option<&windows_core::IUnknown>, ureserved1: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRouterProtocolConfig {}
impl IRouterProtocolConfig_Vtbl {
    pub const fn new<Identity: IRouterProtocolConfig_Impl, const OFFSET: isize>() -> IRouterProtocolConfig_Vtbl {
        unsafe extern "system" fn AddProtocol<Identity: IRouterProtocolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmachinename: windows_core::PCWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: *mut core::ffi::c_void, ureserved1: usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRouterProtocolConfig_Impl::AddProtocol(this, core::mem::transmute(&pszmachinename), core::mem::transmute_copy(&dwtransportid), core::mem::transmute_copy(&dwprotocolid), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&prouter), core::mem::transmute_copy(&ureserved1)).into()
        }
        unsafe extern "system" fn RemoveProtocol<Identity: IRouterProtocolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmachinename: windows_core::PCWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: *mut core::ffi::c_void, ureserved1: usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRouterProtocolConfig_Impl::RemoveProtocol(this, core::mem::transmute(&pszmachinename), core::mem::transmute_copy(&dwtransportid), core::mem::transmute_copy(&dwprotocolid), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&prouter), core::mem::transmute_copy(&ureserved1)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddProtocol: AddProtocol::<Identity, OFFSET>,
            RemoveProtocol: RemoveProtocol::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRouterProtocolConfig as windows_core::Interface>::IID
    }
}
