#[cfg(feature = "Devices_Printers_Extensions")]
pub mod Extensions;
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppAttributeError(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIppAttributeError {
    type Vtable = IIppAttributeError_Vtbl;
}
unsafe impl ::windows::core::Interface for IIppAttributeError {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x750feda1_9eef_5c39_93e4_46149bbcef27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppAttributeError_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IppAttributeErrorReason) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUnsupportedValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUnsupportedValues: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppAttributeValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIppAttributeValue {
    type Vtable = IIppAttributeValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IIppAttributeValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99407fed_e2bb_59a3_988b_28a974052a26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppAttributeValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IppAttributeValueKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetIntegerArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetIntegerArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBooleanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBooleanArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEnumArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEnumArray: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetOctetStringArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetOctetStringArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDateTimeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDateTimeArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetResolutionArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetResolutionArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRangeOfIntegerArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRangeOfIntegerArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCollectionArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCollectionArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTextWithLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTextWithLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNameWithLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNameWithLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTextWithoutLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTextWithoutLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNameWithoutLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNameWithoutLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetKeywordArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetKeywordArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUriArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUriArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUriSchemaArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUriSchemaArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCharsetArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCharsetArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNaturalLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNaturalLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMimeMediaTypeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMimeMediaTypeArray: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppAttributeValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIppAttributeValueStatics {
    type Vtable = IIppAttributeValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IIppAttributeValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10d43942_dd94_5998_b235_afafb6fa7935);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppAttributeValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateUnsupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUnknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateNoValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInteger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateIntegerArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateIntegerArray: usize,
    pub CreateBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateBooleanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateBooleanArray: usize,
    pub CreateEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateEnumArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateEnumArray: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateOctetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateOctetString: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub CreateOctetStringArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    CreateOctetStringArray: usize,
    #[cfg(feature = "Foundation")]
    pub CreateDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateDateTime: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDateTimeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDateTimeArray: usize,
    pub CreateResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateResolutionArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateResolutionArray: usize,
    pub CreateRangeOfInteger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateRangeOfIntegerArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateRangeOfIntegerArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memberattributes: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCollection: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCollectionArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memberattributesarray: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCollectionArray: usize,
    pub CreateTextWithLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateTextWithLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateTextWithLanguageArray: usize,
    pub CreateNameWithLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNameWithLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNameWithLanguageArray: usize,
    pub CreateTextWithoutLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateTextWithoutLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateTextWithoutLanguageArray: usize,
    pub CreateNameWithoutLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNameWithoutLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNameWithoutLanguageArray: usize,
    pub CreateKeyword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateKeywordArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateKeywordArray: usize,
    #[cfg(feature = "Foundation")]
    pub CreateUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateUri: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateUriArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateUriArray: usize,
    pub CreateUriSchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateUriSchemaArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateUriSchemaArray: usize,
    pub CreateCharset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCharsetArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCharsetArray: usize,
    pub CreateNaturalLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNaturalLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNaturalLanguageArray: usize,
    pub CreateMimeMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateMimeMediaArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateMimeMediaArray: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppIntegerRange(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIppIntegerRange {
    type Vtable = IIppIntegerRange_Vtbl;
}
unsafe impl ::windows::core::Interface for IIppIntegerRange {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92907346_c3ea_5ed6_bdb1_3752c62c6f7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppIntegerRange_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppIntegerRangeFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIppIntegerRangeFactory {
    type Vtable = IIppIntegerRangeFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IIppIntegerRangeFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75d4ecae_f87e_54ad_b5d0_465204db7553);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppIntegerRangeFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: i32, end: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppPrintDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIppPrintDevice {
    type Vtable = IIppPrintDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for IIppPrintDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd748ac56_76f3_5dc6_afd4_c2a8686b9359);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppPrintDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PrinterName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PrinterUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrinterUri: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetPrinterAttributesAsBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetPrinterAttributesAsBuffer: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPrinterAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPrinterAttributes: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetPrinterAttributesFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printerattributesbuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPrinterAttributesFromBuffer: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPrinterAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printerattributes: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPrinterAttributes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppPrintDevice2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIppPrintDevice2 {
    type Vtable = IIppPrintDevice2_Vtbl;
}
unsafe impl ::windows::core::Interface for IIppPrintDevice2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7c844c9_9d21_5c63_ac20_3676915be2d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppPrintDevice2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetMaxSupportedPdfSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub GetMaxSupportedPdfVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsPdlPassthroughSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdlcontenttype: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetPdlPassthroughProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppPrintDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIppPrintDeviceStatics {
    type Vtable = IIppPrintDeviceStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IIppPrintDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7dc19f08_7f20_52ab_94a7_894b83b2a17e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppPrintDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FromPrinterName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printername: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsIppPrinter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printername: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppResolution(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIppResolution {
    type Vtable = IIppResolution_Vtbl;
}
unsafe impl ::windows::core::Interface for IIppResolution {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb493f86_6bf3_56f5_86ce_263d08aead63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppResolution_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Unit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IppResolutionUnit) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppResolutionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIppResolutionFactory {
    type Vtable = IIppResolutionFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IIppResolutionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe481c2ae_251a_5326_b173_95543ed99a35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppResolutionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: i32, height: i32, unit: IppResolutionUnit, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppSetAttributesResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIppSetAttributesResult {
    type Vtable = IIppSetAttributesResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IIppSetAttributesResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d1c7f55_aa9d_58a3_90e9_17bdc5281f07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppSetAttributesResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AttributeErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AttributeErrors: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppTextWithLanguage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIppTextWithLanguage {
    type Vtable = IIppTextWithLanguage_Vtbl;
}
unsafe impl ::windows::core::Interface for IIppTextWithLanguage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x326447a6_5149_5936_90e8_0c736036bf77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppTextWithLanguage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppTextWithLanguageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIppTextWithLanguageFactory {
    type Vtable = IIppTextWithLanguageFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IIppTextWithLanguageFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca4a1e8d_2968_5775_997c_8a46f1a574ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppTextWithLanguageFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::core::ffi::c_void, text: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPageConfigurationSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPageConfigurationSettings {
    type Vtable = IPageConfigurationSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IPageConfigurationSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6fc1e02_5331_54ff_95a0_1fcb76bb97a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageConfigurationSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OrientationSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PageConfigurationSource) -> ::windows::core::HRESULT,
    pub SetOrientationSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PageConfigurationSource) -> ::windows::core::HRESULT,
    pub SizeSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PageConfigurationSource) -> ::windows::core::HRESULT,
    pub SetSizeSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PageConfigurationSource) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPdlPassthroughProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPdlPassthroughProvider {
    type Vtable = IPdlPassthroughProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IPdlPassthroughProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23c71dd2_6117_553f_9378_180af5849a49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdlPassthroughProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPdlContentTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPdlContentTypes: usize,
    #[cfg(feature = "Graphics_Printing")]
    pub StartPrintJobWithTaskOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobname: *mut ::core::ffi::c_void, pdlcontenttype: *mut ::core::ffi::c_void, taskoptions: *mut ::core::ffi::c_void, pageconfigurationsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    StartPrintJobWithTaskOptions: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StartPrintJobWithPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobname: *mut ::core::ffi::c_void, pdlcontenttype: *mut ::core::ffi::c_void, printticket: *mut ::core::ffi::c_void, pageconfigurationsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StartPrintJobWithPrintTicket: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPdlPassthroughTarget(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPdlPassthroughTarget {
    type Vtable = IPdlPassthroughTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for IPdlPassthroughTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9840be79_67f8_5385_a5b9_e8c96e0fca76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdlPassthroughTarget_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PrintJobId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetOutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetOutputStream: usize,
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrint3DDevice {
    type Vtable = IPrint3DDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrint3DDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x041c3d19_9713_42a2_9813_7dc3337428d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PrintSchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrint3DDeviceStatics {
    type Vtable = IPrint3DDeviceStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrint3DDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfde3620a_67cd_41b7_a344_5150a1fd75b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSchema(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrintSchema {
    type Vtable = IPrintSchema_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintSchema {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2b98316_26b8_4bfb_8138_9f962c22a35b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchema_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetDefaultPrintTicketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetDefaultPrintTicketAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetCapabilitiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, constrainticket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetCapabilitiesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub MergeAndValidateWithDefaultPrintTicketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deltaticket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    MergeAndValidateWithDefaultPrintTicketAsync: usize,
}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct IppAttributeError(::windows::core::IUnknown);
impl IppAttributeError {
    pub fn Reason(&self) -> ::windows::core::Result<IppAttributeErrorReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Reason)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUnsupportedValues(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IppAttributeValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUnsupportedValues)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for IppAttributeError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IppAttributeError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppAttributeError {}
impl ::core::fmt::Debug for IppAttributeError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppAttributeError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IppAttributeError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppAttributeError;{750feda1-9eef-5c39-93e4-46149bbcef27})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IppAttributeError {
    type Vtable = IIppAttributeError_Vtbl;
}
unsafe impl ::windows::core::Interface for IppAttributeError {
    const IID: ::windows::core::GUID = <IIppAttributeError as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IppAttributeError {
    const NAME: &'static str = "Windows.Devices.Printers.IppAttributeError";
}
::windows::core::interface_hierarchy!(IppAttributeError, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IppAttributeError {}
unsafe impl ::core::marker::Sync for IppAttributeError {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct IppAttributeValue(::windows::core::IUnknown);
impl IppAttributeValue {
    pub fn Kind(&self) -> ::windows::core::Result<IppAttributeValueKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIntegerArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetIntegerArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBooleanArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBooleanArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetEnumArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetEnumArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetOctetStringArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetOctetStringArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDateTimeArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDateTimeArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetResolutionArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IppResolution>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetResolutionArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRangeOfIntegerArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IppIntegerRange>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetRangeOfIntegerArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCollectionArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IppAttributeValue>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCollectionArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTextWithLanguageArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IppTextWithLanguage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetTextWithLanguageArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNameWithLanguageArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IppTextWithLanguage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNameWithLanguageArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTextWithoutLanguageArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetTextWithoutLanguageArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNameWithoutLanguageArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNameWithoutLanguageArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetKeywordArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetKeywordArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUriArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUriArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUriSchemaArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUriSchemaArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCharsetArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCharsetArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNaturalLanguageArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNaturalLanguageArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMimeMediaTypeArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMimeMediaTypeArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateUnsupported() -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateUnsupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateUnknown() -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateUnknown)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateNoValue() -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateNoValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateInteger(value: i32) -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInteger)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateIntegerArray<P0, E0>(values: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateIntegerArray)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateBoolean(value: bool) -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateBoolean)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateBooleanArray<P0, E0>(values: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<bool>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateBooleanArray)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateEnum(value: i32) -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateEnum)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateEnumArray<P0, E0>(values: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateEnumArray)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateOctetString<P0, E0>(value: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateOctetString)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn CreateOctetStringArray<P0, E0>(values: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Storage::Streams::IBuffer>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateOctetStringArray)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateDateTime(value: super::super::Foundation::DateTime) -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateDateTime)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeArray<P0, E0>(values: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateDateTimeArray)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateResolution(value: &IppResolution) -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateResolution)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateResolutionArray<P0, E0>(values: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<IppResolution>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateResolutionArray)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateRangeOfInteger(value: &IppIntegerRange) -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateRangeOfInteger)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateRangeOfIntegerArray<P0, E0>(values: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<IppIntegerRange>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateRangeOfIntegerArray)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCollection<P0, E0>(memberattributes: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IppAttributeValue>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCollection)(::windows::core::Vtable::as_raw(this), memberattributes.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCollectionArray<P0, E0>(memberattributesarray: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IppAttributeValue>>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCollectionArray)(::windows::core::Vtable::as_raw(this), memberattributesarray.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateTextWithLanguage(value: &IppTextWithLanguage) -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTextWithLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateTextWithLanguageArray<P0, E0>(values: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<IppTextWithLanguage>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTextWithLanguageArray)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateNameWithLanguage(value: &IppTextWithLanguage) -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateNameWithLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNameWithLanguageArray<P0, E0>(values: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<IppTextWithLanguage>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateNameWithLanguageArray)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateTextWithoutLanguage(value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTextWithoutLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateTextWithoutLanguageArray<P0, E0>(values: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTextWithoutLanguageArray)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateNameWithoutLanguage(value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateNameWithoutLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNameWithoutLanguageArray<P0, E0>(values: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateNameWithoutLanguageArray)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateKeyword(value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateKeyword)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateKeywordArray<P0, E0>(values: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateKeywordArray)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateUri(value: &super::super::Foundation::Uri) -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateUriArray<P0, E0>(values: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateUriArray)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateUriSchema(value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateUriSchema)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateUriSchemaArray<P0, E0>(values: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateUriSchemaArray)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateCharset(value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCharset)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCharsetArray<P0, E0>(values: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCharsetArray)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateNaturalLanguage(value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateNaturalLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNaturalLanguageArray<P0, E0>(values: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateNaturalLanguageArray)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateMimeMedia(value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateMimeMedia)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateMimeMediaArray<P0, E0>(values: P0) -> ::windows::core::Result<IppAttributeValue>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateMimeMediaArray)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IIppAttributeValueStatics<R, F: FnOnce(&IIppAttributeValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<IppAttributeValue, IIppAttributeValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for IppAttributeValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IppAttributeValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppAttributeValue {}
impl ::core::fmt::Debug for IppAttributeValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppAttributeValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IppAttributeValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppAttributeValue;{99407fed-e2bb-59a3-988b-28a974052a26})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IppAttributeValue {
    type Vtable = IIppAttributeValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IppAttributeValue {
    const IID: ::windows::core::GUID = <IIppAttributeValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IppAttributeValue {
    const NAME: &'static str = "Windows.Devices.Printers.IppAttributeValue";
}
::windows::core::interface_hierarchy!(IppAttributeValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IppAttributeValue {}
unsafe impl ::core::marker::Sync for IppAttributeValue {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct IppIntegerRange(::windows::core::IUnknown);
impl IppIntegerRange {
    pub fn Start(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn End(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).End)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateInstance(start: i32, end: i32) -> ::windows::core::Result<IppIntegerRange> {
        Self::IIppIntegerRangeFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), start, end, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IIppIntegerRangeFactory<R, F: FnOnce(&IIppIntegerRangeFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<IppIntegerRange, IIppIntegerRangeFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for IppIntegerRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IppIntegerRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppIntegerRange {}
impl ::core::fmt::Debug for IppIntegerRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppIntegerRange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IppIntegerRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppIntegerRange;{92907346-c3ea-5ed6-bdb1-3752c62c6f7f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IppIntegerRange {
    type Vtable = IIppIntegerRange_Vtbl;
}
unsafe impl ::windows::core::Interface for IppIntegerRange {
    const IID: ::windows::core::GUID = <IIppIntegerRange as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IppIntegerRange {
    const NAME: &'static str = "Windows.Devices.Printers.IppIntegerRange";
}
::windows::core::interface_hierarchy!(IppIntegerRange, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IppIntegerRange {}
unsafe impl ::core::marker::Sync for IppIntegerRange {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct IppPrintDevice(::windows::core::IUnknown);
impl IppPrintDevice {
    pub fn PrinterName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrinterName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrinterUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrinterUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetPrinterAttributesAsBuffer<P0, E0>(&self, attributenames: P0) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPrinterAttributesAsBuffer)(::windows::core::Vtable::as_raw(this), attributenames.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPrinterAttributes<P0, E0>(&self, attributenames: P0) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IppAttributeValue>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPrinterAttributes)(::windows::core::Vtable::as_raw(this), attributenames.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPrinterAttributesFromBuffer<P0, E0>(&self, printerattributesbuffer: P0) -> ::windows::core::Result<IppSetAttributesResult>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetPrinterAttributesFromBuffer)(::windows::core::Vtable::as_raw(this), printerattributesbuffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetPrinterAttributes<P0, E0>(&self, printerattributes: P0) -> ::windows::core::Result<IppSetAttributesResult>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IppAttributeValue>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetPrinterAttributes)(::windows::core::Vtable::as_raw(this), printerattributes.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetMaxSupportedPdfSize(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IIppPrintDevice2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMaxSupportedPdfSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetMaxSupportedPdfVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IIppPrintDevice2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMaxSupportedPdfVersion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPdlPassthroughSupported(&self, pdlcontenttype: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IIppPrintDevice2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPdlPassthroughSupported)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(pdlcontenttype), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetPdlPassthroughProvider(&self) -> ::windows::core::Result<PdlPassthroughProvider> {
        let this = &::windows::core::Interface::cast::<IIppPrintDevice2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPdlPassthroughProvider)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IIppPrintDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn FromId(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<IppPrintDevice> {
        Self::IIppPrintDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn FromPrinterName(printername: &::windows::core::HSTRING) -> ::windows::core::Result<IppPrintDevice> {
        Self::IIppPrintDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromPrinterName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(printername), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IsIppPrinter(printername: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        Self::IIppPrintDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsIppPrinter)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(printername), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IIppPrintDeviceStatics<R, F: FnOnce(&IIppPrintDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<IppPrintDevice, IIppPrintDeviceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for IppPrintDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IppPrintDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppPrintDevice {}
impl ::core::fmt::Debug for IppPrintDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppPrintDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IppPrintDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppPrintDevice;{d748ac56-76f3-5dc6-afd4-c2a8686b9359})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IppPrintDevice {
    type Vtable = IIppPrintDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for IppPrintDevice {
    const IID: ::windows::core::GUID = <IIppPrintDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IppPrintDevice {
    const NAME: &'static str = "Windows.Devices.Printers.IppPrintDevice";
}
::windows::core::interface_hierarchy!(IppPrintDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IppPrintDevice {}
unsafe impl ::core::marker::Sync for IppPrintDevice {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct IppResolution(::windows::core::IUnknown);
impl IppResolution {
    pub fn Width(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Width)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Height)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Unit(&self) -> ::windows::core::Result<IppResolutionUnit> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Unit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateInstance(width: i32, height: i32, unit: IppResolutionUnit) -> ::windows::core::Result<IppResolution> {
        Self::IIppResolutionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), width, height, unit, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IIppResolutionFactory<R, F: FnOnce(&IIppResolutionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<IppResolution, IIppResolutionFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for IppResolution {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IppResolution {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppResolution {}
impl ::core::fmt::Debug for IppResolution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppResolution").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IppResolution {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppResolution;{cb493f86-6bf3-56f5-86ce-263d08aead63})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IppResolution {
    type Vtable = IIppResolution_Vtbl;
}
unsafe impl ::windows::core::Interface for IppResolution {
    const IID: ::windows::core::GUID = <IIppResolution as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IppResolution {
    const NAME: &'static str = "Windows.Devices.Printers.IppResolution";
}
::windows::core::interface_hierarchy!(IppResolution, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IppResolution {}
unsafe impl ::core::marker::Sync for IppResolution {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct IppSetAttributesResult(::windows::core::IUnknown);
impl IppSetAttributesResult {
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Succeeded)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IppAttributeError>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttributeErrors)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for IppSetAttributesResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IppSetAttributesResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppSetAttributesResult {}
impl ::core::fmt::Debug for IppSetAttributesResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppSetAttributesResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IppSetAttributesResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppSetAttributesResult;{7d1c7f55-aa9d-58a3-90e9-17bdc5281f07})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IppSetAttributesResult {
    type Vtable = IIppSetAttributesResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IppSetAttributesResult {
    const IID: ::windows::core::GUID = <IIppSetAttributesResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IppSetAttributesResult {
    const NAME: &'static str = "Windows.Devices.Printers.IppSetAttributesResult";
}
::windows::core::interface_hierarchy!(IppSetAttributesResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IppSetAttributesResult {}
unsafe impl ::core::marker::Sync for IppSetAttributesResult {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct IppTextWithLanguage(::windows::core::IUnknown);
impl IppTextWithLanguage {
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateInstance(language: &::windows::core::HSTRING, text: &::windows::core::HSTRING) -> ::windows::core::Result<IppTextWithLanguage> {
        Self::IIppTextWithLanguageFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(language), ::core::mem::transmute_copy(text), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IIppTextWithLanguageFactory<R, F: FnOnce(&IIppTextWithLanguageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<IppTextWithLanguage, IIppTextWithLanguageFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for IppTextWithLanguage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IppTextWithLanguage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppTextWithLanguage {}
impl ::core::fmt::Debug for IppTextWithLanguage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppTextWithLanguage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IppTextWithLanguage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppTextWithLanguage;{326447a6-5149-5936-90e8-0c736036bf77})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IppTextWithLanguage {
    type Vtable = IIppTextWithLanguage_Vtbl;
}
unsafe impl ::windows::core::Interface for IppTextWithLanguage {
    const IID: ::windows::core::GUID = <IIppTextWithLanguage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IppTextWithLanguage {
    const NAME: &'static str = "Windows.Devices.Printers.IppTextWithLanguage";
}
::windows::core::interface_hierarchy!(IppTextWithLanguage, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IppTextWithLanguage {}
unsafe impl ::core::marker::Sync for IppTextWithLanguage {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct PageConfigurationSettings(::windows::core::IUnknown);
impl PageConfigurationSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PageConfigurationSettings, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn OrientationSource(&self) -> ::windows::core::Result<PageConfigurationSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OrientationSource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetOrientationSource(&self, value: PageConfigurationSource) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetOrientationSource)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SizeSource(&self) -> ::windows::core::Result<PageConfigurationSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SizeSource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSizeSource(&self, value: PageConfigurationSource) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSizeSource)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for PageConfigurationSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PageConfigurationSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PageConfigurationSettings {}
impl ::core::fmt::Debug for PageConfigurationSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PageConfigurationSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PageConfigurationSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.PageConfigurationSettings;{b6fc1e02-5331-54ff-95a0-1fcb76bb97a9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PageConfigurationSettings {
    type Vtable = IPageConfigurationSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for PageConfigurationSettings {
    const IID: ::windows::core::GUID = <IPageConfigurationSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PageConfigurationSettings {
    const NAME: &'static str = "Windows.Devices.Printers.PageConfigurationSettings";
}
::windows::core::interface_hierarchy!(PageConfigurationSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PageConfigurationSettings {}
unsafe impl ::core::marker::Sync for PageConfigurationSettings {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct PdlPassthroughProvider(::windows::core::IUnknown);
impl PdlPassthroughProvider {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPdlContentTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedPdlContentTypes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing\"`*"]
    #[cfg(feature = "Graphics_Printing")]
    pub fn StartPrintJobWithTaskOptions(&self, jobname: &::windows::core::HSTRING, pdlcontenttype: &::windows::core::HSTRING, taskoptions: &super::super::Graphics::Printing::PrintTaskOptions, pageconfigurationsettings: &PageConfigurationSettings) -> ::windows::core::Result<PdlPassthroughTarget> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartPrintJobWithTaskOptions)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(jobname), ::core::mem::transmute_copy(pdlcontenttype), ::core::mem::transmute_copy(taskoptions), ::core::mem::transmute_copy(pageconfigurationsettings), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn StartPrintJobWithPrintTicket<P0, E0>(&self, jobname: &::windows::core::HSTRING, pdlcontenttype: &::windows::core::HSTRING, printticket: P0, pageconfigurationsettings: &PageConfigurationSettings) -> ::windows::core::Result<PdlPassthroughTarget>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartPrintJobWithPrintTicket)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(jobname), ::core::mem::transmute_copy(pdlcontenttype), printticket.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(pageconfigurationsettings), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PdlPassthroughProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PdlPassthroughProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PdlPassthroughProvider {}
impl ::core::fmt::Debug for PdlPassthroughProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdlPassthroughProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PdlPassthroughProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.PdlPassthroughProvider;{23c71dd2-6117-553f-9378-180af5849a49})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PdlPassthroughProvider {
    type Vtable = IPdlPassthroughProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for PdlPassthroughProvider {
    const IID: ::windows::core::GUID = <IPdlPassthroughProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PdlPassthroughProvider {
    const NAME: &'static str = "Windows.Devices.Printers.PdlPassthroughProvider";
}
::windows::core::interface_hierarchy!(PdlPassthroughProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PdlPassthroughProvider {}
unsafe impl ::core::marker::Sync for PdlPassthroughProvider {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct PdlPassthroughTarget(::windows::core::IUnknown);
impl PdlPassthroughTarget {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn PrintJobId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrintJobId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetOutputStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Submit(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Submit)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for PdlPassthroughTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PdlPassthroughTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PdlPassthroughTarget {}
impl ::core::fmt::Debug for PdlPassthroughTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdlPassthroughTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PdlPassthroughTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.PdlPassthroughTarget;{9840be79-67f8-5385-a5b9-e8c96e0fca76})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PdlPassthroughTarget {
    type Vtable = IPdlPassthroughTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for PdlPassthroughTarget {
    const IID: ::windows::core::GUID = <IPdlPassthroughTarget as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PdlPassthroughTarget {
    const NAME: &'static str = "Windows.Devices.Printers.PdlPassthroughTarget";
}
::windows::core::interface_hierarchy!(PdlPassthroughTarget, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<PdlPassthroughTarget> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PdlPassthroughTarget) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&PdlPassthroughTarget> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PdlPassthroughTarget) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&PdlPassthroughTarget> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PdlPassthroughTarget) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for PdlPassthroughTarget {}
unsafe impl ::core::marker::Sync for PdlPassthroughTarget {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct Print3DDevice(::windows::core::IUnknown);
impl Print3DDevice {
    pub fn PrintSchema(&self) -> ::windows::core::Result<PrintSchema> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrintSchema)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Print3DDevice>> {
        Self::IPrint3DDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPrint3DDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrint3DDeviceStatics<R, F: FnOnce(&IPrint3DDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Print3DDevice, IPrint3DDeviceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Print3DDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Print3DDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DDevice {}
impl ::core::fmt::Debug for Print3DDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Print3DDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Print3DDevice;{041c3d19-9713-42a2-9813-7dc3337428d3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Print3DDevice {
    type Vtable = IPrint3DDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for Print3DDevice {
    const IID: ::windows::core::GUID = <IPrint3DDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Print3DDevice {
    const NAME: &'static str = "Windows.Devices.Printers.Print3DDevice";
}
::windows::core::interface_hierarchy!(Print3DDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Print3DDevice {}
unsafe impl ::core::marker::Sync for Print3DDevice {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct PrintSchema(::windows::core::IUnknown);
impl PrintSchema {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetDefaultPrintTicketAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefaultPrintTicketAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetCapabilitiesAsync<P0, E0>(&self, constrainticket: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStreamWithContentType>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCapabilitiesAsync)(::windows::core::Vtable::as_raw(this), constrainticket.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn MergeAndValidateWithDefaultPrintTicketAsync<P0, E0>(&self, deltaticket: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStreamWithContentType>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MergeAndValidateWithDefaultPrintTicketAsync)(::windows::core::Vtable::as_raw(this), deltaticket.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PrintSchema {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintSchema {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintSchema {}
impl ::core::fmt::Debug for PrintSchema {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintSchema").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintSchema {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.PrintSchema;{c2b98316-26b8-4bfb-8138-9f962c22a35b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PrintSchema {
    type Vtable = IPrintSchema_Vtbl;
}
unsafe impl ::windows::core::Interface for PrintSchema {
    const IID: ::windows::core::GUID = <IPrintSchema as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSchema {
    const NAME: &'static str = "Windows.Devices.Printers.PrintSchema";
}
::windows::core::interface_hierarchy!(PrintSchema, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintSchema {}
unsafe impl ::core::marker::Sync for PrintSchema {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IppAttributeErrorReason(pub i32);
impl IppAttributeErrorReason {
    pub const RequestEntityTooLarge: Self = Self(0i32);
    pub const AttributeNotSupported: Self = Self(1i32);
    pub const AttributeValuesNotSupported: Self = Self(2i32);
    pub const AttributeNotSettable: Self = Self(3i32);
    pub const ConflictingAttributes: Self = Self(4i32);
}
impl ::core::marker::Copy for IppAttributeErrorReason {}
impl ::core::clone::Clone for IppAttributeErrorReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IppAttributeErrorReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IppAttributeErrorReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for IppAttributeErrorReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppAttributeErrorReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IppAttributeErrorReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.IppAttributeErrorReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IppAttributeValueKind(pub i32);
impl IppAttributeValueKind {
    pub const Unsupported: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const NoValue: Self = Self(2i32);
    pub const Integer: Self = Self(3i32);
    pub const Boolean: Self = Self(4i32);
    pub const Enum: Self = Self(5i32);
    pub const OctetString: Self = Self(6i32);
    pub const DateTime: Self = Self(7i32);
    pub const Resolution: Self = Self(8i32);
    pub const RangeOfInteger: Self = Self(9i32);
    pub const Collection: Self = Self(10i32);
    pub const TextWithLanguage: Self = Self(11i32);
    pub const NameWithLanguage: Self = Self(12i32);
    pub const TextWithoutLanguage: Self = Self(13i32);
    pub const NameWithoutLanguage: Self = Self(14i32);
    pub const Keyword: Self = Self(15i32);
    pub const Uri: Self = Self(16i32);
    pub const UriSchema: Self = Self(17i32);
    pub const Charset: Self = Self(18i32);
    pub const NaturalLanguage: Self = Self(19i32);
    pub const MimeMediaType: Self = Self(20i32);
}
impl ::core::marker::Copy for IppAttributeValueKind {}
impl ::core::clone::Clone for IppAttributeValueKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IppAttributeValueKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IppAttributeValueKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for IppAttributeValueKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppAttributeValueKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IppAttributeValueKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.IppAttributeValueKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IppResolutionUnit(pub i32);
impl IppResolutionUnit {
    pub const DotsPerInch: Self = Self(0i32);
    pub const DotsPerCentimeter: Self = Self(1i32);
}
impl ::core::marker::Copy for IppResolutionUnit {}
impl ::core::clone::Clone for IppResolutionUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IppResolutionUnit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IppResolutionUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for IppResolutionUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppResolutionUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IppResolutionUnit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.IppResolutionUnit;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PageConfigurationSource(pub i32);
impl PageConfigurationSource {
    pub const PrintJobConfiguration: Self = Self(0i32);
    pub const PdlContent: Self = Self(1i32);
}
impl ::core::marker::Copy for PageConfigurationSource {}
impl ::core::clone::Clone for PageConfigurationSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PageConfigurationSource {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PageConfigurationSource {
    type Abi = Self;
}
impl ::core::fmt::Debug for PageConfigurationSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PageConfigurationSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PageConfigurationSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.PageConfigurationSource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
