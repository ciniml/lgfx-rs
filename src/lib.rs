type Mutex<T> = std::sync::Mutex<T>;
type MutexGuard<'a, T> = std::sync::MutexGuard<'a, T>;

#[allow(unused)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod lgfx_sys {
    include!(concat!(env!("OUT_DIR"), "/lgfx.rs"));
}
use lgfx_sys::*;
pub use lgfx_sys::textdatum_t;

#[derive(Debug)]
pub enum EpdMode {
    Quality = 1,
    Text = 2,
    Fast = 3,
    Fastest = 4,
}

impl TryFrom<epd_mode_t> for EpdMode {
    type Error = ();
    fn try_from(value: epd_mode_t) -> Result<Self, Self::Error> {
        match value {
            epd_mode_epd_quality => Ok(Self::Quality),
            epd_mode_epd_text => Ok(Self::Text),
            epd_mode_epd_fast => Ok(Self::Fast),
            epd_mode_epd_fastest => Ok(Self::Fastest),
            _ => Err(()),
        }
    }
}
impl From<EpdMode> for epd_mode_t {
    fn from(value: EpdMode) -> Self {
        match value {
            EpdMode::Quality => epd_mode_epd_quality,
            EpdMode::Text => epd_mode_epd_text,
            EpdMode::Fast => epd_mode_epd_fast,
            EpdMode::Fastest => epd_mode_epd_fastest,
        }
    }
}

pub struct Gfx {
    target: Mutex<lgfx_target_t>,
}
unsafe impl Send for Gfx {}

pub struct SharedLgfxTarget<'a> {
    mutex: &'a Mutex<lgfx_target_t>,
}
impl<'a> SharedLgfxTarget<'a> {
    pub fn new(mutex: &'a Mutex<lgfx_target_t>) -> Self {
        Self { mutex }
    }
    pub fn lock<'b>(&'b self) -> LgfxGuard<'b> {
        LgfxGuard::<'b> {
            update_suppressed: false,
            guard: self.mutex.lock().unwrap(),
        }
    }
    pub fn lock_without_auto_update<'b>(&'b self) -> LgfxGuard<'b> {
        let mut guard = self.mutex.lock().unwrap();
        unsafe {
            lgfx_c_start_write(guard.target());
        }
        LgfxGuard::<'b> {
            update_suppressed: true,
            guard,
        }
    }
}
pub struct LgfxGuard<'a> {
    update_suppressed: bool,
    guard: MutexGuard<'a, lgfx_target_t>,
}

impl<'a> LgfxGuard<'a> {
    pub fn is_epd(&mut self) -> bool {
        unsafe { lgfx_c_is_epd(self.target()) }
    }
    pub fn get_epd_mode(&mut self) -> EpdMode {
        let epd_mode = unsafe { lgfx_c_get_epd_mode(self.target()) };
        epd_mode.try_into().expect("unknown EPD mode returned by LGFX.")
    }
    pub fn set_epd_mode(&mut self, mode: EpdMode) {
        unsafe { lgfx_c_set_epd_mode(self.target(), mode.into()); }
    }
    pub fn set_rotation(&mut self, rotation: u8) {
        unsafe { lgfx_c_set_rotation(self.target(), rotation ); }
    }
}

impl<'a> LgfxTarget for LgfxGuard<'a> {
    fn target(&self) -> lgfx_target_t {
        *self.guard
    }
}

impl<'a> Drop for LgfxGuard<'a> {
    fn drop(&mut self) {
        if self.update_suppressed {
            unsafe {
                lgfx_c_end_write(self.guard.target());
            }
        }
    }
}


static mut GFX_INITIALIZED: bool = false;
impl Gfx {
    #[cfg(target_os="espidf")]
    pub fn setup() -> Option<Gfx> {
        if unsafe { GFX_INITIALIZED } {
            None
        } else {
            unsafe {
                GFX_INITIALIZED = true;
            }
            Some(Gfx {
                target: Mutex::new(unsafe { lgfx_c_setup() }),
            })
        }
    }
    #[cfg(target_os="linux")]
    pub fn setup(width: i32, height: i32) -> Option<Gfx> {
        if unsafe { GFX_INITIALIZED } {
            None
        } else {
            unsafe {
                GFX_INITIALIZED = true;
            }
            Some(Gfx {
                target: Mutex::new(unsafe { lgfx_c_setup_with_size(width, height) }),
            })
        }
    }
    pub fn as_shared<'a>(&'a self) -> SharedLgfxTarget<'a> {
        SharedLgfxTarget::new(&self.target)
    }
    pub fn create_sprite(&self, w: i32, h: i32) -> Result<Sprite, ()> {
        Sprite::new(self, w, h)
    }

    #[cfg(target_os="linux")]
    pub fn handle_sdl_event() {
        unsafe { lgfx_c_panel_sdl_event_handler(); }
    }
}
impl LgfxTarget for lgfx_target_t {
    fn target(&self) -> lgfx_target_t {
        *self
    }
}

