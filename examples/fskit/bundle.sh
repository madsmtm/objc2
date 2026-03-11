#!/usr/bin/env bash

set -e

# Identity to use when signing. Defaults to ad-hoc signing ("-").
IDENTITY="${IDENTITY:--}"

# Various paths.
CARGO_PROFILE="dev"
CARGO_PROFILE_DIR="debug"
SOURCE_DIR="$(dirname "$(realpath "$0")")"
BUILD_DIR="$(dirname "$(cargo locate-project --workspace --message-format plain)")/target"
BUNDLE_PATH="$BUILD_DIR/$CARGO_PROFILE_DIR/fskit-example.app"
# App extensions are placed under the special `Extensions` path.
# `PlugIns` is the older option, that works as well (and is probably recommended if you want to support older OSes).
EXTENSION_BUNDLE_PATH="$BUNDLE_PATH/Contents/Extensions/fskit-example-extension.appex"


# Build host and extension binaries.
cargo build -pfskit-example --bin fskit-example --bin fskit-example-extension --profile $CARGO_PROFILE


# Create and sign app extension bundle.
mkdir -p "$EXTENSION_BUNDLE_PATH/Contents/MacOS"

cp "$BUILD_DIR/$CARGO_PROFILE_DIR/fskit-example-extension" "$EXTENSION_BUNDLE_PATH/Contents/MacOS/fskit-example-extension"
cp "$SOURCE_DIR/extension/Info.plist" "$EXTENSION_BUNDLE_PATH/Contents/Info.plist"

if [ "$IDENTITY" = "-" ]; then
codesign --force --sign "$IDENTITY" --timestamp=none --options runtime --entitlements "$SOURCE_DIR/extension/adhoc.entitlements" --generate-entitlement-der "$EXTENSION_BUNDLE_PATH"
else
codesign --force --sign "$IDENTITY" --timestamp=none --options runtime --entitlements "$SOURCE_DIR/extension/main.entitlements" --generate-entitlement-der "$EXTENSION_BUNDLE_PATH"
fi

touch "$EXTENSION_BUNDLE_PATH" # Update creation time.


# Create and sign host app bundle.
mkdir -p "$BUNDLE_PATH/Contents/MacOS"

cp "$BUILD_DIR/$CARGO_PROFILE_DIR/fskit-example" "$BUNDLE_PATH/Contents/MacOS/fskit-example"
cp "$SOURCE_DIR/host/Info.plist" "$BUNDLE_PATH/Contents/Info.plist"

codesign --force --sign "$IDENTITY" --timestamp=none --options runtime --entitlements "$SOURCE_DIR/host/main.entitlements" --generate-entitlement-der "$BUNDLE_PATH"

touch "$BUNDLE_PATH" # Update creation time.


# Register the application with launch services.
#
# This makes the application show up in Spotlight and allows launching it with `open -a fskit-example`.
#
# More importantly, it also makes the app extension available immediately in settings without having
# to launch the host application first.
/System/Library/Frameworks/CoreServices.framework/Versions/Current/Frameworks/LaunchServices.framework/Versions/Current/Support/lsregister -f -R -trusted "$BUNDLE_PATH"
