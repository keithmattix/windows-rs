pub trait ICeeGen_Impl: Sized + windows_core::IUnknownImpl {
    fn EmitString(&self, lpstring: &windows_core::PCWSTR, rva: *mut u32) -> windows_core::Result<()>;
    fn GetString(&self, rva: u32, lpstring: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn AllocateMethodBuffer(&self, cchbuffer: u32, lpbuffer: *mut *mut u8, rva: *mut u32) -> windows_core::Result<()>;
    fn GetMethodBuffer(&self, rva: u32, lpbuffer: *mut *mut u8) -> windows_core::Result<()>;
    fn GetIMapTokenIface(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GenerateCeeFile(&self) -> windows_core::Result<()>;
    fn GetIlSection(&self, section: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetStringSection(&self, section: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddSectionReloc(&self, section: *mut core::ffi::c_void, offset: u32, relativeto: *mut core::ffi::c_void, reloctype: CeeSectionRelocType) -> windows_core::Result<()>;
    fn GetSectionCreate(&self, name: &windows_core::PCSTR, flags: u32, section: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetSectionDataLen(&self, section: *mut core::ffi::c_void, datalen: *mut u32) -> windows_core::Result<()>;
    fn GetSectionBlock(&self, section: *mut core::ffi::c_void, len: u32, align: u32, ppbytes: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn TruncateSection(&self, section: *mut core::ffi::c_void, len: u32) -> windows_core::Result<()>;
    fn GenerateCeeMemoryImage(&self, ppimage: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ComputePointer(&self, section: *mut core::ffi::c_void, rva: u32, lpbuffer: *mut *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICeeGen {}
impl ICeeGen_Vtbl {
    pub const fn new<Identity: ICeeGen_Impl, const OFFSET: isize>() -> ICeeGen_Vtbl {
        unsafe extern "system" fn EmitString<Identity: ICeeGen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpstring: windows_core::PCWSTR, rva: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICeeGen_Impl::EmitString(this, core::mem::transmute(&lpstring), core::mem::transmute_copy(&rva)).into()
        }
        unsafe extern "system" fn GetString<Identity: ICeeGen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rva: u32, lpstring: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICeeGen_Impl::GetString(this, core::mem::transmute_copy(&rva), core::mem::transmute_copy(&lpstring)).into()
        }
        unsafe extern "system" fn AllocateMethodBuffer<Identity: ICeeGen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchbuffer: u32, lpbuffer: *mut *mut u8, rva: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICeeGen_Impl::AllocateMethodBuffer(this, core::mem::transmute_copy(&cchbuffer), core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&rva)).into()
        }
        unsafe extern "system" fn GetMethodBuffer<Identity: ICeeGen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rva: u32, lpbuffer: *mut *mut u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICeeGen_Impl::GetMethodBuffer(this, core::mem::transmute_copy(&rva), core::mem::transmute_copy(&lpbuffer)).into()
        }
        unsafe extern "system" fn GetIMapTokenIface<Identity: ICeeGen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimaptoken: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICeeGen_Impl::GetIMapTokenIface(this) {
                Ok(ok__) => {
                    pimaptoken.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateCeeFile<Identity: ICeeGen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICeeGen_Impl::GenerateCeeFile(this).into()
        }
        unsafe extern "system" fn GetIlSection<Identity: ICeeGen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, section: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICeeGen_Impl::GetIlSection(this, core::mem::transmute_copy(&section)).into()
        }
        unsafe extern "system" fn GetStringSection<Identity: ICeeGen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, section: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICeeGen_Impl::GetStringSection(this, core::mem::transmute_copy(&section)).into()
        }
        unsafe extern "system" fn AddSectionReloc<Identity: ICeeGen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, section: *mut core::ffi::c_void, offset: u32, relativeto: *mut core::ffi::c_void, reloctype: CeeSectionRelocType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICeeGen_Impl::AddSectionReloc(this, core::mem::transmute_copy(&section), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&relativeto), core::mem::transmute_copy(&reloctype)).into()
        }
        unsafe extern "system" fn GetSectionCreate<Identity: ICeeGen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, flags: u32, section: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICeeGen_Impl::GetSectionCreate(this, core::mem::transmute(&name), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&section)).into()
        }
        unsafe extern "system" fn GetSectionDataLen<Identity: ICeeGen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, section: *mut core::ffi::c_void, datalen: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICeeGen_Impl::GetSectionDataLen(this, core::mem::transmute_copy(&section), core::mem::transmute_copy(&datalen)).into()
        }
        unsafe extern "system" fn GetSectionBlock<Identity: ICeeGen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, section: *mut core::ffi::c_void, len: u32, align: u32, ppbytes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICeeGen_Impl::GetSectionBlock(this, core::mem::transmute_copy(&section), core::mem::transmute_copy(&len), core::mem::transmute_copy(&align), core::mem::transmute_copy(&ppbytes)).into()
        }
        unsafe extern "system" fn TruncateSection<Identity: ICeeGen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, section: *mut core::ffi::c_void, len: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICeeGen_Impl::TruncateSection(this, core::mem::transmute_copy(&section), core::mem::transmute_copy(&len)).into()
        }
        unsafe extern "system" fn GenerateCeeMemoryImage<Identity: ICeeGen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICeeGen_Impl::GenerateCeeMemoryImage(this, core::mem::transmute_copy(&ppimage)).into()
        }
        unsafe extern "system" fn ComputePointer<Identity: ICeeGen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, section: *mut core::ffi::c_void, rva: u32, lpbuffer: *mut *mut u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICeeGen_Impl::ComputePointer(this, core::mem::transmute_copy(&section), core::mem::transmute_copy(&rva), core::mem::transmute_copy(&lpbuffer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EmitString: EmitString::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            AllocateMethodBuffer: AllocateMethodBuffer::<Identity, OFFSET>,
            GetMethodBuffer: GetMethodBuffer::<Identity, OFFSET>,
            GetIMapTokenIface: GetIMapTokenIface::<Identity, OFFSET>,
            GenerateCeeFile: GenerateCeeFile::<Identity, OFFSET>,
            GetIlSection: GetIlSection::<Identity, OFFSET>,
            GetStringSection: GetStringSection::<Identity, OFFSET>,
            AddSectionReloc: AddSectionReloc::<Identity, OFFSET>,
            GetSectionCreate: GetSectionCreate::<Identity, OFFSET>,
            GetSectionDataLen: GetSectionDataLen::<Identity, OFFSET>,
            GetSectionBlock: GetSectionBlock::<Identity, OFFSET>,
            TruncateSection: TruncateSection::<Identity, OFFSET>,
            GenerateCeeMemoryImage: GenerateCeeMemoryImage::<Identity, OFFSET>,
            ComputePointer: ComputePointer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICeeGen as windows_core::Interface>::IID
    }
}
pub trait IHostFilter_Impl: Sized + windows_core::IUnknownImpl {
    fn MarkToken(&self, tk: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHostFilter {}
impl IHostFilter_Vtbl {
    pub const fn new<Identity: IHostFilter_Impl, const OFFSET: isize>() -> IHostFilter_Vtbl {
        unsafe extern "system" fn MarkToken<Identity: IHostFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tk: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostFilter_Impl::MarkToken(this, core::mem::transmute_copy(&tk)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), MarkToken: MarkToken::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostFilter as windows_core::Interface>::IID
    }
}
pub trait IMapToken_Impl: Sized + windows_core::IUnknownImpl {
    fn Map(&self, tkimp: u32, tkemit: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMapToken {}
impl IMapToken_Vtbl {
    pub const fn new<Identity: IMapToken_Impl, const OFFSET: isize>() -> IMapToken_Vtbl {
        unsafe extern "system" fn Map<Identity: IMapToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tkimp: u32, tkemit: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMapToken_Impl::Map(this, core::mem::transmute_copy(&tkimp), core::mem::transmute_copy(&tkemit)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Map: Map::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMapToken as windows_core::Interface>::IID
    }
}
pub trait IMetaDataAssemblyEmit_Impl: Sized + windows_core::IUnknownImpl {
    fn DefineAssembly(&self, pbpublickey: *const core::ffi::c_void, cbpublickey: u32, ulhashalgid: u32, szname: &windows_core::PCWSTR, pmetadata: *const ASSEMBLYMETADATA, dwassemblyflags: u32, pma: *mut u32) -> windows_core::Result<()>;
    fn DefineAssemblyRef(&self, pbpublickeyortoken: *const core::ffi::c_void, cbpublickeyortoken: u32, szname: &windows_core::PCWSTR, pmetadata: *const ASSEMBLYMETADATA, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, dwassemblyrefflags: u32, pmdar: *mut u32) -> windows_core::Result<()>;
    fn DefineFile(&self, szname: &windows_core::PCWSTR, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, dwfileflags: u32, pmdf: *mut u32) -> windows_core::Result<()>;
    fn DefineExportedType(&self, szname: &windows_core::PCWSTR, tkimplementation: u32, tktypedef: u32, dwexportedtypeflags: u32, pmdct: *mut u32) -> windows_core::Result<()>;
    fn DefineManifestResource(&self, szname: &windows_core::PCWSTR, tkimplementation: u32, dwoffset: u32, dwresourceflags: u32, pmdmr: *mut u32) -> windows_core::Result<()>;
    fn SetAssemblyProps(&self, pma: u32, pbpublickey: *const core::ffi::c_void, cbpublickey: u32, ulhashalgid: u32, szname: &windows_core::PCWSTR, pmetadata: *const ASSEMBLYMETADATA, dwassemblyflags: u32) -> windows_core::Result<()>;
    fn SetAssemblyRefProps(&self, ar: u32, pbpublickeyortoken: *const core::ffi::c_void, cbpublickeyortoken: u32, szname: &windows_core::PCWSTR, pmetadata: *const ASSEMBLYMETADATA, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, dwassemblyrefflags: u32) -> windows_core::Result<()>;
    fn SetFileProps(&self, file: u32, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, dwfileflags: u32) -> windows_core::Result<()>;
    fn SetExportedTypeProps(&self, ct: u32, tkimplementation: u32, tktypedef: u32, dwexportedtypeflags: u32) -> windows_core::Result<()>;
    fn SetManifestResourceProps(&self, mr: u32, tkimplementation: u32, dwoffset: u32, dwresourceflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMetaDataAssemblyEmit {}
impl IMetaDataAssemblyEmit_Vtbl {
    pub const fn new<Identity: IMetaDataAssemblyEmit_Impl, const OFFSET: isize>() -> IMetaDataAssemblyEmit_Vtbl {
        unsafe extern "system" fn DefineAssembly<Identity: IMetaDataAssemblyEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpublickey: *const core::ffi::c_void, cbpublickey: u32, ulhashalgid: u32, szname: windows_core::PCWSTR, pmetadata: *const ASSEMBLYMETADATA, dwassemblyflags: u32, pma: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyEmit_Impl::DefineAssembly(this, core::mem::transmute_copy(&pbpublickey), core::mem::transmute_copy(&cbpublickey), core::mem::transmute_copy(&ulhashalgid), core::mem::transmute(&szname), core::mem::transmute_copy(&pmetadata), core::mem::transmute_copy(&dwassemblyflags), core::mem::transmute_copy(&pma)).into()
        }
        unsafe extern "system" fn DefineAssemblyRef<Identity: IMetaDataAssemblyEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpublickeyortoken: *const core::ffi::c_void, cbpublickeyortoken: u32, szname: windows_core::PCWSTR, pmetadata: *const ASSEMBLYMETADATA, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, dwassemblyrefflags: u32, pmdar: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyEmit_Impl::DefineAssemblyRef(this, core::mem::transmute_copy(&pbpublickeyortoken), core::mem::transmute_copy(&cbpublickeyortoken), core::mem::transmute(&szname), core::mem::transmute_copy(&pmetadata), core::mem::transmute_copy(&pbhashvalue), core::mem::transmute_copy(&cbhashvalue), core::mem::transmute_copy(&dwassemblyrefflags), core::mem::transmute_copy(&pmdar)).into()
        }
        unsafe extern "system" fn DefineFile<Identity: IMetaDataAssemblyEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, dwfileflags: u32, pmdf: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyEmit_Impl::DefineFile(this, core::mem::transmute(&szname), core::mem::transmute_copy(&pbhashvalue), core::mem::transmute_copy(&cbhashvalue), core::mem::transmute_copy(&dwfileflags), core::mem::transmute_copy(&pmdf)).into()
        }
        unsafe extern "system" fn DefineExportedType<Identity: IMetaDataAssemblyEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, tkimplementation: u32, tktypedef: u32, dwexportedtypeflags: u32, pmdct: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyEmit_Impl::DefineExportedType(this, core::mem::transmute(&szname), core::mem::transmute_copy(&tkimplementation), core::mem::transmute_copy(&tktypedef), core::mem::transmute_copy(&dwexportedtypeflags), core::mem::transmute_copy(&pmdct)).into()
        }
        unsafe extern "system" fn DefineManifestResource<Identity: IMetaDataAssemblyEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, tkimplementation: u32, dwoffset: u32, dwresourceflags: u32, pmdmr: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyEmit_Impl::DefineManifestResource(this, core::mem::transmute(&szname), core::mem::transmute_copy(&tkimplementation), core::mem::transmute_copy(&dwoffset), core::mem::transmute_copy(&dwresourceflags), core::mem::transmute_copy(&pmdmr)).into()
        }
        unsafe extern "system" fn SetAssemblyProps<Identity: IMetaDataAssemblyEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pma: u32, pbpublickey: *const core::ffi::c_void, cbpublickey: u32, ulhashalgid: u32, szname: windows_core::PCWSTR, pmetadata: *const ASSEMBLYMETADATA, dwassemblyflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyEmit_Impl::SetAssemblyProps(this, core::mem::transmute_copy(&pma), core::mem::transmute_copy(&pbpublickey), core::mem::transmute_copy(&cbpublickey), core::mem::transmute_copy(&ulhashalgid), core::mem::transmute(&szname), core::mem::transmute_copy(&pmetadata), core::mem::transmute_copy(&dwassemblyflags)).into()
        }
        unsafe extern "system" fn SetAssemblyRefProps<Identity: IMetaDataAssemblyEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ar: u32, pbpublickeyortoken: *const core::ffi::c_void, cbpublickeyortoken: u32, szname: windows_core::PCWSTR, pmetadata: *const ASSEMBLYMETADATA, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, dwassemblyrefflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyEmit_Impl::SetAssemblyRefProps(this, core::mem::transmute_copy(&ar), core::mem::transmute_copy(&pbpublickeyortoken), core::mem::transmute_copy(&cbpublickeyortoken), core::mem::transmute(&szname), core::mem::transmute_copy(&pmetadata), core::mem::transmute_copy(&pbhashvalue), core::mem::transmute_copy(&cbhashvalue), core::mem::transmute_copy(&dwassemblyrefflags)).into()
        }
        unsafe extern "system" fn SetFileProps<Identity: IMetaDataAssemblyEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: u32, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, dwfileflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyEmit_Impl::SetFileProps(this, core::mem::transmute_copy(&file), core::mem::transmute_copy(&pbhashvalue), core::mem::transmute_copy(&cbhashvalue), core::mem::transmute_copy(&dwfileflags)).into()
        }
        unsafe extern "system" fn SetExportedTypeProps<Identity: IMetaDataAssemblyEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ct: u32, tkimplementation: u32, tktypedef: u32, dwexportedtypeflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyEmit_Impl::SetExportedTypeProps(this, core::mem::transmute_copy(&ct), core::mem::transmute_copy(&tkimplementation), core::mem::transmute_copy(&tktypedef), core::mem::transmute_copy(&dwexportedtypeflags)).into()
        }
        unsafe extern "system" fn SetManifestResourceProps<Identity: IMetaDataAssemblyEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mr: u32, tkimplementation: u32, dwoffset: u32, dwresourceflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyEmit_Impl::SetManifestResourceProps(this, core::mem::transmute_copy(&mr), core::mem::transmute_copy(&tkimplementation), core::mem::transmute_copy(&dwoffset), core::mem::transmute_copy(&dwresourceflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DefineAssembly: DefineAssembly::<Identity, OFFSET>,
            DefineAssemblyRef: DefineAssemblyRef::<Identity, OFFSET>,
            DefineFile: DefineFile::<Identity, OFFSET>,
            DefineExportedType: DefineExportedType::<Identity, OFFSET>,
            DefineManifestResource: DefineManifestResource::<Identity, OFFSET>,
            SetAssemblyProps: SetAssemblyProps::<Identity, OFFSET>,
            SetAssemblyRefProps: SetAssemblyRefProps::<Identity, OFFSET>,
            SetFileProps: SetFileProps::<Identity, OFFSET>,
            SetExportedTypeProps: SetExportedTypeProps::<Identity, OFFSET>,
            SetManifestResourceProps: SetManifestResourceProps::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMetaDataAssemblyEmit as windows_core::Interface>::IID
    }
}
pub trait IMetaDataAssemblyImport_Impl: Sized + windows_core::IUnknownImpl {
    fn GetAssemblyProps(&self, mda: u32, ppbpublickey: *const *const core::ffi::c_void, pcbpublickey: *mut u32, pulhashalgid: *mut u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32, pmetadata: *mut ASSEMBLYMETADATA, pdwassemblyflags: *mut u32) -> windows_core::Result<()>;
    fn GetAssemblyRefProps(&self, mdar: u32, ppbpublickeyortoken: *const *const core::ffi::c_void, pcbpublickeyortoken: *mut u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32, pmetadata: *mut ASSEMBLYMETADATA, ppbhashvalue: *const *const core::ffi::c_void, pcbhashvalue: *mut u32, pdwassemblyrefflags: *mut u32) -> windows_core::Result<()>;
    fn GetFileProps(&self, mdf: u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32, ppbhashvalue: *const *const core::ffi::c_void, pcbhashvalue: *mut u32, pdwfileflags: *mut u32) -> windows_core::Result<()>;
    fn GetExportedTypeProps(&self, mdct: u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32, ptkimplementation: *mut u32, ptktypedef: *mut u32, pdwexportedtypeflags: *mut u32) -> windows_core::Result<()>;
    fn GetManifestResourceProps(&self, mdmr: u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32, ptkimplementation: *mut u32, pdwoffset: *mut u32, pdwresourceflags: *mut u32) -> windows_core::Result<()>;
    fn EnumAssemblyRefs(&self, phenum: *mut *mut core::ffi::c_void, rassemblyrefs: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>;
    fn EnumFiles(&self, phenum: *mut *mut core::ffi::c_void, rfiles: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>;
    fn EnumExportedTypes(&self, phenum: *mut *mut core::ffi::c_void, rexportedtypes: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>;
    fn EnumManifestResources(&self, phenum: *mut *mut core::ffi::c_void, rmanifestresources: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>;
    fn GetAssemblyFromScope(&self, ptkassembly: *mut u32) -> windows_core::Result<()>;
    fn FindExportedTypeByName(&self, szname: &windows_core::PCWSTR, mdtexportedtype: u32, ptkexportedtype: *mut u32) -> windows_core::Result<()>;
    fn FindManifestResourceByName(&self, szname: &windows_core::PCWSTR, ptkmanifestresource: *mut u32) -> windows_core::Result<()>;
    fn CloseEnum(&self, henum: *mut core::ffi::c_void);
    fn FindAssembliesByName(&self, szappbase: &windows_core::PCWSTR, szprivatebin: &windows_core::PCWSTR, szassemblyname: &windows_core::PCWSTR, ppiunk: *mut Option<windows_core::IUnknown>, cmax: u32, pcassemblies: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMetaDataAssemblyImport {}
impl IMetaDataAssemblyImport_Vtbl {
    pub const fn new<Identity: IMetaDataAssemblyImport_Impl, const OFFSET: isize>() -> IMetaDataAssemblyImport_Vtbl {
        unsafe extern "system" fn GetAssemblyProps<Identity: IMetaDataAssemblyImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mda: u32, ppbpublickey: *const *const core::ffi::c_void, pcbpublickey: *mut u32, pulhashalgid: *mut u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32, pmetadata: *mut ASSEMBLYMETADATA, pdwassemblyflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyImport_Impl::GetAssemblyProps(this, core::mem::transmute_copy(&mda), core::mem::transmute_copy(&ppbpublickey), core::mem::transmute_copy(&pcbpublickey), core::mem::transmute_copy(&pulhashalgid), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pchname), core::mem::transmute_copy(&pmetadata), core::mem::transmute_copy(&pdwassemblyflags)).into()
        }
        unsafe extern "system" fn GetAssemblyRefProps<Identity: IMetaDataAssemblyImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mdar: u32, ppbpublickeyortoken: *const *const core::ffi::c_void, pcbpublickeyortoken: *mut u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32, pmetadata: *mut ASSEMBLYMETADATA, ppbhashvalue: *const *const core::ffi::c_void, pcbhashvalue: *mut u32, pdwassemblyrefflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyImport_Impl::GetAssemblyRefProps(this, core::mem::transmute_copy(&mdar), core::mem::transmute_copy(&ppbpublickeyortoken), core::mem::transmute_copy(&pcbpublickeyortoken), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pchname), core::mem::transmute_copy(&pmetadata), core::mem::transmute_copy(&ppbhashvalue), core::mem::transmute_copy(&pcbhashvalue), core::mem::transmute_copy(&pdwassemblyrefflags)).into()
        }
        unsafe extern "system" fn GetFileProps<Identity: IMetaDataAssemblyImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mdf: u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32, ppbhashvalue: *const *const core::ffi::c_void, pcbhashvalue: *mut u32, pdwfileflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyImport_Impl::GetFileProps(this, core::mem::transmute_copy(&mdf), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pchname), core::mem::transmute_copy(&ppbhashvalue), core::mem::transmute_copy(&pcbhashvalue), core::mem::transmute_copy(&pdwfileflags)).into()
        }
        unsafe extern "system" fn GetExportedTypeProps<Identity: IMetaDataAssemblyImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mdct: u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32, ptkimplementation: *mut u32, ptktypedef: *mut u32, pdwexportedtypeflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyImport_Impl::GetExportedTypeProps(this, core::mem::transmute_copy(&mdct), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pchname), core::mem::transmute_copy(&ptkimplementation), core::mem::transmute_copy(&ptktypedef), core::mem::transmute_copy(&pdwexportedtypeflags)).into()
        }
        unsafe extern "system" fn GetManifestResourceProps<Identity: IMetaDataAssemblyImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mdmr: u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32, ptkimplementation: *mut u32, pdwoffset: *mut u32, pdwresourceflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyImport_Impl::GetManifestResourceProps(this, core::mem::transmute_copy(&mdmr), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pchname), core::mem::transmute_copy(&ptkimplementation), core::mem::transmute_copy(&pdwoffset), core::mem::transmute_copy(&pdwresourceflags)).into()
        }
        unsafe extern "system" fn EnumAssemblyRefs<Identity: IMetaDataAssemblyImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, rassemblyrefs: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyImport_Impl::EnumAssemblyRefs(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&rassemblyrefs), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctokens)).into()
        }
        unsafe extern "system" fn EnumFiles<Identity: IMetaDataAssemblyImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, rfiles: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyImport_Impl::EnumFiles(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&rfiles), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctokens)).into()
        }
        unsafe extern "system" fn EnumExportedTypes<Identity: IMetaDataAssemblyImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, rexportedtypes: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyImport_Impl::EnumExportedTypes(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&rexportedtypes), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctokens)).into()
        }
        unsafe extern "system" fn EnumManifestResources<Identity: IMetaDataAssemblyImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, rmanifestresources: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyImport_Impl::EnumManifestResources(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&rmanifestresources), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctokens)).into()
        }
        unsafe extern "system" fn GetAssemblyFromScope<Identity: IMetaDataAssemblyImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptkassembly: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyImport_Impl::GetAssemblyFromScope(this, core::mem::transmute_copy(&ptkassembly)).into()
        }
        unsafe extern "system" fn FindExportedTypeByName<Identity: IMetaDataAssemblyImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, mdtexportedtype: u32, ptkexportedtype: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyImport_Impl::FindExportedTypeByName(this, core::mem::transmute(&szname), core::mem::transmute_copy(&mdtexportedtype), core::mem::transmute_copy(&ptkexportedtype)).into()
        }
        unsafe extern "system" fn FindManifestResourceByName<Identity: IMetaDataAssemblyImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, ptkmanifestresource: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyImport_Impl::FindManifestResourceByName(this, core::mem::transmute(&szname), core::mem::transmute_copy(&ptkmanifestresource)).into()
        }
        unsafe extern "system" fn CloseEnum<Identity: IMetaDataAssemblyImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, henum: *mut core::ffi::c_void) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyImport_Impl::CloseEnum(this, core::mem::transmute_copy(&henum))
        }
        unsafe extern "system" fn FindAssembliesByName<Identity: IMetaDataAssemblyImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szappbase: windows_core::PCWSTR, szprivatebin: windows_core::PCWSTR, szassemblyname: windows_core::PCWSTR, ppiunk: *mut *mut core::ffi::c_void, cmax: u32, pcassemblies: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataAssemblyImport_Impl::FindAssembliesByName(this, core::mem::transmute(&szappbase), core::mem::transmute(&szprivatebin), core::mem::transmute(&szassemblyname), core::mem::transmute_copy(&ppiunk), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pcassemblies)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAssemblyProps: GetAssemblyProps::<Identity, OFFSET>,
            GetAssemblyRefProps: GetAssemblyRefProps::<Identity, OFFSET>,
            GetFileProps: GetFileProps::<Identity, OFFSET>,
            GetExportedTypeProps: GetExportedTypeProps::<Identity, OFFSET>,
            GetManifestResourceProps: GetManifestResourceProps::<Identity, OFFSET>,
            EnumAssemblyRefs: EnumAssemblyRefs::<Identity, OFFSET>,
            EnumFiles: EnumFiles::<Identity, OFFSET>,
            EnumExportedTypes: EnumExportedTypes::<Identity, OFFSET>,
            EnumManifestResources: EnumManifestResources::<Identity, OFFSET>,
            GetAssemblyFromScope: GetAssemblyFromScope::<Identity, OFFSET>,
            FindExportedTypeByName: FindExportedTypeByName::<Identity, OFFSET>,
            FindManifestResourceByName: FindManifestResourceByName::<Identity, OFFSET>,
            CloseEnum: CloseEnum::<Identity, OFFSET>,
            FindAssembliesByName: FindAssembliesByName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMetaDataAssemblyImport as windows_core::Interface>::IID
    }
}
pub trait IMetaDataDispenser_Impl: Sized + windows_core::IUnknownImpl {
    fn DefineScope(&self, rclsid: *const windows_core::GUID, dwcreateflags: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn OpenScope(&self, szscope: &windows_core::PCWSTR, dwopenflags: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn OpenScopeOnMemory(&self, pdata: *const core::ffi::c_void, cbdata: u32, dwopenflags: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IMetaDataDispenser {}
impl IMetaDataDispenser_Vtbl {
    pub const fn new<Identity: IMetaDataDispenser_Impl, const OFFSET: isize>() -> IMetaDataDispenser_Vtbl {
        unsafe extern "system" fn DefineScope<Identity: IMetaDataDispenser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, dwcreateflags: u32, riid: *const windows_core::GUID, ppiunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMetaDataDispenser_Impl::DefineScope(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&dwcreateflags), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppiunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenScope<Identity: IMetaDataDispenser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szscope: windows_core::PCWSTR, dwopenflags: u32, riid: *const windows_core::GUID, ppiunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMetaDataDispenser_Impl::OpenScope(this, core::mem::transmute(&szscope), core::mem::transmute_copy(&dwopenflags), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppiunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenScopeOnMemory<Identity: IMetaDataDispenser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, cbdata: u32, dwopenflags: u32, riid: *const windows_core::GUID, ppiunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMetaDataDispenser_Impl::OpenScopeOnMemory(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&cbdata), core::mem::transmute_copy(&dwopenflags), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppiunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DefineScope: DefineScope::<Identity, OFFSET>,
            OpenScope: OpenScope::<Identity, OFFSET>,
            OpenScopeOnMemory: OpenScopeOnMemory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMetaDataDispenser as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMetaDataDispenserEx_Impl: Sized + IMetaDataDispenser_Impl {
    fn SetOption(&self, optionid: *const windows_core::GUID, value: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetOption(&self, optionid: *const windows_core::GUID, pvalue: *mut windows_core::VARIANT) -> windows_core::Result<()>;
    fn OpenScopeOnITypeInfo(&self, piti: Option<&super::super::Com::ITypeInfo>, dwopenflags: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn GetCORSystemDirectory(&self, szbuffer: windows_core::PWSTR, cchbuffer: u32, pchbuffer: *mut u32) -> windows_core::Result<()>;
    fn FindAssembly(&self, szappbase: &windows_core::PCWSTR, szprivatebin: &windows_core::PCWSTR, szglobalbin: &windows_core::PCWSTR, szassemblyname: &windows_core::PCWSTR, szname: &windows_core::PCWSTR, cchname: u32, pcname: *mut u32) -> windows_core::Result<()>;
    fn FindAssemblyModule(&self, szappbase: &windows_core::PCWSTR, szprivatebin: &windows_core::PCWSTR, szglobalbin: &windows_core::PCWSTR, szassemblyname: &windows_core::PCWSTR, szmodulename: &windows_core::PCWSTR, szname: windows_core::PWSTR, cchname: u32, pcname: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMetaDataDispenserEx {}
#[cfg(feature = "Win32_System_Com")]
impl IMetaDataDispenserEx_Vtbl {
    pub const fn new<Identity: IMetaDataDispenserEx_Impl, const OFFSET: isize>() -> IMetaDataDispenserEx_Vtbl {
        unsafe extern "system" fn SetOption<Identity: IMetaDataDispenserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionid: *const windows_core::GUID, value: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataDispenserEx_Impl::SetOption(this, core::mem::transmute_copy(&optionid), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetOption<Identity: IMetaDataDispenserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionid: *const windows_core::GUID, pvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataDispenserEx_Impl::GetOption(this, core::mem::transmute_copy(&optionid), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn OpenScopeOnITypeInfo<Identity: IMetaDataDispenserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piti: *mut core::ffi::c_void, dwopenflags: u32, riid: *const windows_core::GUID, ppiunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMetaDataDispenserEx_Impl::OpenScopeOnITypeInfo(this, windows_core::from_raw_borrowed(&piti), core::mem::transmute_copy(&dwopenflags), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppiunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCORSystemDirectory<Identity: IMetaDataDispenserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szbuffer: windows_core::PWSTR, cchbuffer: u32, pchbuffer: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataDispenserEx_Impl::GetCORSystemDirectory(this, core::mem::transmute_copy(&szbuffer), core::mem::transmute_copy(&cchbuffer), core::mem::transmute_copy(&pchbuffer)).into()
        }
        unsafe extern "system" fn FindAssembly<Identity: IMetaDataDispenserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szappbase: windows_core::PCWSTR, szprivatebin: windows_core::PCWSTR, szglobalbin: windows_core::PCWSTR, szassemblyname: windows_core::PCWSTR, szname: windows_core::PCWSTR, cchname: u32, pcname: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataDispenserEx_Impl::FindAssembly(this, core::mem::transmute(&szappbase), core::mem::transmute(&szprivatebin), core::mem::transmute(&szglobalbin), core::mem::transmute(&szassemblyname), core::mem::transmute(&szname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pcname)).into()
        }
        unsafe extern "system" fn FindAssemblyModule<Identity: IMetaDataDispenserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szappbase: windows_core::PCWSTR, szprivatebin: windows_core::PCWSTR, szglobalbin: windows_core::PCWSTR, szassemblyname: windows_core::PCWSTR, szmodulename: windows_core::PCWSTR, szname: windows_core::PWSTR, cchname: u32, pcname: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataDispenserEx_Impl::FindAssemblyModule(this, core::mem::transmute(&szappbase), core::mem::transmute(&szprivatebin), core::mem::transmute(&szglobalbin), core::mem::transmute(&szassemblyname), core::mem::transmute(&szmodulename), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pcname)).into()
        }
        Self {
            base__: IMetaDataDispenser_Vtbl::new::<Identity, OFFSET>(),
            SetOption: SetOption::<Identity, OFFSET>,
            GetOption: GetOption::<Identity, OFFSET>,
            OpenScopeOnITypeInfo: OpenScopeOnITypeInfo::<Identity, OFFSET>,
            GetCORSystemDirectory: GetCORSystemDirectory::<Identity, OFFSET>,
            FindAssembly: FindAssembly::<Identity, OFFSET>,
            FindAssemblyModule: FindAssemblyModule::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMetaDataDispenserEx as windows_core::Interface>::IID || iid == &<IMetaDataDispenser as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMetaDataEmit_Impl: Sized + windows_core::IUnknownImpl {
    fn SetModuleProps(&self, szname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Save(&self, szfile: &windows_core::PCWSTR, dwsaveflags: u32) -> windows_core::Result<()>;
    fn SaveToStream(&self, pistream: Option<&super::super::Com::IStream>, dwsaveflags: u32) -> windows_core::Result<()>;
    fn GetSaveSize(&self, fsave: CorSaveSize, pdwsavesize: *mut u32) -> windows_core::Result<()>;
    fn DefineTypeDef(&self, sztypedef: &windows_core::PCWSTR, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32, ptd: *mut u32) -> windows_core::Result<()>;
    fn DefineNestedType(&self, sztypedef: &windows_core::PCWSTR, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32, tdencloser: u32, ptd: *mut u32) -> windows_core::Result<()>;
    fn SetHandler(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DefineMethod(&self, td: u32, szname: &windows_core::PCWSTR, dwmethodflags: u32, pvsigblob: *mut u8, cbsigblob: u32, ulcoderva: u32, dwimplflags: u32, pmd: *mut u32) -> windows_core::Result<()>;
    fn DefineMethodImpl(&self, td: u32, tkbody: u32, tkdecl: u32) -> windows_core::Result<()>;
    fn DefineTypeRefByName(&self, tkresolutionscope: u32, szname: &windows_core::PCWSTR, ptr: *mut u32) -> windows_core::Result<()>;
    fn DefineImportType(&self, passemimport: Option<&IMetaDataAssemblyImport>, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, pimport: Option<&IMetaDataImport>, tdimport: u32, passememit: Option<&IMetaDataAssemblyEmit>, ptr: *mut u32) -> windows_core::Result<()>;
    fn DefineMemberRef(&self, tkimport: u32, szname: &windows_core::PCWSTR, pvsigblob: *mut u8, cbsigblob: u32, pmr: *mut u32) -> windows_core::Result<()>;
    fn DefineImportMember(&self, passemimport: Option<&IMetaDataAssemblyImport>, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, pimport: Option<&IMetaDataImport>, mbmember: u32, passememit: Option<&IMetaDataAssemblyEmit>, tkparent: u32, pmr: *mut u32) -> windows_core::Result<()>;
    fn DefineEvent(&self, td: u32, szevent: &windows_core::PCWSTR, dweventflags: u32, tkeventtype: u32, mdaddon: u32, mdremoveon: u32, mdfire: u32, rmdothermethods: *mut u32, pmdevent: *mut u32) -> windows_core::Result<()>;
    fn SetClassLayout(&self, td: u32, dwpacksize: u32, rfieldoffsets: *mut COR_FIELD_OFFSET, ulclasssize: u32) -> windows_core::Result<()>;
    fn DeleteClassLayout(&self, td: u32) -> windows_core::Result<()>;
    fn SetFieldMarshal(&self, tk: u32, pvnativetype: *mut u8, cbnativetype: u32) -> windows_core::Result<()>;
    fn DeleteFieldMarshal(&self, tk: u32) -> windows_core::Result<()>;
    fn DefinePermissionSet(&self, tk: u32, dwaction: u32, pvpermission: *const core::ffi::c_void, cbpermission: u32, ppm: *mut u32) -> windows_core::Result<()>;
    fn SetRVA(&self, md: u32, ulrva: u32) -> windows_core::Result<()>;
    fn GetTokenFromSig(&self, pvsig: *mut u8, cbsig: u32, pmsig: *mut u32) -> windows_core::Result<()>;
    fn DefineModuleRef(&self, szname: &windows_core::PCWSTR, pmur: *mut u32) -> windows_core::Result<()>;
    fn SetParent(&self, mr: u32, tk: u32) -> windows_core::Result<()>;
    fn GetTokenFromTypeSpec(&self, pvsig: *mut u8, cbsig: u32, ptypespec: *mut u32) -> windows_core::Result<()>;
    fn SaveToMemory(&self, pbdata: *mut core::ffi::c_void, cbdata: u32) -> windows_core::Result<()>;
    fn DefineUserString(&self, szstring: &windows_core::PCWSTR, cchstring: u32, pstk: *mut u32) -> windows_core::Result<()>;
    fn DeleteToken(&self, tkobj: u32) -> windows_core::Result<()>;
    fn SetMethodProps(&self, md: u32, dwmethodflags: u32, ulcoderva: u32, dwimplflags: u32) -> windows_core::Result<()>;
    fn SetTypeDefProps(&self, td: u32, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32) -> windows_core::Result<()>;
    fn SetEventProps(&self, ev: u32, dweventflags: u32, tkeventtype: u32, mdaddon: u32, mdremoveon: u32, mdfire: u32, rmdothermethods: *mut u32) -> windows_core::Result<()>;
    fn SetPermissionSetProps(&self, tk: u32, dwaction: u32, pvpermission: *const core::ffi::c_void, cbpermission: u32, ppm: *mut u32) -> windows_core::Result<()>;
    fn DefinePinvokeMap(&self, tk: u32, dwmappingflags: u32, szimportname: &windows_core::PCWSTR, mrimportdll: u32) -> windows_core::Result<()>;
    fn SetPinvokeMap(&self, tk: u32, dwmappingflags: u32, szimportname: &windows_core::PCWSTR, mrimportdll: u32) -> windows_core::Result<()>;
    fn DeletePinvokeMap(&self, tk: u32) -> windows_core::Result<()>;
    fn DefineCustomAttribute(&self, tkowner: u32, tkctor: u32, pcustomattribute: *const core::ffi::c_void, cbcustomattribute: u32, pcv: *mut u32) -> windows_core::Result<()>;
    fn SetCustomAttributeValue(&self, pcv: u32, pcustomattribute: *const core::ffi::c_void, cbcustomattribute: u32) -> windows_core::Result<()>;
    fn DefineField(&self, td: u32, szname: &windows_core::PCWSTR, dwfieldflags: u32, pvsigblob: *mut u8, cbsigblob: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32, pmd: *mut u32) -> windows_core::Result<()>;
    fn DefineProperty(&self, td: u32, szproperty: &windows_core::PCWSTR, dwpropflags: u32, pvsig: *mut u8, cbsig: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32, mdsetter: u32, mdgetter: u32, rmdothermethods: *mut u32, pmdprop: *mut u32) -> windows_core::Result<()>;
    fn DefineParam(&self, md: u32, ulparamseq: u32, szname: &windows_core::PCWSTR, dwparamflags: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32, ppd: *mut u32) -> windows_core::Result<()>;
    fn SetFieldProps(&self, fd: u32, dwfieldflags: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32) -> windows_core::Result<()>;
    fn SetPropertyProps(&self, pr: u32, dwpropflags: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32, mdsetter: u32, mdgetter: u32, rmdothermethods: *mut u32) -> windows_core::Result<()>;
    fn SetParamProps(&self, pd: u32, szname: &windows_core::PCWSTR, dwparamflags: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32) -> windows_core::Result<()>;
    fn DefineSecurityAttributeSet(&self, tkobj: u32, rsecattrs: *mut COR_SECATTR, csecattrs: u32, pulerrorattr: *mut u32) -> windows_core::Result<()>;
    fn ApplyEditAndContinue(&self, pimport: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn TranslateSigWithScope(&self, passemimport: Option<&IMetaDataAssemblyImport>, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, import: Option<&IMetaDataImport>, pbsigblob: *mut u8, cbsigblob: u32, passememit: Option<&IMetaDataAssemblyEmit>, emit: Option<&IMetaDataEmit>, pvtranslatedsig: *mut u8, cbtranslatedsigmax: u32, pcbtranslatedsig: *mut u32) -> windows_core::Result<()>;
    fn SetMethodImplFlags(&self, md: u32, dwimplflags: u32) -> windows_core::Result<()>;
    fn SetFieldRVA(&self, fd: u32, ulrva: u32) -> windows_core::Result<()>;
    fn Merge(&self, pimport: Option<&IMetaDataImport>, phostmaptoken: Option<&IMapToken>, phandler: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn MergeEnd(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMetaDataEmit {}
#[cfg(feature = "Win32_System_Com")]
impl IMetaDataEmit_Vtbl {
    pub const fn new<Identity: IMetaDataEmit_Impl, const OFFSET: isize>() -> IMetaDataEmit_Vtbl {
        unsafe extern "system" fn SetModuleProps<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SetModuleProps(this, core::mem::transmute(&szname)).into()
        }
        unsafe extern "system" fn Save<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szfile: windows_core::PCWSTR, dwsaveflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::Save(this, core::mem::transmute(&szfile), core::mem::transmute_copy(&dwsaveflags)).into()
        }
        unsafe extern "system" fn SaveToStream<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, dwsaveflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SaveToStream(this, windows_core::from_raw_borrowed(&pistream), core::mem::transmute_copy(&dwsaveflags)).into()
        }
        unsafe extern "system" fn GetSaveSize<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsave: CorSaveSize, pdwsavesize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::GetSaveSize(this, core::mem::transmute_copy(&fsave), core::mem::transmute_copy(&pdwsavesize)).into()
        }
        unsafe extern "system" fn DefineTypeDef<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztypedef: windows_core::PCWSTR, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32, ptd: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefineTypeDef(this, core::mem::transmute(&sztypedef), core::mem::transmute_copy(&dwtypedefflags), core::mem::transmute_copy(&tkextends), core::mem::transmute_copy(&rtkimplements), core::mem::transmute_copy(&ptd)).into()
        }
        unsafe extern "system" fn DefineNestedType<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztypedef: windows_core::PCWSTR, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32, tdencloser: u32, ptd: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefineNestedType(this, core::mem::transmute(&sztypedef), core::mem::transmute_copy(&dwtypedefflags), core::mem::transmute_copy(&tkextends), core::mem::transmute_copy(&rtkimplements), core::mem::transmute_copy(&tdencloser), core::mem::transmute_copy(&ptd)).into()
        }
        unsafe extern "system" fn SetHandler<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SetHandler(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn DefineMethod<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, td: u32, szname: windows_core::PCWSTR, dwmethodflags: u32, pvsigblob: *mut u8, cbsigblob: u32, ulcoderva: u32, dwimplflags: u32, pmd: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefineMethod(this, core::mem::transmute_copy(&td), core::mem::transmute(&szname), core::mem::transmute_copy(&dwmethodflags), core::mem::transmute_copy(&pvsigblob), core::mem::transmute_copy(&cbsigblob), core::mem::transmute_copy(&ulcoderva), core::mem::transmute_copy(&dwimplflags), core::mem::transmute_copy(&pmd)).into()
        }
        unsafe extern "system" fn DefineMethodImpl<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, td: u32, tkbody: u32, tkdecl: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefineMethodImpl(this, core::mem::transmute_copy(&td), core::mem::transmute_copy(&tkbody), core::mem::transmute_copy(&tkdecl)).into()
        }
        unsafe extern "system" fn DefineTypeRefByName<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tkresolutionscope: u32, szname: windows_core::PCWSTR, ptr: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefineTypeRefByName(this, core::mem::transmute_copy(&tkresolutionscope), core::mem::transmute(&szname), core::mem::transmute_copy(&ptr)).into()
        }
        unsafe extern "system" fn DefineImportType<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, passemimport: *mut core::ffi::c_void, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, pimport: *mut core::ffi::c_void, tdimport: u32, passememit: *mut core::ffi::c_void, ptr: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefineImportType(this, windows_core::from_raw_borrowed(&passemimport), core::mem::transmute_copy(&pbhashvalue), core::mem::transmute_copy(&cbhashvalue), windows_core::from_raw_borrowed(&pimport), core::mem::transmute_copy(&tdimport), windows_core::from_raw_borrowed(&passememit), core::mem::transmute_copy(&ptr)).into()
        }
        unsafe extern "system" fn DefineMemberRef<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tkimport: u32, szname: windows_core::PCWSTR, pvsigblob: *mut u8, cbsigblob: u32, pmr: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefineMemberRef(this, core::mem::transmute_copy(&tkimport), core::mem::transmute(&szname), core::mem::transmute_copy(&pvsigblob), core::mem::transmute_copy(&cbsigblob), core::mem::transmute_copy(&pmr)).into()
        }
        unsafe extern "system" fn DefineImportMember<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, passemimport: *mut core::ffi::c_void, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, pimport: *mut core::ffi::c_void, mbmember: u32, passememit: *mut core::ffi::c_void, tkparent: u32, pmr: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefineImportMember(this, windows_core::from_raw_borrowed(&passemimport), core::mem::transmute_copy(&pbhashvalue), core::mem::transmute_copy(&cbhashvalue), windows_core::from_raw_borrowed(&pimport), core::mem::transmute_copy(&mbmember), windows_core::from_raw_borrowed(&passememit), core::mem::transmute_copy(&tkparent), core::mem::transmute_copy(&pmr)).into()
        }
        unsafe extern "system" fn DefineEvent<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, td: u32, szevent: windows_core::PCWSTR, dweventflags: u32, tkeventtype: u32, mdaddon: u32, mdremoveon: u32, mdfire: u32, rmdothermethods: *mut u32, pmdevent: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefineEvent(this, core::mem::transmute_copy(&td), core::mem::transmute(&szevent), core::mem::transmute_copy(&dweventflags), core::mem::transmute_copy(&tkeventtype), core::mem::transmute_copy(&mdaddon), core::mem::transmute_copy(&mdremoveon), core::mem::transmute_copy(&mdfire), core::mem::transmute_copy(&rmdothermethods), core::mem::transmute_copy(&pmdevent)).into()
        }
        unsafe extern "system" fn SetClassLayout<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, td: u32, dwpacksize: u32, rfieldoffsets: *mut COR_FIELD_OFFSET, ulclasssize: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SetClassLayout(this, core::mem::transmute_copy(&td), core::mem::transmute_copy(&dwpacksize), core::mem::transmute_copy(&rfieldoffsets), core::mem::transmute_copy(&ulclasssize)).into()
        }
        unsafe extern "system" fn DeleteClassLayout<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, td: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DeleteClassLayout(this, core::mem::transmute_copy(&td)).into()
        }
        unsafe extern "system" fn SetFieldMarshal<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tk: u32, pvnativetype: *mut u8, cbnativetype: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SetFieldMarshal(this, core::mem::transmute_copy(&tk), core::mem::transmute_copy(&pvnativetype), core::mem::transmute_copy(&cbnativetype)).into()
        }
        unsafe extern "system" fn DeleteFieldMarshal<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tk: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DeleteFieldMarshal(this, core::mem::transmute_copy(&tk)).into()
        }
        unsafe extern "system" fn DefinePermissionSet<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tk: u32, dwaction: u32, pvpermission: *const core::ffi::c_void, cbpermission: u32, ppm: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefinePermissionSet(this, core::mem::transmute_copy(&tk), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&pvpermission), core::mem::transmute_copy(&cbpermission), core::mem::transmute_copy(&ppm)).into()
        }
        unsafe extern "system" fn SetRVA<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, md: u32, ulrva: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SetRVA(this, core::mem::transmute_copy(&md), core::mem::transmute_copy(&ulrva)).into()
        }
        unsafe extern "system" fn GetTokenFromSig<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvsig: *mut u8, cbsig: u32, pmsig: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::GetTokenFromSig(this, core::mem::transmute_copy(&pvsig), core::mem::transmute_copy(&cbsig), core::mem::transmute_copy(&pmsig)).into()
        }
        unsafe extern "system" fn DefineModuleRef<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, pmur: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefineModuleRef(this, core::mem::transmute(&szname), core::mem::transmute_copy(&pmur)).into()
        }
        unsafe extern "system" fn SetParent<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mr: u32, tk: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SetParent(this, core::mem::transmute_copy(&mr), core::mem::transmute_copy(&tk)).into()
        }
        unsafe extern "system" fn GetTokenFromTypeSpec<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvsig: *mut u8, cbsig: u32, ptypespec: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::GetTokenFromTypeSpec(this, core::mem::transmute_copy(&pvsig), core::mem::transmute_copy(&cbsig), core::mem::transmute_copy(&ptypespec)).into()
        }
        unsafe extern "system" fn SaveToMemory<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdata: *mut core::ffi::c_void, cbdata: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SaveToMemory(this, core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn DefineUserString<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szstring: windows_core::PCWSTR, cchstring: u32, pstk: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefineUserString(this, core::mem::transmute(&szstring), core::mem::transmute_copy(&cchstring), core::mem::transmute_copy(&pstk)).into()
        }
        unsafe extern "system" fn DeleteToken<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tkobj: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DeleteToken(this, core::mem::transmute_copy(&tkobj)).into()
        }
        unsafe extern "system" fn SetMethodProps<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, md: u32, dwmethodflags: u32, ulcoderva: u32, dwimplflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SetMethodProps(this, core::mem::transmute_copy(&md), core::mem::transmute_copy(&dwmethodflags), core::mem::transmute_copy(&ulcoderva), core::mem::transmute_copy(&dwimplflags)).into()
        }
        unsafe extern "system" fn SetTypeDefProps<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, td: u32, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SetTypeDefProps(this, core::mem::transmute_copy(&td), core::mem::transmute_copy(&dwtypedefflags), core::mem::transmute_copy(&tkextends), core::mem::transmute_copy(&rtkimplements)).into()
        }
        unsafe extern "system" fn SetEventProps<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ev: u32, dweventflags: u32, tkeventtype: u32, mdaddon: u32, mdremoveon: u32, mdfire: u32, rmdothermethods: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SetEventProps(this, core::mem::transmute_copy(&ev), core::mem::transmute_copy(&dweventflags), core::mem::transmute_copy(&tkeventtype), core::mem::transmute_copy(&mdaddon), core::mem::transmute_copy(&mdremoveon), core::mem::transmute_copy(&mdfire), core::mem::transmute_copy(&rmdothermethods)).into()
        }
        unsafe extern "system" fn SetPermissionSetProps<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tk: u32, dwaction: u32, pvpermission: *const core::ffi::c_void, cbpermission: u32, ppm: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SetPermissionSetProps(this, core::mem::transmute_copy(&tk), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&pvpermission), core::mem::transmute_copy(&cbpermission), core::mem::transmute_copy(&ppm)).into()
        }
        unsafe extern "system" fn DefinePinvokeMap<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tk: u32, dwmappingflags: u32, szimportname: windows_core::PCWSTR, mrimportdll: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefinePinvokeMap(this, core::mem::transmute_copy(&tk), core::mem::transmute_copy(&dwmappingflags), core::mem::transmute(&szimportname), core::mem::transmute_copy(&mrimportdll)).into()
        }
        unsafe extern "system" fn SetPinvokeMap<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tk: u32, dwmappingflags: u32, szimportname: windows_core::PCWSTR, mrimportdll: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SetPinvokeMap(this, core::mem::transmute_copy(&tk), core::mem::transmute_copy(&dwmappingflags), core::mem::transmute(&szimportname), core::mem::transmute_copy(&mrimportdll)).into()
        }
        unsafe extern "system" fn DeletePinvokeMap<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tk: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DeletePinvokeMap(this, core::mem::transmute_copy(&tk)).into()
        }
        unsafe extern "system" fn DefineCustomAttribute<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tkowner: u32, tkctor: u32, pcustomattribute: *const core::ffi::c_void, cbcustomattribute: u32, pcv: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefineCustomAttribute(this, core::mem::transmute_copy(&tkowner), core::mem::transmute_copy(&tkctor), core::mem::transmute_copy(&pcustomattribute), core::mem::transmute_copy(&cbcustomattribute), core::mem::transmute_copy(&pcv)).into()
        }
        unsafe extern "system" fn SetCustomAttributeValue<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcv: u32, pcustomattribute: *const core::ffi::c_void, cbcustomattribute: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SetCustomAttributeValue(this, core::mem::transmute_copy(&pcv), core::mem::transmute_copy(&pcustomattribute), core::mem::transmute_copy(&cbcustomattribute)).into()
        }
        unsafe extern "system" fn DefineField<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, td: u32, szname: windows_core::PCWSTR, dwfieldflags: u32, pvsigblob: *mut u8, cbsigblob: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32, pmd: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefineField(this, core::mem::transmute_copy(&td), core::mem::transmute(&szname), core::mem::transmute_copy(&dwfieldflags), core::mem::transmute_copy(&pvsigblob), core::mem::transmute_copy(&cbsigblob), core::mem::transmute_copy(&dwcplustypeflag), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cchvalue), core::mem::transmute_copy(&pmd)).into()
        }
        unsafe extern "system" fn DefineProperty<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, td: u32, szproperty: windows_core::PCWSTR, dwpropflags: u32, pvsig: *mut u8, cbsig: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32, mdsetter: u32, mdgetter: u32, rmdothermethods: *mut u32, pmdprop: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefineProperty(this, core::mem::transmute_copy(&td), core::mem::transmute(&szproperty), core::mem::transmute_copy(&dwpropflags), core::mem::transmute_copy(&pvsig), core::mem::transmute_copy(&cbsig), core::mem::transmute_copy(&dwcplustypeflag), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cchvalue), core::mem::transmute_copy(&mdsetter), core::mem::transmute_copy(&mdgetter), core::mem::transmute_copy(&rmdothermethods), core::mem::transmute_copy(&pmdprop)).into()
        }
        unsafe extern "system" fn DefineParam<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, md: u32, ulparamseq: u32, szname: windows_core::PCWSTR, dwparamflags: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32, ppd: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefineParam(this, core::mem::transmute_copy(&md), core::mem::transmute_copy(&ulparamseq), core::mem::transmute(&szname), core::mem::transmute_copy(&dwparamflags), core::mem::transmute_copy(&dwcplustypeflag), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cchvalue), core::mem::transmute_copy(&ppd)).into()
        }
        unsafe extern "system" fn SetFieldProps<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fd: u32, dwfieldflags: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SetFieldProps(this, core::mem::transmute_copy(&fd), core::mem::transmute_copy(&dwfieldflags), core::mem::transmute_copy(&dwcplustypeflag), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cchvalue)).into()
        }
        unsafe extern "system" fn SetPropertyProps<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pr: u32, dwpropflags: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32, mdsetter: u32, mdgetter: u32, rmdothermethods: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SetPropertyProps(this, core::mem::transmute_copy(&pr), core::mem::transmute_copy(&dwpropflags), core::mem::transmute_copy(&dwcplustypeflag), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cchvalue), core::mem::transmute_copy(&mdsetter), core::mem::transmute_copy(&mdgetter), core::mem::transmute_copy(&rmdothermethods)).into()
        }
        unsafe extern "system" fn SetParamProps<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pd: u32, szname: windows_core::PCWSTR, dwparamflags: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SetParamProps(this, core::mem::transmute_copy(&pd), core::mem::transmute(&szname), core::mem::transmute_copy(&dwparamflags), core::mem::transmute_copy(&dwcplustypeflag), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cchvalue)).into()
        }
        unsafe extern "system" fn DefineSecurityAttributeSet<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tkobj: u32, rsecattrs: *mut COR_SECATTR, csecattrs: u32, pulerrorattr: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::DefineSecurityAttributeSet(this, core::mem::transmute_copy(&tkobj), core::mem::transmute_copy(&rsecattrs), core::mem::transmute_copy(&csecattrs), core::mem::transmute_copy(&pulerrorattr)).into()
        }
        unsafe extern "system" fn ApplyEditAndContinue<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimport: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::ApplyEditAndContinue(this, windows_core::from_raw_borrowed(&pimport)).into()
        }
        unsafe extern "system" fn TranslateSigWithScope<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, passemimport: *mut core::ffi::c_void, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, import: *mut core::ffi::c_void, pbsigblob: *mut u8, cbsigblob: u32, passememit: *mut core::ffi::c_void, emit: *mut core::ffi::c_void, pvtranslatedsig: *mut u8, cbtranslatedsigmax: u32, pcbtranslatedsig: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::TranslateSigWithScope(this, windows_core::from_raw_borrowed(&passemimport), core::mem::transmute_copy(&pbhashvalue), core::mem::transmute_copy(&cbhashvalue), windows_core::from_raw_borrowed(&import), core::mem::transmute_copy(&pbsigblob), core::mem::transmute_copy(&cbsigblob), windows_core::from_raw_borrowed(&passememit), windows_core::from_raw_borrowed(&emit), core::mem::transmute_copy(&pvtranslatedsig), core::mem::transmute_copy(&cbtranslatedsigmax), core::mem::transmute_copy(&pcbtranslatedsig)).into()
        }
        unsafe extern "system" fn SetMethodImplFlags<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, md: u32, dwimplflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SetMethodImplFlags(this, core::mem::transmute_copy(&md), core::mem::transmute_copy(&dwimplflags)).into()
        }
        unsafe extern "system" fn SetFieldRVA<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fd: u32, ulrva: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::SetFieldRVA(this, core::mem::transmute_copy(&fd), core::mem::transmute_copy(&ulrva)).into()
        }
        unsafe extern "system" fn Merge<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimport: *mut core::ffi::c_void, phostmaptoken: *mut core::ffi::c_void, phandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::Merge(this, windows_core::from_raw_borrowed(&pimport), windows_core::from_raw_borrowed(&phostmaptoken), windows_core::from_raw_borrowed(&phandler)).into()
        }
        unsafe extern "system" fn MergeEnd<Identity: IMetaDataEmit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit_Impl::MergeEnd(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetModuleProps: SetModuleProps::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            SaveToStream: SaveToStream::<Identity, OFFSET>,
            GetSaveSize: GetSaveSize::<Identity, OFFSET>,
            DefineTypeDef: DefineTypeDef::<Identity, OFFSET>,
            DefineNestedType: DefineNestedType::<Identity, OFFSET>,
            SetHandler: SetHandler::<Identity, OFFSET>,
            DefineMethod: DefineMethod::<Identity, OFFSET>,
            DefineMethodImpl: DefineMethodImpl::<Identity, OFFSET>,
            DefineTypeRefByName: DefineTypeRefByName::<Identity, OFFSET>,
            DefineImportType: DefineImportType::<Identity, OFFSET>,
            DefineMemberRef: DefineMemberRef::<Identity, OFFSET>,
            DefineImportMember: DefineImportMember::<Identity, OFFSET>,
            DefineEvent: DefineEvent::<Identity, OFFSET>,
            SetClassLayout: SetClassLayout::<Identity, OFFSET>,
            DeleteClassLayout: DeleteClassLayout::<Identity, OFFSET>,
            SetFieldMarshal: SetFieldMarshal::<Identity, OFFSET>,
            DeleteFieldMarshal: DeleteFieldMarshal::<Identity, OFFSET>,
            DefinePermissionSet: DefinePermissionSet::<Identity, OFFSET>,
            SetRVA: SetRVA::<Identity, OFFSET>,
            GetTokenFromSig: GetTokenFromSig::<Identity, OFFSET>,
            DefineModuleRef: DefineModuleRef::<Identity, OFFSET>,
            SetParent: SetParent::<Identity, OFFSET>,
            GetTokenFromTypeSpec: GetTokenFromTypeSpec::<Identity, OFFSET>,
            SaveToMemory: SaveToMemory::<Identity, OFFSET>,
            DefineUserString: DefineUserString::<Identity, OFFSET>,
            DeleteToken: DeleteToken::<Identity, OFFSET>,
            SetMethodProps: SetMethodProps::<Identity, OFFSET>,
            SetTypeDefProps: SetTypeDefProps::<Identity, OFFSET>,
            SetEventProps: SetEventProps::<Identity, OFFSET>,
            SetPermissionSetProps: SetPermissionSetProps::<Identity, OFFSET>,
            DefinePinvokeMap: DefinePinvokeMap::<Identity, OFFSET>,
            SetPinvokeMap: SetPinvokeMap::<Identity, OFFSET>,
            DeletePinvokeMap: DeletePinvokeMap::<Identity, OFFSET>,
            DefineCustomAttribute: DefineCustomAttribute::<Identity, OFFSET>,
            SetCustomAttributeValue: SetCustomAttributeValue::<Identity, OFFSET>,
            DefineField: DefineField::<Identity, OFFSET>,
            DefineProperty: DefineProperty::<Identity, OFFSET>,
            DefineParam: DefineParam::<Identity, OFFSET>,
            SetFieldProps: SetFieldProps::<Identity, OFFSET>,
            SetPropertyProps: SetPropertyProps::<Identity, OFFSET>,
            SetParamProps: SetParamProps::<Identity, OFFSET>,
            DefineSecurityAttributeSet: DefineSecurityAttributeSet::<Identity, OFFSET>,
            ApplyEditAndContinue: ApplyEditAndContinue::<Identity, OFFSET>,
            TranslateSigWithScope: TranslateSigWithScope::<Identity, OFFSET>,
            SetMethodImplFlags: SetMethodImplFlags::<Identity, OFFSET>,
            SetFieldRVA: SetFieldRVA::<Identity, OFFSET>,
            Merge: Merge::<Identity, OFFSET>,
            MergeEnd: MergeEnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMetaDataEmit as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMetaDataEmit2_Impl: Sized + IMetaDataEmit_Impl {
    fn DefineMethodSpec(&self, tkparent: u32, pvsigblob: *mut u8, cbsigblob: u32, pmi: *mut u32) -> windows_core::Result<()>;
    fn GetDeltaSaveSize(&self, fsave: CorSaveSize, pdwsavesize: *mut u32) -> windows_core::Result<()>;
    fn SaveDelta(&self, szfile: &windows_core::PCWSTR, dwsaveflags: u32) -> windows_core::Result<()>;
    fn SaveDeltaToStream(&self, pistream: Option<&super::super::Com::IStream>, dwsaveflags: u32) -> windows_core::Result<()>;
    fn SaveDeltaToMemory(&self, pbdata: *mut core::ffi::c_void, cbdata: u32) -> windows_core::Result<()>;
    fn DefineGenericParam(&self, tk: u32, ulparamseq: u32, dwparamflags: u32, szname: &windows_core::PCWSTR, reserved: u32, rtkconstraints: *mut u32, pgp: *mut u32) -> windows_core::Result<()>;
    fn SetGenericParamProps(&self, gp: u32, dwparamflags: u32, szname: &windows_core::PCWSTR, reserved: u32, rtkconstraints: *mut u32) -> windows_core::Result<()>;
    fn ResetENCLog(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMetaDataEmit2 {}
#[cfg(feature = "Win32_System_Com")]
impl IMetaDataEmit2_Vtbl {
    pub const fn new<Identity: IMetaDataEmit2_Impl, const OFFSET: isize>() -> IMetaDataEmit2_Vtbl {
        unsafe extern "system" fn DefineMethodSpec<Identity: IMetaDataEmit2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tkparent: u32, pvsigblob: *mut u8, cbsigblob: u32, pmi: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit2_Impl::DefineMethodSpec(this, core::mem::transmute_copy(&tkparent), core::mem::transmute_copy(&pvsigblob), core::mem::transmute_copy(&cbsigblob), core::mem::transmute_copy(&pmi)).into()
        }
        unsafe extern "system" fn GetDeltaSaveSize<Identity: IMetaDataEmit2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsave: CorSaveSize, pdwsavesize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit2_Impl::GetDeltaSaveSize(this, core::mem::transmute_copy(&fsave), core::mem::transmute_copy(&pdwsavesize)).into()
        }
        unsafe extern "system" fn SaveDelta<Identity: IMetaDataEmit2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szfile: windows_core::PCWSTR, dwsaveflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit2_Impl::SaveDelta(this, core::mem::transmute(&szfile), core::mem::transmute_copy(&dwsaveflags)).into()
        }
        unsafe extern "system" fn SaveDeltaToStream<Identity: IMetaDataEmit2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, dwsaveflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit2_Impl::SaveDeltaToStream(this, windows_core::from_raw_borrowed(&pistream), core::mem::transmute_copy(&dwsaveflags)).into()
        }
        unsafe extern "system" fn SaveDeltaToMemory<Identity: IMetaDataEmit2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdata: *mut core::ffi::c_void, cbdata: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit2_Impl::SaveDeltaToMemory(this, core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn DefineGenericParam<Identity: IMetaDataEmit2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tk: u32, ulparamseq: u32, dwparamflags: u32, szname: windows_core::PCWSTR, reserved: u32, rtkconstraints: *mut u32, pgp: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit2_Impl::DefineGenericParam(this, core::mem::transmute_copy(&tk), core::mem::transmute_copy(&ulparamseq), core::mem::transmute_copy(&dwparamflags), core::mem::transmute(&szname), core::mem::transmute_copy(&reserved), core::mem::transmute_copy(&rtkconstraints), core::mem::transmute_copy(&pgp)).into()
        }
        unsafe extern "system" fn SetGenericParamProps<Identity: IMetaDataEmit2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gp: u32, dwparamflags: u32, szname: windows_core::PCWSTR, reserved: u32, rtkconstraints: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit2_Impl::SetGenericParamProps(this, core::mem::transmute_copy(&gp), core::mem::transmute_copy(&dwparamflags), core::mem::transmute(&szname), core::mem::transmute_copy(&reserved), core::mem::transmute_copy(&rtkconstraints)).into()
        }
        unsafe extern "system" fn ResetENCLog<Identity: IMetaDataEmit2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataEmit2_Impl::ResetENCLog(this).into()
        }
        Self {
            base__: IMetaDataEmit_Vtbl::new::<Identity, OFFSET>(),
            DefineMethodSpec: DefineMethodSpec::<Identity, OFFSET>,
            GetDeltaSaveSize: GetDeltaSaveSize::<Identity, OFFSET>,
            SaveDelta: SaveDelta::<Identity, OFFSET>,
            SaveDeltaToStream: SaveDeltaToStream::<Identity, OFFSET>,
            SaveDeltaToMemory: SaveDeltaToMemory::<Identity, OFFSET>,
            DefineGenericParam: DefineGenericParam::<Identity, OFFSET>,
            SetGenericParamProps: SetGenericParamProps::<Identity, OFFSET>,
            ResetENCLog: ResetENCLog::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMetaDataEmit2 as windows_core::Interface>::IID || iid == &<IMetaDataEmit as windows_core::Interface>::IID
    }
}
pub trait IMetaDataError_Impl: Sized + windows_core::IUnknownImpl {
    fn OnError(&self, hrerror: windows_core::HRESULT, token: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMetaDataError {}
impl IMetaDataError_Vtbl {
    pub const fn new<Identity: IMetaDataError_Impl, const OFFSET: isize>() -> IMetaDataError_Vtbl {
        unsafe extern "system" fn OnError<Identity: IMetaDataError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrerror: windows_core::HRESULT, token: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataError_Impl::OnError(this, core::mem::transmute_copy(&hrerror), core::mem::transmute_copy(&token)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnError: OnError::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMetaDataError as windows_core::Interface>::IID
    }
}
pub trait IMetaDataFilter_Impl: Sized + windows_core::IUnknownImpl {
    fn UnmarkAll(&self) -> windows_core::Result<()>;
    fn MarkToken(&self, tk: u32) -> windows_core::Result<()>;
    fn IsTokenMarked(&self, tk: u32, pismarked: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMetaDataFilter {}
impl IMetaDataFilter_Vtbl {
    pub const fn new<Identity: IMetaDataFilter_Impl, const OFFSET: isize>() -> IMetaDataFilter_Vtbl {
        unsafe extern "system" fn UnmarkAll<Identity: IMetaDataFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataFilter_Impl::UnmarkAll(this).into()
        }
        unsafe extern "system" fn MarkToken<Identity: IMetaDataFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tk: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataFilter_Impl::MarkToken(this, core::mem::transmute_copy(&tk)).into()
        }
        unsafe extern "system" fn IsTokenMarked<Identity: IMetaDataFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tk: u32, pismarked: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataFilter_Impl::IsTokenMarked(this, core::mem::transmute_copy(&tk), core::mem::transmute_copy(&pismarked)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            UnmarkAll: UnmarkAll::<Identity, OFFSET>,
            MarkToken: MarkToken::<Identity, OFFSET>,
            IsTokenMarked: IsTokenMarked::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMetaDataFilter as windows_core::Interface>::IID
    }
}
pub trait IMetaDataImport_Impl: Sized + windows_core::IUnknownImpl {
    fn CloseEnum(&self, henum: *mut core::ffi::c_void);
    fn CountEnum(&self, henum: *mut core::ffi::c_void, pulcount: *mut u32) -> windows_core::Result<()>;
    fn ResetEnum(&self, henum: *mut core::ffi::c_void, ulpos: u32) -> windows_core::Result<()>;
    fn EnumTypeDefs(&self, phenum: *mut *mut core::ffi::c_void, rtypedefs: *mut u32, cmax: u32, pctypedefs: *mut u32) -> windows_core::Result<()>;
    fn EnumInterfaceImpls(&self, phenum: *mut *mut core::ffi::c_void, td: u32, rimpls: *mut u32, cmax: u32, pcimpls: *mut u32) -> windows_core::Result<()>;
    fn EnumTypeRefs(&self, phenum: *mut *mut core::ffi::c_void, rtyperefs: *mut u32, cmax: u32, pctyperefs: *mut u32) -> windows_core::Result<()>;
    fn FindTypeDefByName(&self, sztypedef: &windows_core::PCWSTR, tkenclosingclass: u32, ptd: *mut u32) -> windows_core::Result<()>;
    fn GetScopeProps(&self, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32, pmvid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetModuleFromScope(&self, pmd: *mut u32) -> windows_core::Result<()>;
    fn GetTypeDefProps(&self, td: u32, sztypedef: windows_core::PWSTR, cchtypedef: u32, pchtypedef: *mut u32, pdwtypedefflags: *mut u32, ptkextends: *mut u32) -> windows_core::Result<()>;
    fn GetInterfaceImplProps(&self, iiimpl: u32, pclass: *mut u32, ptkiface: *mut u32) -> windows_core::Result<()>;
    fn GetTypeRefProps(&self, tr: u32, ptkresolutionscope: *mut u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32) -> windows_core::Result<()>;
    fn ResolveTypeRef(&self, tr: u32, riid: *const windows_core::GUID, ppiscope: *mut Option<windows_core::IUnknown>, ptd: *mut u32) -> windows_core::Result<()>;
    fn EnumMembers(&self, phenum: *mut *mut core::ffi::c_void, cl: u32, rmembers: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>;
    fn EnumMembersWithName(&self, phenum: *mut *mut core::ffi::c_void, cl: u32, szname: &windows_core::PCWSTR, rmembers: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>;
    fn EnumMethods(&self, phenum: *mut *mut core::ffi::c_void, cl: u32, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>;
    fn EnumMethodsWithName(&self, phenum: *mut *mut core::ffi::c_void, cl: u32, szname: &windows_core::PCWSTR, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>;
    fn EnumFields(&self, phenum: *mut *mut core::ffi::c_void, cl: u32, rfields: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>;
    fn EnumFieldsWithName(&self, phenum: *mut *mut core::ffi::c_void, cl: u32, szname: &windows_core::PCWSTR, rfields: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>;
    fn EnumParams(&self, phenum: *mut *mut core::ffi::c_void, mb: u32, rparams: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>;
    fn EnumMemberRefs(&self, phenum: *mut *mut core::ffi::c_void, tkparent: u32, rmemberrefs: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>;
    fn EnumMethodImpls(&self, phenum: *mut *mut core::ffi::c_void, td: u32, rmethodbody: *mut u32, rmethoddecl: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>;
    fn EnumPermissionSets(&self, phenum: *mut *mut core::ffi::c_void, tk: u32, dwactions: u32, rpermission: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>;
    fn FindMember(&self, td: u32, szname: &windows_core::PCWSTR, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> windows_core::Result<()>;
    fn FindMethod(&self, td: u32, szname: &windows_core::PCWSTR, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> windows_core::Result<()>;
    fn FindField(&self, td: u32, szname: &windows_core::PCWSTR, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> windows_core::Result<()>;
    fn FindMemberRef(&self, td: u32, szname: &windows_core::PCWSTR, pvsigblob: *mut u8, cbsigblob: u32, pmr: *mut u32) -> windows_core::Result<()>;
    fn GetMethodProps(&self, mb: u32, pclass: *mut u32, szmethod: windows_core::PWSTR, cchmethod: u32, pchmethod: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pulcoderva: *mut u32, pdwimplflags: *mut u32) -> windows_core::Result<()>;
    fn GetMemberRefProps(&self, mr: u32, ptk: *mut u32, szmember: windows_core::PWSTR, cchmember: u32, pchmember: *mut u32, ppvsigblob: *mut *mut u8, pbsig: *mut u32) -> windows_core::Result<()>;
    fn EnumProperties(&self, phenum: *mut *mut core::ffi::c_void, td: u32, rproperties: *mut u32, cmax: u32, pcproperties: *mut u32) -> windows_core::Result<()>;
    fn EnumEvents(&self, phenum: *mut *mut core::ffi::c_void, td: u32, revents: *mut u32, cmax: u32, pcevents: *mut u32) -> windows_core::Result<()>;
    fn GetEventProps(&self, ev: u32, pclass: *mut u32, szevent: &windows_core::PCWSTR, cchevent: u32, pchevent: *mut u32, pdweventflags: *mut u32, ptkeventtype: *mut u32, pmdaddon: *mut u32, pmdremoveon: *mut u32, pmdfire: *mut u32, rmdothermethod: *mut u32, cmax: u32, pcothermethod: *mut u32) -> windows_core::Result<()>;
    fn EnumMethodSemantics(&self, phenum: *mut *mut core::ffi::c_void, mb: u32, reventprop: *mut u32, cmax: u32, pceventprop: *mut u32) -> windows_core::Result<()>;
    fn GetMethodSemantics(&self, mb: u32, tkeventprop: u32, pdwsemanticsflags: *mut u32) -> windows_core::Result<()>;
    fn GetClassLayout(&self, td: u32, pdwpacksize: *mut u32, rfieldoffset: *mut COR_FIELD_OFFSET, cmax: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> windows_core::Result<()>;
    fn GetFieldMarshal(&self, tk: u32, ppvnativetype: *mut *mut u8, pcbnativetype: *mut u32) -> windows_core::Result<()>;
    fn GetRVA(&self, tk: u32, pulcoderva: *mut u32, pdwimplflags: *mut u32) -> windows_core::Result<()>;
    fn GetPermissionSetProps(&self, pm: u32, pdwaction: *mut u32, ppvpermission: *const *const core::ffi::c_void, pcbpermission: *mut u32) -> windows_core::Result<()>;
    fn GetSigFromToken(&self, mdsig: u32, ppvsig: *mut *mut u8, pcbsig: *mut u32) -> windows_core::Result<()>;
    fn GetModuleRefProps(&self, mur: u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32) -> windows_core::Result<()>;
    fn EnumModuleRefs(&self, phenum: *mut *mut core::ffi::c_void, rmodulerefs: *mut u32, cmax: u32, pcmodulerefs: *mut u32) -> windows_core::Result<()>;
    fn GetTypeSpecFromToken(&self, typespec: u32, ppvsig: *mut *mut u8, pcbsig: *mut u32) -> windows_core::Result<()>;
    fn GetNameFromToken(&self, tk: u32, pszutf8nameptr: *mut *mut i8) -> windows_core::Result<()>;
    fn EnumUnresolvedMethods(&self, phenum: *mut *mut core::ffi::c_void, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>;
    fn GetUserString(&self, stk: u32, szstring: windows_core::PWSTR, cchstring: u32, pchstring: *mut u32) -> windows_core::Result<()>;
    fn GetPinvokeMap(&self, tk: u32, pdwmappingflags: *mut u32, szimportname: windows_core::PWSTR, cchimportname: u32, pchimportname: *mut u32, pmrimportdll: *mut u32) -> windows_core::Result<()>;
    fn EnumSignatures(&self, phenum: *mut *mut core::ffi::c_void, rsignatures: *mut u32, cmax: u32, pcsignatures: *mut u32) -> windows_core::Result<()>;
    fn EnumTypeSpecs(&self, phenum: *mut *mut core::ffi::c_void, rtypespecs: *mut u32, cmax: u32, pctypespecs: *mut u32) -> windows_core::Result<()>;
    fn EnumUserStrings(&self, phenum: *mut *mut core::ffi::c_void, rstrings: *mut u32, cmax: u32, pcstrings: *mut u32) -> windows_core::Result<()>;
    fn GetParamForMethodIndex(&self, md: u32, ulparamseq: u32, ppd: *mut u32) -> windows_core::Result<()>;
    fn EnumCustomAttributes(&self, phenum: *mut *mut core::ffi::c_void, tk: u32, tktype: u32, rcustomattributes: *mut u32, cmax: u32, pccustomattributes: *mut u32) -> windows_core::Result<()>;
    fn GetCustomAttributeProps(&self, cv: u32, ptkobj: *mut u32, ptktype: *mut u32, ppblob: *const *const core::ffi::c_void, pcbsize: *mut u32) -> windows_core::Result<()>;
    fn FindTypeRef(&self, tkresolutionscope: u32, szname: &windows_core::PCWSTR, ptr: *mut u32) -> windows_core::Result<()>;
    fn GetMemberProps(&self, mb: u32, pclass: *mut u32, szmember: windows_core::PWSTR, cchmember: u32, pchmember: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pulcoderva: *mut u32, pdwimplflags: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut core::ffi::c_void, pcchvalue: *mut u32) -> windows_core::Result<()>;
    fn GetFieldProps(&self, mb: u32, pclass: *mut u32, szfield: windows_core::PWSTR, cchfield: u32, pchfield: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut core::ffi::c_void, pcchvalue: *mut u32) -> windows_core::Result<()>;
    fn GetPropertyProps(&self, prop: u32, pclass: *mut u32, szproperty: &windows_core::PCWSTR, cchproperty: u32, pchproperty: *mut u32, pdwpropflags: *mut u32, ppvsig: *mut *mut u8, pbsig: *mut u32, pdwcplustypeflag: *mut u32, ppdefaultvalue: *mut *mut core::ffi::c_void, pcchdefaultvalue: *mut u32, pmdsetter: *mut u32, pmdgetter: *mut u32, rmdothermethod: *mut u32, cmax: u32, pcothermethod: *mut u32) -> windows_core::Result<()>;
    fn GetParamProps(&self, tk: u32, pmd: *mut u32, pulsequence: *mut u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32, pdwattr: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut core::ffi::c_void, pcchvalue: *mut u32) -> windows_core::Result<()>;
    fn GetCustomAttributeByName(&self, tkobj: u32, szname: &windows_core::PCWSTR, ppdata: *const *const core::ffi::c_void, pcbdata: *mut u32) -> windows_core::Result<()>;
    fn IsValidToken(&self, tk: u32) -> super::super::super::Foundation::BOOL;
    fn GetNestedClassProps(&self, tdnestedclass: u32, ptdenclosingclass: *mut u32) -> windows_core::Result<()>;
    fn GetNativeCallConvFromSig(&self, pvsig: *const core::ffi::c_void, cbsig: u32, pcallconv: *mut u32) -> windows_core::Result<()>;
    fn IsGlobal(&self, pd: u32, pbglobal: *mut i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMetaDataImport {}
impl IMetaDataImport_Vtbl {
    pub const fn new<Identity: IMetaDataImport_Impl, const OFFSET: isize>() -> IMetaDataImport_Vtbl {
        unsafe extern "system" fn CloseEnum<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, henum: *mut core::ffi::c_void) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::CloseEnum(this, core::mem::transmute_copy(&henum))
        }
        unsafe extern "system" fn CountEnum<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, henum: *mut core::ffi::c_void, pulcount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::CountEnum(this, core::mem::transmute_copy(&henum), core::mem::transmute_copy(&pulcount)).into()
        }
        unsafe extern "system" fn ResetEnum<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, henum: *mut core::ffi::c_void, ulpos: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::ResetEnum(this, core::mem::transmute_copy(&henum), core::mem::transmute_copy(&ulpos)).into()
        }
        unsafe extern "system" fn EnumTypeDefs<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, rtypedefs: *mut u32, cmax: u32, pctypedefs: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumTypeDefs(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&rtypedefs), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctypedefs)).into()
        }
        unsafe extern "system" fn EnumInterfaceImpls<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, td: u32, rimpls: *mut u32, cmax: u32, pcimpls: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumInterfaceImpls(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&td), core::mem::transmute_copy(&rimpls), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pcimpls)).into()
        }
        unsafe extern "system" fn EnumTypeRefs<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, rtyperefs: *mut u32, cmax: u32, pctyperefs: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumTypeRefs(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&rtyperefs), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctyperefs)).into()
        }
        unsafe extern "system" fn FindTypeDefByName<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztypedef: windows_core::PCWSTR, tkenclosingclass: u32, ptd: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::FindTypeDefByName(this, core::mem::transmute(&sztypedef), core::mem::transmute_copy(&tkenclosingclass), core::mem::transmute_copy(&ptd)).into()
        }
        unsafe extern "system" fn GetScopeProps<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32, pmvid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetScopeProps(this, core::mem::transmute_copy(&szname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pchname), core::mem::transmute_copy(&pmvid)).into()
        }
        unsafe extern "system" fn GetModuleFromScope<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmd: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetModuleFromScope(this, core::mem::transmute_copy(&pmd)).into()
        }
        unsafe extern "system" fn GetTypeDefProps<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, td: u32, sztypedef: windows_core::PWSTR, cchtypedef: u32, pchtypedef: *mut u32, pdwtypedefflags: *mut u32, ptkextends: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetTypeDefProps(this, core::mem::transmute_copy(&td), core::mem::transmute_copy(&sztypedef), core::mem::transmute_copy(&cchtypedef), core::mem::transmute_copy(&pchtypedef), core::mem::transmute_copy(&pdwtypedefflags), core::mem::transmute_copy(&ptkextends)).into()
        }
        unsafe extern "system" fn GetInterfaceImplProps<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iiimpl: u32, pclass: *mut u32, ptkiface: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetInterfaceImplProps(this, core::mem::transmute_copy(&iiimpl), core::mem::transmute_copy(&pclass), core::mem::transmute_copy(&ptkiface)).into()
        }
        unsafe extern "system" fn GetTypeRefProps<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tr: u32, ptkresolutionscope: *mut u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetTypeRefProps(this, core::mem::transmute_copy(&tr), core::mem::transmute_copy(&ptkresolutionscope), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pchname)).into()
        }
        unsafe extern "system" fn ResolveTypeRef<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tr: u32, riid: *const windows_core::GUID, ppiscope: *mut *mut core::ffi::c_void, ptd: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::ResolveTypeRef(this, core::mem::transmute_copy(&tr), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppiscope), core::mem::transmute_copy(&ptd)).into()
        }
        unsafe extern "system" fn EnumMembers<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, cl: u32, rmembers: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumMembers(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&cl), core::mem::transmute_copy(&rmembers), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctokens)).into()
        }
        unsafe extern "system" fn EnumMembersWithName<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, cl: u32, szname: windows_core::PCWSTR, rmembers: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumMembersWithName(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&cl), core::mem::transmute(&szname), core::mem::transmute_copy(&rmembers), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctokens)).into()
        }
        unsafe extern "system" fn EnumMethods<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, cl: u32, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumMethods(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&cl), core::mem::transmute_copy(&rmethods), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctokens)).into()
        }
        unsafe extern "system" fn EnumMethodsWithName<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, cl: u32, szname: windows_core::PCWSTR, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumMethodsWithName(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&cl), core::mem::transmute(&szname), core::mem::transmute_copy(&rmethods), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctokens)).into()
        }
        unsafe extern "system" fn EnumFields<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, cl: u32, rfields: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumFields(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&cl), core::mem::transmute_copy(&rfields), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctokens)).into()
        }
        unsafe extern "system" fn EnumFieldsWithName<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, cl: u32, szname: windows_core::PCWSTR, rfields: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumFieldsWithName(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&cl), core::mem::transmute(&szname), core::mem::transmute_copy(&rfields), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctokens)).into()
        }
        unsafe extern "system" fn EnumParams<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, mb: u32, rparams: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumParams(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&mb), core::mem::transmute_copy(&rparams), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctokens)).into()
        }
        unsafe extern "system" fn EnumMemberRefs<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, tkparent: u32, rmemberrefs: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumMemberRefs(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&tkparent), core::mem::transmute_copy(&rmemberrefs), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctokens)).into()
        }
        unsafe extern "system" fn EnumMethodImpls<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, td: u32, rmethodbody: *mut u32, rmethoddecl: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumMethodImpls(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&td), core::mem::transmute_copy(&rmethodbody), core::mem::transmute_copy(&rmethoddecl), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctokens)).into()
        }
        unsafe extern "system" fn EnumPermissionSets<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, tk: u32, dwactions: u32, rpermission: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumPermissionSets(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&tk), core::mem::transmute_copy(&dwactions), core::mem::transmute_copy(&rpermission), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctokens)).into()
        }
        unsafe extern "system" fn FindMember<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, td: u32, szname: windows_core::PCWSTR, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::FindMember(this, core::mem::transmute_copy(&td), core::mem::transmute(&szname), core::mem::transmute_copy(&pvsigblob), core::mem::transmute_copy(&cbsigblob), core::mem::transmute_copy(&pmb)).into()
        }
        unsafe extern "system" fn FindMethod<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, td: u32, szname: windows_core::PCWSTR, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::FindMethod(this, core::mem::transmute_copy(&td), core::mem::transmute(&szname), core::mem::transmute_copy(&pvsigblob), core::mem::transmute_copy(&cbsigblob), core::mem::transmute_copy(&pmb)).into()
        }
        unsafe extern "system" fn FindField<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, td: u32, szname: windows_core::PCWSTR, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::FindField(this, core::mem::transmute_copy(&td), core::mem::transmute(&szname), core::mem::transmute_copy(&pvsigblob), core::mem::transmute_copy(&cbsigblob), core::mem::transmute_copy(&pmb)).into()
        }
        unsafe extern "system" fn FindMemberRef<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, td: u32, szname: windows_core::PCWSTR, pvsigblob: *mut u8, cbsigblob: u32, pmr: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::FindMemberRef(this, core::mem::transmute_copy(&td), core::mem::transmute(&szname), core::mem::transmute_copy(&pvsigblob), core::mem::transmute_copy(&cbsigblob), core::mem::transmute_copy(&pmr)).into()
        }
        unsafe extern "system" fn GetMethodProps<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mb: u32, pclass: *mut u32, szmethod: windows_core::PWSTR, cchmethod: u32, pchmethod: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pulcoderva: *mut u32, pdwimplflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetMethodProps(this, core::mem::transmute_copy(&mb), core::mem::transmute_copy(&pclass), core::mem::transmute_copy(&szmethod), core::mem::transmute_copy(&cchmethod), core::mem::transmute_copy(&pchmethod), core::mem::transmute_copy(&pdwattr), core::mem::transmute_copy(&ppvsigblob), core::mem::transmute_copy(&pcbsigblob), core::mem::transmute_copy(&pulcoderva), core::mem::transmute_copy(&pdwimplflags)).into()
        }
        unsafe extern "system" fn GetMemberRefProps<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mr: u32, ptk: *mut u32, szmember: windows_core::PWSTR, cchmember: u32, pchmember: *mut u32, ppvsigblob: *mut *mut u8, pbsig: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetMemberRefProps(this, core::mem::transmute_copy(&mr), core::mem::transmute_copy(&ptk), core::mem::transmute_copy(&szmember), core::mem::transmute_copy(&cchmember), core::mem::transmute_copy(&pchmember), core::mem::transmute_copy(&ppvsigblob), core::mem::transmute_copy(&pbsig)).into()
        }
        unsafe extern "system" fn EnumProperties<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, td: u32, rproperties: *mut u32, cmax: u32, pcproperties: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumProperties(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&td), core::mem::transmute_copy(&rproperties), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pcproperties)).into()
        }
        unsafe extern "system" fn EnumEvents<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, td: u32, revents: *mut u32, cmax: u32, pcevents: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumEvents(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&td), core::mem::transmute_copy(&revents), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pcevents)).into()
        }
        unsafe extern "system" fn GetEventProps<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ev: u32, pclass: *mut u32, szevent: windows_core::PCWSTR, cchevent: u32, pchevent: *mut u32, pdweventflags: *mut u32, ptkeventtype: *mut u32, pmdaddon: *mut u32, pmdremoveon: *mut u32, pmdfire: *mut u32, rmdothermethod: *mut u32, cmax: u32, pcothermethod: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetEventProps(
                this,
                core::mem::transmute_copy(&ev),
                core::mem::transmute_copy(&pclass),
                core::mem::transmute(&szevent),
                core::mem::transmute_copy(&cchevent),
                core::mem::transmute_copy(&pchevent),
                core::mem::transmute_copy(&pdweventflags),
                core::mem::transmute_copy(&ptkeventtype),
                core::mem::transmute_copy(&pmdaddon),
                core::mem::transmute_copy(&pmdremoveon),
                core::mem::transmute_copy(&pmdfire),
                core::mem::transmute_copy(&rmdothermethod),
                core::mem::transmute_copy(&cmax),
                core::mem::transmute_copy(&pcothermethod),
            )
            .into()
        }
        unsafe extern "system" fn EnumMethodSemantics<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, mb: u32, reventprop: *mut u32, cmax: u32, pceventprop: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumMethodSemantics(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&mb), core::mem::transmute_copy(&reventprop), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pceventprop)).into()
        }
        unsafe extern "system" fn GetMethodSemantics<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mb: u32, tkeventprop: u32, pdwsemanticsflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetMethodSemantics(this, core::mem::transmute_copy(&mb), core::mem::transmute_copy(&tkeventprop), core::mem::transmute_copy(&pdwsemanticsflags)).into()
        }
        unsafe extern "system" fn GetClassLayout<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, td: u32, pdwpacksize: *mut u32, rfieldoffset: *mut COR_FIELD_OFFSET, cmax: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetClassLayout(this, core::mem::transmute_copy(&td), core::mem::transmute_copy(&pdwpacksize), core::mem::transmute_copy(&rfieldoffset), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pcfieldoffset), core::mem::transmute_copy(&pulclasssize)).into()
        }
        unsafe extern "system" fn GetFieldMarshal<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tk: u32, ppvnativetype: *mut *mut u8, pcbnativetype: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetFieldMarshal(this, core::mem::transmute_copy(&tk), core::mem::transmute_copy(&ppvnativetype), core::mem::transmute_copy(&pcbnativetype)).into()
        }
        unsafe extern "system" fn GetRVA<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tk: u32, pulcoderva: *mut u32, pdwimplflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetRVA(this, core::mem::transmute_copy(&tk), core::mem::transmute_copy(&pulcoderva), core::mem::transmute_copy(&pdwimplflags)).into()
        }
        unsafe extern "system" fn GetPermissionSetProps<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pm: u32, pdwaction: *mut u32, ppvpermission: *const *const core::ffi::c_void, pcbpermission: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetPermissionSetProps(this, core::mem::transmute_copy(&pm), core::mem::transmute_copy(&pdwaction), core::mem::transmute_copy(&ppvpermission), core::mem::transmute_copy(&pcbpermission)).into()
        }
        unsafe extern "system" fn GetSigFromToken<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mdsig: u32, ppvsig: *mut *mut u8, pcbsig: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetSigFromToken(this, core::mem::transmute_copy(&mdsig), core::mem::transmute_copy(&ppvsig), core::mem::transmute_copy(&pcbsig)).into()
        }
        unsafe extern "system" fn GetModuleRefProps<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mur: u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetModuleRefProps(this, core::mem::transmute_copy(&mur), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pchname)).into()
        }
        unsafe extern "system" fn EnumModuleRefs<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, rmodulerefs: *mut u32, cmax: u32, pcmodulerefs: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumModuleRefs(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&rmodulerefs), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pcmodulerefs)).into()
        }
        unsafe extern "system" fn GetTypeSpecFromToken<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, typespec: u32, ppvsig: *mut *mut u8, pcbsig: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetTypeSpecFromToken(this, core::mem::transmute_copy(&typespec), core::mem::transmute_copy(&ppvsig), core::mem::transmute_copy(&pcbsig)).into()
        }
        unsafe extern "system" fn GetNameFromToken<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tk: u32, pszutf8nameptr: *mut *mut i8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetNameFromToken(this, core::mem::transmute_copy(&tk), core::mem::transmute_copy(&pszutf8nameptr)).into()
        }
        unsafe extern "system" fn EnumUnresolvedMethods<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumUnresolvedMethods(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&rmethods), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctokens)).into()
        }
        unsafe extern "system" fn GetUserString<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stk: u32, szstring: windows_core::PWSTR, cchstring: u32, pchstring: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetUserString(this, core::mem::transmute_copy(&stk), core::mem::transmute_copy(&szstring), core::mem::transmute_copy(&cchstring), core::mem::transmute_copy(&pchstring)).into()
        }
        unsafe extern "system" fn GetPinvokeMap<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tk: u32, pdwmappingflags: *mut u32, szimportname: windows_core::PWSTR, cchimportname: u32, pchimportname: *mut u32, pmrimportdll: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetPinvokeMap(this, core::mem::transmute_copy(&tk), core::mem::transmute_copy(&pdwmappingflags), core::mem::transmute_copy(&szimportname), core::mem::transmute_copy(&cchimportname), core::mem::transmute_copy(&pchimportname), core::mem::transmute_copy(&pmrimportdll)).into()
        }
        unsafe extern "system" fn EnumSignatures<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, rsignatures: *mut u32, cmax: u32, pcsignatures: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumSignatures(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&rsignatures), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pcsignatures)).into()
        }
        unsafe extern "system" fn EnumTypeSpecs<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, rtypespecs: *mut u32, cmax: u32, pctypespecs: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumTypeSpecs(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&rtypespecs), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pctypespecs)).into()
        }
        unsafe extern "system" fn EnumUserStrings<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, rstrings: *mut u32, cmax: u32, pcstrings: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumUserStrings(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&rstrings), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pcstrings)).into()
        }
        unsafe extern "system" fn GetParamForMethodIndex<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, md: u32, ulparamseq: u32, ppd: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetParamForMethodIndex(this, core::mem::transmute_copy(&md), core::mem::transmute_copy(&ulparamseq), core::mem::transmute_copy(&ppd)).into()
        }
        unsafe extern "system" fn EnumCustomAttributes<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, tk: u32, tktype: u32, rcustomattributes: *mut u32, cmax: u32, pccustomattributes: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::EnumCustomAttributes(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&tk), core::mem::transmute_copy(&tktype), core::mem::transmute_copy(&rcustomattributes), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pccustomattributes)).into()
        }
        unsafe extern "system" fn GetCustomAttributeProps<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cv: u32, ptkobj: *mut u32, ptktype: *mut u32, ppblob: *const *const core::ffi::c_void, pcbsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetCustomAttributeProps(this, core::mem::transmute_copy(&cv), core::mem::transmute_copy(&ptkobj), core::mem::transmute_copy(&ptktype), core::mem::transmute_copy(&ppblob), core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn FindTypeRef<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tkresolutionscope: u32, szname: windows_core::PCWSTR, ptr: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::FindTypeRef(this, core::mem::transmute_copy(&tkresolutionscope), core::mem::transmute(&szname), core::mem::transmute_copy(&ptr)).into()
        }
        unsafe extern "system" fn GetMemberProps<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mb: u32, pclass: *mut u32, szmember: windows_core::PWSTR, cchmember: u32, pchmember: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pulcoderva: *mut u32, pdwimplflags: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut core::ffi::c_void, pcchvalue: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetMemberProps(
                this,
                core::mem::transmute_copy(&mb),
                core::mem::transmute_copy(&pclass),
                core::mem::transmute_copy(&szmember),
                core::mem::transmute_copy(&cchmember),
                core::mem::transmute_copy(&pchmember),
                core::mem::transmute_copy(&pdwattr),
                core::mem::transmute_copy(&ppvsigblob),
                core::mem::transmute_copy(&pcbsigblob),
                core::mem::transmute_copy(&pulcoderva),
                core::mem::transmute_copy(&pdwimplflags),
                core::mem::transmute_copy(&pdwcplustypeflag),
                core::mem::transmute_copy(&ppvalue),
                core::mem::transmute_copy(&pcchvalue),
            )
            .into()
        }
        unsafe extern "system" fn GetFieldProps<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mb: u32, pclass: *mut u32, szfield: windows_core::PWSTR, cchfield: u32, pchfield: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut core::ffi::c_void, pcchvalue: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetFieldProps(this, core::mem::transmute_copy(&mb), core::mem::transmute_copy(&pclass), core::mem::transmute_copy(&szfield), core::mem::transmute_copy(&cchfield), core::mem::transmute_copy(&pchfield), core::mem::transmute_copy(&pdwattr), core::mem::transmute_copy(&ppvsigblob), core::mem::transmute_copy(&pcbsigblob), core::mem::transmute_copy(&pdwcplustypeflag), core::mem::transmute_copy(&ppvalue), core::mem::transmute_copy(&pcchvalue)).into()
        }
        unsafe extern "system" fn GetPropertyProps<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prop: u32, pclass: *mut u32, szproperty: windows_core::PCWSTR, cchproperty: u32, pchproperty: *mut u32, pdwpropflags: *mut u32, ppvsig: *mut *mut u8, pbsig: *mut u32, pdwcplustypeflag: *mut u32, ppdefaultvalue: *mut *mut core::ffi::c_void, pcchdefaultvalue: *mut u32, pmdsetter: *mut u32, pmdgetter: *mut u32, rmdothermethod: *mut u32, cmax: u32, pcothermethod: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetPropertyProps(
                this,
                core::mem::transmute_copy(&prop),
                core::mem::transmute_copy(&pclass),
                core::mem::transmute(&szproperty),
                core::mem::transmute_copy(&cchproperty),
                core::mem::transmute_copy(&pchproperty),
                core::mem::transmute_copy(&pdwpropflags),
                core::mem::transmute_copy(&ppvsig),
                core::mem::transmute_copy(&pbsig),
                core::mem::transmute_copy(&pdwcplustypeflag),
                core::mem::transmute_copy(&ppdefaultvalue),
                core::mem::transmute_copy(&pcchdefaultvalue),
                core::mem::transmute_copy(&pmdsetter),
                core::mem::transmute_copy(&pmdgetter),
                core::mem::transmute_copy(&rmdothermethod),
                core::mem::transmute_copy(&cmax),
                core::mem::transmute_copy(&pcothermethod),
            )
            .into()
        }
        unsafe extern "system" fn GetParamProps<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tk: u32, pmd: *mut u32, pulsequence: *mut u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32, pdwattr: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut core::ffi::c_void, pcchvalue: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetParamProps(this, core::mem::transmute_copy(&tk), core::mem::transmute_copy(&pmd), core::mem::transmute_copy(&pulsequence), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pchname), core::mem::transmute_copy(&pdwattr), core::mem::transmute_copy(&pdwcplustypeflag), core::mem::transmute_copy(&ppvalue), core::mem::transmute_copy(&pcchvalue)).into()
        }
        unsafe extern "system" fn GetCustomAttributeByName<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tkobj: u32, szname: windows_core::PCWSTR, ppdata: *const *const core::ffi::c_void, pcbdata: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetCustomAttributeByName(this, core::mem::transmute_copy(&tkobj), core::mem::transmute(&szname), core::mem::transmute_copy(&ppdata), core::mem::transmute_copy(&pcbdata)).into()
        }
        unsafe extern "system" fn IsValidToken<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tk: u32) -> super::super::super::Foundation::BOOL {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::IsValidToken(this, core::mem::transmute_copy(&tk))
        }
        unsafe extern "system" fn GetNestedClassProps<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tdnestedclass: u32, ptdenclosingclass: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetNestedClassProps(this, core::mem::transmute_copy(&tdnestedclass), core::mem::transmute_copy(&ptdenclosingclass)).into()
        }
        unsafe extern "system" fn GetNativeCallConvFromSig<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvsig: *const core::ffi::c_void, cbsig: u32, pcallconv: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::GetNativeCallConvFromSig(this, core::mem::transmute_copy(&pvsig), core::mem::transmute_copy(&cbsig), core::mem::transmute_copy(&pcallconv)).into()
        }
        unsafe extern "system" fn IsGlobal<Identity: IMetaDataImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pd: u32, pbglobal: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport_Impl::IsGlobal(this, core::mem::transmute_copy(&pd), core::mem::transmute_copy(&pbglobal)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CloseEnum: CloseEnum::<Identity, OFFSET>,
            CountEnum: CountEnum::<Identity, OFFSET>,
            ResetEnum: ResetEnum::<Identity, OFFSET>,
            EnumTypeDefs: EnumTypeDefs::<Identity, OFFSET>,
            EnumInterfaceImpls: EnumInterfaceImpls::<Identity, OFFSET>,
            EnumTypeRefs: EnumTypeRefs::<Identity, OFFSET>,
            FindTypeDefByName: FindTypeDefByName::<Identity, OFFSET>,
            GetScopeProps: GetScopeProps::<Identity, OFFSET>,
            GetModuleFromScope: GetModuleFromScope::<Identity, OFFSET>,
            GetTypeDefProps: GetTypeDefProps::<Identity, OFFSET>,
            GetInterfaceImplProps: GetInterfaceImplProps::<Identity, OFFSET>,
            GetTypeRefProps: GetTypeRefProps::<Identity, OFFSET>,
            ResolveTypeRef: ResolveTypeRef::<Identity, OFFSET>,
            EnumMembers: EnumMembers::<Identity, OFFSET>,
            EnumMembersWithName: EnumMembersWithName::<Identity, OFFSET>,
            EnumMethods: EnumMethods::<Identity, OFFSET>,
            EnumMethodsWithName: EnumMethodsWithName::<Identity, OFFSET>,
            EnumFields: EnumFields::<Identity, OFFSET>,
            EnumFieldsWithName: EnumFieldsWithName::<Identity, OFFSET>,
            EnumParams: EnumParams::<Identity, OFFSET>,
            EnumMemberRefs: EnumMemberRefs::<Identity, OFFSET>,
            EnumMethodImpls: EnumMethodImpls::<Identity, OFFSET>,
            EnumPermissionSets: EnumPermissionSets::<Identity, OFFSET>,
            FindMember: FindMember::<Identity, OFFSET>,
            FindMethod: FindMethod::<Identity, OFFSET>,
            FindField: FindField::<Identity, OFFSET>,
            FindMemberRef: FindMemberRef::<Identity, OFFSET>,
            GetMethodProps: GetMethodProps::<Identity, OFFSET>,
            GetMemberRefProps: GetMemberRefProps::<Identity, OFFSET>,
            EnumProperties: EnumProperties::<Identity, OFFSET>,
            EnumEvents: EnumEvents::<Identity, OFFSET>,
            GetEventProps: GetEventProps::<Identity, OFFSET>,
            EnumMethodSemantics: EnumMethodSemantics::<Identity, OFFSET>,
            GetMethodSemantics: GetMethodSemantics::<Identity, OFFSET>,
            GetClassLayout: GetClassLayout::<Identity, OFFSET>,
            GetFieldMarshal: GetFieldMarshal::<Identity, OFFSET>,
            GetRVA: GetRVA::<Identity, OFFSET>,
            GetPermissionSetProps: GetPermissionSetProps::<Identity, OFFSET>,
            GetSigFromToken: GetSigFromToken::<Identity, OFFSET>,
            GetModuleRefProps: GetModuleRefProps::<Identity, OFFSET>,
            EnumModuleRefs: EnumModuleRefs::<Identity, OFFSET>,
            GetTypeSpecFromToken: GetTypeSpecFromToken::<Identity, OFFSET>,
            GetNameFromToken: GetNameFromToken::<Identity, OFFSET>,
            EnumUnresolvedMethods: EnumUnresolvedMethods::<Identity, OFFSET>,
            GetUserString: GetUserString::<Identity, OFFSET>,
            GetPinvokeMap: GetPinvokeMap::<Identity, OFFSET>,
            EnumSignatures: EnumSignatures::<Identity, OFFSET>,
            EnumTypeSpecs: EnumTypeSpecs::<Identity, OFFSET>,
            EnumUserStrings: EnumUserStrings::<Identity, OFFSET>,
            GetParamForMethodIndex: GetParamForMethodIndex::<Identity, OFFSET>,
            EnumCustomAttributes: EnumCustomAttributes::<Identity, OFFSET>,
            GetCustomAttributeProps: GetCustomAttributeProps::<Identity, OFFSET>,
            FindTypeRef: FindTypeRef::<Identity, OFFSET>,
            GetMemberProps: GetMemberProps::<Identity, OFFSET>,
            GetFieldProps: GetFieldProps::<Identity, OFFSET>,
            GetPropertyProps: GetPropertyProps::<Identity, OFFSET>,
            GetParamProps: GetParamProps::<Identity, OFFSET>,
            GetCustomAttributeByName: GetCustomAttributeByName::<Identity, OFFSET>,
            IsValidToken: IsValidToken::<Identity, OFFSET>,
            GetNestedClassProps: GetNestedClassProps::<Identity, OFFSET>,
            GetNativeCallConvFromSig: GetNativeCallConvFromSig::<Identity, OFFSET>,
            IsGlobal: IsGlobal::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMetaDataImport as windows_core::Interface>::IID
    }
}
pub trait IMetaDataImport2_Impl: Sized + IMetaDataImport_Impl {
    fn EnumGenericParams(&self, phenum: *mut *mut core::ffi::c_void, tk: u32, rgenericparams: *mut u32, cmax: u32, pcgenericparams: *mut u32) -> windows_core::Result<()>;
    fn GetGenericParamProps(&self, gp: u32, pulparamseq: *mut u32, pdwparamflags: *mut u32, ptowner: *mut u32, reserved: *mut u32, wzname: windows_core::PWSTR, cchname: u32, pchname: *mut u32) -> windows_core::Result<()>;
    fn GetMethodSpecProps(&self, mi: u32, tkparent: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32) -> windows_core::Result<()>;
    fn EnumGenericParamConstraints(&self, phenum: *mut *mut core::ffi::c_void, tk: u32, rgenericparamconstraints: *mut u32, cmax: u32, pcgenericparamconstraints: *mut u32) -> windows_core::Result<()>;
    fn GetGenericParamConstraintProps(&self, gpc: u32, ptgenericparam: *mut u32, ptkconstrainttype: *mut u32) -> windows_core::Result<()>;
    fn GetPEKind(&self, pdwpekind: *mut u32, pdwmachine: *mut u32) -> windows_core::Result<()>;
    fn GetVersionString(&self, pwzbuf: windows_core::PWSTR, ccbufsize: u32, pccbufsize: *mut u32) -> windows_core::Result<()>;
    fn EnumMethodSpecs(&self, phenum: *mut *mut core::ffi::c_void, tk: u32, rmethodspecs: *mut u32, cmax: u32, pcmethodspecs: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMetaDataImport2 {}
impl IMetaDataImport2_Vtbl {
    pub const fn new<Identity: IMetaDataImport2_Impl, const OFFSET: isize>() -> IMetaDataImport2_Vtbl {
        unsafe extern "system" fn EnumGenericParams<Identity: IMetaDataImport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, tk: u32, rgenericparams: *mut u32, cmax: u32, pcgenericparams: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport2_Impl::EnumGenericParams(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&tk), core::mem::transmute_copy(&rgenericparams), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pcgenericparams)).into()
        }
        unsafe extern "system" fn GetGenericParamProps<Identity: IMetaDataImport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gp: u32, pulparamseq: *mut u32, pdwparamflags: *mut u32, ptowner: *mut u32, reserved: *mut u32, wzname: windows_core::PWSTR, cchname: u32, pchname: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport2_Impl::GetGenericParamProps(this, core::mem::transmute_copy(&gp), core::mem::transmute_copy(&pulparamseq), core::mem::transmute_copy(&pdwparamflags), core::mem::transmute_copy(&ptowner), core::mem::transmute_copy(&reserved), core::mem::transmute_copy(&wzname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pchname)).into()
        }
        unsafe extern "system" fn GetMethodSpecProps<Identity: IMetaDataImport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mi: u32, tkparent: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport2_Impl::GetMethodSpecProps(this, core::mem::transmute_copy(&mi), core::mem::transmute_copy(&tkparent), core::mem::transmute_copy(&ppvsigblob), core::mem::transmute_copy(&pcbsigblob)).into()
        }
        unsafe extern "system" fn EnumGenericParamConstraints<Identity: IMetaDataImport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, tk: u32, rgenericparamconstraints: *mut u32, cmax: u32, pcgenericparamconstraints: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport2_Impl::EnumGenericParamConstraints(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&tk), core::mem::transmute_copy(&rgenericparamconstraints), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pcgenericparamconstraints)).into()
        }
        unsafe extern "system" fn GetGenericParamConstraintProps<Identity: IMetaDataImport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpc: u32, ptgenericparam: *mut u32, ptkconstrainttype: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport2_Impl::GetGenericParamConstraintProps(this, core::mem::transmute_copy(&gpc), core::mem::transmute_copy(&ptgenericparam), core::mem::transmute_copy(&ptkconstrainttype)).into()
        }
        unsafe extern "system" fn GetPEKind<Identity: IMetaDataImport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpekind: *mut u32, pdwmachine: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport2_Impl::GetPEKind(this, core::mem::transmute_copy(&pdwpekind), core::mem::transmute_copy(&pdwmachine)).into()
        }
        unsafe extern "system" fn GetVersionString<Identity: IMetaDataImport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzbuf: windows_core::PWSTR, ccbufsize: u32, pccbufsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport2_Impl::GetVersionString(this, core::mem::transmute_copy(&pwzbuf), core::mem::transmute_copy(&ccbufsize), core::mem::transmute_copy(&pccbufsize)).into()
        }
        unsafe extern "system" fn EnumMethodSpecs<Identity: IMetaDataImport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phenum: *mut *mut core::ffi::c_void, tk: u32, rmethodspecs: *mut u32, cmax: u32, pcmethodspecs: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataImport2_Impl::EnumMethodSpecs(this, core::mem::transmute_copy(&phenum), core::mem::transmute_copy(&tk), core::mem::transmute_copy(&rmethodspecs), core::mem::transmute_copy(&cmax), core::mem::transmute_copy(&pcmethodspecs)).into()
        }
        Self {
            base__: IMetaDataImport_Vtbl::new::<Identity, OFFSET>(),
            EnumGenericParams: EnumGenericParams::<Identity, OFFSET>,
            GetGenericParamProps: GetGenericParamProps::<Identity, OFFSET>,
            GetMethodSpecProps: GetMethodSpecProps::<Identity, OFFSET>,
            EnumGenericParamConstraints: EnumGenericParamConstraints::<Identity, OFFSET>,
            GetGenericParamConstraintProps: GetGenericParamConstraintProps::<Identity, OFFSET>,
            GetPEKind: GetPEKind::<Identity, OFFSET>,
            GetVersionString: GetVersionString::<Identity, OFFSET>,
            EnumMethodSpecs: EnumMethodSpecs::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMetaDataImport2 as windows_core::Interface>::IID || iid == &<IMetaDataImport as windows_core::Interface>::IID
    }
}
pub trait IMetaDataInfo_Impl: Sized + windows_core::IUnknownImpl {
    fn GetFileMapping(&self, ppvdata: *const *const core::ffi::c_void, pcbdata: *mut u64, pdwmappingtype: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMetaDataInfo {}
impl IMetaDataInfo_Vtbl {
    pub const fn new<Identity: IMetaDataInfo_Impl, const OFFSET: isize>() -> IMetaDataInfo_Vtbl {
        unsafe extern "system" fn GetFileMapping<Identity: IMetaDataInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvdata: *const *const core::ffi::c_void, pcbdata: *mut u64, pdwmappingtype: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataInfo_Impl::GetFileMapping(this, core::mem::transmute_copy(&ppvdata), core::mem::transmute_copy(&pcbdata), core::mem::transmute_copy(&pdwmappingtype)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetFileMapping: GetFileMapping::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMetaDataInfo as windows_core::Interface>::IID
    }
}
pub trait IMetaDataTables_Impl: Sized + windows_core::IUnknownImpl {
    fn GetStringHeapSize(&self, pcbstrings: *mut u32) -> windows_core::Result<()>;
    fn GetBlobHeapSize(&self, pcbblobs: *mut u32) -> windows_core::Result<()>;
    fn GetGuidHeapSize(&self, pcbguids: *mut u32) -> windows_core::Result<()>;
    fn GetUserStringHeapSize(&self, pcbblobs: *mut u32) -> windows_core::Result<()>;
    fn GetNumTables(&self, pctables: *mut u32) -> windows_core::Result<()>;
    fn GetTableIndex(&self, token: u32, pixtbl: *mut u32) -> windows_core::Result<()>;
    fn GetTableInfo(&self, ixtbl: u32, pcbrow: *mut u32, pcrows: *mut u32, pccols: *mut u32, pikey: *mut u32, ppname: *const *const i8) -> windows_core::Result<()>;
    fn GetColumnInfo(&self, ixtbl: u32, ixcol: u32, pocol: *mut u32, pcbcol: *mut u32, ptype: *mut u32, ppname: *const *const i8) -> windows_core::Result<()>;
    fn GetCodedTokenInfo(&self, ixcdtkn: u32, pctokens: *mut u32, pptokens: *mut *mut u32, ppname: *const *const i8) -> windows_core::Result<()>;
    fn GetRow(&self, ixtbl: u32, rid: u32, pprow: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetColumn(&self, ixtbl: u32, ixcol: u32, rid: u32, pval: *mut u32) -> windows_core::Result<()>;
    fn GetString(&self, ixstring: u32, ppstring: *const *const i8) -> windows_core::Result<()>;
    fn GetBlob(&self, ixblob: u32, pcbdata: *mut u32, ppdata: *const *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetGuid(&self, ixguid: u32, ppguid: *const *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetUserString(&self, ixuserstring: u32, pcbdata: *mut u32, ppdata: *const *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetNextString(&self, ixstring: u32, pnext: *mut u32) -> windows_core::Result<()>;
    fn GetNextBlob(&self, ixblob: u32, pnext: *mut u32) -> windows_core::Result<()>;
    fn GetNextGuid(&self, ixguid: u32, pnext: *mut u32) -> windows_core::Result<()>;
    fn GetNextUserString(&self, ixuserstring: u32, pnext: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMetaDataTables {}
impl IMetaDataTables_Vtbl {
    pub const fn new<Identity: IMetaDataTables_Impl, const OFFSET: isize>() -> IMetaDataTables_Vtbl {
        unsafe extern "system" fn GetStringHeapSize<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbstrings: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetStringHeapSize(this, core::mem::transmute_copy(&pcbstrings)).into()
        }
        unsafe extern "system" fn GetBlobHeapSize<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbblobs: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetBlobHeapSize(this, core::mem::transmute_copy(&pcbblobs)).into()
        }
        unsafe extern "system" fn GetGuidHeapSize<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbguids: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetGuidHeapSize(this, core::mem::transmute_copy(&pcbguids)).into()
        }
        unsafe extern "system" fn GetUserStringHeapSize<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbblobs: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetUserStringHeapSize(this, core::mem::transmute_copy(&pcbblobs)).into()
        }
        unsafe extern "system" fn GetNumTables<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctables: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetNumTables(this, core::mem::transmute_copy(&pctables)).into()
        }
        unsafe extern "system" fn GetTableIndex<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: u32, pixtbl: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetTableIndex(this, core::mem::transmute_copy(&token), core::mem::transmute_copy(&pixtbl)).into()
        }
        unsafe extern "system" fn GetTableInfo<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ixtbl: u32, pcbrow: *mut u32, pcrows: *mut u32, pccols: *mut u32, pikey: *mut u32, ppname: *const *const i8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetTableInfo(this, core::mem::transmute_copy(&ixtbl), core::mem::transmute_copy(&pcbrow), core::mem::transmute_copy(&pcrows), core::mem::transmute_copy(&pccols), core::mem::transmute_copy(&pikey), core::mem::transmute_copy(&ppname)).into()
        }
        unsafe extern "system" fn GetColumnInfo<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ixtbl: u32, ixcol: u32, pocol: *mut u32, pcbcol: *mut u32, ptype: *mut u32, ppname: *const *const i8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetColumnInfo(this, core::mem::transmute_copy(&ixtbl), core::mem::transmute_copy(&ixcol), core::mem::transmute_copy(&pocol), core::mem::transmute_copy(&pcbcol), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&ppname)).into()
        }
        unsafe extern "system" fn GetCodedTokenInfo<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ixcdtkn: u32, pctokens: *mut u32, pptokens: *mut *mut u32, ppname: *const *const i8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetCodedTokenInfo(this, core::mem::transmute_copy(&ixcdtkn), core::mem::transmute_copy(&pctokens), core::mem::transmute_copy(&pptokens), core::mem::transmute_copy(&ppname)).into()
        }
        unsafe extern "system" fn GetRow<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ixtbl: u32, rid: u32, pprow: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetRow(this, core::mem::transmute_copy(&ixtbl), core::mem::transmute_copy(&rid), core::mem::transmute_copy(&pprow)).into()
        }
        unsafe extern "system" fn GetColumn<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ixtbl: u32, ixcol: u32, rid: u32, pval: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetColumn(this, core::mem::transmute_copy(&ixtbl), core::mem::transmute_copy(&ixcol), core::mem::transmute_copy(&rid), core::mem::transmute_copy(&pval)).into()
        }
        unsafe extern "system" fn GetString<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ixstring: u32, ppstring: *const *const i8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetString(this, core::mem::transmute_copy(&ixstring), core::mem::transmute_copy(&ppstring)).into()
        }
        unsafe extern "system" fn GetBlob<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ixblob: u32, pcbdata: *mut u32, ppdata: *const *const core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetBlob(this, core::mem::transmute_copy(&ixblob), core::mem::transmute_copy(&pcbdata), core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn GetGuid<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ixguid: u32, ppguid: *const *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetGuid(this, core::mem::transmute_copy(&ixguid), core::mem::transmute_copy(&ppguid)).into()
        }
        unsafe extern "system" fn GetUserString<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ixuserstring: u32, pcbdata: *mut u32, ppdata: *const *const core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetUserString(this, core::mem::transmute_copy(&ixuserstring), core::mem::transmute_copy(&pcbdata), core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn GetNextString<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ixstring: u32, pnext: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetNextString(this, core::mem::transmute_copy(&ixstring), core::mem::transmute_copy(&pnext)).into()
        }
        unsafe extern "system" fn GetNextBlob<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ixblob: u32, pnext: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetNextBlob(this, core::mem::transmute_copy(&ixblob), core::mem::transmute_copy(&pnext)).into()
        }
        unsafe extern "system" fn GetNextGuid<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ixguid: u32, pnext: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetNextGuid(this, core::mem::transmute_copy(&ixguid), core::mem::transmute_copy(&pnext)).into()
        }
        unsafe extern "system" fn GetNextUserString<Identity: IMetaDataTables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ixuserstring: u32, pnext: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables_Impl::GetNextUserString(this, core::mem::transmute_copy(&ixuserstring), core::mem::transmute_copy(&pnext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStringHeapSize: GetStringHeapSize::<Identity, OFFSET>,
            GetBlobHeapSize: GetBlobHeapSize::<Identity, OFFSET>,
            GetGuidHeapSize: GetGuidHeapSize::<Identity, OFFSET>,
            GetUserStringHeapSize: GetUserStringHeapSize::<Identity, OFFSET>,
            GetNumTables: GetNumTables::<Identity, OFFSET>,
            GetTableIndex: GetTableIndex::<Identity, OFFSET>,
            GetTableInfo: GetTableInfo::<Identity, OFFSET>,
            GetColumnInfo: GetColumnInfo::<Identity, OFFSET>,
            GetCodedTokenInfo: GetCodedTokenInfo::<Identity, OFFSET>,
            GetRow: GetRow::<Identity, OFFSET>,
            GetColumn: GetColumn::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            GetBlob: GetBlob::<Identity, OFFSET>,
            GetGuid: GetGuid::<Identity, OFFSET>,
            GetUserString: GetUserString::<Identity, OFFSET>,
            GetNextString: GetNextString::<Identity, OFFSET>,
            GetNextBlob: GetNextBlob::<Identity, OFFSET>,
            GetNextGuid: GetNextGuid::<Identity, OFFSET>,
            GetNextUserString: GetNextUserString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMetaDataTables as windows_core::Interface>::IID
    }
}
pub trait IMetaDataTables2_Impl: Sized + IMetaDataTables_Impl {
    fn GetMetaDataStorage(&self, ppvmd: *const *const core::ffi::c_void, pcbmd: *mut u32) -> windows_core::Result<()>;
    fn GetMetaDataStreamInfo(&self, ix: u32, ppchname: *const *const i8, ppv: *const *const core::ffi::c_void, pcb: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMetaDataTables2 {}
impl IMetaDataTables2_Vtbl {
    pub const fn new<Identity: IMetaDataTables2_Impl, const OFFSET: isize>() -> IMetaDataTables2_Vtbl {
        unsafe extern "system" fn GetMetaDataStorage<Identity: IMetaDataTables2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvmd: *const *const core::ffi::c_void, pcbmd: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables2_Impl::GetMetaDataStorage(this, core::mem::transmute_copy(&ppvmd), core::mem::transmute_copy(&pcbmd)).into()
        }
        unsafe extern "system" fn GetMetaDataStreamInfo<Identity: IMetaDataTables2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ix: u32, ppchname: *const *const i8, ppv: *const *const core::ffi::c_void, pcb: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataTables2_Impl::GetMetaDataStreamInfo(this, core::mem::transmute_copy(&ix), core::mem::transmute_copy(&ppchname), core::mem::transmute_copy(&ppv), core::mem::transmute_copy(&pcb)).into()
        }
        Self {
            base__: IMetaDataTables_Vtbl::new::<Identity, OFFSET>(),
            GetMetaDataStorage: GetMetaDataStorage::<Identity, OFFSET>,
            GetMetaDataStreamInfo: GetMetaDataStreamInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMetaDataTables2 as windows_core::Interface>::IID || iid == &<IMetaDataTables as windows_core::Interface>::IID
    }
}
pub trait IMetaDataValidate_Impl: Sized + windows_core::IUnknownImpl {
    fn ValidatorInit(&self, dwmoduletype: u32, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ValidateMetaData(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMetaDataValidate {}
impl IMetaDataValidate_Vtbl {
    pub const fn new<Identity: IMetaDataValidate_Impl, const OFFSET: isize>() -> IMetaDataValidate_Vtbl {
        unsafe extern "system" fn ValidatorInit<Identity: IMetaDataValidate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmoduletype: u32, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataValidate_Impl::ValidatorInit(this, core::mem::transmute_copy(&dwmoduletype), windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn ValidateMetaData<Identity: IMetaDataValidate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataValidate_Impl::ValidateMetaData(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ValidatorInit: ValidatorInit::<Identity, OFFSET>,
            ValidateMetaData: ValidateMetaData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMetaDataValidate as windows_core::Interface>::IID
    }
}
pub trait IMetaDataWinMDImport_Impl: Sized + windows_core::IUnknownImpl {
    fn GetUntransformedTypeRefProps(&self, tr: u32, ptkresolutionscope: *mut u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMetaDataWinMDImport {}
impl IMetaDataWinMDImport_Vtbl {
    pub const fn new<Identity: IMetaDataWinMDImport_Impl, const OFFSET: isize>() -> IMetaDataWinMDImport_Vtbl {
        unsafe extern "system" fn GetUntransformedTypeRefProps<Identity: IMetaDataWinMDImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tr: u32, ptkresolutionscope: *mut u32, szname: windows_core::PWSTR, cchname: u32, pchname: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaDataWinMDImport_Impl::GetUntransformedTypeRefProps(this, core::mem::transmute_copy(&tr), core::mem::transmute_copy(&ptkresolutionscope), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pchname)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetUntransformedTypeRefProps: GetUntransformedTypeRefProps::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMetaDataWinMDImport as windows_core::Interface>::IID
    }
}
pub trait IRoMetaDataLocator_Impl: Sized {
    fn Locate(&self, nameelement: &windows_core::PCWSTR, metadatadestination: Option<&IRoSimpleMetaDataBuilder>) -> windows_core::Result<()>;
}
impl IRoMetaDataLocator_Vtbl {
    pub const fn new<Impl: IRoMetaDataLocator_Impl>() -> IRoMetaDataLocator_Vtbl {
        unsafe extern "system" fn Locate<Impl: IRoMetaDataLocator_Impl>(this: *mut core::ffi::c_void, nameelement: windows_core::PCWSTR, metadatadestination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IRoMetaDataLocator_Impl::Locate(this, core::mem::transmute(&nameelement), windows_core::from_raw_borrowed(&metadatadestination)).into()
        }
        Self { Locate: Locate::<Impl> }
    }
}
#[doc(hidden)]
struct IRoMetaDataLocator_ImplVtbl<T: IRoMetaDataLocator_Impl>(std::marker::PhantomData<T>);
impl<T: IRoMetaDataLocator_Impl> IRoMetaDataLocator_ImplVtbl<T> {
    const VTABLE: IRoMetaDataLocator_Vtbl = IRoMetaDataLocator_Vtbl::new::<T>();
}
impl IRoMetaDataLocator {
    pub fn new<'a, T: IRoMetaDataLocator_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IRoMetaDataLocator_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait IRoSimpleMetaDataBuilder_Impl: Sized {
    fn SetWinRtInterface(&self, iid: &windows_core::GUID) -> windows_core::Result<()>;
    fn SetDelegate(&self, iid: &windows_core::GUID) -> windows_core::Result<()>;
    fn SetInterfaceGroupSimpleDefault(&self, name: &windows_core::PCWSTR, defaultinterfacename: &windows_core::PCWSTR, defaultinterfaceiid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetInterfaceGroupParameterizedDefault(&self, name: &windows_core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetRuntimeClassSimpleDefault(&self, name: &windows_core::PCWSTR, defaultinterfacename: &windows_core::PCWSTR, defaultinterfaceiid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetRuntimeClassParameterizedDefault(&self, name: &windows_core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetStruct(&self, name: &windows_core::PCWSTR, numfields: u32, fieldtypenames: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetEnum(&self, name: &windows_core::PCWSTR, basetype: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetParameterizedInterface(&self, piid: &windows_core::GUID, numargs: u32) -> windows_core::Result<()>;
    fn SetParameterizedDelegate(&self, piid: &windows_core::GUID, numargs: u32) -> windows_core::Result<()>;
}
impl IRoSimpleMetaDataBuilder_Vtbl {
    pub const fn new<Impl: IRoSimpleMetaDataBuilder_Impl>() -> IRoSimpleMetaDataBuilder_Vtbl {
        unsafe extern "system" fn SetWinRtInterface<Impl: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, iid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IRoSimpleMetaDataBuilder_Impl::SetWinRtInterface(this, core::mem::transmute(&iid)).into()
        }
        unsafe extern "system" fn SetDelegate<Impl: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, iid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IRoSimpleMetaDataBuilder_Impl::SetDelegate(this, core::mem::transmute(&iid)).into()
        }
        unsafe extern "system" fn SetInterfaceGroupSimpleDefault<Impl: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, defaultinterfacename: windows_core::PCWSTR, defaultinterfaceiid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IRoSimpleMetaDataBuilder_Impl::SetInterfaceGroupSimpleDefault(this, core::mem::transmute(&name), core::mem::transmute(&defaultinterfacename), core::mem::transmute_copy(&defaultinterfaceiid)).into()
        }
        unsafe extern "system" fn SetInterfaceGroupParameterizedDefault<Impl: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IRoSimpleMetaDataBuilder_Impl::SetInterfaceGroupParameterizedDefault(this, core::mem::transmute(&name), core::mem::transmute_copy(&elementcount), core::mem::transmute_copy(&defaultinterfacenameelements)).into()
        }
        unsafe extern "system" fn SetRuntimeClassSimpleDefault<Impl: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, defaultinterfacename: windows_core::PCWSTR, defaultinterfaceiid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IRoSimpleMetaDataBuilder_Impl::SetRuntimeClassSimpleDefault(this, core::mem::transmute(&name), core::mem::transmute(&defaultinterfacename), core::mem::transmute_copy(&defaultinterfaceiid)).into()
        }
        unsafe extern "system" fn SetRuntimeClassParameterizedDefault<Impl: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IRoSimpleMetaDataBuilder_Impl::SetRuntimeClassParameterizedDefault(this, core::mem::transmute(&name), core::mem::transmute_copy(&elementcount), core::mem::transmute_copy(&defaultinterfacenameelements)).into()
        }
        unsafe extern "system" fn SetStruct<Impl: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, numfields: u32, fieldtypenames: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IRoSimpleMetaDataBuilder_Impl::SetStruct(this, core::mem::transmute(&name), core::mem::transmute_copy(&numfields), core::mem::transmute_copy(&fieldtypenames)).into()
        }
        unsafe extern "system" fn SetEnum<Impl: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, basetype: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IRoSimpleMetaDataBuilder_Impl::SetEnum(this, core::mem::transmute(&name), core::mem::transmute(&basetype)).into()
        }
        unsafe extern "system" fn SetParameterizedInterface<Impl: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, piid: windows_core::GUID, numargs: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IRoSimpleMetaDataBuilder_Impl::SetParameterizedInterface(this, core::mem::transmute(&piid), core::mem::transmute_copy(&numargs)).into()
        }
        unsafe extern "system" fn SetParameterizedDelegate<Impl: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, piid: windows_core::GUID, numargs: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IRoSimpleMetaDataBuilder_Impl::SetParameterizedDelegate(this, core::mem::transmute(&piid), core::mem::transmute_copy(&numargs)).into()
        }
        Self {
            SetWinRtInterface: SetWinRtInterface::<Impl>,
            SetDelegate: SetDelegate::<Impl>,
            SetInterfaceGroupSimpleDefault: SetInterfaceGroupSimpleDefault::<Impl>,
            SetInterfaceGroupParameterizedDefault: SetInterfaceGroupParameterizedDefault::<Impl>,
            SetRuntimeClassSimpleDefault: SetRuntimeClassSimpleDefault::<Impl>,
            SetRuntimeClassParameterizedDefault: SetRuntimeClassParameterizedDefault::<Impl>,
            SetStruct: SetStruct::<Impl>,
            SetEnum: SetEnum::<Impl>,
            SetParameterizedInterface: SetParameterizedInterface::<Impl>,
            SetParameterizedDelegate: SetParameterizedDelegate::<Impl>,
        }
    }
}
#[doc(hidden)]
struct IRoSimpleMetaDataBuilder_ImplVtbl<T: IRoSimpleMetaDataBuilder_Impl>(std::marker::PhantomData<T>);
impl<T: IRoSimpleMetaDataBuilder_Impl> IRoSimpleMetaDataBuilder_ImplVtbl<T> {
    const VTABLE: IRoSimpleMetaDataBuilder_Vtbl = IRoSimpleMetaDataBuilder_Vtbl::new::<T>();
}
impl IRoSimpleMetaDataBuilder {
    pub fn new<'a, T: IRoSimpleMetaDataBuilder_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IRoSimpleMetaDataBuilder_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
