use crate::{ez_ffi, EzTransError, TransErr, DLL_PATH, LIBRARY, TRANSLATE_MMNTW};

use std::collections::HashSet;
use std::ffi::{c_void, CStr, CString};
use std::fmt::Write;
use std::os::raw::c_char;

use std::sync::LazyLock;

const SPECIAL_CHARS: LazyLock<HashSet<char>> = LazyLock::new(|| {
    [
        '↔', '◁', '◀', '▷', '▶', '♤', '♠', '♡', '♥', '♧', '♣', '⊙', '◈', '▣', '◐', '◑', '▒', '▤',
        '▥', '▨', '▧', '▦', '▩', '♨', '☏', '☎', '☜', '☞', '↕', '↗', '↙', '↖', '↘', '♩', '♬', '㉿',
        '㈜', '㏇', '™', '㏂', '㏘', '＂', '＇', '∼', 'ˇ', '˘', '˝', '¡', '˚', '˙', '˛', '¿', 'ː',
        '∏', '￦', '℉', '€', '㎕', '㎖', '㎗', 'ℓ', '㎘', '㎣', '㎤', '㎥', '㎦', '㎙', '㎚', '㎛',
        '㎟', '㎠', '㎢', '㏊', '㎍', '㏏', '㎈', '㎉', '㏈', '㎧', '㎨', '㎰', '㎱', '㎲', '㎳',
        '㎴', '㎵', '㎶', '㎷', '㎸', '㎀', '㎁', '㎂', '㎃', '㎄', '㎺', '㎻', '㎼', '㎽', '㎾',
        '㎿', '㎐', '㎑', '㎒', '㎓', '㎔', 'Ω', '㏀', '㏁', '㎊', '㎋', '㎌', '㏖', '㏅', '㎭',
        '㎮', '㎯', '㏛', '㎩', '㎪', '㎫', '㎬', '㏝', '㏐', '㏓', '㏃', '㏉', '㏜', '㏆', '┒',
        '┑', '┚', '┙', '┖', '┕', '┎', '┍', '┞', '┟', '┡', '┢', '┦', '┧', '┪', '┭', '┮', '┵', '┶',
        '┹', '┺', '┽', '┾', '╀', '╁', '╃', '╄', '╅', '╆', '╇', '╈', '╉', '╊', '┱', '┲', 'ⅰ', 'ⅱ',
        'ⅲ', 'ⅳ', 'ⅴ', 'ⅵ', 'ⅶ', 'ⅷ', 'ⅸ', 'ⅹ', '½', '⅓', '⅔', '¼', '¾', '⅛', '⅜', '⅝', '⅞', 'ⁿ',
        '₁', '₂', '₃', '₄', 'Ŋ', 'đ', 'Ħ', 'Ĳ', 'Ŀ', 'Ł', 'Œ', 'Ŧ', 'ħ', 'ı', 'ĳ', 'ĸ', 'ŀ', 'ł',
        'œ', 'ŧ', 'ŋ', 'ŉ', '㉠', '㉡', '㉢', '㉣', '㉤', '㉥', '㉦', '㉧', '㉨', '㉩', '㉪', '㉫',
        '㉬', '㉭', '㉮', '㉯', '㉰', '㉱', '㉲', '㉳', '㉴', '㉵', '㉶', '㉷', '㉸', '㉹', '㉺',
        '㉻', '㈀', '㈁', '㈂', '㈃', '㈄', '㈅', '㈆', '㈇', '㈈', '㈉', '㈊', '㈋', '㈌', '㈍',
        '㈎', '㈏', '㈐', '㈑', '㈒', '㈓', '㈔', '㈕', '㈖', '㈗', '㈘', '㈙', '㈚', '㈛', 'ⓐ',
        'ⓑ', 'ⓒ', 'ⓓ', 'ⓔ', 'ⓕ', 'ⓖ', 'ⓗ', 'ⓘ', 'ⓙ', 'ⓚ', 'ⓛ', 'ⓜ', 'ⓝ', 'ⓞ', 'ⓟ', 'ⓠ', 'ⓡ', 'ⓢ',
        'ⓣ', 'ⓤ', 'ⓥ', 'ⓦ', 'ⓧ', 'ⓨ', 'ⓩ', '①', '②', '③', '④', '⑤', '⑥', '⑦', '⑧', '⑨', '⑩', '⑪',
        '⑫', '⑬', '⑭', '⑮', '⒜', '⒝', '⒞', '⒟', '⒠', '⒡', '⒢', '⒣', '⒤', '⒥', '⒦', '⒧', '⒨', '⒩',
        '⒪', '⒫', '⒬', '⒭', '⒮', '⒯', '⒰', '⒱', '⒲', '⒳', '⒴', '⒵', '⑴', '⑵', '⑶', '⑷', '⑸', '⑹',
        '⑺', '⑻', '⑼', '⑽', '⑾', '⑿', '⒀', '⒁', '⒂',
    ]
    .iter()
    .cloned()
    .collect()
});

