/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::dyld::{ConstantExports, HostConstant};
use crate::frameworks::foundation::NSInteger;
use crate::objc::{id, nil, release, retain, ClassExports, HostObject, NSZonePtr, autorelease, msg};
use crate::objc_classes;

/// `NSString*`
pub type NSErrorDomain = id;

pub const NSCocoaErrorDomain: &str = "NSCocoaErrorDomain";
pub const NSOSStatusErrorDomain: &str = "NSOSStatusErrorDomain";

pub const NSFileReadNoSuchFileError: NSInteger = 260;

struct ErrorHostObject {
    domain: NSErrorDomain,
    code: NSInteger,
    user_info: id,
}
impl HostObject for ErrorHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

// TODO: Return errors in all methods that are supposed to do it.
@implementation NSError: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(ErrorHostObject {
        domain: nil,
        code: 0,
        user_info: nil
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

+ (id)errorWithDomain:(id)domain
                 code:(NSInteger)code
             userInfo:(id)userInfo {
    let err = msg![env; this alloc];
    let err = msg![env; err initWithDomain:domain code:code userInfo:userInfo];
    autorelease(env, err)
}

- (id)initWithDomain:(NSErrorDomain)domain
                code:(NSInteger)code
            userInfo:(id)user_info {
    retain(env, domain);
    retain(env, user_info);
    let host_obj = env.objc.borrow_mut::<ErrorHostObject>(this);
    host_obj.domain = domain;
    host_obj.code = code;
    host_obj.user_info = user_info;
    this
}

- (())dealloc {
    let &ErrorHostObject{domain, user_info, ..} = env.objc.borrow(this);
    release(env, domain);
    release(env, user_info);

    env.objc.dealloc_object(this, &mut env.mem);
}

- (NSInteger)code {
    env.objc.borrow::<ErrorHostObject>(this).code
}

- (id)description {
    let &ErrorHostObject { domain, code, .. } = env.objc.borrow(this);
    let domain_str = crate::frameworks::foundation::ns_string::to_rust_string(env, domain);
    let rust_string = format!("Error Domain={} Code={}", domain_str, code);
    let ns_string = crate::frameworks::foundation::ns_string::from_rust_string(env, rust_string);
    autorelease(env, ns_string)
}

@end

};

pub const CONSTANTS: ConstantExports = &[
    (
        "_NSLocalizedDescriptionKey",
        HostConstant::NSString("NSLocalizedDescriptionKey"),
    ),
    (
        "_NSCocoaErrorDomain",
        HostConstant::NSString(NSCocoaErrorDomain),
    ),
    (
        "_NSOSStatusErrorDomain",
        HostConstant::NSString(NSOSStatusErrorDomain),
    ),
];
