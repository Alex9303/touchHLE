/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! The Core Foundation framework.
//!
//! In Apple's implementation, this is a layer independent of, or below,
//! Foundation, and there is "Toll-Free Bridging" that lets some Foundation
//! types be used as if they were the corresponding Core Foundation types and
//! vice-versa. But in this implementation we will cheat and implement things
//! backwards (Core Foundation on top of Foundation) where we can get away with
//! it.
//!
//! Useful resources:
//! - Apple's [Core Foundation Design Concepts](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFDesignConcepts/CFDesignConcepts.html)
//! - Apple's [Memory Management Programming Guide for Core Foundation](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/CFMemoryMgmt.html)

pub mod cf_allocator;
pub mod cf_array;
pub mod cf_bundle;
pub mod cf_data;
pub mod cf_dictionary;
pub mod cf_locale;
pub mod cf_number;
pub mod cf_preferences;
pub mod cf_run_loop;
pub mod cf_run_loop_timer;
pub mod cf_socket;
pub mod cf_string;
pub mod cf_type;
pub mod cf_url;
pub mod cf_uuid;
pub mod time;

pub const DYLIB: crate::dyld::HostDylib = crate::dyld::HostDylib {
    path: "/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation",
    aliases: &[],
    class_exports: &[
        cf_run_loop_timer::CLASSES, // Special internal classes.
        cf_uuid::CLASSES,
    ],
    constant_exports: &[
        cf_allocator::CONSTANTS,
        cf_bundle::CONSTANTS,
        cf_dictionary::CONSTANTS,
        cf_locale::CONSTANTS,
        cf_number::CONSTANTS,
        cf_preferences::CONSTANTS,
        cf_run_loop::CONSTANTS,
    ],
    function_exports: &[
        FUNCTIONS,
        cf_array::FUNCTIONS,
        cf_dictionary::FUNCTIONS,
        cf_bundle::FUNCTIONS,
        cf_socket::FUNCTIONS,
        cf_data::FUNCTIONS,
        cf_locale::FUNCTIONS,
        cf_number::FUNCTIONS,
        cf_preferences::FUNCTIONS,
        cf_run_loop::FUNCTIONS,
        cf_run_loop_timer::FUNCTIONS,
        cf_string::FUNCTIONS,
        cf_type::FUNCTIONS,
        cf_url::FUNCTIONS,
        cf_uuid::FUNCTIONS,
        time::FUNCTIONS,
    ],
};

pub use cf_type::{CFRelease, CFRetain, CFTypeRef};

pub type CFHashCode = u32;
pub type CFIndex = i32;
pub type CFOptionFlags = u32;
pub type CFComparisonResult = CFIndex;

use crate::abi::GuestArg;
use crate::dyld::FunctionExports;
use crate::environment::Environment;
use crate::frameworks::foundation::ns_string::to_rust_string;
use crate::mem::{SafeRead, ConstVoidPtr};
use crate::objc::{id};
use crate::{export_c_func, impl_GuestRet_for_large_struct, msg};

pub const kCFNotFound: CFIndex = -1;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct CFRange {
    pub location: CFIndex,
    pub length: CFIndex,
}

unsafe impl SafeRead for CFRange {}
impl_GuestRet_for_large_struct!(CFRange);
impl GuestArg for CFRange {
    const REG_COUNT: usize = 2;

    fn from_regs(regs: &[u32]) -> Self {
        CFRange {
            location: GuestArg::from_regs(&regs[0..1]),
            length: GuestArg::from_regs(&regs[1..2]),
        }
    }
    fn to_regs(self, regs: &mut [u32]) {
        self.location.to_regs(&mut regs[0..1]);
        self.length.to_regs(&mut regs[1..2]);
    }
}

fn CFShow(env: &mut Environment, obj: CFTypeRef) {
    // TODO: support opaque types
    // TODO: use description callbacks if defined
    let description: id = msg![env; obj description];
    // The output should be printed to stderr without any prefix,
    // but CFShow() is meant to be used for debugging purposes,
    // so just logging with CF module prefix should be fine too.
    log!("{}", to_rust_string(env, description));
}

fn CFHTTPMessageCreateRequest(
    env: &mut Environment,
    alloc: ConstVoidPtr,
    request_method: id,
    url: id,
    http_version: id,
) -> id {
    log!("TODO: CFHTTPMessageCreateRequest(alloc: {:?}, method: {:?}, url: {:?}, version: {:?})", alloc, request_method, url, http_version);

    let cls = env.objc.get_known_class("NSObject", &mut env.mem);
    crate::objc::msg![env; cls new]
}

