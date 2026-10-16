//! Read a generic password from the keychain without modifying it.
//!
//! Run with `cargo run -p examples-security --example generic_password -- SERVICE ACCOUNT`.
//! Accessing an existing password may prompt for permission. Only its length is printed.
use std::process::ExitCode;
use std::ptr;

use objc2_core_foundation::{
    cf_dictionary, CFBoolean, CFData, CFDictionary, CFRetained, CFString, CFType,
};
use objc2_security::{
    errSecItemNotFound, errSecSuccess, kSecAttrAccount, kSecAttrService, kSecClass,
    kSecClassGenericPassword, kSecMatchLimit, kSecMatchLimitOne, kSecReturnData,
    SecCopyErrorMessageString, SecItemCopyMatching,
};

fn generic_password(service: &str, account: &str) -> Result<Option<CFRetained<CFData>>, i32> {
    let service = CFString::from_str(service);
    let account = CFString::from_str(account);

    // SAFETY: Accessing these external statics is safe.
    let query: CFRetained<CFDictionary<CFString, CFType>> = unsafe {
        cf_dictionary! {
            kSecClass => kSecClassGenericPassword,
            kSecAttrService => &service,
            kSecAttrAccount => &account,
            kSecMatchLimit => kSecMatchLimitOne,
            kSecReturnData => CFBoolean::new(true),
        }
    };

    // The output must be None on entry. The binding takes ownership of the
    // returned object's retain count, so no manual retain or release is needed.
    let mut result = None;
    // SAFETY: Each query value has the type required by its key.
    let status = unsafe { SecItemCopyMatching(&query, Some(&mut result)) };
    if status == errSecItemNotFound {
        return Ok(None);
    }
    if status != errSecSuccess {
        return Err(status);
    }

    // kSecReturnData with kSecMatchLimitOne returns a single CFData.
    let data = result
        .expect("successful query did not return data")
        .downcast::<CFData>()
        .expect("password query did not return CFData");
    Ok(Some(data))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: generic_password SERVICE ACCOUNT");
        return ExitCode::FAILURE;
    }

    match generic_password(&args[1], &args[2]) {
        Ok(Some(data)) => {
            println!("Found password ({} bytes)", data.len());
            ExitCode::SUCCESS
        }
        Ok(None) => {
            println!("No matching password found");
            ExitCode::SUCCESS
        }
        Err(status) => {
            // SAFETY: The reserved parameter must be NULL.
            let message = unsafe { SecCopyErrorMessageString(status, ptr::null_mut()) };
            if let Some(message) = message {
                eprintln!("Keychain query failed ({status}): {message}");
            } else {
                eprintln!("Keychain query failed ({status})");
            }
            ExitCode::FAILURE
        }
    }
}
