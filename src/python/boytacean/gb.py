from enum import Enum

from PIL.Image import Image, frombytes

from .palettes import PALETTES

from .boytacean import DISPLAY_WIDTH, DISPLAY_HEIGHT, CPU_FREQ, GameBoy as GameBoyRust


class GameBoyMode(Enum):
    DMG = 1
    CGB = 2
    SGB = 3


class GameBoy:
    def __init__(
        self,
        mode=GameBoyMode.DMG,
        ppu_enabled=True,
        apu_enabled=True,
        dma_enabled=True,
        timer_enabled=True,
        serial_enabled=True,
        load=True,
    ):
        super().__init__()
        self._system = GameBoyRust(mode.value)
        self._system.set_ppu_enabled(ppu_enabled)
        self._system.set_apu_enabled(apu_enabled)
        self._system.set_dma_enabled(dma_enabled)
        self._system.set_timer_enabled(timer_enabled)
        self._system.set_serial_enabled(serial_enabled)
        if load:
            self.load()

    def _repr_markdown_(self) -> str:
        return f"""
# Boytacean
This is a [Game Boy](https://en.wikipedia.org/wiki/Game_Boy) emulator built using the [Rust Programming Language](https://www.rust-lang.org) and is running inside this browser with the help of [WebAssembly](https://webassembly.org).

| Field   | Value               |
| ------- | ------------------- |
| Version | {self.version}      |
| Clock   | {self.clock_freq_s} |
"""

    def load(self):
        self._system.load()

    def load_rom(self, filename: str):
        self._system.load_rom_file(filename)

    def load_rom_data(self, data: bytes):
        self._system.load_rom(data)

    def clock(self) -> int:
        return self._system.clock()

    def clock_m(self, count: int) -> int:
        return self._system.clock_m(count)

    def clocks(self, count: int) -> int:
        return self._system.clocks(count)

    def next_frame(self) -> int:
        return self._system.next_frame()

    def frame_buffer(self):
        return self._system.frame_buffer()

    def image(self) -> Image:
        frame_buffer = self._system.frame_buffer()
        image = frombytes("RGB", (DISPLAY_WIDTH, DISPLAY_HEIGHT), frame_buffer, "raw")
        return image

    def save_image(self, filename: str, format: str = "PNG"):
        image = self.image()
        image.save(filename, format=format)

    def set_palette(self, name: str):
        if not name in PALETTES:
            raise ValueError(f"Unknown palette: {name}")
        palette = PALETTES[name]
        self.set_palette_colors(palette)

    def set_palette_colors(self, colors_hex: str):
        self._system.set_palette_colors(colors_hex)

    @property
    def ppu_enabled(self) -> bool:
        return self._system.ppu_enabled()

    def set_ppu_enabled(self, value: bool):
        self._system.set_ppu_enabled(value)

    @property
    def apu_enabled(self) -> bool:
        return self._system.apu_enabled()

    def set_apu_enabled(self, value: bool):
        self._system.set_apu_enabled(value)

    @property
    def dma_enabled(self) -> bool:
        return self._system.dma_enabled()

    def set_dma_enabled(self, value: bool):
        self._system.set_dma_enabled(value)

    @property
    def timer_enabled(self) -> bool:
        return self._system.timer_enabled()

    def set_timer_enabled(self, value: bool):
        self._system.set_timer_enabled(value)

    @property
    def serial_enabled(self) -> bool:
        return self._system.serial_enabled()

    def set_serial_enabled(self, value: bool):
        self._system.set_serial_enabled(value)

    @property
    def version(self) -> str:
        return self._system.version()

    @property
    def clock_freq_s(self) -> str:
        return self._system.clock_freq_s()
