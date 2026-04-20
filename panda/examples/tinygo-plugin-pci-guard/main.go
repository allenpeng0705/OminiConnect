package main

import "unsafe"

// Min consecutive ASCII digits before treating as a PAN-like sequence (PCI-style guard).
// 13 matches Visa-length minimum; adjust in fork if you need stricter looser policy.
const minDigitRun = 13

var pluginHeaderName = []byte("x-panda-wasm-plugin")
var pluginHeaderValue = []byte("pci-digit-guard")
var blockHeaderName = []byte("x-panda-pci-digit-block")
var blockHeaderValue = []byte("1")

//export panda_abi_version
func panda_abi_version() int32 {
	return PANDA_WASM_ABI_VERSION
}

//export panda_on_request
func panda_on_request() int32 {
	setHeader(pluginHeaderName, pluginHeaderValue)
	return RC_ALLOW
}

//export panda_on_request_body
func panda_on_request_body(ptr int32, length int32) int32 {
	if ptr < 0 || length <= 0 {
		return RC_ALLOW
	}
	run := 0
	for i := int32(0); i < length; i++ {
		b := *(*byte)(unsafe.Pointer(uintptr(ptr) + uintptr(i)))
		if b >= '0' && b <= '9' {
			run++
			if run >= minDigitRun {
				setHeader(blockHeaderName, blockHeaderValue)
				return RC_REJECT_POLICY_DENIED
			}
		} else {
			run = 0
		}
	}
	return RC_ALLOW
}

//export panda_on_response_chunk
func panda_on_response_chunk(ptr int32, length int32) int32 {
	_ = ptr
	_ = length
	return RC_ALLOW
}

func main() {}
