use crate::ffi::{PyObject, PyUnicode_Check, Py_UCS1, Py_UCS2, Py_UCS4, Py_hash_t, Py_ssize_t};
use libc::wchar_t;
use std::os::raw::{c_char, c_int, c_uint, c_void};

// skipped Py_UNICODE_ISSPACE()
// skipped Py_UNICODE_ISLOWER()
// skipped Py_UNICODE_ISUPPER()
// skipped Py_UNICODE_ISTITLE()
// skipped Py_UNICODE_ISLINEBREAK
// skipped Py_UNICODE_TOLOWER
// skipped Py_UNICODE_TOUPPER
// skipped Py_UNICODE_TOTITLE
// skipped Py_UNICODE_ISDECIMAL
// skipped Py_UNICODE_ISDIGIT
// skipped Py_UNICODE_ISNUMERIC
// skipped Py_UNICODE_ISPRINTABLE
// skipped Py_UNICODE_TODECIMAL
// skipped Py_UNICODE_TODIGIT
// skipped Py_UNICODE_TONUMERIC
// skipped Py_UNICODE_ISALPHA
// skipped Py_UNICODE_ISALNUM
// skipped Py_UNICODE_COPY
// skipped Py_UNICODE_FILL
// skipped Py_UNICODE_IS_SURROGATE
// skipped Py_UNICODE_IS_HIGH_SURROGATE
// skipped Py_UNICODE_IS_LOW_SURROGATE
// skipped Py_UNICODE_JOIN_SURROGATES
// skipped Py_UNICODE_HIGH_SURROGATE
// skipped Py_UNICODE_LOW_SURROGATE

#[repr(C)]
pub struct PyASCIIObject {
    pub ob_base: PyObject,
    pub length: Py_ssize_t,
    pub hash: Py_hash_t,
    /// A bit field with various properties.
    ///
    /// Rust doesn't expose bitfields. So we have accessor functions for
    /// retrieving values.
    ///
    /// unsigned int interned:2; // SSTATE_* constants.
    /// unsigned int kind:3;     // PyUnicode_*_KIND constants.
    /// unsigned int compact:1;
    /// unsigned int ascii:1;
    /// unsigned int ready:1;
    /// unsigned int :24;
    pub state: u32,
    pub wstr: *mut wchar_t,
}

impl PyASCIIObject {
    #[inline]
    pub fn interned(&self) -> c_uint {
        self.state & 3
    }

    #[inline]
    pub fn kind(&self) -> c_uint {
        (self.state >> 2) & 7
    }

    #[inline]
    pub fn compact(&self) -> c_uint {
        (self.state >> 5) & 1
    }

    #[inline]
    pub fn ascii(&self) -> c_uint {
        (self.state >> 6) & 1
    }

    #[inline]
    pub fn ready(&self) -> c_uint {
        (self.state >> 7) & 1
    }
}

#[repr(C)]
pub struct PyCompactUnicodeObject {
    pub _base: PyASCIIObject,
    pub utf8_length: Py_ssize_t,
    pub utf8: *mut c_char,
    pub wstr_length: Py_ssize_t,
}

#[repr(C)]
pub union PyUnicodeObjectData {
    any: *mut c_void,
    latin1: *mut Py_UCS1,
    ucs2: *mut Py_UCS2,
    ucs4: *mut Py_UCS4,
}

#[repr(C)]
pub struct PyUnicodeObject {
    pub _base: PyCompactUnicodeObject,
    pub data: PyUnicodeObjectData,
}

extern "C" {
    #[cfg(not(PyPy))]
    pub fn _PyUnicode_CheckConsistency(op: *mut PyObject, check_content: c_int) -> c_int;
}

// skipped PyUnicode_GET_SIZE
// skipped PyUnicode_GET_DATA_SIZE
// skipped PyUnicode_AS_UNICODE
// skipped PyUnicode_AS_DATA

pub const SSTATE_NOT_INTERNED: c_uint = 0;
pub const SSTATE_INTERNED_MORTAL: c_uint = 1;
pub const SSTATE_INTERNED_IMMORTAL: c_uint = 2;

#[inline]
pub unsafe fn PyUnicode_IS_ASCII(op: *mut PyObject) -> c_uint {
    debug_assert!(PyUnicode_Check(op) != 0);
    debug_assert!(PyUnicode_IS_READY(op) != 0);

    (*(op as *mut PyASCIIObject)).ascii()
}