fn SecItemCopyMatching(
    env: &mut Environment,
    query: id,                      // CFDictionaryRef
    result: crate::mem::MutPtr<id>, // CFTypeRef* (Guest address)
) -> i32 {                          // OSStatus
    log!("TODO: SecItemCopyMatching(query: {:?}, result: {:?})", query, result);

    if !result.is_null() {
        let empty_dict: id = crate::objc::msg_class![env; NSDictionary dictionary];
        crate::objc::retain(env, empty_dict);
        env.mem.write(result, empty_dict);
        
        return 0; // errSecSuccess
    }

    // errSecItemNotFound = -25300
    -25300
}

fn CFHTTPMessageSetHeaderFieldValue(
    _env: &mut Environment,
    message: id,
    header_field: id,
    value: id,
) {
    log!("TODO: CFHTTPMessageSetHeaderFieldValue(message: {:?}, field: {:?}, value: {:?})", message, header_field, value);
}

fn CFReadStreamCreateForHTTPRequest(
    env: &mut Environment,
    alloc: crate::mem::ConstVoidPtr,
    request: id,
) -> id {
    log!("TODO: CFReadStreamCreateForHTTPRequest(alloc: {:?}, request: {:?})", alloc, request);

    let cls = env.objc.get_known_class("NSObject", &mut env.mem);
    crate::objc::msg![env; cls new]
}

fn CFNetworkCopySystemProxySettings(env: &mut Environment) -> id {
    log!("TODO: CFNetworkCopySystemProxySettings()");
    let dict: id = crate::objc::msg_class![env; NSDictionary dictionary];
    crate::objc::retain(env, dict)
}

fn CFHTTPMessageCopyAllHeaderFields(env: &mut Environment, message: id) -> id {
    log!("TODO: CFHTTPMessageCopyAllHeaderFields({:?})", message);
    let dict: id = crate::objc::msg_class![env; NSDictionary dictionary];
    crate::objc::retain(env, dict)
}

fn CFHTTPMessageCopyHeaderFieldValue(env: &mut Environment, message: id, field: id) -> id {
    log!("TODO: CFHTTPMessageCopyHeaderFieldValue({:?}, {:?})", message, field);
    let string: id = crate::objc::msg_class![env; NSString string];
    crate::objc::retain(env, string)
}

fn CFHTTPMessageCopyBody(env: &mut Environment, message: id) -> id {
    log!("TODO: CFHTTPMessageCopyBody({:?})", message);
    let data: id = crate::objc::msg_class![env; NSData data];
    crate::objc::retain(env, data)
}

fn CFHTTPMessageSetBody(_env: &mut Environment, message: id, body: id) {
    log!("TODO: CFHTTPMessageSetBody({:?}, {:?})", message, body);
}

fn CFHTTPMessageAppendBytes(_env: &mut Environment, message: id, new_bytes: crate::mem::ConstVoidPtr, num_bytes: u32) {
    log!("TODO: CFHTTPMessageAppendBytes({:?}, {:?}, {})", message, new_bytes, num_bytes);
}

fn CFHTTPMessageIsHeaderComplete(_env: &mut Environment, message: id) -> u8 {
    log!("TODO: CFHTTPMessageIsHeaderComplete({:?})", message);
    1
}

fn CFReadStreamCreateForStreamedHTTPRequest(
    env: &mut Environment,
    alloc: crate::mem::ConstVoidPtr,
    request: id,
    read_stream: id,
) -> id {
    log!("TODO: CFReadStreamCreateForStreamedHTTPRequest({:?}, {:?}, {:?})", alloc, request, read_stream);
    let cls = env.objc.get_known_class("NSObject", &mut env.mem);
    crate::objc::msg![env; cls new]
}

fn CFNetworkCopyProxiesForURL(env: &mut Environment, url: id, proxy_settings: id) -> id {
    log!("TODO: CFNetworkCopyProxiesForURL({:?}, {:?})", url, proxy_settings);

    let dummy_dict: id = crate::objc::msg_class![env; NSDictionary dictionary];
    let array: id = crate::objc::msg_class![env; NSMutableArray array];
    
    let _: () = crate::objc::msg![env; array addObject:dummy_dict];

    crate::objc::retain(env, array)
}

fn CFHTTPMessageCopyRequestMethod(env: &mut Environment, message: id) -> id {
    log!("TODO: CFHTTPMessageCopyRequestMethod({:?})", message);
    let string: id = crate::objc::msg_class![env; NSString string];
    crate::objc::retain(env, string)
}

