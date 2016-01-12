// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to runtimeobject.
#![no_std]
#![experimental]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn RoInitialize(initType: RO_INIT_TYPE,) -> HRESULT;
    pub fn RoUninitialize() -> HRESULT;
    pub fn RoGetActivationFactory(
        activatableClassId: HSTRING, iid: REFIID, factory: *mut *mut c_void,    
    ) -> HRESULT;
}
