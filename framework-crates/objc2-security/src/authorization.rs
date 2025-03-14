use core::ffi::c_char;
use core::ptr::NonNull;

use crate::{AuthorizationFlags, AuthorizationRef, AuthorizationString, OSStatus};

// Manual re-definition: see #711.
extern "C-unwind" {
    /// Run an executable tool with enhanced privileges after passing
    /// suitable authorization procedures.
    ///
    ///
    /// Parameter `authorization`: An authorization reference that is used to authorize
    /// access to the enhanced privileges. It is also passed to the tool for
    /// further access control.
    ///
    /// Parameter `pathToTool`: Full pathname to the tool that should be executed
    /// with enhanced privileges.
    ///
    /// Parameter `options`: Option bits (reserved). Must be zero.
    ///
    /// Parameter `arguments`: An argv-style vector of strings to be passed to the tool.
    ///
    /// Parameter `communicationsPipe`: Assigned a UNIX stdio FILE pointer for
    /// a bidirectional pipe to communicate with the tool. The tool will have
    /// this pipe as its standard I/O channels (stdin/stdout). If NULL, do not
    /// establish a communications pipe.
    ///
    ///
    /// This function has been deprecated and should no longer be used.
    /// Use a launchd-launched helper tool and/or the Service Management framework
    /// for this functionality.
    #[deprecated]
    pub fn AuthorizationExecuteWithPrivileges(
        authorization: AuthorizationRef,
        path_to_tool: NonNull<c_char>,
        options: AuthorizationFlags,
        arguments: NonNull<AuthorizationString>,
        communications_pipe: *mut *mut libc::FILE,
    ) -> OSStatus;
}
