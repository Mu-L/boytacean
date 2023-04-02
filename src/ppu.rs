use core::fmt;
use std::{
    borrow::BorrowMut,
    fmt::{Display, Formatter},
};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::warnln;

pub const VRAM_SIZE: usize = 8192;
pub const HRAM_SIZE: usize = 128;
pub const OAM_SIZE: usize = 260;
pub const PALETTE_SIZE: usize = 4;
pub const RGB_SIZE: usize = 3;
pub const TILE_WIDTH: usize = 8;
pub const TILE_HEIGHT: usize = 8;
pub const TILE_DOUBLE_HEIGHT: usize = 16;

/// The number of tiles that can be store in Game Boy's
/// VRAM memory according to specifications.
pub const TILE_COUNT: usize = 384;

/// The number of objects/sprites that can be handled at
/// the same time by the Game Boy.
pub const OBJ_COUNT: usize = 40;

/// The width of the Game Boy screen in pixels.
pub const DISPLAY_WIDTH: usize = 160;

/// The height of the Game Boy screen in pixels.
pub const DISPLAY_HEIGHT: usize = 144;

/// The size to be used by the buffer of colors
/// for the Game Boy screen the values there should
/// range from 0 to 3.
pub const COLOR_BUFFER_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

/// The size of the RGB frame buffer in bytes.
pub const FRAME_BUFFER_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT * RGB_SIZE;

/// The base colors to be used to populate the
/// custom palettes of the Game Boy.
pub const PALETTE_COLORS: Palette = [[255, 255, 255], [192, 192, 192], [96, 96, 96], [0, 0, 0]];

/// Defines the Game Boy pixel type as a buffer
/// with the size of RGB (3 bytes).
pub type Pixel = [u8; RGB_SIZE];

/// Defines a type that represents a color palette
/// within the Game Boy context.
pub type Palette = [Pixel; PALETTE_SIZE];

/// Represents a palette with the metadata that is
/// associated with it.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone, PartialEq, Eq)]
pub struct PaletteInfo {
    name: String,
    colors: Palette,
}

impl PaletteInfo {
    pub fn new(name: &str, colors: Palette) -> Self {
        Self {
            name: String::from(name),
            colors,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn colors(&self) -> &Palette {
        &self.colors
    }
}

/// Represents a tile within the Game Boy context,
/// should contain the pixel buffer of the tile.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    buffer: [u8; 64],
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Tile {
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.buffer[y * TILE_WIDTH + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.buffer[y * TILE_WIDTH + x] = value;
    }

    pub fn buffer(&self) -> Vec<u8> {
        self.buffer.to_vec()
    }
}

impl Tile {
    pub fn get_row(&self, y: usize) -> &[u8] {
        &self.buffer[y * TILE_WIDTH..(y + 1) * TILE_WIDTH]
    }
}

impl Tile {
    pub fn palette_buffer(&self, palette: Palette) -> Vec<u8> {
        self.buffer
            .iter()
            .flat_map(|p| palette[*p as usize])
            .collect()
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();
        for y in 0..8 {
            for x in 0..8 {
                buffer.push_str(format!("{}", self.get(x, y)).as_str());
            }
            buffer.push('\n');
        }
        write!(f, "{}", buffer)
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ObjectData {
    x: i16,
    y: i16,
    tile: u8,
    palette: u8,
    xflip: bool,
    yflip: bool,
    bg_over: bool,
    index: u8,
}

impl Display for ObjectData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Index => {}\nX => {}\nY => {}\nTile => {}",
            self.index, self.x, self.y, self.tile
        )
    }
}

pub struct PpuRegisters {
    pub scy: u8,
    pub scx: u8,
    pub wy: u8,
    pub wx: u8,
    pub ly: u8,
    pub lyc: u8,
}

/// Represents the Game Boy PPU (Pixel Processing Unit) and controls
/// all of the logic behind the graphics processing and presentation.
/// Should store both the VRAM and HRAM together with the internal
/// graphic related registers.
/// Outputs the screen as a RGB 8 bit frame buffer.
///
/// # Basic usage
/// ```rust
/// use boytacean::ppu::Ppu;
/// let mut ppu = Ppu::new();
/// ppu.clock(8);
/// ```
pub struct Ppu {
    /// The color buffer that is going to store the colors
    /// (from 0 to 3) for all the pixels in the screen.
    pub color_buffer: Box<[u8; COLOR_BUFFER_SIZE]>,

