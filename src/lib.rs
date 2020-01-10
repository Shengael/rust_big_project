#[no_mangle]
pub extern fn process() -> i32 {
    return unsafe { i4_max(3, 10) }
}

struct Pixel {
    r: i32,
    g: i32,
    b: i32
}

struct Image {
    height: i32,
    width: i32,
    pixels: Vec<Pixel>
}

#[link(name = "ppma_io")]
extern "C" {
    fn i4_max(val: i32, val1: i32) -> i32;
}
