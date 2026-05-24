/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSHTTPCookie`.

use crate::objc::{
    autorelease, id, msg, msg_class, msg_super, nil, objc_classes, release, retain,
    ClassExports, HostObject, NSZonePtr,
};

struct NSHTTPCookieHostObject {
    properties: id, // NSDictionary
}
impl HostObject for NSHTTPCookieHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSHTTPCookie: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(NSHTTPCookieHostObject {
        properties: nil,
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

+ (id)cookieWithProperties:(id)properties {
    let new_cookie: id = msg![env; this alloc];
    let initialized: id = msg![env; new_cookie initWithProperties:properties];
    autorelease(env, initialized)
}

+ (id)cookiesWithResponseHeaderFields:(id)header_fields forURL:(id)url {
    log!("TODO: [(NSHTTPCookie) cookiesWithResponseHeaderFields:{:?} forURL:{:?}]", header_fields, url);
    msg_class![env; NSArray array]
}

+ (id)requestHeaderFieldsWithCookies:(id)cookies {
    log!("TODO: [(NSHTTPCookie) requestHeaderFieldsWithCookies:{:?}]", cookies);
    msg_class![env; NSDictionary dictionary]
}

- (())dealloc {
    let &NSHTTPCookieHostObject { properties } = env.objc.borrow(this);
    release(env, properties);
    env.objc.dealloc_object(this, &mut env.mem);
}

- (id)initWithProperties:(id)properties {
    let this: id = msg_super![env; this init];
    if this != nil {
        retain(env, properties);
        env.objc.borrow_mut::<NSHTTPCookieHostObject>(this).properties = properties;
    }
    this
}

- (id)properties {
    env.objc.borrow::<NSHTTPCookieHostObject>(this).properties
}

- (id)domain {
    log!("TODO: [(NSHTTPCookie*) {:?} domain]", this);
    msg_class![env; NSString string]
}

- (id)path {
    log!("TODO: [(NSHTTPCookie*) {:?} path]", this);
    msg_class![env; NSString string]
}

- (id)portList {
    log!("TODO: [(NSHTTPCookie*) {:?} portList]", this);
    msg_class![env; NSString string]
}

- (id)name {
    log!("TODO: [(NSHTTPCookie*) {:?} name]", this);
    msg_class![env; NSString string]
}

- (id)value {
    log!("TODO: [(NSHTTPCookie*) {:?} value]", this);
    msg_class![env; NSString string]
}

- (u64)version {
    log!("TODO: [(NSHTTPCookie*) {:?} version]", this);
    0
}

- (id)expiresDate {
    log!("TODO: [(NSHTTPCookie*) {:?} expiresDate]", this);
    nil
}

- (bool)isSessionOnly {
    log!("TODO: [(NSHTTPCookie*) {:?} isSessionOnly]", this);
    false
}

- (bool)isHTTPOnly {
    log!("TODO: [(NSHTTPCookie*) {:?} isHTTPOnly]", this);
    false
}

- (bool)isSecure {
    log!("TODO: [(NSHTTPCookie*) {:?} isSecure]", this);
    false
}

- (id)sameSitePolicy {
    log!("TODO: [(NSHTTPCookie*) {:?} sameSitePolicy]", this);
    nil
}

- (id)comment {
    log!("TODO: [(NSHTTPCookie*) {:?} comment]", this);
    msg_class![env; NSString string]
}

- (id)commentURL {
    log!("TODO: [(NSHTTPCookie*) {:?} commentURL]", this);
    msg_class![env; NSString string]
}

@end

};
