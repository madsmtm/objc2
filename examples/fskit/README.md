# FSKit app extension example

[FSKit](https://developer.apple.com/documentation/fskit) is a framework introduced in macOS 15.4 that allows to providing custom filesystem support from user space.

It builds upon Apple's existing app extension support, see [WWDC 2014 217](https://nonstrict.eu/wwdcindex/wwdc2014/217/) and [Apple's documentation](https://developer.apple.com/library/archive/documentation/General/Conceptual/ExtensibilityPG/index.html) for an understanding on how this system works.

Note in particular that **extensions must be bundled inside another application**, and must be sandboxed. To allow communication between the host application and the app extension, one should use "app groups". We won't do so in this example though.


## Usage

1. Build with:
    ```sh
    ./examples/fskit/bundle.sh
    ```

2. Navigate to `System Settings > General > Login Items & Extensions`, and scroll down to the Extensions section.
    - Opening the link `x-apple.systempreferences:com.apple.LoginItems-Settings.extension` should do the trick.
    - In the File System Extensions cell, click the ⓘ symbol, and enable "FSKit Example Extension".

4. Create a directory to serve as the mount target for the file system:
    ```sh
    mkdir /tmp/fskit-mount-target
    ```

5. Create a dummy file and construct a new block device from it:
    ```sh
    mkfile -n 100m ./target/dummy-block-device
    DISK="$(basename $(hdiutil attach -imagekey diskimage-class=CRawDiskImage -nomount ./target/dummy-block-device))"
    echo $DISK
    ```

    - On macOS 26, it should be possible to avoid the block device, and simply do:
        ```sh
        mount -F -t fskitexample ~/some-path /tmp/fskit-mount-target
        ```

        If you set `FSSupportsPathURLs` and/or `FSSupportsGenericURLResources` to `true`, and handle the resource appropriately with `resource.downcast_ref::<FSPathURLResource>()` and/or `resource.downcast_ref::<FSGenericURLResource>()`.

6. Optional: In another terminal, tun the following to get the logging output:
    ```sh
    /usr/bin/log stream --predicate 'subsystem = "fskit-example"' --style compact --level debug
    ```

    See [`println!` debugging](#println-debugging) below for details.

7. Mount the block device:
    ```sh
    mount -F -t fskitexample $DISK /tmp/fskit-mount-target
    ```


## App extension setup

Xcode provides templates to create app extensions (go to `File > New > Target`), but for clarity we do it here without Xcode and Swift.

The general setup is:
1. Two Cargo binaries, the host application `fskit-example` and the app extension `fskit-example-extension`.
2. A custom `build.rs` which configures `fskit-example-extension` to use `NSExtensionMain` instead of Rust's usual `fn main` as the entrypoint.
3. A `FSUnaryFileSystem` subclass, which is registered before `NSExtensionMain` is called by using `#[ctor]`.
  - `EXExtensionPrincipalClass` is used to point to our `FSUnaryFileSystem` class (an alternative to using Swift's `UnaryFileSystemExtension`).
  - This is also what's done [by Apple's own `msdosfs`](https://github.com/apple-oss-distributions/msdosfs/blob/msdosfs-788.40.4/msdos_appex/Info.plist).
4. Signing and bundling is done in `bundle.sh`.

This general setup should be transferable to other app extensions.


## Internals overview

My rough understanding of how FSKit works internally, gleaned from attaching to various processes and inspecting their state:

A system service somewhere (`fskitd` or `fskit_helper`?) receives custom `mount`/`umount`/... requests from with the kernel, and delegates these to each custom filesystem.

Upon `mount`, it then does roughly the following on a filesystem extension:
1. If no instance of the application extension exists, tell `launchd` to spawn a "root" filesystem process.
  - This is a normal executable, but it's linked with `_NSExtensionMain` as the entry point.
  - The main thread is then put in a runloop state, listening for new events over XPC from `fskitd`.
2. Message root process above, call `probeResource:replyHandler:` (on a worker thread)
3. If successful, tell `launchd` to spawn a new filesystem process.
  - Similarly to the root process, this starts a runloop and listens for events.
  - Difference seems to be that it's launched with `LaunchInstanceID=$RANDOM_UUID` instead of `LaunchInstanceID=00000000-0000-0000-0000-000000000001`?
4. Message newly created process, call `loadResource:replyHandler:` (on a worker thread).
5. Further messages are sent to that same process, which is later handled by `FSVolume`.

On `umount`:
- `deactivateWithOptions:replyHandler:` is called
- But nothing else? Quick testing didn't seem to call `unmountWithReplyHandler:` nor `unloadResource:replyHandler:`, but there might be a way to opt into that?

At some point, the root process is killed if there are no more individual mount processes left.


## Debugging

Ideally, you'll want to attach a debugger. That can be a bit troublesome though, because:
- We do not control the process launch (`launchd` / `fskitd` does).
- There is no way (that I've found) to tell `fskitd` that you want processes spawned with `POSIX_SPAWN_START_SUSPENDED` set.
- Which in turn means that any attempt to attach a debugger is inherently racy.
  - A trick here is to add `std::thread::sleep_ms(100)` to the top of `fn setup()`.

Even more annoying is that `fskitd` seems to have an internal timer that kills the process if it takes more than ~10 seconds to launch. So it's a lot of juggling.

That said, I have had some luck with:

```sh
lldb --wait-for --attach-name fskit-example-extension # Optionally: --one-line "continue"
```

Depending on what step you're debugging, you might end up catching the "wrong" process with this though, because as stated above, `fskitd` seems to spawn two extension processes (root process, and one for the new volume) when starting up.


### `println!` debugging

Generally a good alternative when debuggers are unavailable, though also perilous here: `stdout` and `stderr` are redirected to `/dev/null` by `launchd`.

Writing to `/tmp` files won't work either, as those are sandboxed away too.
- `std::env::temp_dir()` should work, though that means it's gonna be buried in something like `~/Library/Containers/com.example.fskit-example.extension/Data/tmp`.
- Alternatively, maybe add [`com.apple.security.temporary-exception.files.absolute-path.read-write`](https://developer.apple.com/library/archive/documentation/Miscellaneous/Reference/EntitlementKeyReference/Chapters/AppSandboxTemporaryExceptionEntitlements.html#//apple_ref/doc/uid/TP40011195-CH5-SW7)?

Instead, we use `tracing-oslog` to log errors via Apple's OSLog. These can be viewed with:
```sh
/usr/bin/log stream --predicate 'subsystem = "fskit-example"' --style compact --level debug
```

That said, I _have_ seen it fail at the boundaries / entry/exit points (`deactivateWithOptions:replyHandler:` for example isn't logged IIRC), so it's not a perfect solution.


## Troubleshooting

### Nothing is spawned.

Make sure that it's the only extension with the given `FSShortName`:

```sh
/System/Library/Frameworks/CoreServices.framework/Versions/Current/Frameworks/LaunchServices.framework/Versions/Current/Support/lsregister -dump | grep FSShortName
```

You can view the entirety of the registered FSKit entries with:

```sh
/System/Library/Frameworks/CoreServices.framework/Versions/Current/Frameworks/LaunchServices.framework/Versions/Current/Support/lsregister -dump | awk 'BEGIN{RS="------"; FS="\n"} /com\.apple\.fskit\.fsmodule/ {print $0}'
```

Also try:

```sh
pluginkit --match --all-versions --duplicates -vv --identifier com.example.fskit-example.extension
```


### My logs aren't showing up / weirdness

Try `trash ./target/debug/fskit-example.app`, empty trash, and rerun `./example/fskit/bundle.sh`.

TODO: Maybe we can use smth. like `RegisterExecutionPolicyException` to avoid the delay?


### Resource busy

Run `killall fskit-example-extension` and try again after a few seconds.


### Probing resource: Couldn’t communicate with a helper application.

You might get this error if attempting to `mount` right after bundling the extension. Try again.