    /// The 8 bit based RGB frame buffer with the
    /// processed set of pixels ready to be displayed on screen.
    pub frame_buffer: Box<[u8; FRAME_BUFFER_SIZE]>,

    /// Video dedicated memory (VRAM) where both the tiles and
    /// the sprites/objects are going to be stored.
    vram: [u8; VRAM_SIZE],

    /// High RAM memory that should provide extra speed for regular
    /// operations.
    hram: [u8; HRAM_SIZE],

    /// OAM RAM (Sprite Attribute Table ) used for the storage of the
    /// sprite attributes for each of the 40 sprites of the Game Boy.
    oam: [u8; OAM_SIZE],

    /// The current set of processed tiles that are store in the
    /// PPU related structures.
    tiles: [Tile; TILE_COUNT],

    /// The meta information about the sprites/objects that are going
    /// to be drawn to the screen,
    obj_data: [ObjectData; OBJ_COUNT],

    /// The base colors that are going to be used in the registration
    /// of the concrete palettes, this value basically controls the
    /// colors that are going to be shown for each of the four base
    /// values - 0x00, 0x01, 0x02, and 0x03.
    palette_colors: Palette,

    /// The palette of colors that is currently loaded in Game Boy
    /// and used for background (tiles).
    palette: Palette,

    /// The palette that is going to be used for sprites/objects #0.
    palette_obj_0: Palette,

    /// The palette that is going to be used for sprites/objects #1.
    palette_obj_1: Palette,

    /// The complete set of palettes in binary data so that they can
    /// be re-read if required by the system.
    palettes: [u8; 3],

    /// The scroll Y register that controls the Y offset
    /// of the background.
    scy: u8,

    /// The scroll X register that controls the X offset
    /// of the background.
    scx: u8,

    /// The top most Y coordinate of the window,
    /// going to be used while drawing the window.
    wy: u8,

    /// The top most X coordinate of the window plus 7,
    /// going to be used while drawing the window.
    wx: u8,

    /// The current scan line in processing, should
    /// range between 0 (0x00) and 153 (0x99), representing
    /// the 154 lines plus 10 extra V-Blank lines.
    ly: u8,

    /// The line compare register that is going to be used
    /// in the STATE and associated interrupts.
    lyc: u8,

    /// The current execution mode of the PPU, should change
    /// between states over the drawing of a frame.
    mode: PpuMode,

    /// Internal clock counter used to control the time in ticks
    /// spent in each of the PPU modes.
    mode_clock: u16,

    /// Controls if the background is going to be drawn to screen.
    switch_bg: bool,

    /// Controls if the sprites/objects are going to be drawn to screen.
    switch_obj: bool,

    /// Defines the size in pixels of the object (false=8x8, true=8x16).
    obj_size: bool,

    /// Controls the map that is going to be drawn to screen, the
    /// offset in VRAM will be adjusted according to this
    /// (false=0x9800, true=0x9c000).
    bg_map: bool,

    /// If the background tile set is active meaning that the
    /// negative based indexes are going to be used.
    bg_tile: bool,

    /// Controls if the window is meant to be drawn.
    switch_window: bool,

    /// Controls the offset of the map that is going to be drawn
    /// for the window section of the screen.
    window_map: bool,

    /// Flag that controls if the LCD screen is ON and displaying
    /// content.
    switch_lcd: bool,

    // Internal window counter value used to control the ines that
    // were effectively rendered as part of the window tile drawing process.
    // A line is only considered rendered when the WX and WY registers
    // are within the valid screen range and the window switch register
    // is valid.
    window_counter: u8,

    /// Flag that controls if the frame currently in rendering is the
    /// first one, preventing actions.
    first_frame: bool,

    /// Almost unique identifier of the frame that can be used to debug
    /// and uniquely identify the frame that is currently ind drawing,
    /// the identifier wraps on the u16 edges.
    frame_index: u16,

    stat_hblank: bool,
    stat_vblank: bool,
    stat_oam: bool,
    stat_lyc: bool,

    /// Boolean value set when the V-Blank interrupt should be handled
    /// by the next CPU clock operation.
    int_vblank: bool,

