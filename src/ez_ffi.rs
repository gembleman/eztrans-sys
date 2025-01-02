#![allow(non_camel_case_types)]
use std::{
    ffi::{c_char, c_int, c_void},
    sync::{LazyLock, OnceLock},
};

use libloading::{Library, Symbol};

use crate::EzTransError;

// Type definitions for all EzTrans engine functions
#[cfg(feature = "free-mem")]
pub type J2K_FreeMem = unsafe extern "stdcall" fn(*mut c_void);
#[cfg(feature = "get-prior-dict")]
pub type J2K_GetPriorDict = unsafe extern "stdcall" fn() -> c_int;
#[cfg(feature = "get-property")]
pub type J2K_GetProperty = unsafe extern "stdcall" fn(c_int) -> c_int;
#[cfg(feature = "initialize")]
pub type J2K_Initialize = unsafe extern "stdcall" fn() -> c_int;
#[cfg(feature = "initialize-ex")]
pub type J2K_InitializeEx = unsafe extern "stdcall" fn(*const c_char, *const c_char) -> c_int;
#[cfg(feature = "reload-user-dict")]
pub type J2K_ReloadUserDict = unsafe extern "stdcall" fn() -> c_int;
#[cfg(feature = "set-del-jpn")]
pub type J2K_SetDelJPN = unsafe extern "stdcall" fn(c_int) -> c_int;
#[cfg(feature = "set-field")]
pub type J2K_SetField = unsafe extern "stdcall" fn(c_int) -> c_int;
#[cfg(feature = "set-hnj2han")]
pub type J2K_SetHnj2han = unsafe extern "stdcall" fn(c_int) -> c_int;
#[cfg(feature = "set-jwin")]
pub type J2K_SetJWin = unsafe extern "stdcall" fn(c_int) -> c_int;
#[cfg(feature = "set-prior-dict")]
pub type J2K_SetPriorDict = unsafe extern "stdcall" fn(*const c_char) -> c_int;
#[cfg(feature = "set-property")]
pub type J2K_SetProperty = unsafe extern "stdcall" fn(c_int, c_int) -> c_int;
#[cfg(feature = "stop-translation")]
pub type J2K_StopTranslation = unsafe extern "stdcall" fn() -> c_int;
#[cfg(feature = "terminate")]
pub type J2K_Terminate = unsafe extern "stdcall" fn() -> c_int;
#[cfg(feature = "translate-chat")]
pub type J2K_TranslateChat = unsafe extern "stdcall" fn(*const c_char) -> *mut c_char;
#[cfg(feature = "translate-fm")]
pub type J2K_TranslateFM = unsafe extern "stdcall" fn(*const c_char) -> *mut c_char;
#[cfg(feature = "translate-mm")]
pub type J2K_TranslateMM = unsafe extern "stdcall" fn(*const c_char) -> *mut c_char;
#[cfg(feature = "translate-mmex")]
pub type J2K_TranslateMMEx = unsafe extern "stdcall" fn(c_int, *const c_char) -> *mut c_char;
#[cfg(feature = "translate-mmnt")]
pub type J2K_TranslateMMNT = unsafe extern "stdcall" fn(c_int, *const c_char) -> *mut c_char;
#[cfg(feature = "translate-mmntw")]
pub type J2K_TranslateMMNTW = unsafe extern "stdcall" fn(c_int, *const u16) -> *mut u16;

// DLL 경로를 저장할 전역 변수
pub static DLL_PATH: OnceLock<String> = OnceLock::new();

pub static LIBRARY: LazyLock<Result<Library, EzTransError>> = LazyLock::new(|| {
    let path = DLL_PATH.get().ok_or(EzTransError::DllPathNotSet)?;
    unsafe { Library::new(path).map_err(|e| EzTransError::LibraryLoadError(e.to_string().into())) }
});

//메모리 해제 함수. EzTrans 엔진에 의해 할당된 메모리를 해제합니다.
#[cfg(feature = "free-mem")]
pub static FREE_MEM: LazyLock<Result<Symbol<J2K_FreeMem>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_FreeMem")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string().into()))
    });
/// 현재 설정된 사용자 사전의 우선순위를 가져옵니다.
#[cfg(feature = "get-prior-dict")]
pub static GET_PRIOR_DICT: LazyLock<Result<Symbol<J2K_GetPriorDict>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_GetPriorDict")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string()))
    });
/// 특정 속성의 현재 값을 가져옵니다.
#[cfg(feature = "get-property")]
pub static GET_PROPERTY: LazyLock<Result<Symbol<J2K_GetProperty>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_GetProperty")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string()))
    });
