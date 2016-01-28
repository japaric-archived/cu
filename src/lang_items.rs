#[lang = "eh_personality"]
fn eh_personality() {}

#[lang = "panic_fmt"]
fn panic_fmt() {}

#[lang = "start"]
fn start(_: *const u8, _: isize, _: *const *const u8) -> isize {
    0
}