    /// Boolean value when the LCD STAT interrupt should be handled by
    /// the next CPU clock operation.
    int_stat: bool,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PpuMode {
    HBlank = 0,
    VBlank = 1,
    OamRead = 2,
    VramRead = 3,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            color_buffer: Box::new([0u8; COLOR_BUFFER_SIZE]),
            frame_buffer: Box::new([0u8; FRAME_BUFFER_SIZE]),
            vram: [0u8; VRAM_SIZE],
            hram: [0u8; HRAM_SIZE],
            oam: [0u8; OAM_SIZE],
            tiles: [Tile { buffer: [0u8; 64] }; TILE_COUNT],
            obj_data: [ObjectData {
                x: 0,
                y: 0,
                tile: 0,
                palette: 0,
                xflip: false,
                yflip: false,
                bg_over: false,
                index: 0,
            }; OBJ_COUNT],
            palette_colors: PALETTE_COLORS,
            palette: [[0u8; RGB_SIZE]; PALETTE_SIZE],
            palette_obj_0: [[0u8; RGB_SIZE]; PALETTE_SIZE],
            palette_obj_1: [[0u8; RGB_SIZE]; PALETTE_SIZE],
            palettes: [0u8; 3],
            scy: 0x0,
            scx: 0x0,
            wy: 0x0,
            wx: 0x0,
            ly: 0x0,
            lyc: 0x0,
            mode: PpuMode::OamRead,
            mode_clock: 0,
            switch_bg: false,
            switch_obj: false,
            obj_size: false,
            bg_map: false,
            bg_tile: false,
            switch_window: false,
            window_map: false,
            switch_lcd: false,
            window_counter: 0x0,
            first_frame: false,
            frame_index: 0,
            stat_hblank: false,
            stat_vblank: false,
            stat_oam: false,
            stat_lyc: false,
            int_vblank: false,
            int_stat: false,
        }
    }

    pub fn reset(&mut self) {
        self.color_buffer = Box::new([0u8; COLOR_BUFFER_SIZE]);
        self.frame_buffer = Box::new([0u8; FRAME_BUFFER_SIZE]);
        self.vram = [0u8; VRAM_SIZE];
        self.hram = [0u8; HRAM_SIZE];
        self.tiles = [Tile { buffer: [0u8; 64] }; TILE_COUNT];
        self.palette = [[0u8; RGB_SIZE]; PALETTE_SIZE];
        self.palette_obj_0 = [[0u8; RGB_SIZE]; PALETTE_SIZE];
        self.palette_obj_1 = [[0u8; RGB_SIZE]; PALETTE_SIZE];
        self.palettes = [0u8; 3];
        self.scy = 0x0;
        self.scx = 0x0;
        self.ly = 0x0;
        self.lyc = 0x0;
        self.mode = PpuMode::OamRead;
        self.mode_clock = 0;
        self.switch_bg = false;
        self.switch_obj = false;
        self.obj_size = false;
        self.bg_map = false;
        self.bg_tile = false;
        self.switch_window = false;
        self.window_map = false;
        self.switch_lcd = false;
        self.first_frame = false;
        self.frame_index = 0;
        self.stat_hblank = false;
        self.stat_vblank = false;
        self.stat_oam = false;
        self.stat_lyc = false;
        self.int_vblank = false;
        self.int_stat = false;
    }

