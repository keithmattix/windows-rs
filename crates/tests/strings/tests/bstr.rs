use windows::{core::Result, Win32::Foundation::*};
use windows_strings::*;

#[test]
fn test() {
    let b: BSTR = "hello".into();
    assert_eq!(b, "hello");
}

#[test]
fn clone() {
    let a: BSTR = "hello".into();
    assert!(!a.is_empty());
    assert!(a.len() == 5);
    let b = a.clone();
    assert_eq!(a, "hello");
    assert_eq!(b, "hello");
    assert_eq!("hello", a);

    let a = BSTR::default();
    assert!(a.is_empty());
    assert!(a.len() == 0);
    let b = a.clone();
    assert_eq!(a, "");
    assert_eq!(b, "");

    let a = BSTR::new();
    assert!(a.is_empty());
    assert!(a.len() == 0);
    assert_eq!(a.len(), 0);
    assert_eq!(a.as_wide().len(), 0);

    let wide = &[0x68, 0x65, 0x6c, 0x6c, 0x6f];
    let a = BSTR::from_wide(wide);
    assert!(!a.is_empty());
    assert!(a.len() == 5);
    assert_eq!(a.as_wide().len(), 5);
    assert_eq!(a.as_wide(), wide);
    assert_eq!(a, "hello");

    let a: BSTR = "".into();
    assert!(a.is_empty());
    assert!(a.len() == 0);

    let a: BSTR = unsafe { SysAllocStringLen(None) };
    assert!(a.is_empty());
    assert!(a.len() == 0);

    let a = BSTR::from("a");
    assert_eq!(a, String::from("a"));
    assert_eq!(String::from("a"), a);
}

#[test]
fn interop() -> Result<()> {
    unsafe {
        let b: BSTR = "hello".into();
        SysAddRefString(&b)?;
        SysFreeString(&b);
        Ok(())
    }
}