impl<Target> DrawImage for Target
where
    Target: LgfxTarget,
{
    fn draw_png<'a>(&mut self, data: &'a [u8]) -> DrawPng<'a> {
        DrawPng::new(self.target(), data)
    }
}

pub struct Sprite {
    target: lgfx_target_t,
}
impl Sprite {
    fn new(gfx: &Gfx, w: i32, h: i32) -> Result<Self, ()> {
        let mut target = gfx.as_shared().mutex.lock().unwrap();
        let sprite = unsafe { lgfx_c_create_sprite(target.target(), w, h) };
        if sprite == core::ptr::null_mut() {
            Err(())
        } else {
            Ok(Self { target: sprite })
        }
    }

    /// Pushes the sprite to the GFX.
    /// gfx: The parent GFX of this sprite.
    pub fn push_sprite(&self, gfx: &Gfx, x: i32, y: i32) {
        let _target = gfx.as_shared().mutex.lock().unwrap(); // Just lock the parent GFX.
        unsafe { lgfx_c_push_sprite(self.target, x, y) };
    }
}
impl LgfxTarget for Sprite {
    fn target(&self) -> lgfx_target_t {
        self.target
    }
}
impl Drop for Sprite {
    fn drop(&mut self) {
        unsafe { lgfx_c_delete_sprite(self.target) };
    }
}

pub trait LgfxTarget {
    fn target(&self) -> lgfx_target_t;
}

pub trait DrawImage {
    fn draw_png<'a>(&mut self, data: &'a [u8]) -> DrawPng<'a>;
}

pub trait Color: Clone {
    fn as_u32(&self) -> u32;
}

#[derive(Debug, Copy, Clone)]
pub struct ColorRgb332 {
    raw: u8,
}
impl ColorRgb332 {
    pub fn new(raw: u8) -> Self {
        Self { raw }
    }
}
impl Color for ColorRgb332 {
    fn as_u32(&self) -> u32 {
        let r = (self.raw & 0xe0) << 0;
        let g = (self.raw & 0x1c) << 3;
        let b = (self.raw & 0x03) << 6;
        (((r | ((0u8.wrapping_sub((r >> 5) & 1)) & 0x1f)) as u32) << 16)
            | (((g | ((0u8.wrapping_sub((g >> 5) & 1)) & 0x1f)) as u32) << 8)
            | ((b | ((0u8.wrapping_sub((b >> 6) & 1)) & 0x3f)) as u32)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ColorRgb888 {
    raw: u32,
}
impl ColorRgb888 {
    pub fn new(raw: u32) -> Self {
        Self { raw }
    }
}
impl Color for ColorRgb888 {
    fn as_u32(&self) -> u32 {
        self.raw & 0xffffff
    }
}

pub trait Screen {
    fn size(&self) -> (i32, i32);
}
impl<Target> Screen for Target
where
    Target: LgfxTarget,
{
    fn size(&self) -> (i32, i32) {
        unsafe { (lgfx_c_width(self.target()), lgfx_c_height(self.target())) }
    }
}

pub trait DrawPrimitives<C: Color> {
    fn clear(&mut self, color: C);
    fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: C);
    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: C);
}