    pub fn clock(&mut self, cycles: u8) {
        // in case the LCD is currently off then we skip the current
        // clock operation the PPU should not work
        if !self.switch_lcd {
            return;
        }

        // increments the current mode clock by the provided amount
        // of CPU cycles (probably coming from a previous CPU clock)
        self.mode_clock += cycles as u16;

        match self.mode {
            PpuMode::OamRead => {
                if self.mode_clock >= 80 {
                    self.mode = PpuMode::VramRead;
                    self.mode_clock -= 80;
                }
            }
            PpuMode::VramRead => {
                if self.mode_clock >= 172 {
                    self.render_line();

                    self.mode = PpuMode::HBlank;
                    self.mode_clock -= 172;
                    self.update_stat()
                }
            }
            PpuMode::HBlank => {
                if self.mode_clock >= 204 {
                    // increments the window counter making sure that the
                    // valid is only incremented when both the WX and WY
                    // registers make sense (are within range), the window
                    // switch is on and the line in drawing is above WY
                    if self.switch_window
                        && self.wx as i16 - 7 < DISPLAY_WIDTH as i16
                        && self.wy < DISPLAY_HEIGHT as u8
                        && self.ly >= self.wy
                    {
                        self.window_counter += 1;
                    }

                    // increments the register that holds the
                    // information about the current line in drawing
                    self.ly += 1;

                    // in case we've reached the end of the
                    // screen we're now entering the V-Blank
                    if self.ly == 144 {
                        self.int_vblank = true;
                        self.mode = PpuMode::VBlank;
                    } else {
                        self.mode = PpuMode::OamRead;
                    }

                    self.mode_clock -= 204;
                    self.update_stat()
                }
            }
            PpuMode::VBlank => {
                if self.mode_clock >= 456 {
                    // increments the register that controls the line count,
                    // notice that these represent the extra 10 horizontal
                    // scanlines that are virtual and not real (off-screen)
                    self.ly += 1;

                    // in case the end of V-Blank has been reached then
                    // we must jump again to the OAM read mode and reset
                    // the scan line counter to the zero value
                    if self.ly == 154 {
                        self.mode = PpuMode::OamRead;
                        self.ly = 0;
                        self.window_counter = 0;
                        self.first_frame = false;
                        self.frame_index = self.frame_index.wrapping_add(1);
                        self.update_stat()
                    }

                    self.mode_clock -= 456;
                }
            }
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9fff => self.vram[(addr & 0x1fff) as usize],
            0xfe00..=0xfe9f => self.oam[(addr & 0x009f) as usize],
            0xff80..=0xfffe => self.hram[(addr & 0x007f) as usize],
            0xff40 =>
            {
                #[allow(clippy::bool_to_int_with_if)]
                (if self.switch_bg { 0x01 } else { 0x00 }
                    | if self.switch_obj { 0x02 } else { 0x00 }
                    | if self.obj_size { 0x04 } else { 0x00 }
                    | if self.bg_map { 0x08 } else { 0x00 }
                    | if self.bg_tile { 0x10 } else { 0x00 }
                    | if self.switch_window { 0x20 } else { 0x00 }
                    | if self.window_map { 0x40 } else { 0x00 }
                    | if self.switch_lcd { 0x80 } else { 0x00 })
            }
            0xff41 => {
                (if self.stat_hblank { 0x08 } else { 0x00 }
                    | if self.stat_vblank { 0x10 } else { 0x00 }
                    | if self.stat_oam { 0x20 } else { 0x00 }
                    | if self.stat_lyc { 0x40 } else { 0x00 }
                    | if self.lyc == self.ly { 0x04 } else { 0x00 }
                    | (self.mode as u8 & 0x03))
            }
            0xff42 => self.scy,
            0xff43 => self.scx,
            0xff44 => self.ly,
            0xff45 => self.lyc,
            0xff47 => self.palettes[0],
            0xff48 => self.palettes[1],
            0xff49 => self.palettes[2],
            0xff4a => self.wy,
            0xff4b => self.wx,
            // 0xFF4F — VBK (CGB only)
            0xff4f => 0xff,
            _ => {
                warnln!("Reading from unknown PPU location 0x{:04x}", addr);
                0xff
            }
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..=0x9fff => {
                self.vram[(addr & 0x1fff) as usize] = value;
                if addr < 0x9800 {
                    self.update_tile(addr, value);
                }
            }
            0xfe00..=0xfe9f => {
                self.oam[(addr & 0x009f) as usize] = value;
                self.update_object(addr, value);
            }
            0xff80..=0xfffe => self.hram[(addr & 0x007f) as usize] = value,
            0xff40 => {
                self.switch_bg = value & 0x01 == 0x01;
                self.switch_obj = value & 0x02 == 0x02;
                self.obj_size = value & 0x04 == 0x04;
                self.bg_map = value & 0x08 == 0x08;
                self.bg_tile = value & 0x10 == 0x10;
                self.switch_window = value & 0x20 == 0x20;
                self.window_map = value & 0x40 == 0x40;
                self.switch_lcd = value & 0x80 == 0x80;

                // in case the LCD is off takes the opportunity
                // to clear the screen, this is the expected
                // behaviour for this specific situation
                if !self.switch_lcd {
                    self.mode = PpuMode::HBlank;
                    self.mode_clock = 0;
                    self.ly = 0;
                    self.int_vblank = false;
                    self.int_stat = false;
                    self.first_frame = true;
                    self.clear_frame_buffer();
                }
            }
            0xff41 => {
                self.stat_hblank = value & 0x08 == 0x08;
                self.stat_vblank = value & 0x10 == 0x10;
                self.stat_oam = value & 0x20 == 0x20;
                self.stat_lyc = value & 0x40 == 0x40;
            }
            0xff42 => self.scy = value,
            0xff43 => self.scx = value,
            0xff45 => self.lyc = value,
            0xff47 => {
                Self::compute_palette(&mut self.palette, &self.palette_colors, value);
                self.palettes[0] = value;
            }
            0xff48 => {
                Self::compute_palette(&mut self.palette_obj_0, &self.palette_colors, value);
                self.palettes[1] = value;
            }
            0xff49 => {
                Self::compute_palette(&mut self.palette_obj_1, &self.palette_colors, value);
                self.palettes[2] = value;
            }
            0xff4a => self.wy = value,
            0xff4b => self.wx = value,
            // 0xFF4F — VBK (CGB only)
            0xff4f => (),
            0xff7f => (),
            _ => warnln!("Writing in unknown PPU location 0x{:04x}", addr),
        }
    }

