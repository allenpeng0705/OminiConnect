package main

import "unsafe"

const (
	PANDA_WASM_ABI_VERSION    = int32(1)
	RC_ALLOW                  = int32(0)
	RC_REJECT_POLICY_DENIED   = int32(1)
	RC_REJECT_MALFORMED_INPUT = int32(2)
)

//go:wasmimport panda_host panda_set_header
func pandaSetHeader(namePtr int32, nameLen int32, valuePtr int32, valueLen int32)

//go:wasmimport panda_host panda_set_body
func pandaSetBody(ptr int32, length int32)

//go:wasmimport panda_host panda_set_response_chunk
func pandaSetResponseChunk(ptr int32, length int32)

func setHeader(name []byte, value []byte) {
	pandaSetHeader(
		int32(uintptr(unsafe.Pointer(&name[0]))),
		int32(len(name)),
		int32(uintptr(unsafe.Pointer(&value[0]))),
		int32(len(value)),
	)
}

func setBody(body []byte) {
	pandaSetBody(int32(uintptr(unsafe.Pointer(&body[0]))), int32(len(body)))
}

func setResponseChunk(chunk []byte) {
	pandaSetResponseChunk(int32(uintptr(unsafe.Pointer(&chunk[0]))), int32(len(chunk)))
}
