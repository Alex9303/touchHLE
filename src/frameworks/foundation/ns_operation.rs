/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSOperation`.

use crate::objc::{id, msg, msg_super, nil, objc_classes, ClassExports, SEL};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSOperation: NSObject

- (id)init {
    msg_super![env; this init]
}

- (())start {
    if !(msg![env; this isCancelled]) {
        msg![env; this main]
    }
}

- (())main {}

- (bool)isExecuting {
    false
}
- (bool)isFinished {
    true
}
- (bool)isCancelled {
    false
}
- (bool)isReady {
    true
}
- (bool)isConcurrent {
    false
}
- (bool)isAsynchronous {
    false
}

- (())cancel {
    // TODO
}

- (())addDependency:(id)_op {
    // TODO
}
- (())removeDependency:(id)_op {
    // TODO
}
- (id)dependencies {
    nil
}

@end

@implementation NSInvocationOperation: NSOperation
- (id)initWithTarget:(id)_t action:(SEL)_a {
    msg![env; this init]
}
- (id)initWithTarget:(id)_t action:(SEL)_a object:(id)_o {
    msg![env; this init]
}
@end

@implementation NSBlockOperation: NSOperation
+ (id)blockOperationWithBlock:(id)_b { msg![env; this new] }
- (())addExecutionBlock:(id)_b {}
@end

@implementation NSOperationQueue: NSObject
- (id)init {
    msg_super![env; this init]
}
- (())addOperation:(id)op {
    msg![env; op start]
}
- (())addOperationWithBlock:(id)_b {
    // TODO
}
- (())setMaxConcurrentOperationCount:(i32)_c {
    // TODO
}
@end

};
