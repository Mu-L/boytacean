use crate::warnln;

pub trait SerialDevice {
    fn send(&mut self) -> u8;
    fn receive(&mut self, byte: u8);

    /// Whether the serial device "driver" supports slave mode
    /// simulating an external clock source. Or if instead the
    /// clock should always be generated by the running device.
    fn allow_slave(&self) -> bool;

    /// Returns a short description of the serial device.
    fn description(&self) -> String;
}

pub struct Serial {
    data: u8,
    control: u8,
    shift_clock: bool,
    clock_speed: bool,
    transferring: bool,
    timer: i16,
    length: u16,
    bit_count: u8,
    byte_receive: u8,
    int_serial: bool,
    device: Box<dyn SerialDevice>,
}

impl Serial {
    pub fn new() -> Self {
        Self {
            data: 0x0,
            control: 0x0,
            shift_clock: false,
            clock_speed: false,
            transferring: false,
            timer: 0,
            length: 512,
            bit_count: 0,
            byte_receive: 0x0,
            int_serial: false,
            device: Box::<NullDevice>::default(),
        }
    }

    pub fn reset(&mut self) {
        self.data = 0x0;
        self.control = 0x0;
        self.shift_clock = false;
        self.clock_speed = false;
        self.transferring = false;
        self.timer = 0;
        self.length = 512;
        self.bit_count = 0;
        self.byte_receive = 0x0;
        self.int_serial = false;
    }

    pub fn clock(&mut self, cycles: u8) {
        if !self.transferring {
            return;
        }

        self.timer = self.timer.saturating_sub(cycles as i16);
        if self.timer <= 0 {
            let bit = (self.byte_receive >> (7 - self.bit_count)) & 0x01;
            self.data = (self.data << 1) | bit;

            self.tick_transfer();

            self.timer = self.length as i16;
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr & 0x00ff {
            0x01 => self.data,
            0x02 =>
            {
                #[allow(clippy::bool_to_int_with_if)]
                (if self.shift_clock { 0x01 } else { 0x00 }
                    | if self.clock_speed { 0x02 } else { 0x00 }
                    | if self.transferring { 0x80 } else { 0x00 })
            }
            _ => {
                warnln!("Reding from unknown Serial location 0x{:04x}", addr);
                0xff
            }
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr & 0x00ff {
            0x01 => self.data = value,
            0x02 => {
                self.shift_clock = value & 0x01 == 0x01;
                self.clock_speed = value & 0x02 == 0x02;
                self.transferring = value & 0x80 == 0x80;

                // in case the clock is meant to be set by the attached device
                // and the current Game Boy is meant to be running in slave mode
                // then checks if the attached device "driver" allows clock set
                // by external device and if not then ignores the transfer request
                // by immediately disabling the transferring flag
                if !self.shift_clock && !self.device.allow_slave() {
                    self.transferring = false;
                }

                // in case a transfer of byte has been requested and
                // this is the then we need to start the transfer setup
                if self.transferring {
                    // @TODO: if the GBC mode exists there should
                    // be special check logic here
                    //self.length = if self.gb.is_cgb() && self.clock_speed { 16 } else { 512 };
                    self.length = 512;
                    self.bit_count = 0;
                    self.timer = self.length as i16;

                    // executes the send and receive operation immediately
                    // this is considered an operational optimization with
                    // no real effect on the emulation (ex: no timing issues)
                    self.byte_receive = self.device.send();
                    self.device.receive(self.data);
                }
            }
            _ => warnln!("Writing to unknown Serial location 0x{:04x}", addr),
        }
    }

    pub fn send(&self) -> bool {
        if self.shift_clock {
            true
        } else {
            self.data & 0x80 == 0x80
        }
    }

    pub fn receive(&mut self, bit: bool) {
        if !self.shift_clock {
            self.data = (self.data << 1) | bit as u8;
            self.tick_transfer();
        }
    }

    #[inline(always)]
    pub fn int_serial(&self) -> bool {
        self.int_serial
    }

    #[inline(always)]
    pub fn set_int_serial(&mut self, value: bool) {
        self.int_serial = value;
    }

    #[inline(always)]
    pub fn ack_serial(&mut self) {
        self.set_int_serial(false);
    }

    pub fn device(&self) -> &dyn SerialDevice {
        self.device.as_ref()
    }

    pub fn set_device(&mut self, device: Box<dyn SerialDevice>) {
        self.device = device;
    }

    fn tick_transfer(&mut self) {
        self.bit_count += 1;
        if self.bit_count == 8 {
            self.transferring = false;
            self.length = 0;
            self.bit_count = 0;

            // signals the interrupt for the serial
            // transfer completion, indicating that
            // a new byte is ready to be read
            self.int_serial = true;
        }
    }
}

impl Default for Serial {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NullDevice {}

impl NullDevice {
    pub fn new() -> Self {
        Self {}
    }
}

impl SerialDevice for NullDevice {
    fn send(&mut self) -> u8 {
        0xff
    }

    fn receive(&mut self, _: u8) {}

    fn allow_slave(&self) -> bool {
        false
    }

    fn description(&self) -> String {
        String::from("Null")
    }
}

impl Default for NullDevice {
    fn default() -> Self {
        Self::new()
    }
}
