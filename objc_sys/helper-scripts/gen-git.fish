#!/usr/local/bin/fish

# Yup, this is terrible, but was a great help in creating the correct implementations

# Source repo should be a path to https://github.com/madsmtm/objc4-mirror.git
set source_repo $argv[1]
set to_repo $argv[2]

git init $to_repo -b master

cp headers.h $source_repo/runtime/headers.h

set tags \
    # For these, headers.h needs to be set up differently
    # macos-10.0 \
    # macos-10.1 \
    # macos-10.2 \
    # macos-10.3 \
    # macos-10.3.3 \
    # macos-10.4 \
    # macos-10.4.3 \
    # macos-10.4.4.x86 \
    # macos-10.4.6.ppc \
    macos-10.5 \
    macos-10.5.2 \
    macos-10.5.5 \
    macos-10.6 \
    macos-10.6.2 \
    macos-10.6.8 \
    # The actual minimum target we're interested in
    macos-10.7 \
    macos-10.7.3 \
    macos-10.8 \
    macos-10.8.2 \
    macos-10.9 \
    macos-10.10 \
    macos-10.10.2 \
    macos-10.11 \
    macos-10.12 \
    macos-10.12.4 \
    macos-10.12.6 \
    macos-10.13 \
    macos-10.14 \
    macos-10.14.1 \
    macos-10.14.4 \
    macos-10.15 \
    macos-10.15.1 \
    macos-10.15.2 \
    macos-10.15.4 \
    macos-11.0.1

# Uncomment for easier debugging
# set tags master

function bfn
    echo "--blocklist-function=$argv[1]"
end

function bty
    echo "--blocklist-type=$argv[1]"
end

function blocklisted
    # Added in 10.8
    bfn objc_terminate

    # Added in 10.10
    bfn object_isClass

    # Removed in 10.11
    bfn objc_setFutureClass

    # Added in 10.12
    bfn object_setIvarWithStrongDefault
    bfn object_setInstanceVariableWithStrongDefault
    bfn protocol_copyPropertyList2

    # Not present between 10.8 and 10.12, and declared OBJC_UNAVAILABLE
    bty objc_objectptr_t
    bfn objc_retainedObject
    bfn objc_unretainedObject
    bfn objc_unretainedPointer

    # Removed in 10.13
    bfn objc_sync_wait
    bfn objc_sync_notify
    bfn objc_sync_notifyAll

    # Added in 10.14
    bty objc_hook_getImageName
    bfn objc_setHook_getImageName

    # Added in 10.14.4
    bty objc_hook_getClass
    bfn objc_setHook_getClass
    bfn _objc_realizeClassFromSwift

    # Added in 10.15, removed in 11.0.1
    bty objc_hook_setAssociatedObject
    bfn objc_setHook_setAssociatedObject

    # Added in 10.15
    bty mach_header
    bty objc_func_loadImage
    bfn objc_addLoadImageFunc

    # Added in 11.0.1
    bty objc_hook_lazyClassNamer
    bfn objc_setHook_lazyClassNamer
end

# Marked with __[PLATFORM]_DEPRECATED
function deprecated
    bfn class_setSuperclass # not recommended
    bfn class_lookupMethod # use class_getMethodImplementation instead
    bfn class_respondsToMethod # use class_respondsToSelector instead
    bfn _objc_flush_caches # not recommended

    # Marked with OBJC_OSX_DEPRECATED_OTHERS_UNAVAILABLE
    bfn object_copyFromZone # use object_copy instead
    bfn class_createInstanceFromZone # use class_createInstance instead
end

# Marked with OBJC_OSX_AVAILABLE_OTHERS_UNAVAILABLE
function macos_only
    bty objc_exception_handler
    bfn objc_addExceptionHandler
    bfn objc_removeExceptionHandler
end

# Protected by #if !(TARGET_OS_OSX && __i386__) block
function 64bit_only
    bfn objc_setHook_getClass
    bfn objc_setHook_lazyClassNamer
    bfn _objc_swiftMetadataInitializer
    bfn _objc_realizeClassFromSwift
end

# Marked with OBJC2_UNAVAILABLE
function unavailable
    # message.h
    bty marg_list
    bfn objc_msgSendv
    bfn objc_msgSendv_stret
    bfn objc_msgSendv_fpret

    # runtime.h
    bfn object_realloc
    bfn object_reallocFromZone
    bfn objc_getClasses
    bfn objc_addClass
    bfn objc_setClassHandler
    bfn objc_setMultithreaded
    bfn class_addMethods
    bfn class_removeMethods
    bfn _objc_resolve_categories_for_class
    bfn class_poseAs
    bfn method_getSizeOfArguments
    bfn method_getArgumentInfo
    bfn objc_getOrigClass
    bfn class_nextMethodList

    bty objc_method_list

    bfn _alloc
    bfn _copy
    bfn _realloc
    bfn _dealloc
    bfn _zoneAlloc
    bfn _zoneRealloc
    bfn _zoneCopy
    bfn _error

    # Marked with OBJC_OSX_DEPRECATED_OTHERS_UNAVAILABLE
    bfn object_copyFromZone # use object_copy instead
    bfn class_createInstanceFromZone # use class_createInstance instead
end

# Marked with OBJC_ARC_UNAVAILABLE
function arc_unavailable
    bfn object_copy
    bfn object_dispose
    bfn object_setInstanceVariable
    bfn object_setInstanceVariableWithStrongDefault
    bfn object_getInstanceVariable
    bfn objc_getFutureClass
    bfn objc_constructInstance
    bfn objc_destructInstance
end

function custom
    echo "--raw-line=use crate::*;"

    # Defined in types.rs
    bty BOOL
    bty objc_class
    bty objc_ivar
    bty objc_method
    bty objc_object
    bty objc_property
    bty objc_selector

    # Defined in message.rs
    bfn class_getMethodImplementation
    bfn class_getMethodImplementation_stret

    # Not used anywhere
    bty objc_category
end


for tag in $tags
    git -C $source_repo checkout $tag

    bindgen $source_repo/runtime/headers.h -o $to_repo/bindings.rs \
        --no-layout-tests \
        --no-doc-comments \
        --size_t-is-usize \
        --allowlist-function="(sel|object|class|objc|method|protocol|ivar|property|imp|_objc|_)_.*" \
        --blocklist-type=__darwin_size_t \
        --generate="functions" \
        (blocklisted) \
        (deprecated) \
        (macos_only) \
        (64bit_only) \
        (unavailable) \
        (arc_unavailable) \
        (custom)

        # --allowlist-var=".*OBJC.*" \

    git -C $to_repo add bindings.rs
    git -C $to_repo commit -a -m "From $tag"
end

rm $source_repo/runtime/headers.h
