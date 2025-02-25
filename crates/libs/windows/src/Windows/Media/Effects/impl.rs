#[cfg(feature = "Foundation_Collections")]
pub trait IAudioEffectDefinition_Impl: Sized + windows_core::IUnknownImpl {
    fn ActivatableClassId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IAudioEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.IAudioEffectDefinition";
}
#[cfg(feature = "Foundation_Collections")]
impl IAudioEffectDefinition_Vtbl {
    pub const fn new<Identity: IAudioEffectDefinition_Impl, const OFFSET: isize>() -> IAudioEffectDefinition_Vtbl {
        unsafe extern "system" fn ActivatableClassId<Identity: IAudioEffectDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioEffectDefinition_Impl::ActivatableClassId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: IAudioEffectDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioEffectDefinition_Impl::Properties(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAudioEffectDefinition, OFFSET>(),
            ActivatableClassId: ActivatableClassId::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEffectDefinition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
pub trait IBasicAudioEffect_Impl: Sized + windows_core::IUnknownImpl + super::IMediaExtension_Impl {
    fn UseInputFrameForOutput(&self) -> windows_core::Result<bool>;
    fn SupportedEncodingProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::AudioEncodingProperties>>;
    fn SetEncodingProperties(&self, encodingproperties: Option<&super::MediaProperties::AudioEncodingProperties>) -> windows_core::Result<()>;
    fn ProcessFrame(&self, context: Option<&ProcessAudioFrameContext>) -> windows_core::Result<()>;
    fn Close(&self, reason: MediaEffectClosedReason) -> windows_core::Result<()>;
    fn DiscardQueuedFrames(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
impl windows_core::RuntimeName for IBasicAudioEffect {
    const NAME: &'static str = "Windows.Media.Effects.IBasicAudioEffect";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
impl IBasicAudioEffect_Vtbl {
    pub const fn new<Identity: IBasicAudioEffect_Impl, const OFFSET: isize>() -> IBasicAudioEffect_Vtbl {
        unsafe extern "system" fn UseInputFrameForOutput<Identity: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBasicAudioEffect_Impl::UseInputFrameForOutput(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedEncodingProperties<Identity: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBasicAudioEffect_Impl::SupportedEncodingProperties(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncodingProperties<Identity: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encodingproperties: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBasicAudioEffect_Impl::SetEncodingProperties(this, windows_core::from_raw_borrowed(&encodingproperties)).into()
        }
        unsafe extern "system" fn ProcessFrame<Identity: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBasicAudioEffect_Impl::ProcessFrame(this, windows_core::from_raw_borrowed(&context)).into()
        }
        unsafe extern "system" fn Close<Identity: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reason: MediaEffectClosedReason) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBasicAudioEffect_Impl::Close(this, reason).into()
        }
        unsafe extern "system" fn DiscardQueuedFrames<Identity: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBasicAudioEffect_Impl::DiscardQueuedFrames(this).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBasicAudioEffect, OFFSET>(),
            UseInputFrameForOutput: UseInputFrameForOutput::<Identity, OFFSET>,
            SupportedEncodingProperties: SupportedEncodingProperties::<Identity, OFFSET>,
            SetEncodingProperties: SetEncodingProperties::<Identity, OFFSET>,
            ProcessFrame: ProcessFrame::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            DiscardQueuedFrames: DiscardQueuedFrames::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBasicAudioEffect as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
pub trait IBasicVideoEffect_Impl: Sized + windows_core::IUnknownImpl + super::IMediaExtension_Impl {
    fn IsReadOnly(&self) -> windows_core::Result<bool>;
    fn SupportedMemoryTypes(&self) -> windows_core::Result<MediaMemoryTypes>;
    fn TimeIndependent(&self) -> windows_core::Result<bool>;
    fn SupportedEncodingProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::VideoEncodingProperties>>;
    fn SetEncodingProperties(&self, encodingproperties: Option<&super::MediaProperties::VideoEncodingProperties>, device: Option<&super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>) -> windows_core::Result<()>;
    fn ProcessFrame(&self, context: Option<&ProcessVideoFrameContext>) -> windows_core::Result<()>;
    fn Close(&self, reason: MediaEffectClosedReason) -> windows_core::Result<()>;
    fn DiscardQueuedFrames(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
impl windows_core::RuntimeName for IBasicVideoEffect {
    const NAME: &'static str = "Windows.Media.Effects.IBasicVideoEffect";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
impl IBasicVideoEffect_Vtbl {
    pub const fn new<Identity: IBasicVideoEffect_Impl, const OFFSET: isize>() -> IBasicVideoEffect_Vtbl {
        unsafe extern "system" fn IsReadOnly<Identity: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBasicVideoEffect_Impl::IsReadOnly(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedMemoryTypes<Identity: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut MediaMemoryTypes) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBasicVideoEffect_Impl::SupportedMemoryTypes(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeIndependent<Identity: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBasicVideoEffect_Impl::TimeIndependent(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedEncodingProperties<Identity: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBasicVideoEffect_Impl::SupportedEncodingProperties(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncodingProperties<Identity: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encodingproperties: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBasicVideoEffect_Impl::SetEncodingProperties(this, windows_core::from_raw_borrowed(&encodingproperties), windows_core::from_raw_borrowed(&device)).into()
        }
        unsafe extern "system" fn ProcessFrame<Identity: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBasicVideoEffect_Impl::ProcessFrame(this, windows_core::from_raw_borrowed(&context)).into()
        }
        unsafe extern "system" fn Close<Identity: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reason: MediaEffectClosedReason) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBasicVideoEffect_Impl::Close(this, reason).into()
        }
        unsafe extern "system" fn DiscardQueuedFrames<Identity: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBasicVideoEffect_Impl::DiscardQueuedFrames(this).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBasicVideoEffect, OFFSET>(),
            IsReadOnly: IsReadOnly::<Identity, OFFSET>,
            SupportedMemoryTypes: SupportedMemoryTypes::<Identity, OFFSET>,
            TimeIndependent: TimeIndependent::<Identity, OFFSET>,
            SupportedEncodingProperties: SupportedEncodingProperties::<Identity, OFFSET>,
            SetEncodingProperties: SetEncodingProperties::<Identity, OFFSET>,
            ProcessFrame: ProcessFrame::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            DiscardQueuedFrames: DiscardQueuedFrames::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBasicVideoEffect as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
pub trait IVideoCompositor_Impl: Sized + windows_core::IUnknownImpl + super::IMediaExtension_Impl {
    fn TimeIndependent(&self) -> windows_core::Result<bool>;
    fn SetEncodingProperties(&self, backgroundproperties: Option<&super::MediaProperties::VideoEncodingProperties>, device: Option<&super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>) -> windows_core::Result<()>;
    fn CompositeFrame(&self, context: Option<&CompositeVideoFrameContext>) -> windows_core::Result<()>;
    fn Close(&self, reason: MediaEffectClosedReason) -> windows_core::Result<()>;
    fn DiscardQueuedFrames(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
impl windows_core::RuntimeName for IVideoCompositor {
    const NAME: &'static str = "Windows.Media.Effects.IVideoCompositor";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
impl IVideoCompositor_Vtbl {
    pub const fn new<Identity: IVideoCompositor_Impl, const OFFSET: isize>() -> IVideoCompositor_Vtbl {
        unsafe extern "system" fn TimeIndependent<Identity: IVideoCompositor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVideoCompositor_Impl::TimeIndependent(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncodingProperties<Identity: IVideoCompositor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, backgroundproperties: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVideoCompositor_Impl::SetEncodingProperties(this, windows_core::from_raw_borrowed(&backgroundproperties), windows_core::from_raw_borrowed(&device)).into()
        }
        unsafe extern "system" fn CompositeFrame<Identity: IVideoCompositor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVideoCompositor_Impl::CompositeFrame(this, windows_core::from_raw_borrowed(&context)).into()
        }
        unsafe extern "system" fn Close<Identity: IVideoCompositor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reason: MediaEffectClosedReason) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVideoCompositor_Impl::Close(this, reason).into()
        }
        unsafe extern "system" fn DiscardQueuedFrames<Identity: IVideoCompositor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVideoCompositor_Impl::DiscardQueuedFrames(this).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVideoCompositor, OFFSET>(),
            TimeIndependent: TimeIndependent::<Identity, OFFSET>,
            SetEncodingProperties: SetEncodingProperties::<Identity, OFFSET>,
            CompositeFrame: CompositeFrame::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            DiscardQueuedFrames: DiscardQueuedFrames::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVideoCompositor as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVideoCompositorDefinition_Impl: Sized + windows_core::IUnknownImpl {
    fn ActivatableClassId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IVideoCompositorDefinition {
    const NAME: &'static str = "Windows.Media.Effects.IVideoCompositorDefinition";
}
#[cfg(feature = "Foundation_Collections")]
impl IVideoCompositorDefinition_Vtbl {
    pub const fn new<Identity: IVideoCompositorDefinition_Impl, const OFFSET: isize>() -> IVideoCompositorDefinition_Vtbl {
        unsafe extern "system" fn ActivatableClassId<Identity: IVideoCompositorDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVideoCompositorDefinition_Impl::ActivatableClassId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: IVideoCompositorDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVideoCompositorDefinition_Impl::Properties(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVideoCompositorDefinition, OFFSET>(),
            ActivatableClassId: ActivatableClassId::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVideoCompositorDefinition as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVideoEffectDefinition_Impl: Sized + windows_core::IUnknownImpl {
    fn ActivatableClassId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IVideoEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.IVideoEffectDefinition";
}
#[cfg(feature = "Foundation_Collections")]
impl IVideoEffectDefinition_Vtbl {
    pub const fn new<Identity: IVideoEffectDefinition_Impl, const OFFSET: isize>() -> IVideoEffectDefinition_Vtbl {
        unsafe extern "system" fn ActivatableClassId<Identity: IVideoEffectDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVideoEffectDefinition_Impl::ActivatableClassId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: IVideoEffectDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVideoEffectDefinition_Impl::Properties(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVideoEffectDefinition, OFFSET>(),
            ActivatableClassId: ActivatableClassId::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVideoEffectDefinition as windows_core::Interface>::IID
    }
}
