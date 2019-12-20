// Copyright 2015-2019 Capital One Services, LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::Result;
use libloading::Library;
use libloading::Symbol;
use std::ffi::OsStr;
use wascc_codec::capabilities::CapabilityProvider;

/// Represents a native capability provider compiled as a shared object library.
/// These plugins are OS-specific, so they will be `.so` files on Linux, `.dylib`
/// files on macOS, etc.
pub struct NativeCapability {
    pub(crate) capid: String,
    pub(crate) plugin: Box<dyn CapabilityProvider>,
    // This field is solely used to keep the FFI library instance allocated for the same
    // lifetime as the boxed plugin
    #[allow(dead_code)]
    library: Library,
}

impl NativeCapability {
    /// Reads a capability provider from a file. The capability provider must implement the
    /// correct FFI interface to support waSCC plugins
    pub fn from_file<P: AsRef<OsStr>>(filename: P) -> Result<Self> {
        type PluginCreate = unsafe fn() -> *mut dyn CapabilityProvider;

        let library = Library::new(filename.as_ref())?;

        let plugin = unsafe {
            let constructor: Symbol<PluginCreate> = library.get(b"__capability_provider_create")?;
            let boxed_raw = constructor();

            Box::from_raw(boxed_raw)
        };
        info!(
            "Loaded capability: {}, native provider: {}",
            plugin.capability_id(),
            plugin.name()
        );

        let capid = plugin.capability_id().to_string();
        Ok(NativeCapability {
            capid,
            plugin,
            library,
        })
    }
}