pub struct EzTransLib {
    pub ehnd_support: bool,
}

const DEFAULT_PATH: &str = "C:/Program Files (x86)/ChangShinSoft/ezTrans XP";

impl EzTransLib {
    pub fn new(folder_path: Option<&str>) -> Result<Self, EzTransError> {
        let dll_path = format!("{}/J2KEngine.dll", folder_path.unwrap_or(DEFAULT_PATH)); //"C:/Program Files (x86)/ChangShinSoft/ezTrans XP/J2KEngine.dll"
        DLL_PATH
            .set(dll_path.to_string())
            .map_err(|e| EzTransError::OnceLockError(e.into()))?;

        LIBRARY.as_ref().map_err(|e| e.clone())?;

        let ehnd_support = TRANSLATE_MMNTW.is_ok();

        Ok(EzTransLib { ehnd_support })
    }

    pub fn initialize(
        &self,
        init_str: Option<&str>,
        folder_dir: Option<&str>,
    ) -> Result<(), EzTransError> {
        let init_str = CString::new(init_str.unwrap_or("CSUSER123455"))
            .map_err(EzTransError::InvalidString)?;
        let home_dir = CString::new(format!("{}/Dat", folder_dir.unwrap_or(DEFAULT_PATH))) //C:/Program Files (x86)/ChangShinSoft/ezTrans XP/Dat
            .map_err(EzTransError::InvalidString)?;

        let initialize_ex = ez_ffi::INITIALIZE_EX.as_ref().map_err(|e| e.clone())?;
        let ret = unsafe { initialize_ex(init_str.as_ptr(), home_dir.as_ptr()) };
        if ret == 1 {
            Ok(())
        } else {
            Err(EzTransError::InitializationError)
        }
    }

    pub fn translate_and_encode(&self, input: &str) -> Result<String, EzTransError> {
        // 한글이나 특수 문자가 있는지 확인
        let needs_encoding = input.chars().any(|c| {
            c == '@' || c == '\0' || self.is_hangul_range(c as u32) || self.needs_encoding(c)
        });

        let result = if needs_encoding {
            let encoded = self.hangul_encode(input);
            let translated = self.translate(&encoded)?;
            self.hangul_decode(&translated)
        } else {
            // 인코딩/디코딩 없이 직접 번역
            self.translate(input)?
        };

        Ok(result)
    }

    pub fn translate(&self, input: &str) -> Result<String, EzTransError> {
        if self.ehnd_support {
            self.translate_mmntw(input)
        } else {
            self.translate_mmnt(input)
        }
    }

    /// EHND를 사용하여 번역합니다.
    pub fn translate_mmntw(&self, input: &str) -> Result<String, EzTransError> {
        // Convert input to UTF-16/ 끝에 NULL 문자를 추가
        let input_wide: Vec<u16> = input.encode_utf16().chain(std::iter::once(0)).collect();

        let translate_mmntw = ez_ffi::TRANSLATE_MMNTW.as_ref().map_err(|e| e.clone())?;
        let ret = unsafe { translate_mmntw(0, input_wide.as_ptr() as *mut u16) };

        if ret.is_null() {
            return Err(EzTransError::TranslationError(TransErr::NullPointer));
        }

        // 안전하게 UTF-16 문자열의 길이를 찾기
        let result = unsafe {
            let len = (0..).find(|&i| *ret.add(i) == 0).unwrap_or(0);
            String::from_utf16(&std::slice::from_raw_parts(ret, len))
                .map_err(|e| EzTransError::Utf16Error(e.to_string()))?
        };
        self.free_memory(ret as *mut c_void)?;
        Ok(result)
    }

    pub fn translate_mmnt(&self, input: &str) -> Result<String, EzTransError> {
        // Convert input to Shift-JIS
        let input_sjis = encoding_rs::SHIFT_JIS.encode(input).0.to_vec();

        let translate_mmnt = ez_ffi::TRANSLATE_MMNT.as_ref().map_err(|e| e.clone())?;
        let ret = unsafe { translate_mmnt(0, input_sjis.as_ptr() as *mut c_char) };

        if ret.is_null() {
            return Err(EzTransError::TranslationError(TransErr::NullPointer));
        }
        // Convert the result from EUC-KR to UTF-8
        let result = unsafe {
            let c_str = CStr::from_ptr(ret);
            let (decoded, _, had_errors) = encoding_rs::EUC_KR.decode(c_str.to_bytes());
            if had_errors {
                return Err(EzTransError::TranslationError(TransErr::EucKrDecodeFailed));
            }
            decoded.into_owned()
        };

        self.free_memory(ret as *mut c_void)?;

        Ok(result)
    }