    pub fn vram(&self) -> &[u8; VRAM_SIZE] {
        &self.vram
    }

    pub fn hram(&self) -> &[u8; HRAM_SIZE] {
        &self.hram
    }

    pub fn tiles(&self) -> &[Tile; TILE_COUNT] {
        &self.tiles
    }

    pub fn set_palette_colors(&mut self, value: &Palette) {
        self.palette_colors = *value;
        self.compute_palettes()
    }

    pub fn palette(&self) -> Palette {
        self.palette
    }

    pub fn palette_obj_0(&self) -> Palette {
        self.palette_obj_0
    }

    pub fn palette_obj_1(&self) -> Palette {
        self.palette_obj_1
    }

    pub fn ly(&self) -> u8 {
        self.ly
    }

    pub fn mode(&self) -> PpuMode {
        self.mode
    }

    pub fn frame_index(&self) -> u16 {
        self.frame_index
    }

    pub fn int_vblank(&self) -> bool {
        self.int_vblank
    }

    pub fn set_int_vblank(&mut self, value: bool) {
        self.int_vblank = value;
    }

    pub fn ack_vblank(&mut self) {
        self.set_int_vblank(false);
    }

    pub fn int_stat(&self) -> bool {
        self.int_stat
    }

    pub fn set_int_stat(&mut self, value: bool) {
        self.int_stat = value;
    }

    pub fn ack_stat(&mut self) {
        self.set_int_stat(false);
    }

    /// Fills the frame buffer with pixels of the provided color,
    /// this method must represent the fastest way of achieving
    /// the fill background with color operation.
    pub fn fill_frame_buffer(&mut self, color: Pixel) {
        self.color_buffer.fill(0);
        for index in (0..self.frame_buffer.len()).step_by(RGB_SIZE) {
            self.frame_buffer[index] = color[0];
            self.frame_buffer[index + 1] = color[1];
            self.frame_buffer[index + 2] = color[2];
        }
    }

    /// Clears the current frame buffer, setting the background color
    /// for all the pixels in the frame buffer.
    pub fn clear_frame_buffer(&mut self) {
        self.fill_frame_buffer(self.palette_colors[0]);
    }

    /// Prints the tile data information to the stdout, this is
    /// useful for debugging purposes.
    pub fn print_tile_stdout(&self, tile_index: usize) {
        println!("{}", self.tiles[tile_index]);
    }

    /// Updates the tile structure with the value that has
    /// just been written to a location on the VRAM associated
    /// with tiles.
    fn update_tile(&mut self, addr: u16, _value: u8) {
        let addr = (addr & 0x1ffe) as usize;
        let tile_index = (addr >> 4) & 0x01ff;
        let tile = self.tiles[tile_index].borrow_mut();
        let y = (addr >> 1) & 0x0007;

        let mut mask;

        for x in 0..TILE_WIDTH {
            mask = 1 << (7 - x);
            #[allow(clippy::bool_to_int_with_if)]
            tile.set(
                x,
                y,
                if self.vram[addr] & mask > 0 { 0x1 } else { 0x0 }
                    | if self.vram[addr + 1] & mask > 0 {
                        0x2
                    } else {
                        0x0
                    },
            );
        }
    }

