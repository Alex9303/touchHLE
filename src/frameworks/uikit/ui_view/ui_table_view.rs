/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UITableView`.

use crate::frameworks::core_graphics::{CGRect, CGFloat};
use crate::objc::{id, msg, objc_classes, ClassExports};
use crate::frameworks::uikit::ui_view::NSInteger;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UITableView: UIScrollView

- (id)initWithFrame:(CGRect)frame style:(NSInteger)_style {
    msg![env; this initWithFrame:frame]
}

- (())setDataSource:(id)_ds {
    // TODO
}
- (())setDelegate:(id)_del {
    // TODO
}
- (())setRowHeight:(CGFloat)_h {
    // TODO
}
- (())setSeparatorStyle:(NSInteger)_s {
    // TODO
}
- (())reloadData {
    // TODO
}

@end

};
