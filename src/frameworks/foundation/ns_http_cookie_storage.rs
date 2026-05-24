/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSHTTPCookieStorage`.

use crate::objc::{
    id, msg, msg_class, objc_classes, ClassExports, HostObject, NSZonePtr,
};

#[derive(Default)]
struct NSHTTPCookieStorageHostObject {}
impl HostObject for NSHTTPCookieStorageHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSHTTPCookieStorage: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<NSHTTPCookieStorageHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

+ (id)sharedHTTPCookieStorage {
    let instance: id = msg![env; this alloc];
    msg![env; instance init]
}

+ (id)sharedCookieStorageForGroupContainerIdentifier:(id)identifier {
    log!("TODO: [(NSHTTPCookieStorage) sharedCookieStorageForGroupContainerIdentifier:{:?}]", identifier);
    let instance: id = msg![env; this alloc];
    msg![env; instance init]
}

- (())dealloc {
    env.objc.dealloc_object(this, &mut env.mem);
}

- (i32)cookieAcceptPolicy {
    log!("TODO: [(NSHTTPCookieStorage*) {:?} cookieAcceptPolicy]", this);
    0 // NSHTTPCookieAcceptPolicyAlways
}

- (())setCookieAcceptPolicy:(i32)policy {
    log!("TODO: [(NSHTTPCookieStorage*) {:?} setCookieAcceptPolicy:{}]", this, policy);
}

- (())removeCookiesSinceDate:(id)date {
    log!("TODO: [(NSHTTPCookieStorage*) {:?} removeCookiesSinceDate:{:?}]", this, date);
}

- (())deleteCookie:(id)cookie {
    log!("TODO: [(NSHTTPCookieStorage*) {:?} deleteCookie:{:?}]", this, cookie);
}

- (())setCookie:(id)cookie {
    log!("TODO: [(NSHTTPCookieStorage*) {:?} setCookie:{:?}]", this, cookie);
}

- (())setCookies:(id)cookies forURL:(id)url mainDocumentURL:(id)mainDocumentURL {
    log!("TODO: [(NSHTTPCookieStorage*) {:?} setCookies:{:?} forURL:{:?} mainDocumentURL:{:?}]", this, cookies, url, mainDocumentURL);
}

- (())storeCookies:(id)cookies forTask:(id)task {
    log!("TODO: [(NSHTTPCookieStorage*) {:?} storeCookies:{:?} forTask:{:?}]", this, cookies, task);
}

- (id)cookies {
    log!("TODO: [(NSHTTPCookieStorage*) {:?} cookies]", this);
    msg_class![env; NSArray array]
}

- (())getCookiesForTask:(id)task completionHandler:(id)handler {
    log!("TODO: [(NSHTTPCookieStorage*) {:?} getCookiesForTask:{:?} completionHandler:{:?}]", this, task, handler);
}

- (id)cookiesForURL:(id)url {
    log!("TODO: [(NSHTTPCookieStorage*) {:?} cookiesForURL:{:?}]", this, url);
    msg_class![env; NSArray array]
}

- (id)sortedCookiesUsingDescriptors:(id)sortDescriptors {
    log!("TODO: [(NSHTTPCookieStorage*) {:?} sortedCookiesUsingDescriptors:{:?}]", this, sortDescriptors);
    msg_class![env; NSArray array]
}

@end

};