    fn update_object(&mut self, addr: u16, value: u8) {
        let addr = (addr & 0x01ff) as usize;
        let obj_index = addr >> 2;
        if obj_index >= OBJ_COUNT {
            return;
        }
        let mut obj = self.obj_data[obj_index].borrow_mut();
        match addr & 0x03 {
            0x00 => obj.y = value as i16 - 16,
            0x01 => obj.x = value as i16 - 8,
            0x02 => obj.tile = value,
            0x03 => {
                obj.palette = (value & 0x10 == 0x10) as u8;
                obj.xflip = value & 0x20 == 0x20;
                obj.yflip = value & 0x40 == 0x40;
                obj.bg_over = value & 0x80 == 0x80;
                obj.index = obj_index as u8;
            }
            _ => (),
        }
    }

    pub fn registers(&self) -> PpuRegisters {
        PpuRegisters {
            scy: self.scy,
            scx: self.scx,
            wy: self.wy,
            wx: self.wx,
            ly: self.ly,
            lyc: self.lyc,
        }
    }

    fn render_line(&mut self) {
        if self.first_frame {
            return;
        }
        if self.switch_bg {
            self.render_map(self.bg_map, self.scx, self.scy, 0, 0, self.ly);
        }
        if self.switch_window {
            self.render_map(self.window_map, 0, 0, self.wx, self.wy, self.window_counter);
        }
        if self.switch_obj {
            self.render_objects();
        }
    }

    fn render_map(&mut self, map: bool, scx: u8, scy: u8, wx: u8, wy: u8, ld: u8) {
        // in case the target window Y position has not yet been reached
        // then there's nothing to be done, returns control flow immediately
        if self.ly < wy {
            return;
        }

        // calculates the row offset for the tile by using the current line
        // index and the DY (scroll Y) divided by 8 (as the tiles are 8x8 pixels),
        // on top of that ensures that the result is modulus 32 meaning that the
        // drawing wraps around the Y axis
        let row_offset = (((ld as usize + scy as usize) & 0xff) >> 3) % 32;

        // obtains the base address of the background map using the bg map flag
        // that control which background map is going to be used
        let mut map_offset: usize = if map { 0x1c00 } else { 0x1800 };

        // increments the map offset by the row offset multiplied by the number
        // of tiles in each row (32)
        map_offset += row_offset * 32;

        // calculates the sprite line offset by using the SCX register
        // shifted by 3 meaning that the tiles are 8x8
        let mut line_offset: usize = (scx >> 3) as usize;

        // calculates the index of the initial tile in drawing,
        // if the tile data set in use is #1, the indexes are
        // signed, then calculates a real tile offset
        let mut tile_index = self.vram[map_offset + line_offset] as usize;
        if !self.bg_tile && tile_index < 128 {
            tile_index += 256;
        }

        // calculates the offset that is going to be used in the update of the color buffer
        // which stores Game Boy colors from 0 to 3
        let mut color_offset = self.ly as usize * DISPLAY_WIDTH;

        // calculates the frame buffer offset position assuming the proper
        // Game Boy screen width and RGB pixel (3 bytes) size
        let mut frame_offset = self.ly as usize * DISPLAY_WIDTH * RGB_SIZE;

        // calculates both the current Y and X positions within the tiles
        // using the bitwise and operation as an effective modulus 8
        let y = (ld as usize + scy as usize) & 0x07;
        let mut x = (scx & 0x07) as usize;

        for index in 0..DISPLAY_WIDTH {
            // in case the current pixel to be drawn for the line
            // is visible within the window draws it an increments
            // the X coordinate of the tile
            if index as i16 >= wx as i16 - 7 {
                // obtains the current pixel data from the tile and
                // re-maps it according to the current palette
                let pixel = self.tiles[tile_index].get(x, y);
                let color = self.palette[pixel as usize];

                // updates the pixel in the color buffer, which stores
                // the raw pixel color information (unmapped)
                self.color_buffer[color_offset] = pixel;

                // set the color pixel in the frame buffer
                self.frame_buffer[frame_offset] = color[0];
                self.frame_buffer[frame_offset + 1] = color[1];
                self.frame_buffer[frame_offset + 2] = color[2];

                // increments the current tile X position in drawing
                x += 1;

                // in case the end of tile width has been reached then
                // a new tile must be retrieved for plotting
                if x == TILE_WIDTH {
                    // resets the tile X position to the base value
                    // as a new tile is going to be drawn
                    x = 0;

                    // calculates the new line tile offset making sure that
                    // the maximum of 32 is not overflown
                    line_offset = (line_offset + 1) % 32;

                    // calculates the tile index and makes sure the value
                    // takes into consideration the bg tile value
                    tile_index = self.vram[map_offset + line_offset] as usize;
                    if !self.bg_tile && tile_index < 128 {
                        tile_index += 256;
                    }
                }
            }

            // increments the color offset by one, representing
            // the drawing of one pixel
            color_offset += 1;

            // increments the offset of the frame buffer by the
            // size of an RGB pixel (which is 3 bytes)
            frame_offset += RGB_SIZE;
        }
    }

