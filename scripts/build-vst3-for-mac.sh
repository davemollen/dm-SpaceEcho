PACKAGE_NAME=(`./scripts/get-package-name.sh nih-plug`)
NAME=$(echo $PACKAGE_NAME | perl -pe 's/(?<=[^\W_])_+([^\W_])|_+/-\U$1/g')
OLD_VST_NAME="$PACKAGE_NAME.vst3"
NEW_VST_NAME="$NAME.vst3"
MOVE_FROM="./target/bundled/$OLD_VST_NAME"
MOVE_TO="/Library/Audio/Plug-Ins/VST3/$NEW_VST_NAME"

cd nih-plug
cargo xtask bundle $PACKAGE_NAME --release

if [ -d "$MOVE_TO" ]; then
    rm -r "$MOVE_TO"
fi

if mv "$MOVE_FROM" "$MOVE_TO"; then
    echo "Copied VST3 bundle to $MOVE_TO"
fi