/// EzTrans 엔진을 초기화합니다. 기본 설정을 사용합니다.
#[cfg(feature = "initialize")]
pub static INITIALIZE: LazyLock<Result<Symbol<J2K_Initialize>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_Initialize")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string()))
    });
/// EzTrans 엔진을 초기화합니다. 사용자 지정 설정을 사용할 수 있습니다.
#[cfg(feature = "initialize-ex")]
pub static INITIALIZE_EX: LazyLock<Result<Symbol<J2K_InitializeEx>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_InitializeEx")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string().into()))
    });
/// 사용자 사전을 다시 로드합니다.
#[cfg(feature = "reload-user-dict")]
pub static RELOAD_USER_DICT: LazyLock<Result<Symbol<J2K_ReloadUserDict>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_ReloadUserDict")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string()))
    });
/// 일본어 문장 구분 기능을 설정합니다.
#[cfg(feature = "set-del-jpn")]
pub static SET_DEL_JPN: LazyLock<Result<Symbol<J2K_SetDelJPN>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_SetDelJPN")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string()))
    });
/// 번역 분야를 설정합니다. 예: 일반, 과학기술, 컴퓨터 등.
#[cfg(feature = "set-field")]
pub static SET_FIELD: LazyLock<Result<Symbol<J2K_SetField>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_SetField")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string()))
    });
/// 한자를 한글로 변환하는 옵션을 설정합니다.
#[cfg(feature = "set-hnj2han")]
pub static SET_HNJ2HAN: LazyLock<Result<Symbol<J2K_SetHnj2han>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_SetHnj2han")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string()))
    });
/// J-Win 모드를 설정합니다. (일본어 윈도우 호환성 관련)
#[cfg(feature = "set-jwin")]
pub static SET_JWIN: LazyLock<Result<Symbol<J2K_SetJWin>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_SetJWin")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string()))
    });
/// 사용자 사전의 우선순위를 설정합니다.
#[cfg(feature = "set-prior-dict")]
pub static SET_PRIOR_DICT: LazyLock<Result<Symbol<J2K_SetPriorDict>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_SetPriorDict")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string()))
    });
/// 특정 속성의 값을 설정합니다.
#[cfg(feature = "set-property")]
pub static SET_PROPERTY: LazyLock<Result<Symbol<J2K_SetProperty>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_SetProperty")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string()))
    });
/// 현재 진행 중인 번역 작업을 중지합니다.
#[cfg(feature = "stop-translation")]
pub static STOP_TRANSLATION: LazyLock<Result<Symbol<J2K_StopTranslation>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_StopTranslation")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string()))
    });
/// EzTrans 엔진을 종료하고 사용된 리소스를 해제합니다.
#[cfg(feature = "terminate")]
pub static TERMINATE: LazyLock<Result<Symbol<J2K_Terminate>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_Terminate")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string().into()))
    });
/// 채팅 모드에서 텍스트를 번역합니다.
#[cfg(feature = "translate-chat")]
pub static TRANSLATE_CHAT: LazyLock<Result<Symbol<J2K_TranslateChat>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_TranslateChat")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string()))
    });
/// 전문 번역 모드에서 텍스트를 번역합니다.
#[cfg(feature = "translate-fm")]
pub static TRANSLATE_FM: LazyLock<Result<Symbol<J2K_TranslateFM>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_TranslateFM")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string()))
    });
/// 일반 번역 모드에서 텍스트를 번역합니다.
#[cfg(feature = "translate-mm")]
pub static TRANSLATE_MM: LazyLock<Result<Symbol<J2K_TranslateMM>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_TranslateMM")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string()))
    });
/// 확장된 일반 번역 모드에서 텍스트를 번역합니다.
#[cfg(feature = "translate-mmex")]
pub static TRANSLATE_MMEX: LazyLock<Result<Symbol<J2K_TranslateMMEx>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_TranslateMMEx")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string()))
    });
/// No Thread 모드에서 텍스트를 번역합니다. (멀티스레드 환경에서 유용)
#[cfg(feature = "translate-mmnt")]
pub static TRANSLATE_MMNT: LazyLock<Result<Symbol<J2K_TranslateMMNT>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_TranslateMMNT")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string()))
    });
/// No Thread 모드에서 와이드 문자열(Unicode) 텍스트를 번역합니다.
#[cfg(feature = "translate-mmntw")]
pub static TRANSLATE_MMNTW: LazyLock<Result<Symbol<J2K_TranslateMMNTW>, EzTransError>> =
    LazyLock::new(|| unsafe {
        LIBRARY
            .as_ref()
            .map_err(|e| e.clone())?
            .get(b"J2K_TranslateMMNTW")
            .map_err(|e| EzTransError::SymbolLoadError(e.to_string()))
    });