    fn render_objects(&mut self) {
        let mut draw_count = 0u8;

        // allocates the buffer that is going to be used to determine
        // drawing priority for overlapping pixels between different
        // objects, in MBR mode the object that has the smallest X
        // coordinate takes priority in drawing the pixel
        let mut index_buffer = [-256i16; DISPLAY_WIDTH];

        for index in 0..OBJ_COUNT {
            // in case the limit on the number of objects to be draw per
            // line has been reached breaks the loop avoiding more draws
            if draw_count == 10 {
                break;
            }

            // obtains the meta data of the object that is currently
            // under iteration to be checked for drawing
            let obj = self.obj_data[index];

            let obj_height = if self.obj_size {
                TILE_DOUBLE_HEIGHT
            } else {
                TILE_HEIGHT
            };

            // verifies if the sprite is currently located at the
            // current line that is going to be drawn and skips it
            // in case it's not
            let is_contained =
                (obj.y <= self.ly as i16) && ((obj.y + obj_height as i16) > self.ly as i16);
            if !is_contained {
                continue;
            }

            let palette = if obj.palette == 0 {
                self.palette_obj_0
            } else {
                self.palette_obj_1
            };

            // calculates the offset in the color buffer (raw color information
            // from 0 to 3) for the sprit that is going to be drawn, this value
            // is kept as a signed integer to allow proper negative number math
            let mut color_offset = self.ly as i32 * DISPLAY_WIDTH as i32 + obj.x as i32;

            // calculates the offset in the frame buffer for the sprite
            // that is going to be drawn, this is going to be the starting
            // point for the draw operation to be performed
            let mut frame_offset =
                (self.ly as i32 * DISPLAY_WIDTH as i32 + obj.x as i32) * RGB_SIZE as i32;

            // the relative title offset should range from 0 to 7 in 8x8
            // objects and from 0 to 15 in 8x16 objects
            let mut tile_offset = self.ly as i16 - obj.y;

            // in case we're flipping the object we must recompute the
            // tile offset as an inverted value using the object's height
            if obj.yflip {
                tile_offset = obj_height as i16 - tile_offset - 1;
            }

            // saves some space for the reference to the tile that
            // is going to be used in the current operation
            let tile: &Tile;

            // in case we're facing a 8x16 object then we must
            // differentiate between the handling of the top tile
            // and the bottom tile through bitwise manipulation
            // of the tile index
            if self.obj_size {
                if tile_offset < 8 {
                    tile = &self.tiles[obj.tile as usize & 0xfe];
                } else {
                    tile = &self.tiles[obj.tile as usize | 0x01];
                    tile_offset -= 8;
                }
            }
            // otherwise we're facing a 8x8 sprite and we should grab
            // the tile directly from the object's tile index
            else {
                tile = &self.tiles[obj.tile as usize];
            }

            let tile_row = tile.get_row(tile_offset as usize);

            for tile_x in 0..TILE_WIDTH {
                let x = obj.x + tile_x as i16;
                let is_contained = (x >= 0) && (x < DISPLAY_WIDTH as i16);
                if is_contained {
                    // the object is only considered visible if no background or
                    // window should be drawn over or if the underlying pixel
                    // is transparent (zero value) meaning there's no background
                    // or window for the provided pixel
                    let is_visible = !obj.bg_over || self.color_buffer[color_offset as usize] == 0;

                    // determines if the current pixel has priority over a possible
                    // one that has been drawn by a previous object, this happens
                    // in case the current object has a small X coordinate according
                    // to the MBR algorithm
                    let has_priority =
                        index_buffer[x as usize] == -256 || obj.x < index_buffer[x as usize];

                    let pixel = tile_row[if obj.xflip { 7 - tile_x } else { tile_x }];
                    if is_visible && has_priority && pixel != 0 {
                        // marks the current pixel in iteration as "owned"
                        // by the object with the defined X base position,
                        // to be used in priority calculus
                        index_buffer[x as usize] = obj.x;

                        // obtains the current pixel data from the tile row and
                        // re-maps it according to the object palette
                        let color = palette[pixel as usize];

                        // updates the pixel in the color buffer, which stores
                        // the raw pixel color information (unmapped)
                        self.color_buffer[color_offset as usize] = pixel;

                        // sets the color pixel in the frame buffer
                        self.frame_buffer[frame_offset as usize] = color[0];
                        self.frame_buffer[frame_offset as usize + 1] = color[1];
                        self.frame_buffer[frame_offset as usize + 2] = color[2];
                    }
                }

                // increment the color offset by one as this represents
                // the advance of one color pixel
                color_offset += 1;

                // increments the offset of the frame buffer by the
                // size of an RGB pixel (which is 3 bytes)
                frame_offset += RGB_SIZE as i32;
            }

            // increments the counter so that we're able to keep
            // track on the number of object drawn
            draw_count += 1;
        }
    }

