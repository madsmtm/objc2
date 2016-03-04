// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[macro_use]
extern crate objc;
extern crate cocoa;
extern crate metal;
extern crate winit;

use cocoa::base::{selector, id, class, nil, BOOL, NO, YES};
use cocoa::foundation::{NSUInteger, NSRect, NSPoint, NSSize,
                        NSAutoreleasePool, NSProcessInfo, NSString};
use cocoa::appkit::{NSApp,
                    NSApplication, NSApplicationActivationPolicyRegular,
                    NSWindow, NSTitledWindowMask, NSBackingStoreBuffered,
                    NSMenu, NSMenuItem, NSRunningApplication, NSView,
                    NSApplicationActivateIgnoringOtherApps};

use metal::*;

use winit::os::macos::WindowExt;

use std::ffi::CStr;
use std::mem;

#[derive(Copy, Clone)]
struct CAMetalDrawable(id);

impl CAMetalDrawable {
    unsafe fn texture(self) -> id {
        msg_send![self.0, texture]
    }

    unsafe fn layer(self) -> id {
        msg_send![self.0, layer]
    }
}

trait CAMetalLayer {
    unsafe fn layer(_: Self) -> id {
        msg_send![class("CAMetalLayer"), layer]
    }

    unsafe fn device(self) -> id;
    unsafe fn setDevice_(self, device: id);

    unsafe fn pixelFormat(self) -> id;
    unsafe fn setPixelFormat_(self, format: MTLPixelFormat);

    unsafe fn drawableSize(self) -> NSSize;
    unsafe fn setDrawableSize(self, drawableSize: NSSize);

    unsafe fn nextDrawable(self) -> CAMetalDrawable;
}

impl CAMetalLayer for id {
    unsafe fn device(self) -> id {
        msg_send![self, device]
    }

    unsafe fn setDevice_(self, device: id) {
        msg_send![self, setDevice:device]
    }

    unsafe fn pixelFormat(self) -> id {
        msg_send![self, pixelFormat]
    }

    unsafe fn setPixelFormat_(self, format: MTLPixelFormat) {
        msg_send![self, setPixelFormat:format]
    }

    unsafe fn drawableSize(self) -> NSSize {
        msg_send![self, drawableSize]
    }

    unsafe fn setDrawableSize(self, drawableSize: NSSize) {
        msg_send![self, setDrawableSize:drawableSize]
    }

    unsafe fn nextDrawable(self) -> CAMetalDrawable {
        CAMetalDrawable(msg_send![self, nextDrawable])
    }
}

unsafe fn prepare_renderpass_descriptor(descriptor: id, texture: id) {
    let color_attachment = MTLRenderPassColorAttachmentDescriptorArray::objectAtIndexedSubscript(MTLRenderPassDescriptor::colorAttachments(descriptor), 0 as NSUInteger);

    color_attachment.setTexture(texture);
    color_attachment.setLoadAction(MTLLoadAction::MTLLoadActionClear);
    color_attachment.setClearColor(MTLClearColor::new(0.5, 0.2, 0.2, 1.0));
    color_attachment.setStoreAction(MTLStoreAction::MTLStoreActionStore);
}

fn main() {
    let glutin_window = winit::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title("Metal".into()).build().unwrap();

    unsafe {
        let window: id = mem::transmute(glutin_window.get_nswindow());
        let device = MTLCreateSystemDefaultDevice();

        let layer = CAMetalLayer::layer(nil);
        layer.setDevice_(device);
        layer.setPixelFormat_(MTLPixelFormat::MTLPixelFormatBGRA8Unorm);

        let view = window.contentView();
        view.setWantsBestResolutionOpenGLSurface_(YES);
        view.setWantsLayer(YES);
        view.setLayer(layer);

        let draw_size = glutin_window.get_inner_size().unwrap();
        layer.setDrawableSize(NSSize::new(draw_size.0 as f64, draw_size.1 as f64));

        println!("device: {:?}", CStr::from_ptr(MTLDevice::name(device).UTF8String()));
        println!("threadgroup: {:?}", device.maxThreadsPerThreadgroup());

        let mut drawable = None;
        let renderpass_descriptor = MTLRenderPassDescriptor::renderPassDescriptor(nil);

        let commandqueue = device.newCommandQueue();

        loop {
            for event in glutin_window.poll_events() {
                match event {
                    winit::Event::Closed => break,
                    _ => ()
                }
            }

            //let pool = NSAutoreleasePool::new(nil);

            match drawable {
                Some(_) => {},
                None => drawable = Some(layer.nextDrawable())
            };

            prepare_renderpass_descriptor(renderpass_descriptor, drawable.unwrap().texture());

            let commandbuffer: id = commandqueue.commandBuffer();
            let encoder = commandbuffer.renderCommandEncoderWithDescriptor(renderpass_descriptor);
            encoder.endEncoding();

            commandbuffer.presentDrawable(drawable.unwrap().0);
            commandbuffer.commit();

            let _: () = msg_send![encoder, release];
            let _: () = msg_send![commandbuffer, release];

            let _: () = msg_send![drawable.unwrap().0, release];

            drawable = None;
        }
    }
}
