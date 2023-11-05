from time import time
from pyboy import PyBoy

with PyBoy("../../res/roms/demo/pocket.gb", disable_renderer=False) as pyboy:
    pyboy.set_emulation_speed(0)
    print(pyboy.cartridge_title())
    start = time()
    for _ in range(6000):
        pyboy.tick()
    print(f"Time taken: {(time() - start)}")
    pyboy.screen_image().save("pocket_pyboy.png")