fn CFHTTPMessageCopyRequestURL(env: &mut Environment, message: id) -> id {
    log!("TODO: CFHTTPMessageCopyRequestURL({:?})", message);
    // Return a safe, empty NSURL object
    let empty_str = crate::frameworks::foundation::ns_string::get_static_str(env, "");
    let url: id = crate::objc::msg_class![env; NSURL URLWithString:empty_str];
    crate::objc::retain(env, url)
}

fn CFHTTPMessageGetResponseStatusCode(_env: &mut Environment, message: id) -> i32 {
    log!("TODO: CFHTTPMessageGetResponseStatusCode({:?})", message);
    // Simulate a successful 200 OK response
    200
}

fn CFReadStreamSetProperty(
    _env: &mut Environment,
    _stream: ConstVoidPtr,
    _property: id,
    _value: id,
) -> bool {
    true
}

fn CFReadStreamScheduleWithRunLoop(
    _env: &mut Environment,
    stream: id,
    run_loop: id,
    run_loop_mode: id,
) {
    log!(
        "TODO: CFReadStreamScheduleWithRunLoop(stream: {:?}, run_loop: {:?}, mode: {:?})",
        stream,
        run_loop,
        run_loop_mode
    );
}

fn CFReadStreamSetClient(
    _env: &mut Environment,
    stream: id,
    stream_events: u32, // CFOptionFlags
    client_cb: crate::mem::ConstVoidPtr,
    client_context: crate::mem::ConstVoidPtr,
) -> u8 {
    log!("TODO: CFReadStreamSetClient({:?}, {}, {:?}, {:?})", stream, stream_events, client_cb, client_context);
    1
}

fn CFReadStreamOpen(_env: &mut Environment, stream: id) -> u8 {
    log!("TODO: CFReadStreamOpen({:?})", stream);
    1
}

fn CFReadStreamClose(_env: &mut Environment, stream: id) {
    log!("TODO: CFReadStreamClose({:?})", stream);
}

fn CFReadStreamRead(
    _env: &mut Environment,
    stream: id,
    buffer: crate::mem::MutVoidPtr,
    buffer_length: i32, // CFIndex
) -> i32 { // CFIndex
    log!("TODO: CFReadStreamRead({:?}, {:?}, {})", stream, buffer, buffer_length);
    0 // EOF
}

fn CFReadStreamUnscheduleFromRunLoop(
    _env: &mut Environment,
    stream: id,
    run_loop: id,
    run_loop_mode: id,
) {
    log!("TODO: CFReadStreamUnscheduleFromRunLoop({:?}, {:?}, {:?})", stream, run_loop, run_loop_mode);
}

fn CFReadStreamCopyProperty(
    _env: &mut Environment,
    stream: id,
    property_name: id,
) -> id {
    log!("TODO: CFReadStreamCopyProperty({:?}, {:?})", stream, property_name);
    crate::objc::nil
}

const FUNCTIONS: FunctionExports = &[
    export_c_func!(CFShow(_)),
    export_c_func!(CFHTTPMessageCreateRequest(_, _, _, _)),
    export_c_func!(SecItemCopyMatching(_, _)),
    export_c_func!(CFHTTPMessageSetHeaderFieldValue(_, _, _)),
    export_c_func!(CFReadStreamCreateForHTTPRequest(_, _)),
    export_c_func!(CFNetworkCopySystemProxySettings()),
    export_c_func!(CFHTTPMessageCopyAllHeaderFields(_)),
    export_c_func!(CFHTTPMessageCopyHeaderFieldValue(_, _)),
    export_c_func!(CFHTTPMessageCopyBody(_)),
    export_c_func!(CFHTTPMessageSetBody(_, _)),
    export_c_func!(CFHTTPMessageAppendBytes(_, _, _)),
    export_c_func!(CFHTTPMessageIsHeaderComplete(_)),
    export_c_func!(CFReadStreamCreateForStreamedHTTPRequest(_, _, _)),
    export_c_func!(CFNetworkCopyProxiesForURL(_, _)),
    export_c_func!(CFHTTPMessageCopyRequestMethod(_)),
    export_c_func!(CFHTTPMessageCopyRequestURL(_)),
    export_c_func!(CFHTTPMessageGetResponseStatusCode(_)),
    export_c_func!(CFReadStreamSetProperty(_, _, _)),
    export_c_func!(CFReadStreamScheduleWithRunLoop(_, _, _)),
    export_c_func!(CFReadStreamSetClient(_, _, _, _)),
    export_c_func!(CFReadStreamOpen(_)),
    export_c_func!(CFReadStreamClose(_)),
    export_c_func!(CFReadStreamRead(_, _, _)),
    export_c_func!(CFReadStreamUnscheduleFromRunLoop(_, _, _)),
    export_c_func!(CFReadStreamCopyProperty(_, _)),
];