impl<Target> DrawPrimitives<ColorRgb332> for Target
where
    Target: LgfxTarget,
{
    fn clear(&mut self, color: ColorRgb332) {
        unsafe {
            lgfx_c_clear_rgb332(self.target(), color.raw);
        }
    }
    fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: ColorRgb332) {
        unsafe {
            lgfx_c_fill_rect_rgb332(self.target(), x, y, w, h, color.raw);
        }
    }
    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: ColorRgb332) {
        unsafe {
            lgfx_c_draw_line_rgb332(self.target(), x0, y0, x1, y1, color.raw);
        }
    }
}
impl<Target> DrawPrimitives<ColorRgb888> for Target
where
    Target: LgfxTarget,
{
    fn clear(&mut self, color: ColorRgb888) {
        unsafe {
            lgfx_c_clear_rgb888(self.target(), color.raw);
        }
    }
    fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: ColorRgb888) {
        unsafe {
            lgfx_c_fill_rect_rgb888(self.target(), x, y, w, h, color.raw);
        }
    }
    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: ColorRgb888) {
        unsafe {
            lgfx_c_draw_line_rgb888(self.target(), x0, y0, x1, y1, color.raw);
        }
    }
}

pub trait DrawChar<C: Color> {
    fn draw_char(&self, c: char, x: i32, y: i32, fg: C, bg: C, size_x: f32, size_y: f32) -> i32;
}
pub trait DrawChars<C: Color> {
    fn draw_chars(&self, s: &str, x: i32, y: i32, fg: C, bg: C, size_x: f32, size_y: f32) -> i32;
}

