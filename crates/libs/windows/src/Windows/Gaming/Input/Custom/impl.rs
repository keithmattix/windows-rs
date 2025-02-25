pub trait ICustomGameControllerFactory_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateGameController(&self, provider: Option<&IGameControllerProvider>) -> windows_core::Result<windows_core::IInspectable>;
    fn OnGameControllerAdded(&self, value: Option<&super::IGameController>) -> windows_core::Result<()>;
    fn OnGameControllerRemoved(&self, value: Option<&super::IGameController>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICustomGameControllerFactory {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.ICustomGameControllerFactory";
}
impl ICustomGameControllerFactory_Vtbl {
    pub const fn new<Identity: ICustomGameControllerFactory_Impl, const OFFSET: isize>() -> ICustomGameControllerFactory_Vtbl {
        unsafe extern "system" fn CreateGameController<Identity: ICustomGameControllerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICustomGameControllerFactory_Impl::CreateGameController(this, windows_core::from_raw_borrowed(&provider)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnGameControllerAdded<Identity: ICustomGameControllerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICustomGameControllerFactory_Impl::OnGameControllerAdded(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn OnGameControllerRemoved<Identity: ICustomGameControllerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICustomGameControllerFactory_Impl::OnGameControllerRemoved(this, windows_core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICustomGameControllerFactory, OFFSET>(),
            CreateGameController: CreateGameController::<Identity, OFFSET>,
            OnGameControllerAdded: OnGameControllerAdded::<Identity, OFFSET>,
            OnGameControllerRemoved: OnGameControllerRemoved::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICustomGameControllerFactory as windows_core::Interface>::IID
    }
}
pub trait IGameControllerInputSink_Impl: Sized + windows_core::IUnknownImpl {
    fn OnInputResumed(&self, timestamp: u64) -> windows_core::Result<()>;
    fn OnInputSuspended(&self, timestamp: u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGameControllerInputSink";
}
impl IGameControllerInputSink_Vtbl {
    pub const fn new<Identity: IGameControllerInputSink_Impl, const OFFSET: isize>() -> IGameControllerInputSink_Vtbl {
        unsafe extern "system" fn OnInputResumed<Identity: IGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGameControllerInputSink_Impl::OnInputResumed(this, timestamp).into()
        }
        unsafe extern "system" fn OnInputSuspended<Identity: IGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGameControllerInputSink_Impl::OnInputSuspended(this, timestamp).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGameControllerInputSink, OFFSET>(),
            OnInputResumed: OnInputResumed::<Identity, OFFSET>,
            OnInputSuspended: OnInputSuspended::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameControllerInputSink as windows_core::Interface>::IID
    }
}
pub trait IGameControllerProvider_Impl: Sized + windows_core::IUnknownImpl {
    fn FirmwareVersionInfo(&self) -> windows_core::Result<GameControllerVersionInfo>;
    fn HardwareProductId(&self) -> windows_core::Result<u16>;
    fn HardwareVendorId(&self) -> windows_core::Result<u16>;
    fn HardwareVersionInfo(&self) -> windows_core::Result<GameControllerVersionInfo>;
    fn IsConnected(&self) -> windows_core::Result<bool>;
}
impl windows_core::RuntimeName for IGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGameControllerProvider";
}
impl IGameControllerProvider_Vtbl {
    pub const fn new<Identity: IGameControllerProvider_Impl, const OFFSET: isize>() -> IGameControllerProvider_Vtbl {
        unsafe extern "system" fn FirmwareVersionInfo<Identity: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameControllerProvider_Impl::FirmwareVersionInfo(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareProductId<Identity: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameControllerProvider_Impl::HardwareProductId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareVendorId<Identity: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameControllerProvider_Impl::HardwareVendorId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareVersionInfo<Identity: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameControllerProvider_Impl::HardwareVersionInfo(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Identity: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameControllerProvider_Impl::IsConnected(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGameControllerProvider, OFFSET>(),
            FirmwareVersionInfo: FirmwareVersionInfo::<Identity, OFFSET>,
            HardwareProductId: HardwareProductId::<Identity, OFFSET>,
            HardwareVendorId: HardwareVendorId::<Identity, OFFSET>,
            HardwareVersionInfo: HardwareVersionInfo::<Identity, OFFSET>,
            IsConnected: IsConnected::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameControllerProvider as windows_core::Interface>::IID
    }
}
pub trait IGipGameControllerInputSink_Impl: Sized + windows_core::IUnknownImpl + IGameControllerInputSink_Impl {
    fn OnKeyReceived(&self, timestamp: u64, keycode: u8, ispressed: bool) -> windows_core::Result<()>;
    fn OnMessageReceived(&self, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messagebuffer: &[u8]) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IGipGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGipGameControllerInputSink";
}
impl IGipGameControllerInputSink_Vtbl {
    pub const fn new<Identity: IGipGameControllerInputSink_Impl, const OFFSET: isize>() -> IGipGameControllerInputSink_Vtbl {
        unsafe extern "system" fn OnKeyReceived<Identity: IGipGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u64, keycode: u8, ispressed: bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGipGameControllerInputSink_Impl::OnKeyReceived(this, timestamp, keycode, ispressed).into()
        }
        unsafe extern "system" fn OnMessageReceived<Identity: IGipGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGipGameControllerInputSink_Impl::OnMessageReceived(this, timestamp, messageclass, messageid, sequenceid, core::slice::from_raw_parts(core::mem::transmute_copy(&messagebuffer), messageBuffer_array_size as usize)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGipGameControllerInputSink, OFFSET>(),
            OnKeyReceived: OnKeyReceived::<Identity, OFFSET>,
            OnMessageReceived: OnMessageReceived::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGipGameControllerInputSink as windows_core::Interface>::IID
    }
}
pub trait IHidGameControllerInputSink_Impl: Sized + windows_core::IUnknownImpl + IGameControllerInputSink_Impl {
    fn OnInputReportReceived(&self, timestamp: u64, reportid: u8, reportbuffer: &[u8]) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHidGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IHidGameControllerInputSink";
}
impl IHidGameControllerInputSink_Vtbl {
    pub const fn new<Identity: IHidGameControllerInputSink_Impl, const OFFSET: isize>() -> IHidGameControllerInputSink_Vtbl {
        unsafe extern "system" fn OnInputReportReceived<Identity: IHidGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u64, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHidGameControllerInputSink_Impl::OnInputReportReceived(this, timestamp, reportid, core::slice::from_raw_parts(core::mem::transmute_copy(&reportbuffer), reportBuffer_array_size as usize)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IHidGameControllerInputSink, OFFSET>(),
            OnInputReportReceived: OnInputReportReceived::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHidGameControllerInputSink as windows_core::Interface>::IID
    }
}
pub trait IXusbGameControllerInputSink_Impl: Sized + windows_core::IUnknownImpl + IGameControllerInputSink_Impl {
    fn OnInputReceived(&self, timestamp: u64, reportid: u8, inputbuffer: &[u8]) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXusbGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IXusbGameControllerInputSink";
}
impl IXusbGameControllerInputSink_Vtbl {
    pub const fn new<Identity: IXusbGameControllerInputSink_Impl, const OFFSET: isize>() -> IXusbGameControllerInputSink_Vtbl {
        unsafe extern "system" fn OnInputReceived<Identity: IXusbGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u64, reportid: u8, inputBuffer_array_size: u32, inputbuffer: *const u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXusbGameControllerInputSink_Impl::OnInputReceived(this, timestamp, reportid, core::slice::from_raw_parts(core::mem::transmute_copy(&inputbuffer), inputBuffer_array_size as usize)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IXusbGameControllerInputSink, OFFSET>(),
            OnInputReceived: OnInputReceived::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXusbGameControllerInputSink as windows_core::Interface>::IID
    }
}