    /// Runs an update operation on the LCD STAT interrupt meaning
    /// that the flag that control will be updated in case the conditions
    /// required for the LCD STAT interrupt to be triggered are met.
    fn update_stat(&mut self) {
        self.int_stat = self.stat_level();
    }

    /// Obtains the current level of the LCD STAT interrupt by
    /// checking the current PPU state in various sections.
    fn stat_level(&self) -> bool {
        self.stat_lyc && self.lyc == self.ly
            || self.stat_oam && self.mode == PpuMode::OamRead
            || self.stat_vblank && self.mode == PpuMode::VBlank
            || self.stat_hblank && self.mode == PpuMode::HBlank
    }

    /// Computes the values for all of the palettes, this method
    /// is useful to "flush" color computation whenever the base
    /// palette colors are changed.
    fn compute_palettes(&mut self) {
        // re-computes the complete set of palettes according to
        // the currently set palette colors (that may have changed)
        Self::compute_palette(&mut self.palette, &self.palette_colors, self.palettes[0]);
        Self::compute_palette(
            &mut self.palette_obj_0,
            &self.palette_colors,
            self.palettes[1],
        );
        Self::compute_palette(
            &mut self.palette_obj_1,
            &self.palette_colors,
            self.palettes[2],
        );

        // clears the frame buffer to allow the new background
        // color to be used
        self.clear_frame_buffer();
    }

    /// Static method used for the base logic of computation of RGB
    /// based palettes from the internal Game Boy color indexes.
    /// This method should be called whenever the palette indexes
    /// are changed.
    fn compute_palette(palette: &mut Palette, palette_colors: &Palette, value: u8) {
        for (index, palette_item) in palette.iter_mut().enumerate() {
            let color_index: usize = (value as usize >> (index * 2)) & 3;
            match color_index {
                0..=3 => *palette_item = palette_colors[color_index],
                color_index => panic!("Invalid palette color index {:04x}", color_index),
            }
        }
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Ppu;

    #[test]
    fn test_update_tile_simple() {
        let mut ppu = Ppu::new();
        ppu.vram[0x0000] = 0xff;
        ppu.vram[0x0001] = 0xff;

        let result = ppu.tiles()[0].get(0, 0);
        assert_eq!(result, 0);

        ppu.update_tile(0x8000, 0x00);
        let result = ppu.tiles()[0].get(0, 0);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_update_tile_upper() {
        let mut ppu = Ppu::new();
        ppu.vram[0x1000] = 0xff;
        ppu.vram[0x1001] = 0xff;

        let result = ppu.tiles()[256].get(0, 0);
        assert_eq!(result, 0);

        ppu.update_tile(0x9000, 0x00);
        let result = ppu.tiles()[256].get(0, 0);
        assert_eq!(result, 3);
    }
}
