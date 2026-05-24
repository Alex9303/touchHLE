/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSURL`.

use super::ns_string::{from_rust_string, get_static_str, to_rust_string, NSUTF8StringEncoding};
use super::NSUInteger;
use crate::fs::{GuestPath, GuestPathBuf};
use crate::mem::MutPtr;
use crate::objc::{
    autorelease, id, msg, msg_class, nil, objc_classes, release, retain, ClassExports, HostObject,
    NSZonePtr,
};
use crate::Environment;
use std::borrow::Cow;

/// It seems like there's two kinds of NSURLs: ones for file paths, and others.
/// So far only the former is implemented (TODO).
enum NSURLHostObject {
    /// This is a file URL. The NSString is a system path (no `file:///`).
    ///
    /// This is a wrapper around NSString so that conversions between NSURL
    /// and NSString, which happen often, can be simple and efficient.
    FileURL {
        ns_string: id,
        // Relative file URL save the working directory at the time of creation
        // At the moment, used in the description selector.
        working_directory: GuestPathBuf,
    },
    /// Non-file URL.
    OtherURL { ns_string: id },
}
impl HostObject for NSURLHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSURL: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = NSURLHostObject::FileURL { ns_string: nil, working_directory: env.fs.working_directory().into() };
    env.objc.alloc_object(this, Box::new(host_object), &mut env.mem)
}

+ (id)URLWithString:(id)url { // NSString*
    let new: id = msg![env; this alloc];
    let new: id = msg![env; new initWithString:url];
    autorelease(env, new)
}

+ (id)fileURLWithPath:(id)path { // NSString*
    let new: id = msg![env; this alloc];
    let new: id = msg![env; new initFileURLWithPath:path];
    autorelease(env, new)
}

+ (id)fileURLWithPath:(id)path // NSString*
          isDirectory:(bool)is_dir {
    let new: id = msg![env; this alloc];
    let new: id = msg![env; new initFileURLWithPath:path isDirectory:is_dir];
    autorelease(env, new)
}

- (())dealloc {
    match *env.objc.borrow(this) {
        NSURLHostObject::FileURL { ns_string, .. } => release(env, ns_string),
        NSURLHostObject::OtherURL { ns_string } => release(env, ns_string),
    }
    env.objc.dealloc_object(this, &mut env.mem)
}

// NSCopying implementation
- (id)copyWithZone:(NSZonePtr)_zone {
    retain(env, this)
}

- (id)initFileURLWithPath:(id)path { // NSString*
    // FIXME: this should guess whether the path is a directory
    msg![env; this initFileURLWithPath:path isDirectory:false]
}

- (id)initFileURLWithPath:(id)path // NSString*
              isDirectory:(bool)_is_dir {
    let path_rust = to_rust_string(env, path);
    // Tolerate file:/// prefix just in case
    let path_rust = path_rust.strip_prefix("file://").unwrap_or(&path_rust);
    let clean_path: id = from_rust_string(env, path_rust.to_string());
    let expanded_path: id = msg![env; clean_path stringByExpandingTildeInPath];
    let final_path: id = msg![env; expanded_path copy];
    *env.objc.borrow_mut(this) = NSURLHostObject::FileURL { ns_string: final_path, working_directory: env.fs.working_directory().into() };
    this
}

- (id)initWithString:(id)url { // NSString*
    if url == nil {
        return nil;
    }
    let url_copy: id = msg![env; url copy];
    *env.objc.borrow_mut(this) = NSURLHostObject::OtherURL { ns_string: url_copy };
    this
}

- (bool)isFileURL {
    match env.objc.borrow(this) {
        NSURLHostObject::FileURL { .. } => true,
        NSURLHostObject::OtherURL { .. } => false,
    }
}

- (id)description {
    match env.objc.borrow(this) {
        NSURLHostObject::FileURL { ns_string, working_directory } => {
            let working_directory = working_directory.as_str().to_string();
            let mut description = to_rust_string(env, *ns_string).to_string().clone();
            if !description.starts_with('/') {
                description = format!("{} -- file://localhost{}", description.trim_start_matches("./"), working_directory );
            }
            let desc = from_rust_string(env, description);
            autorelease(env, desc)
        },
        NSURLHostObject::OtherURL { ns_string } => *ns_string,
    }
}

- (id)path {
    match env.objc.borrow(this) {
        NSURLHostObject::FileURL { ns_string, .. } => *ns_string,
        NSURLHostObject::OtherURL { ns_string } => {
            let s = to_rust_string(env, *ns_string);
            if let Some(idx) = s.find("://") {
                let rest = &s[idx + 3..];
                if let Some(slash_idx) = rest.find('/') {
                    let path_query = &rest[slash_idx..];
                    let path = path_query.split('?').next().unwrap_or(path_query);
                    let path_ns = from_rust_string(env, path.to_string());
                    return autorelease(env, path_ns);
                }
            }
            nil
        }
    }
}

