#!/bin/bash

BINDGEN="~/.cargo/bin/bindgen"

if [ -f "${BINDGEN}" ]; then
	echo "Error: bindgen not found"
	exit 1
fi

exec $BINDGEN src/wrapper.h \
	--output src/bindings.rs \
	--allowlist-type Attachment \
	--allowlist-type MAPIProperty \
	--allowlist-type MAPIPropertyTagList \
	--allowlist-type MAPIProps \
	--allowlist-type TNEFFileInfo \
	--allowlist-type TNEFHandler \
	--allowlist-type TNEFMemInfo \
	--allowlist-type TNEFStruct \
	--allowlist-function MAPIFindProperty \
	--allowlist-function MAPIFindUserProp \
	--allowlist-function MAPIPrint \
	--allowlist-function MAPISysTimetoDTR \
	--allowlist-function SwapDDWord \
	--allowlist-function SwapDWord \
	--allowlist-function SwapWord \
	--allowlist-function TNEFCheckForSignature \
	--allowlist-function TNEFFree \
	--allowlist-function TNEFFreeAttachment \
	--allowlist-function TNEFFreeMapProps \
	--allowlist-function TNEFInitAttachment \
	--allowlist-function TNEFInitMapi \
	--allowlist-function TNEFInitialize \
	--allowlist-function TNEFParse \
	--allowlist-function TNEFParseFile \
	--allowlist-function TNEFParseMemory \
	--allowlist-function TNEFPrintDate

if [ "$?" != "0" ]; then
	echo "bindgen failed"
	exit 1
fi
