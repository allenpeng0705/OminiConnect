// Minimal Panda guest: redact sk_live_ / password in request bodies (TinyGo).
package main

import "panda"

//export panda_abi_version
func panda_abi_version() int32 {
	return panda.AbiVersion
}

//export panda_on_request
func panda_on_request() int32 {
	panda.SetHeader("x-panda-plugin", "go-pii-mini")
	return panda.RCAllow
}

//export panda_on_request_body
func panda_on_request_body(ptr int32, len int32) int32 {
	buf := panda.GuestBytes(ptr, len)
	if len(buf) == 0 {
		return panda.RCAllow
	}
	out := append([]byte(nil), buf...)
	var ch bool
	if x, ok := panda.ReplaceAll(out, []byte("sk_live_"), []byte("[REDACTED]")); ok {
		out = x
		ch = true
	}
	if x, ok := panda.ReplaceAll(out, []byte("password"), []byte("[REDACTED]")); ok {
		out = x
		ch = true
	}
	if ch {
		panda.SetHeader("x-panda-pii", "go")
		panda.SetBody(out)
	}
	return panda.RCAllow
}

func main() {}
