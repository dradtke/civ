extern crate cocoa;
#[macro_use] extern crate objc;

use objc::runtime::Object;
use cocoa::appkit::{CGFloat, NSScreen};
use cocoa::base::nil;

fn main() {
    unsafe {
        let screens = NSScreen::screens(nil);
        let screen_count: usize = msg_send![screens, count];
        println!("Found {:?} screen(s)", screen_count);
        let enumerator: *const Object = msg_send![screens, objectEnumerator];
        loop {
            let screen: *const Object = msg_send![enumerator, nextObject];
            if screen == nil {
                break
            }
            let scale: CGFloat = msg_send![screen, backingScaleFactor];
            println!("scale: {}", scale);
        }
    }
}