- (id)absoluteString {
    match *env.objc.borrow(this) {
        // FIXME: don't assume URL is already absolute
        NSURLHostObject::FileURL { ns_string, .. } => ns_string,
        NSURLHostObject::OtherURL { ns_string } => {
            // TODO: full RFC 1808 resolution
            assert!(to_rust_string(env, ns_string).starts_with("http"));
            ns_string
        },
    }
}

- (id)absoluteURL {
    // FIXME: don't assume URL is already absolute
    let &NSURLHostObject::OtherURL { .. } = env.objc.borrow(this) else {
        unimplemented!(); // TODO
    };
    this
}

- (bool)getFileSystemRepresentation:(MutPtr<u8>)buffer
                          maxLength:(NSUInteger)buffer_size {
    let &NSURLHostObject::FileURL { ns_string, .. } = env.objc.borrow(this) else {
        unimplemented!(); // TODO
    };
    msg![env; ns_string getCString:buffer
                         maxLength:buffer_size
                          encoding:NSUTF8StringEncoding]
}

- (id)URLByAppendingPathComponent:(id)path_component // NSString *
                      isDirectory:(bool)is_directory {
    let &NSURLHostObject::FileURL { ns_string, .. } = env.objc.borrow(this) else {
        unimplemented!(); // TODO
    };
    let mut path: id = msg![env; ns_string stringByAppendingPathComponent:path_component];
    if is_directory {
        path = msg![env; path stringByAppendingString:(get_static_str(env, "/"))];
    }
    msg_class![env; NSURL fileURLWithPath:path]
}

- (id)URLByDeletingLastPathComponent {
    let &NSURLHostObject::FileURL { ns_string, .. } = env.objc.borrow(this) else {
        unimplemented!(); // TODO
    };
    let path: id = msg![env; ns_string stringByDeletingLastPathComponent];
    msg_class![env; NSURL fileURLWithPath:path]
}

- (id)scheme {
    match env.objc.borrow(this) {
        NSURLHostObject::FileURL { .. } => get_static_str(env, "file"),
        NSURLHostObject::OtherURL { ns_string } => {
            let s = to_rust_string(env, *ns_string);
            if let Some(idx) = s.find("://") {
                let scheme_ns = from_rust_string(env, s[..idx].to_string());
                autorelease(env, scheme_ns)
            } else {
                nil
            }
        }
    }
}

- (id)host {
    match env.objc.borrow(this) {
        NSURLHostObject::FileURL { .. } => get_static_str(env, "localhost"),
        NSURLHostObject::OtherURL { ns_string } => {
            let s = to_rust_string(env, *ns_string);
            if let Some(idx) = s.find("://") {
                let rest = &s[idx + 3..];
                let host_port = rest.split('/').next().unwrap_or(rest);
                let host = host_port.split(':').next().unwrap_or(host_port);
                if !host.is_empty() {
                    let host_ns = from_rust_string(env, host.to_string());
                    return autorelease(env, host_ns);
                }
            }
            nil
        }
    }
}

- (id)port {
    match env.objc.borrow(this) {
        NSURLHostObject::FileURL { .. } => nil,
        NSURLHostObject::OtherURL { ns_string } => {
            let s = to_rust_string(env, *ns_string);
            if let Some(idx) = s.find("://") {
                let rest = &s[idx + 3..];
                let host_port = rest.split('/').next().unwrap_or(rest);
                if let Some(colon_idx) = host_port.find(':') {
                    let port_str = &host_port[colon_idx + 1..];
                    if let Ok(port_num) = port_str.parse::<u32>() {
                        let num_obj: id = msg_class![env; NSNumber numberWithUnsignedInt:port_num];
                        return num_obj;
                    }
                }
            }
            nil
        }
    }
}

- (id)query {
    match env.objc.borrow(this) {
        NSURLHostObject::FileURL { .. } => nil,
        NSURLHostObject::OtherURL { ns_string } => {
            let s = to_rust_string(env, *ns_string);
            if let Some(idx) = s.find('?') {
                let query_fragment = &s[idx + 1..];
                let query = query_fragment.split('#').next().unwrap_or(query_fragment);
                let query_ns = from_rust_string(env, query.to_string());
                return autorelease(env, query_ns);
            }
            nil
        }
    }
}

// TODO: more constructors, more accessors

@end

// A caching layer a top of NSURL, it's OK to stub
// as we don't have yet a networking support
@implementation NSURLCache: NSObject
+ (id)sharedURLCache {
    // TODO
    nil
}
@end

};

/// Shortcut for host code, provides a view of a URL as a path.
/// TODO: Try to avoid allocating a new GuestPathBuf in more cases.
pub fn to_rust_path(env: &mut Environment, url: id) -> Cow<'static, GuestPath> {
    let path_string: id = msg![env; url path];

    match to_rust_string(env, path_string) {
        Cow::Borrowed(path) => Cow::Borrowed(path.as_ref()),
        Cow::Owned(path_buf) => Cow::Owned(path_buf.into()),
    }
}
