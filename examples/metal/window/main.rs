// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate cocoa;
extern crate metal;
extern crate winit;
#[macro_use]
extern crate objc;
extern crate objc_id;
extern crate objc_foundation;

use cocoa::base::{selector, id, class, nil};
use cocoa::foundation::{NSUInteger, NSRect, NSPoint, NSSize,
                        NSAutoreleasePool, NSProcessInfo};
use cocoa::appkit::{NSApp,
                    NSApplication, NSApplicationActivationPolicyRegular,
                    NSWindow, NSTitledWindowMask, NSBackingStoreBuffered,
                    NSMenu, NSMenuItem, NSRunningApplication, NSView,
                    NSApplicationActivateIgnoringOtherApps};

use objc::Message;
use objc::runtime::{Object, Class, BOOL, YES, NO};
use objc_id::{Id, ShareId};
use objc_foundation::{INSObject, NSString, INSString};

use metal::*;

use winit::os::macos::WindowExt;

use std::ffi::CStr;
use std::mem;
use std::marker::PhantomData;

enum CAMetalDrawable { }
trait ICAMetalDrawable<'a> : INSObject {
    fn texture(&'a self) -> &'a MTLTexture {
        unsafe {
            msg_send![self, texture]
        }
    }
}

impl INSObject for CAMetalDrawable {
    fn class() -> &'static Class {
        Class::get("CAMetalDrawable").unwrap()
    }
}

unsafe impl Message for CAMetalDrawable { }
impl<'a> ICAMetalDrawable<'a> for CAMetalDrawable { }

enum CAMetalLayer { }
trait ICAMetalLayer<'a> : INSObject {
    fn layer() -> &'a CAMetalLayer {
        unsafe {
            msg_send![Self::class(), layer]
        }
    }

    fn set_device(&self, device: &MTLDevice) {
        unsafe {
            msg_send![self, setDevice:device]
        }
    }

    fn pixel_format(&self) -> MTLPixelFormat {
        unsafe {
            msg_send![self, pixelFormat]
        }
    }

    fn set_pixel_format(&self, pixel_format: MTLPixelFormat) {
        unsafe {
            msg_send![self, setPixelFormat:pixel_format]
        }
    }
    
    fn drawable_size(&self) -> NSSize {
        unsafe {
            msg_send![self, drawableSize]
        }
    }

    fn set_drawable_size(&self, size: NSSize) {
        unsafe {
            msg_send![self, setDrawableSize:size]
        }
    }

    fn next_drawable(&'a self) -> &'a CAMetalDrawable {
        unsafe {
            msg_send![self, nextDrawable]
        }
    }
}

impl INSObject for CAMetalLayer {
    fn class() -> &'static Class {
        Class::get("CAMetalLayer").unwrap()
    }
}

unsafe impl Message for CAMetalLayer { }
impl<'a> ICAMetalLayer<'a> for CAMetalLayer { }

fn prepare_pipeline_state<'a>(device: &'a Id<MTLDevice>, library: Id<MTLLibrary>) -> &'a MTLRenderPipelineState {
    let vert = library.get_function("triangle_vertex").unwrap();
    let frag = library.get_function("triangle_fragment").unwrap();

    let pipeline_state_descriptor = MTLRenderPipelineDescriptor::new();
    pipeline_state_descriptor.set_vertex_function(vert);
    pipeline_state_descriptor.set_fragment_function(frag);
    pipeline_state_descriptor.color_attachments().object_at(0).set_pixel_format(MTLPixelFormat::BGRA8Unorm);

    let pipeline_state = device.new_render_pipeline_state(&*pipeline_state_descriptor).unwrap();

    pipeline_state
}

fn prepare_render_pass_descriptor(descriptor: &ShareId<MTLRenderPassDescriptor>, texture: &MTLTexture) {
    let color_attachment = descriptor.color_attachments().object_at(0);

    color_attachment.set_texture(texture);
    color_attachment.set_load_action(MTLLoadAction::Clear);
    color_attachment.set_clear_color(MTLClearColor::new(0.5, 0.2, 0.2, 1.0));
    color_attachment.set_store_action(MTLStoreAction::Store);
}

fn main() {
    let glutin_window = winit::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title("Metal".into()).build().unwrap();

    unsafe {
        let window: id = mem::transmute(glutin_window.get_nswindow());
        let device = create_system_default_device().share();

        let layer = CAMetalLayer::layer();
        layer.set_device(&*(device.clone()));
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);

        let view = window.contentView();
        view.setWantsBestResolutionOpenGLSurface_(YES);
        view.setWantsLayer(YES);
        view.setLayer(mem::transmute(layer));

        let draw_size = glutin_window.get_inner_size().unwrap();
        layer.set_drawable_size(NSSize::new(draw_size.0 as f64, draw_size.1 as f64));

        let mut drawable = None;

        let library = device.new_default_library();
        let render_pass_descriptor = MTLRenderPassDescriptor::new().share();
        let command_queue = device.new_command_queue();

        let vbuf = {
            let vertex_data = [
                 -0.5, -0.5, 1.0, 0.0, 0.0,
                  0.5, -0.5, 0.0, 1.0, 0.0,
                  0.0,  0.5, 0.0, 0.0, 1.0,
            ];

            device.new_buffer(mem::transmute(vertex_data.as_ptr()), vertex_data.len() * mem::size_of::<f32>(), MTLResourceOptionCPUCacheModeDefault)
        };

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
                None => drawable = Some(layer.next_drawable())
            };

            prepare_render_pass_descriptor(&render_pass_descriptor, &drawable.unwrap().texture());

            let command_buffer = command_queue.new_command_buffer();
            let encoder = command_buffer.new_render_command_encoder(&render_pass_descriptor);
            encoder.end_encoding();

            command_buffer.present_drawable(mem::transmute(drawable.unwrap()));
            command_buffer.commit();

            //let _: () = msg_send![drawable.unwrap().0, release];

            drawable = None;
        }
    }
}