#[inline]
pub unsafe fn PyUnicode_IS_COMPACT(op: *mut PyObject) -> c_uint {
    (*(op as *mut PyASCIIObject)).compact()
}

#[inline]
pub unsafe fn PyUnicode_IS_COMPACT_ASCII(op: *mut PyObject) -> c_uint {
    if (*(op as *mut PyASCIIObject)).ascii() != 0 && PyUnicode_IS_COMPACT(op) != 0 {
        1
    } else {
        0
    }
}

#[cfg(not(Py_3_12))]
#[cfg_attr(Py_3_10, deprecated(note = "Python 3.10"))]
pub const PyUnicode_WCHAR_KIND: c_uint = 0;

pub const PyUnicode_1BYTE_KIND: c_uint = 1;
pub const PyUnicode_2BYTE_KIND: c_uint = 2;
pub const PyUnicode_4BYTE_KIND: c_uint = 4;

#[inline]
pub unsafe fn PyUnicode_1BYTE_DATA(op: *mut PyObject) -> *mut Py_UCS1 {
    PyUnicode_DATA(op) as *mut Py_UCS1
}

#[inline]
pub unsafe fn PyUnicode_2BYTE_DATA(op: *mut PyObject) -> *mut Py_UCS2 {
    PyUnicode_DATA(op) as *mut Py_UCS2
}

#[inline]
pub unsafe fn PyUnicode_4BYTE_DATA(op: *mut PyObject) -> *mut Py_UCS4 {
    PyUnicode_DATA(op) as *mut Py_UCS4
}

#[inline]
pub unsafe fn PyUnicode_KIND(op: *mut PyObject) -> c_uint {
    debug_assert!(PyUnicode_Check(op) != 0);
    debug_assert!(PyUnicode_IS_READY(op) != 0);

    (*(op as *mut PyASCIIObject)).kind()
}

#[inline]
pub unsafe fn _PyUnicode_COMPACT_DATA(op: *mut PyObject) -> *mut c_void {
    if PyUnicode_IS_ASCII(op) != 0 {
        (op as *mut PyASCIIObject).offset(1) as *mut c_void
    } else {
        (op as *mut PyCompactUnicodeObject).offset(1) as *mut c_void
    }
}

#[inline]
pub unsafe fn _PyUnicode_NONCOMPACT_DATA(op: *mut PyObject) -> *mut c_void {
    debug_assert!(!(*(op as *mut PyUnicodeObject)).data.any.is_null());

    (*(op as *mut PyUnicodeObject)).data.any
}

#[inline]
pub unsafe fn PyUnicode_DATA(op: *mut PyObject) -> *mut c_void {
    debug_assert!(PyUnicode_Check(op) != 0);

    if PyUnicode_IS_COMPACT(op) != 0 {
        _PyUnicode_COMPACT_DATA(op)
    } else {
        _PyUnicode_NONCOMPACT_DATA(op)
    }
}

// skipped PyUnicode_WRITE
// skipped PyUnicode_READ
// skipped PyUnicode_READ_CHAR

#[inline]
pub unsafe fn PyUnicode_GET_LENGTH(op: *mut PyObject) -> Py_ssize_t {
    debug_assert!(PyUnicode_Check(op) != 0);
    debug_assert!(PyUnicode_IS_READY(op) != 0);

    (*(op as *mut PyASCIIObject)).length
}

#[inline]
pub unsafe fn PyUnicode_IS_READY(op: *mut PyObject) -> c_uint {
    (*(op as *mut PyASCIIObject)).ready()
}

#[cfg(not(Py_3_12))]
#[cfg_attr(Py_3_10, deprecated(note = "Python 3.10"))]
#[inline]
pub unsafe fn PyUnicode_READY(op: *mut PyObject) -> c_int {
    debug_assert!(PyUnicode_Check(op) != 0);

    if PyUnicode_IS_READY(op) != 0 {
        0
    } else {
        _PyUnicode_Ready(op)
    }
}

// skipped PyUnicode_MAX_CHAR_VALUE
// skipped _PyUnicode_get_wstr_length
// skipped PyUnicode_WSTR_LENGTH

extern "C" {
    // move PyUnicode_New

    pub fn _PyUnicode_Ready(unicode: *mut PyObject) -> c_int;

    // skipped _PyUnicode_Copy
    // move PyUnicode_CopyCharacters
    // skipped _PyUnicode_FastCopyCharacters
    // move PyUnicode_Fill
    // skipped _PyUnicode_FastFill
    // move PyUnicode_FromUnicode
    // move PyUnicode_FromKindAndData
    // skipped _PyUnicode_FromASCII
    // skipped _PyUnicode_FindMaxChar
    // move PyUnicode_AsUnicode
    // skipped _PyUnicode_AsUnicode
    // move PyUnicode_AsUnicodeAndSize
    // skipped PyUnicode_GetMax
}

