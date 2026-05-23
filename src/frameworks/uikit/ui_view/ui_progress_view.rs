/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIProgressView`.

use crate::{msg, msg_super};
use crate::objc::{id, nil, objc_classes, ClassExports};
use crate::frameworks::core_graphics::cg_geometry::CGRect;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIProgressView: UIView

- (id)initWithFrame:(CGRect)frame {
    let this: id = msg_super![env; this initWithFrame:frame];
    if this != nil {
        log!("TODO: [(UIProgressView*) {:?} initWithFrame:]", this);
    }
    this
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];
    if this != nil {
        log!("TODO: [(UIProgressView*) {:?} initWithCoder:{:?}]", this, coder);
    }
    this
}

- (id)initWithProgressViewStyle:(i32)style {
    // UIProgressViewStyleDefault = 0, UIProgressViewStyleBar = 1
    let frame = CGRect::default(); 
    let this: id = msg![env; this initWithFrame:frame];
    if this != nil {
        log!("TODO: [(UIProgressView*) {:?} initWithProgressViewStyle:{}]", this, style);
    }
    this
}

- (f32)progress {
    log!("TODO: [(UIProgressView*) {:?} progress]", this);
    0.0
}

- (())setProgress:(f32)progress {
    log!("TODO: [(UIProgressView*) {:?} setProgress:{}]", this, progress);
}

- (())setProgress:(f32)progress animated:(bool)animated {
    log!("TODO: [(UIProgressView*) {:?} setProgress:{} animated:{}]", this, progress, animated);
}

- (i32)progressViewStyle {
    0
}

- (())setProgressViewStyle:(i32)style {
    log!("TODO: [(UIProgressView*) {:?} setProgressViewStyle:{}]", this, style);
}

// Color and Image properties 
- (())setProgressTintColor:(id)color {
    log!("TODO: [(UIProgressView*) {:?} setProgressTintColor:{:?}]", this, color);
}

- (id)progressTintColor {
    nil
}

- (())setTrackTintColor:(id)color {
    log!("TODO: [(UIProgressView*) {:?} setTrackTintColor:{:?}]", this, color);
}

- (id)trackTintColor {
    nil
}

- (())setProgressImage:(id)image {
    log!("TODO: [(UIProgressView*) {:?} setProgressImage:{:?}]", this, image);
}

- (id)progressImage {
    nil
}

- (())setTrackImage:(id)image {
    log!("TODO: [(UIProgressView*) {:?} setTrackImage:{:?}]", this, image);
}

- (id)trackImage {
    nil
}

@end

};