impl<Target> DrawChar<ColorRgb332> for Target
where
    Target: LgfxTarget,
{
    fn draw_char(
        &self,
        c: char,
        x: i32,
        y: i32,
        fg: ColorRgb332,
        bg: ColorRgb332,
        size_x: f32,
        size_y: f32,
    ) -> i32 {
        let mut buf = [0u16; 2];
        let encoded = c.encode_utf16(&mut buf);
        let mut width = 0;

        width += if encoded.len() >= 1 {
            unsafe {
                lgfx_c_draw_char_rgb332(
                    self.target(),
                    x,
                    y,
                    encoded[0],
                    fg.raw,
                    bg.raw,
                    size_x,
                    size_y,
                ) as i32
            }
        } else {
            0
        };
        width += if encoded.len() >= 2 {
            unsafe {
                lgfx_c_draw_char_rgb332(
                    self.target(),
                    x,
                    y,
                    encoded[1],
                    fg.raw,
                    bg.raw,
                    size_x,
                    size_y,
                ) as i32
            }
        } else {
            0
        };
        width
    }
}
impl<Target> DrawChar<ColorRgb888> for Target
where
    Target: LgfxTarget,
{
    fn draw_char(
        &self,
        c: char,
        x: i32,
        y: i32,
        fg: ColorRgb888,
        bg: ColorRgb888,
        size_x: f32,
        size_y: f32,
    ) -> i32 {
        let mut buf = [0u16; 2];
        let encoded = c.encode_utf16(&mut buf);
        let mut width = 0;

        width += if encoded.len() >= 1 {
            unsafe {
                lgfx_c_draw_char_rgb888(
                    self.target(),
                    x,
                    y,
                    encoded[0],
                    fg.raw,
                    bg.raw,
                    size_x,
                    size_y,
                ) as i32
            }
        } else {
            0
        };
        width += if encoded.len() >= 2 {
            unsafe {
                lgfx_c_draw_char_rgb888(
                    self.target(),
                    x,
                    y,
                    encoded[1],
                    fg.raw,
                    bg.raw,
                    size_x,
                    size_y,
                ) as i32
            }
        } else {
            0
        };
        width
    }
}
impl<Target, C> DrawChars<C> for Target
where
    Target: LgfxTarget + DrawChar<C>,
    C: Color,
{
    fn draw_chars(&self, s: &str, x: i32, y: i32, fg: C, bg: C, size_x: f32, size_y: f32) -> i32 {
        let mut width = 0;
        for c in s.chars() {
            width += self.draw_char(c, x + width, y, fg.clone(), bg.clone(), size_x, size_y);
        }
        width
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LgfxFont {
    pub(crate) ptr: *const core::ffi::c_void,
}
unsafe impl Sync for LgfxFont {}
unsafe impl Send for LgfxFont {}

pub trait FontManupulation {
    fn font_height(&mut self) -> i32;
    fn set_font(&mut self, font: LgfxFont) -> Result<(), ()>;
    fn set_text_size(&mut self, sx: f32, sy: f32);
}
impl<Target: LgfxTarget> FontManupulation for Target {
    fn font_height(&mut self) -> i32 {
        unsafe { lgfx_c_font_height(self.target()) }
    }
    fn set_font(&mut self, font: LgfxFont) -> Result<(), ()> {
        let success = unsafe { lgfx_c_set_font(self.target(), font.ptr) };
        if success {
            Ok(())
        } else {
            Err(())
        }
    }
    fn set_text_size(&mut self, sx: f32, sy: f32) {
        unsafe {
            lgfx_c_set_text_size(self.target(), sx, sy);
        }
    }
}

#[must_use]
pub struct DrawPng<'a> {
    target: lgfx_target_t,
    data: &'a [u8],
    x: i32,
    y: i32,
    max_width: i32,
    max_height: i32,
    offset_x: i32,
    offset_y: i32,
    scale_x: f32,
    scale_y: f32,
    datum_: textdatum_t,
}

impl<'a> DrawPng<'a> {
    const fn new(target: lgfx_target_t, data: &'a [u8]) -> Self {
        Self {
            target,
            data,
            x: 0,
            y: 0,
            max_width: 0,
            max_height: 0,
            offset_x: 0,
            offset_y: 0,
            scale_x: 1.0,
            scale_y: 0.0,
            datum_: textdatum_top_left,
        }
    }
    pub fn postion(mut self, x: i32, y: i32) -> Self {
        self.x = x;
        self.y = y;
        self
    }
    pub fn max_size(mut self, max_width: i32, max_height: i32) -> Self {
        self.max_width = max_width;
        self.max_height = max_height;
        self
    }
    pub fn offset(mut self, offset_x: i32, offset_y: i32) -> Self {
        self.offset_x = offset_x;
        self.offset_y = offset_y;
        self
    }
    pub fn scale(mut self, scale_x: f32, scale_y: f32) -> Self {
        self.scale_x = scale_x;
        self.scale_y = scale_y;
        self
    }
    pub fn datum(mut self, datum: textdatum_t) -> Self {
        self.datum_ = datum;
        self
    }
    pub fn execute(self) {
        unsafe {
            lgfx_c_draw_png(
                self.target,
                self.data.as_ptr(),
                self.data.len() as u32,
                self.x,
                self.y,
                self.max_width,
                self.max_height,
                self.offset_x,
                self.offset_y,
                self.scale_x,
                self.scale_y,
                self.datum_,
            )
        };
    }
}

pub struct LgfxDisplay<'a, Target: LgfxTarget> {
    target: &'a mut Target,
}
impl<'a, Target: LgfxTarget> LgfxDisplay<'a, Target> {
    pub fn new(target: &'a mut Target) -> Self {
        Self { target }
    }
}
impl<'a, Target: LgfxTarget> LgfxTarget for LgfxDisplay<'a, Target> {
    fn target(&self) -> lgfx_target_t {
        self.target.target()
    }
}
impl<'a, Target: LgfxTarget> embedded_graphics::prelude::OriginDimensions
    for LgfxDisplay<'a, Target>
{
    fn size(&self) -> embedded_graphics::prelude::Size {
        let size = Screen::size(self);
        embedded_graphics::prelude::Size::new(size.0 as u32, size.1 as u32)
    }
}
impl<'a, Target: LgfxTarget> embedded_graphics::prelude::DrawTarget for LgfxDisplay<'a, Target> {
    type Color = embedded_graphics::pixelcolor::Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        for embedded_graphics::Pixel(coord, color) in pixels.into_iter() {
            self.draw_line(
                coord.x,
                coord.y,
                coord.x,
                coord.y,
                ColorRgb888::new(embedded_graphics::pixelcolor::IntoStorage::into_storage(
                    color,
                )),
            );
        }
        Ok(())
    }
    fn fill_solid(
        &mut self,
        area: &embedded_graphics::primitives::Rectangle,
        color: Self::Color,
    ) -> Result<(), Self::Error> {
        self.fill_rect(
            area.top_left.x as i32,
            area.top_left.y as i32,
            area.size.width as i32,
            area.size.height as i32,
            ColorRgb888::new(embedded_graphics::pixelcolor::IntoStorage::into_storage(
                color,
            )),
        );
        Ok(())
    }
}

// TODO: ピクセルバッファを確保してpush imageするfill_contiguous実装を作る

// Font definitions
pub mod fonts {
    use super::LgfxFont;
    include!(concat!(env!("OUT_DIR"), "/lgfx_fonts.rs"));
}