    /// 한글 문자나 특수 문자는 "+x" 또는 "+X" 접두사와 함께 16진수 유니코드 값으로 변환됩니다. 그 외의 문자는 그대로 유지됩니다.
    /// 왜 필요? 이지트랜스 엔진은 한글 문자나 특수 문자를 처리할 때 문제가 발생할 수 있습니다. 이를 방지하기 위해 문자를 변환한다고 추측하고 있습니다.
    pub fn hangul_encode(&self, input: &str) -> String {
        let mut output = String::with_capacity(input.len() * 2);

        for c in input.chars() {
            if c == '@' || c == '\0' || self.is_hangul_range(c as u32) {
                write!(&mut output, "+x{:04X}", c as u32).unwrap();
            } else if self.needs_encoding(c) {
                write!(&mut output, "+X{:04X}", c as u32).unwrap();
            } else {
                output.push(c);
            }
        }
        output
    }

    /// 한글 문자를 판별합니다.
    pub const fn is_hangul_range(&self, code: u32) -> bool {
        // Hangul Jamo //한글 자모
        (code >= 0x1100 && code <= 0x11FF)
        // Hangul Compatibility Jamo //한글 호환 자모
        || (code >= 0x3130 && code <= 0x318F)
        // Hangul Jamo Extended-A //한글 자모 확장-A
        || (code >= 0xA960 && code <= 0xA97F)
        // Hangul Syllables //한글 음절
        || (code >= 0xAC00 && code <= 0xD7A3)
        // Hangul Jamo Extended-B //한글 자모 확장-B
        || (code >= 0xD7B0 && code <= 0xD7FF)
    }

    /// 이지트랜스 엔진이 처리할 수 없는 문자가 문자열에 들어있는지 확인합니다.
    fn needs_encoding(&self, c: char) -> bool {
        SPECIAL_CHARS.contains(&c)
    }

    /// "+x" 또는 "+X" 접두사와 함께 16진수 유니코드 값으로 변환된 문자를 원래 문자로 변환합니다.
    pub fn hangul_decode(&self, input: &str) -> String {
        let mut output = String::with_capacity(input.len());
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '+' => match chars.peek() {
                    Some('x') | Some('X') => {
                        chars.next(); // Consume 'x' or 'X'
                        let hex: String = chars.by_ref().take(4).collect();
                        if hex.len() == 4 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
                            if let Ok(code) = u32::from_str_radix(&hex, 16) {
                                if let Some(decoded_char) = std::char::from_u32(code) {
                                    output.push(decoded_char);
                                    continue;
                                }
                            }
                        }
                        output.push('+');
                        output.push_str(&hex);
                    }
                    _ => output.push('+'),
                },
                _ => output.push(ch),
            }
        }

        output
    }

    /// 메모리를 해제합니다.
    pub fn free_memory(&self, ret: *mut c_void) -> Result<(), EzTransError> {
        unsafe { ez_ffi::FREE_MEM.as_ref().map_err(|e| e.clone())?(ret) };
        Ok(())
    }

    pub fn terminate(&self) -> Result<(), EzTransError> {
        let terminate = ez_ffi::TERMINATE.as_ref().map_err(|e| e.clone())?;
        let ret = unsafe { terminate() };

        if ret == 0 {
            Ok(())
        } else {
            Err(EzTransError::TerminationError)
        }
    }
}

impl Drop for EzTransLib {
    fn drop(&mut self) {
        let _ = self.terminate();
    }
}

// Implement hangul_encode and hangul_decode functions here
// These functions should be implemented based on the C++ code in eztrans.cpp

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hangul_encode_decode() {
        let ez_trans = EzTransLib::new(None).unwrap();
        ez_trans.initialize(None, None).unwrap();
        let original = "테스트@漢字㉷";
        let encoded = ez_trans.hangul_encode(original);
        println!("encoded: {}", encoded);
        let decoded = ez_trans.hangul_decode(&encoded);
        println!("decoded: {}", decoded);
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_hangul_encode() {
        let ez_trans = EzTransLib::new(None).unwrap();
        ez_trans.initialize(None, None).unwrap();
        let input = "Hello@세계";
        let expected = "Hello+x0040+xC138+xACC4"; // 예시, 실제 인코딩 결과에 맞게 수정
        let encoded = ez_trans.hangul_encode(input);
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_hangul_decode() {
        let ez_trans = EzTransLib::new(None).unwrap();
        ez_trans.initialize(None, None).unwrap();
        let input = "Hello+x0040세계";
        let expected = "Hello@세계";
        let decoded = ez_trans.hangul_decode(input);
        assert_eq!(decoded, expected);
    }

    #[test]
    fn test_translate() {
        let ez_trans = EzTransLib::new(None).unwrap();
        ez_trans.initialize(None, None).unwrap();
        let original = "おはようございます。";
        let translated = ez_trans.translate(original).unwrap();
        // 기대하는 번역 결과에 맞게 assert 작성
        assert_eq!(translated, "안녕하세요.");
    }
}