// skipped _PyUnicodeWriter
// skipped _PyUnicodeWriter_Init
// skipped _PyUnicodeWriter_Prepare
// skipped _PyUnicodeWriter_PrepareInternal
// skipped _PyUnicodeWriter_PrepareKind
// skipped _PyUnicodeWriter_PrepareKindInternal
// skipped _PyUnicodeWriter_WriteChar
// skipped _PyUnicodeWriter_WriteStr
// skipped _PyUnicodeWriter_WriteSubstring
// skipped _PyUnicodeWriter_WriteASCIIString
// skipped _PyUnicodeWriter_WriteLatin1String
// skipped _PyUnicodeWriter_Finish
// skipped _PyUnicodeWriter_Dealloc
// skipped _PyUnicode_FormatAdvancedWriter

// move PyUnicode_AsUTF8AndSize
// skipped _PyUnicode_AsStringAndSize
// move PyUnicode_AsUTF8
// skipped _PyUnicode_AsString

// move PyUnicode_Encode
// move PyUnicode_EncodeUTF7
// skipped _PyUnicode_EncodeUTF7

// skipped _PyUnicode_AsUTF8String
// move PyUnicode_EncodeUTF8

// move PyUnicode_EncodeUTF32
// skipped _PyUnicode_EncodeUTF32

// move PyUnicode_EncodeUTF16
// skipped _PyUnicode_EncodeUTF16

// skipped _PyUnicode_DecodeUnicodeEscape
// move PyUnicode_EncodeUnicodeEscape
// move PyUnicode_EncodeRawUnicodeEscape

// skipped _PyUnicode_AsLatin1String
// move PyUnicode_EncodeLatin1

// skipped _PyUnicode_AsASCIIString
// move PyUnicode_EncodeASCII

// move PyUnicode_EncodeCharmap
// skipped _PyUnicode_EncodeCharmap
// move PyUnicode_TranslateCharmap

// skipped PyUnicode_EncodeMBCS

// move PyUnicode_EncodeDecimal
// move PyUnicode_TransformDecimalToASCII
// skipped _PyUnicode_TransformDecimalAndSpaceToASCII

// skipped _PyUnicode_JoinArray
// skipped _PyUnicode_EqualToASCIIId
// skipped _PyUnicode_EqualToASCIIString
// skipped _PyUnicode_XStrip
// skipped _PyUnicode_InsertThousandsGrouping

// skipped _Py_ascii_whitespace

