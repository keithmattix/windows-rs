pub trait IUIApplication_Impl: Sized + windows_core::IUnknownImpl {
    fn OnViewChanged(&self, viewid: u32, typeid: UI_VIEWTYPE, view: Option<&windows_core::IUnknown>, verb: UI_VIEWVERB, ureasoncode: i32) -> windows_core::Result<()>;
    fn OnCreateUICommand(&self, commandid: u32, typeid: UI_COMMANDTYPE) -> windows_core::Result<IUICommandHandler>;
    fn OnDestroyUICommand(&self, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: Option<&IUICommandHandler>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIApplication {}
impl IUIApplication_Vtbl {
    pub const fn new<Identity: IUIApplication_Impl, const OFFSET: isize>() -> IUIApplication_Vtbl {
        unsafe extern "system" fn OnViewChanged<Identity: IUIApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewid: u32, typeid: UI_VIEWTYPE, view: *mut core::ffi::c_void, verb: UI_VIEWVERB, ureasoncode: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIApplication_Impl::OnViewChanged(this, core::mem::transmute_copy(&viewid), core::mem::transmute_copy(&typeid), windows_core::from_raw_borrowed(&view), core::mem::transmute_copy(&verb), core::mem::transmute_copy(&ureasoncode)).into()
        }
        unsafe extern "system" fn OnCreateUICommand<Identity: IUIApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIApplication_Impl::OnCreateUICommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&typeid)) {
                Ok(ok__) => {
                    commandhandler.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDestroyUICommand<Identity: IUIApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIApplication_Impl::OnDestroyUICommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&typeid), windows_core::from_raw_borrowed(&commandhandler)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnViewChanged: OnViewChanged::<Identity, OFFSET>,
            OnCreateUICommand: OnCreateUICommand::<Identity, OFFSET>,
            OnDestroyUICommand: OnDestroyUICommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIApplication as windows_core::Interface>::IID
    }
}
pub trait IUICollection_Impl: Sized + windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetItem(&self, index: u32) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, item: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Insert(&self, index: u32, item: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn Replace(&self, indexreplaced: u32, itemreplacewith: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUICollection {}
impl IUICollection_Vtbl {
    pub const fn new<Identity: IUICollection_Impl, const OFFSET: isize>() -> IUICollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUICollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Identity: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUICollection_Impl::GetItem(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    item.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUICollection_Impl::Add(this, windows_core::from_raw_borrowed(&item)).into()
        }
        unsafe extern "system" fn Insert<Identity: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, item: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUICollection_Impl::Insert(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&item)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUICollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Replace<Identity: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexreplaced: u32, itemreplacewith: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUICollection_Impl::Replace(this, core::mem::transmute_copy(&indexreplaced), windows_core::from_raw_borrowed(&itemreplacewith)).into()
        }
        unsafe extern "system" fn Clear<Identity: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUICollection_Impl::Clear(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetItem: GetItem::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Insert: Insert::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            Replace: Replace::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUICollection as windows_core::Interface>::IID
    }
}
pub trait IUICollectionChangedEvent_Impl: Sized + windows_core::IUnknownImpl {
    fn OnChanged(&self, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: Option<&windows_core::IUnknown>, newindex: u32, newitem: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUICollectionChangedEvent {}
impl IUICollectionChangedEvent_Vtbl {
    pub const fn new<Identity: IUICollectionChangedEvent_Impl, const OFFSET: isize>() -> IUICollectionChangedEvent_Vtbl {
        unsafe extern "system" fn OnChanged<Identity: IUICollectionChangedEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: *mut core::ffi::c_void, newindex: u32, newitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUICollectionChangedEvent_Impl::OnChanged(this, core::mem::transmute_copy(&action), core::mem::transmute_copy(&oldindex), windows_core::from_raw_borrowed(&olditem), core::mem::transmute_copy(&newindex), windows_core::from_raw_borrowed(&newitem)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnChanged: OnChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUICollectionChangedEvent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IUICommandHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn Execute(&self, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const windows_core::PROPVARIANT, commandexecutionproperties: Option<&IUISimplePropertySet>) -> windows_core::Result<()>;
    fn UpdateProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<windows_core::PROPVARIANT>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IUICommandHandler {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IUICommandHandler_Vtbl {
    pub const fn new<Identity: IUICommandHandler_Impl, const OFFSET: isize>() -> IUICommandHandler_Vtbl {
        unsafe extern "system" fn Execute<Identity: IUICommandHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, commandexecutionproperties: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUICommandHandler_Impl::Execute(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&verb), core::mem::transmute_copy(&key), core::mem::transmute_copy(&currentvalue), windows_core::from_raw_borrowed(&commandexecutionproperties)).into()
        }
        unsafe extern "system" fn UpdateProperty<Identity: IUICommandHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, newvalue: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUICommandHandler_Impl::UpdateProperty(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&key), core::mem::transmute_copy(&currentvalue)) {
                Ok(ok__) => {
                    newvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Execute: Execute::<Identity, OFFSET>,
            UpdateProperty: UpdateProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUICommandHandler as windows_core::Interface>::IID
    }
}
pub trait IUIContextualUI_Impl: Sized + windows_core::IUnknownImpl {
    fn ShowAtLocation(&self, x: i32, y: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIContextualUI {}
impl IUIContextualUI_Vtbl {
    pub const fn new<Identity: IUIContextualUI_Impl, const OFFSET: isize>() -> IUIContextualUI_Vtbl {
        unsafe extern "system" fn ShowAtLocation<Identity: IUIContextualUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIContextualUI_Impl::ShowAtLocation(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ShowAtLocation: ShowAtLocation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIContextualUI as windows_core::Interface>::IID
    }
}
pub trait IUIEventLogger_Impl: Sized + windows_core::IUnknownImpl {
    fn OnUIEvent(&self, peventparams: *const UI_EVENTPARAMS);
}
impl windows_core::RuntimeName for IUIEventLogger {}
impl IUIEventLogger_Vtbl {
    pub const fn new<Identity: IUIEventLogger_Impl, const OFFSET: isize>() -> IUIEventLogger_Vtbl {
        unsafe extern "system" fn OnUIEvent<Identity: IUIEventLogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventparams: *const UI_EVENTPARAMS) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIEventLogger_Impl::OnUIEvent(this, core::mem::transmute_copy(&peventparams))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUIEvent: OnUIEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIEventLogger as windows_core::Interface>::IID
    }
}
pub trait IUIEventingManager_Impl: Sized + windows_core::IUnknownImpl {
    fn SetEventLogger(&self, eventlogger: Option<&IUIEventLogger>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIEventingManager {}
impl IUIEventingManager_Vtbl {
    pub const fn new<Identity: IUIEventingManager_Impl, const OFFSET: isize>() -> IUIEventingManager_Vtbl {
        unsafe extern "system" fn SetEventLogger<Identity: IUIEventingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventlogger: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIEventingManager_Impl::SetEventLogger(this, windows_core::from_raw_borrowed(&eventlogger)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetEventLogger: SetEventLogger::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIEventingManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IUIFramework_Impl: Sized + windows_core::IUnknownImpl {
    fn Initialize(&self, framewnd: super::super::Foundation::HWND, application: Option<&IUIApplication>) -> windows_core::Result<()>;
    fn Destroy(&self) -> windows_core::Result<()>;
    fn LoadUI(&self, instance: super::super::Foundation::HINSTANCE, resourcename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetView(&self, viewid: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetUICommandProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<windows_core::PROPVARIANT>;
    fn SetUICommandProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn InvalidateUICommand(&self, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()>;
    fn FlushPendingInvalidations(&self) -> windows_core::Result<()>;
    fn SetModes(&self, imodes: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IUIFramework {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IUIFramework_Vtbl {
    pub const fn new<Identity: IUIFramework_Impl, const OFFSET: isize>() -> IUIFramework_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, framewnd: super::super::Foundation::HWND, application: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIFramework_Impl::Initialize(this, core::mem::transmute_copy(&framewnd), windows_core::from_raw_borrowed(&application)).into()
        }
        unsafe extern "system" fn Destroy<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIFramework_Impl::Destroy(this).into()
        }
        unsafe extern "system" fn LoadUI<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instance: super::super::Foundation::HINSTANCE, resourcename: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIFramework_Impl::LoadUI(this, core::mem::transmute_copy(&instance), core::mem::transmute(&resourcename)).into()
        }
        unsafe extern "system" fn GetView<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewid: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIFramework_Impl::GetView(this, core::mem::transmute_copy(&viewid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetUICommandProperty<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIFramework_Impl::GetUICommandProperty(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUICommandProperty<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIFramework_Impl::SetUICommandProperty(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn InvalidateUICommand<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIFramework_Impl::InvalidateUICommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn FlushPendingInvalidations<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIFramework_Impl::FlushPendingInvalidations(this).into()
        }
        unsafe extern "system" fn SetModes<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imodes: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIFramework_Impl::SetModes(this, core::mem::transmute_copy(&imodes)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Destroy: Destroy::<Identity, OFFSET>,
            LoadUI: LoadUI::<Identity, OFFSET>,
            GetView: GetView::<Identity, OFFSET>,
            GetUICommandProperty: GetUICommandProperty::<Identity, OFFSET>,
            SetUICommandProperty: SetUICommandProperty::<Identity, OFFSET>,
            InvalidateUICommand: InvalidateUICommand::<Identity, OFFSET>,
            FlushPendingInvalidations: FlushPendingInvalidations::<Identity, OFFSET>,
            SetModes: SetModes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIFramework as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IUIImage_Impl: Sized + windows_core::IUnknownImpl {
    fn GetBitmap(&self) -> windows_core::Result<super::super::Graphics::Gdi::HBITMAP>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IUIImage {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IUIImage_Vtbl {
    pub const fn new<Identity: IUIImage_Impl, const OFFSET: isize>() -> IUIImage_Vtbl {
        unsafe extern "system" fn GetBitmap<Identity: IUIImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIImage_Impl::GetBitmap(this) {
                Ok(ok__) => {
                    bitmap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetBitmap: GetBitmap::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIImage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IUIImageFromBitmap_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateImage(&self, bitmap: super::super::Graphics::Gdi::HBITMAP, options: UI_OWNERSHIP) -> windows_core::Result<IUIImage>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IUIImageFromBitmap {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IUIImageFromBitmap_Vtbl {
    pub const fn new<Identity: IUIImageFromBitmap_Impl, const OFFSET: isize>() -> IUIImageFromBitmap_Vtbl {
        unsafe extern "system" fn CreateImage<Identity: IUIImageFromBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: super::super::Graphics::Gdi::HBITMAP, options: UI_OWNERSHIP, image: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIImageFromBitmap_Impl::CreateImage(this, core::mem::transmute_copy(&bitmap), core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    image.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateImage: CreateImage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIImageFromBitmap as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUIRibbon_Impl: Sized + windows_core::IUnknownImpl {
    fn GetHeight(&self) -> windows_core::Result<u32>;
    fn LoadSettingsFromStream(&self, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn SaveSettingsToStream(&self, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUIRibbon {}
#[cfg(feature = "Win32_System_Com")]
impl IUIRibbon_Vtbl {
    pub const fn new<Identity: IUIRibbon_Impl, const OFFSET: isize>() -> IUIRibbon_Vtbl {
        unsafe extern "system" fn GetHeight<Identity: IUIRibbon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cy: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIRibbon_Impl::GetHeight(this) {
                Ok(ok__) => {
                    cy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadSettingsFromStream<Identity: IUIRibbon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIRibbon_Impl::LoadSettingsFromStream(this, windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn SaveSettingsToStream<Identity: IUIRibbon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIRibbon_Impl::SaveSettingsToStream(this, windows_core::from_raw_borrowed(&pstream)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHeight: GetHeight::<Identity, OFFSET>,
            LoadSettingsFromStream: LoadSettingsFromStream::<Identity, OFFSET>,
            SaveSettingsToStream: SaveSettingsToStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIRibbon as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IUISimplePropertySet_Impl: Sized + windows_core::IUnknownImpl {
    fn GetValue(&self, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<windows_core::PROPVARIANT>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IUISimplePropertySet {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IUISimplePropertySet_Vtbl {
    pub const fn new<Identity: IUISimplePropertySet_Impl, const OFFSET: isize>() -> IUISimplePropertySet_Vtbl {
        unsafe extern "system" fn GetValue<Identity: IUISimplePropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUISimplePropertySet_Impl::GetValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetValue: GetValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUISimplePropertySet as windows_core::Interface>::IID
    }
}
