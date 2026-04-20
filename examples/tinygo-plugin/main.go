package main

var headerName = []byte("x-panda-plugin")
var headerValue = []byte("tinygo-sample")

//export panda_abi_version
func panda_abi_version() int32 {
	return PANDA_WASM_ABI_VERSION
}

//export panda_on_request
func panda_on_request() int32 {
	setHeader(headerName, headerValue)
	return RC_ALLOW
}

//export panda_on_request_body
func panda_on_request_body(ptr int32, length int32) int32 {
	// v0 sample: no-op allow.
	_ = ptr
	_ = length
	return RC_ALLOW
}

//export panda_on_response_chunk
func panda_on_response_chunk(ptr int32, length int32) int32 {
	_ = ptr
	_ = length
	return RC_ALLOW
}

func main() {}
