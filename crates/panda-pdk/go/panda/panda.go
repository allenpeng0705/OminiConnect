// Package panda is a TinyGo guest SDK for the Panda Wasm plugin ABI v1.
// Build only with: tinygo build -target=wasm ...
package panda

import (
	"bytes"
	"unsafe"
)

const (
	AbiVersion              int32 = 1
	RCAllow                 int32 = 0
	RCRejectPolicyDenied    int32 = 1
	RCRejectMalformed       int32 = 2
)

//go:wasmimport env panda_set_header
func wasmSetHeader(namePtr, nameLen, valuePtr, valueLen uint32)

//go:wasmimport env panda_set_body
func wasmSetBody(ptr, length uint32)

//go:wasmimport env panda_set_response_chunk
func wasmSetResponseChunk(ptr, length uint32)

// SetHeader appends a request header (valid during panda_on_request / body hook).
func SetHeader(name, value string) {
	if len(name) == 0 {
		return
	}
	nb := []byte(name)
	vb := []byte(value)
	wasmSetHeader(
		uint32(uintptr(unsafe.Pointer(&nb[0]))), uint32(len(nb)),
		uint32(uintptr(unsafe.Pointer(&vb[0]))), uint32(len(vb)),
	)
}

// SetBody replaces the buffered request body.
func SetBody(b []byte) {
	if len(b) == 0 {
		return
	}
	wasmSetBody(uint32(uintptr(unsafe.Pointer(&b[0]))), uint32(len(b)))
}

// SetResponseChunk replaces the current streaming response chunk.
func SetResponseChunk(b []byte) {
	if len(b) == 0 {
		return
	}
	wasmSetResponseChunk(uint32(uintptr(unsafe.Pointer(&b[0]))), uint32(len(b)))
}

// GuestBytes views host-provided linear memory for the duration of a hook call.
func GuestBytes(ptr, length int32) []byte {
	if ptr < 0 || length <= 0 {
		return nil
	}
	return unsafe.Slice((*byte)(unsafe.Pointer(uintptr(ptr))), int(length))
}

// ReplaceAll returns a copy with non-overlapping occurrences of needle replaced.
func ReplaceAll(data, needle, replacement []byte) ([]byte, bool) {
	if len(needle) == 0 {
		return nil, false
	}
	changed := false
	out := make([]byte, 0, len(data))
	i := 0
	for i < len(data) {
		if i+len(needle) <= len(data) && bytes.Equal(data[i:i+len(needle)], needle) {
			out = append(out, replacement...)
			i += len(needle)
			changed = true
			continue
		}
		out = append(out, data[i])
		i++
	}
	if !changed {
		return nil, false
	}
	return out, true
}
