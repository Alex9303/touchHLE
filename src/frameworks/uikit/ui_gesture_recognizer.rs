/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIGestureRecognizer`.

use crate::objc::{id, msg, objc_classes, ClassExports, SEL};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIGestureRecognizer: NSObject

- (id)initWithTarget:(id)_target action:(SEL)_action {
    msg![env; this init]
}

- (())setDelegate:(id)_del {
    // TODO
}
- (())setEnabled:(bool)_enabled {
    // TODO
}
- (())setCancelsTouchesInView:(bool)_v {
    // TODO
}
- (())setDelaysTouchesBegan:(bool)_v {
    // TODO
}
- (())setDelaysTouchesEnded:(bool)_v {
    // TODO
}

@end

@implementation UITapGestureRecognizer: UIGestureRecognizer

- (())setNumberOfTapsRequired:(u32)_n {
    // TODO
}
- (())setNumberOfTouchesRequired:(u32)_n {
    // TODO
}

@end

};