// skipped _PyUnicode_IsLowercase
// skipped _PyUnicode_IsUppercase
// skipped _PyUnicode_IsTitlecase
// skipped _PyUnicode_IsXidStart
// skipped _PyUnicode_IsXidContinue
// skipped _PyUnicode_IsWhitespace
// skipped _PyUnicode_IsLinebreak
// skipped _PyUnicode_ToLowercase
// skipped _PyUnicode_ToUppercase
// skipped _PyUnicode_ToTitlecase
// skipped _PyUnicode_ToLowerFull
// skipped _PyUnicode_ToTitleFull
// skipped _PyUnicode_ToUpperFull
// skipped _PyUnicode_ToFoldedFull
// skipped _PyUnicode_IsCaseIgnorable
// skipped _PyUnicode_IsCased
// skipped _PyUnicode_ToDecimalDigit
// skipped _PyUnicode_ToDigit
// skipped _PyUnicode_ToNumeric
// skipped _PyUnicode_IsDecimalDigit
// skipped _PyUnicode_IsDigit
// skipped _PyUnicode_IsNumeric
// skipped _PyUnicode_IsPrintable
// skipped _PyUnicode_IsAlpha
// skipped Py_UNICODE_strlen
// skipped Py_UNICODE_strcpy
// skipped Py_UNICODE_strcat
// skipped Py_UNICODE_strncpy
// skipped Py_UNICODE_strcmp
// skipped Py_UNICODE_strncmp
// skipped Py_UNICODE_strchr
// skipped Py_UNICODE_strrchr
// skipped _PyUnicode_FormatLong
// skipped PyUnicode_AsUnicodeCopy
// skipped _PyUnicode_FromId
// skipped _PyUnicode_EQ
// skipped _PyUnicode_ScanIdentifier

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::PyString;
    use crate::{AsPyPointer, Python};

    #[test]
    fn ascii_object_bitfield() {
        let ob_base: PyObject = unsafe { std::mem::zeroed() };

        let mut o = PyASCIIObject {
            ob_base,
            length: 0,
            hash: 0,
            state: 0,
            wstr: std::ptr::null_mut() as *mut wchar_t,
        };

        assert_eq!(o.interned(), 0);
        assert_eq!(o.kind(), 0);
        assert_eq!(o.compact(), 0);
        assert_eq!(o.ascii(), 0);
        assert_eq!(o.ready(), 0);

        for i in 0..4 {
            o.state = i;
            assert_eq!(o.interned(), i);
        }

        for i in 0..8 {
            o.state = i << 2;
            assert_eq!(o.kind(), i);
        }

        o.state = 1 << 5;
        assert_eq!(o.compact(), 1);

        o.state = 1 << 6;
        assert_eq!(o.ascii(), 1);

        o.state = 1 << 7;
        assert_eq!(o.ready(), 1);
    }

    #[test]
    fn ascii() {
        Python::with_gil(|py| {
            // This test relies on implementation details of PyString.
            let s = PyString::new(py, "hello, world");
            let ptr = s.as_ptr();

            unsafe {
                let ascii_ptr = ptr as *mut PyASCIIObject;
                let ascii = ascii_ptr.as_ref().unwrap();

                assert_eq!(ascii.interned(), 0);
                assert_eq!(ascii.kind(), PyUnicode_1BYTE_KIND);
                assert_eq!(ascii.compact(), 1);
                assert_eq!(ascii.ascii(), 1);
                assert_eq!(ascii.ready(), 1);

                assert_eq!(PyUnicode_IS_ASCII(ptr), 1);
                assert_eq!(PyUnicode_IS_COMPACT(ptr), 1);
                assert_eq!(PyUnicode_IS_COMPACT_ASCII(ptr), 1);

                assert!(!PyUnicode_1BYTE_DATA(ptr).is_null());
                // 2 and 4 byte macros return nonsense for this string instance.
                assert_eq!(PyUnicode_KIND(ptr), PyUnicode_1BYTE_KIND);

                assert!(!_PyUnicode_COMPACT_DATA(ptr).is_null());
                // _PyUnicode_NONCOMPACT_DATA isn't valid for compact strings.
                assert!(!PyUnicode_DATA(ptr).is_null());

                assert_eq!(PyUnicode_GET_LENGTH(ptr), s.len().unwrap() as _);
                assert_eq!(PyUnicode_IS_READY(ptr), 1);

                // This has potential to mutate object. But it should be a no-op since
                // we're already ready.
                assert_eq!(PyUnicode_READY(ptr), 0);
            }
        })
    }

    #[test]
    fn ucs4() {
        Python::with_gil(|py| {
            let s = "哈哈🐈";
            let py_string = PyString::new(py, s);
            let ptr = py_string.as_ptr();

            unsafe {
                let ascii_ptr = ptr as *mut PyASCIIObject;
                let ascii = ascii_ptr.as_ref().unwrap();

                assert_eq!(ascii.interned(), 0);
                assert_eq!(ascii.kind(), PyUnicode_4BYTE_KIND);
                assert_eq!(ascii.compact(), 1);
                assert_eq!(ascii.ascii(), 0);
                assert_eq!(ascii.ready(), 1);

                assert_eq!(PyUnicode_IS_ASCII(ptr), 0);
                assert_eq!(PyUnicode_IS_COMPACT(ptr), 1);
                assert_eq!(PyUnicode_IS_COMPACT_ASCII(ptr), 0);

                assert!(!PyUnicode_4BYTE_DATA(ptr).is_null());
                assert_eq!(PyUnicode_KIND(ptr), PyUnicode_4BYTE_KIND);

                assert!(!_PyUnicode_COMPACT_DATA(ptr).is_null());
                // _PyUnicode_NONCOMPACT_DATA isn't valid for compact strings.
                assert!(!PyUnicode_DATA(ptr).is_null());

                assert_eq!(PyUnicode_GET_LENGTH(ptr), py_string.len().unwrap() as _);
                assert_eq!(PyUnicode_IS_READY(ptr), 1);

                // This has potential to mutate object. But it should be a no-op since
                // we're already ready.
                assert_eq!(PyUnicode_READY(ptr), 0);
            }
        })
    }
}