# [ doc = "Timer 0." ]
# [ repr ( C ) ]
pub struct Timer {
    # [ doc = "0x00 - Start Timer." ]
    pub tasks_start: TasksStart,
    # [ doc = "0x04 - Stop Timer." ]
    pub tasks_stop: TasksStop,
    # [ doc = "0x08 - Increment Timer (In counter mode)." ]
    pub tasks_count: TasksCount,
    # [ doc = "0x0c - Clear timer." ]
    pub tasks_clear: TasksClear,
    # [ doc = "0x10 - Shutdown timer." ]
    pub tasks_shutdown: TasksShutdown,
    _reserved0: [u8; 44usize],
    # [ doc = "0x40 - Capture Timer value to CC[n] registers." ]
    pub tasks_capture0: TasksCapture,
    # [ doc = "0x44 - Capture Timer value to CC[n] registers." ]
    pub tasks_capture1: TasksCapture,
    # [ doc = "0x48 - Capture Timer value to CC[n] registers." ]
    pub tasks_capture2: TasksCapture,
    # [ doc = "0x4c - Capture Timer value to CC[n] registers." ]
    pub tasks_capture3: TasksCapture,
    _reserved1: [u8; 240usize],
    # [ doc = "0x140 - Compare event on CC[n] match." ]
    pub events_compare0: EventsCompare,
    # [ doc = "0x144 - Compare event on CC[n] match." ]
    pub events_compare1: EventsCompare,
    # [ doc = "0x148 - Compare event on CC[n] match." ]
    pub events_compare2: EventsCompare,
    # [ doc = "0x14c - Compare event on CC[n] match." ]
    pub events_compare3: EventsCompare,
    _reserved2: [u8; 176usize],
    # [ doc = "0x200 - Shortcuts for Timer." ]
    pub shorts: Shorts,
    _reserved3: [u8; 256usize],
    # [ doc = "0x304 - Interrupt enable set register." ]
    pub intenset: Intenset,
    # [ doc = "0x308 - Interrupt enable clear register." ]
    pub intenclr: Intenclr,
    _reserved4: [u8; 504usize],
    # [ doc = "0x504 - Timer Mode selection." ]
    pub mode: Mode,
    # [ doc = "0x508 - Sets timer behaviour." ]
    pub bitmode: Bitmode,
    _reserved5: [u8; 4usize],
    # [ doc = "0x510 - 4-bit prescaler to source clock frequency (max value 9). Source clock frequency is divided by 2^SCALE." ]
    pub prescaler: Prescaler,
    _reserved6: [u8; 44usize],
    # [ doc = "0x540 - Capture/compare registers." ]
    pub cc0: Cc,
    # [ doc = "0x544 - Capture/compare registers." ]
    pub cc1: Cc,
    # [ doc = "0x548 - Capture/compare registers." ]
    pub cc2: Cc,
    # [ doc = "0x54c - Capture/compare registers." ]
    pub cc3: Cc,
    _reserved7: [u8; 2732usize],
    # [ doc = "0xffc - Peripheral power control." ]
    pub power: Power,
}

# [ repr ( C ) ]
pub struct TasksStart {
    register: ::volatile_register::WO<u32>,
}

impl TasksStart {
    pub fn write(&mut self, value: u32) {
        self.register.write(value);
    }
}

# [ repr ( C ) ]
pub struct TasksStop {
    register: ::volatile_register::WO<u32>,
}

impl TasksStop {
    pub fn write(&mut self, value: u32) {
        self.register.write(value);
    }
}

# [ repr ( C ) ]
pub struct TasksCount {
    register: ::volatile_register::WO<u32>,
}

impl TasksCount {
    pub fn write(&mut self, value: u32) {
        self.register.write(value);
    }
}

# [ repr ( C ) ]
pub struct TasksClear {
    register: ::volatile_register::WO<u32>,
}

impl TasksClear {
    pub fn write(&mut self, value: u32) {
        self.register.write(value);
    }
}

# [ repr ( C ) ]
pub struct TasksShutdown {
    register: ::volatile_register::WO<u32>,
}

impl TasksShutdown {
    pub fn write(&mut self, value: u32) {
        self.register.write(value);
    }
}

# [ repr ( C ) ]
pub struct TasksCapture {
    register: ::volatile_register::WO<u32>,
}

impl TasksCapture {
    pub fn write(&mut self, value: u32) {
        self.register.write(value);
    }
}

# [ repr ( C ) ]
pub struct EventsCompare {
    register: ::volatile_register::RW<u32>,
}

impl EventsCompare {
    pub fn read(&self) -> u32 {
        self.register.read()
    }
    pub fn write(&mut self, value: u32) {
        self.register.write(value);
    }
}

# [ repr ( C ) ]
pub struct Shorts {
    register: ::volatile_register::RW<u32>,
}

impl Shorts {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ShortsR, &'w mut ShortsW) -> &'w mut ShortsW
    {
        let bits = self.register.read();
        let r = ShortsR { bits: bits };
        let mut w = ShortsW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ShortsR {
        ShortsR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ShortsW) -> &mut ShortsW
    {
        let mut w = ShortsW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ShortsR {
    bits: u32,
}

impl ShortsR {
    # [ doc = "Bit 0 - Shortcut between CC[0] event and the CLEAR task." ]
    pub fn compare0_clear(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Shortcut between CC[1] event and the CLEAR task." ]
    pub fn compare1_clear(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Shortcut between CC[2] event and the CLEAR task." ]
    pub fn compare2_clear(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Shortcut between CC[3] event and the CLEAR task." ]
    pub fn compare3_clear(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Shortcut between CC[0] event and the STOP task." ]
    pub fn compare0_stop(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Shortcut between CC[1] event and the STOP task." ]
    pub fn compare1_stop(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Shortcut between CC[2] event and the STOP task." ]
    pub fn compare2_stop(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Shortcut between CC[3] event and the STOP task." ]
    pub fn compare3_stop(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ShortsW {
    bits: u32,
}

impl ShortsW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ShortsW { bits: 0 }
    }
    # [ doc = "Bit 0 - Shortcut between CC[0] event and the CLEAR task." ]
    pub fn compare0_clear(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Shortcut between CC[1] event and the CLEAR task." ]
    pub fn compare1_clear(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Shortcut between CC[2] event and the CLEAR task." ]
    pub fn compare2_clear(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Shortcut between CC[3] event and the CLEAR task." ]
    pub fn compare3_clear(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Shortcut between CC[0] event and the STOP task." ]
    pub fn compare0_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Shortcut between CC[1] event and the STOP task." ]
    pub fn compare1_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Shortcut between CC[2] event and the STOP task." ]
    pub fn compare2_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Shortcut between CC[3] event and the STOP task." ]
    pub fn compare3_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Intenset {
    register: ::volatile_register::RW<u32>,
}

impl Intenset {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&IntensetR, &'w mut IntensetW)
                                -> &'w mut IntensetW
    {
        let bits = self.register.read();
        let r = IntensetR { bits: bits };
        let mut w = IntensetW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IntensetR {
        IntensetR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IntensetW) -> &mut IntensetW
    {
        let mut w = IntensetW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IntensetR {
    bits: u32,
}

impl IntensetR {
    # [ doc = "Bit 16 - Enable interrupt on COMPARE[0]" ]
    pub fn compare0(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Enable interrupt on COMPARE[1]" ]
    pub fn compare1(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Enable interrupt on COMPARE[2]" ]
    pub fn compare2(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Enable interrupt on COMPARE[3]" ]
    pub fn compare3(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IntensetW {
    bits: u32,
}

impl IntensetW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IntensetW { bits: 0 }
    }
    # [ doc = "Bit 16 - Enable interrupt on COMPARE[0]" ]
    pub fn compare0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Enable interrupt on COMPARE[1]" ]
    pub fn compare1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Enable interrupt on COMPARE[2]" ]
    pub fn compare2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Enable interrupt on COMPARE[3]" ]
    pub fn compare3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Intenclr {
    register: ::volatile_register::RW<u32>,
}

impl Intenclr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&IntenclrR, &'w mut IntenclrW)
                                -> &'w mut IntenclrW
    {
        let bits = self.register.read();
        let r = IntenclrR { bits: bits };
        let mut w = IntenclrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IntenclrR {
        IntenclrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IntenclrW) -> &mut IntenclrW
    {
        let mut w = IntenclrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IntenclrR {
    bits: u32,
}

impl IntenclrR {
    # [ doc = "Bit 16 - Disable interrupt on COMPARE[0]" ]
    pub fn compare0(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Disable interrupt on COMPARE[1]" ]
    pub fn compare1(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Disable interrupt on COMPARE[2]" ]
    pub fn compare2(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Disable interrupt on COMPARE[3]" ]
    pub fn compare3(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IntenclrW {
    bits: u32,
}

impl IntenclrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IntenclrW { bits: 0 }
    }
    # [ doc = "Bit 16 - Disable interrupt on COMPARE[0]" ]
    pub fn compare0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Disable interrupt on COMPARE[1]" ]
    pub fn compare1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Disable interrupt on COMPARE[2]" ]
    pub fn compare2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Disable interrupt on COMPARE[3]" ]
    pub fn compare3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Mode {
    register: ::volatile_register::RW<u32>,
}

impl Mode {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ModeR, &'w mut ModeW) -> &'w mut ModeW
    {
        let bits = self.register.read();
        let r = ModeR { bits: bits };
        let mut w = ModeW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ModeR {
        ModeR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ModeW) -> &mut ModeW
    {
        let mut w = ModeW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ModeR {
    bits: u32,
}

impl ModeR {
    # [ doc = "Bit 0 - Select Normal or Counter mode." ]
    pub fn mode(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ModeW {
    bits: u32,
}

impl ModeW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ModeW { bits: 0 }
    }
    # [ doc = "Bit 0 - Select Normal or Counter mode." ]
    pub fn mode(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Bitmode {
    register: ::volatile_register::RW<u32>,
}

impl Bitmode {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BitmodeR, &'w mut BitmodeW) -> &'w mut BitmodeW
    {
        let bits = self.register.read();
        let r = BitmodeR { bits: bits };
        let mut w = BitmodeW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BitmodeR {
        BitmodeR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BitmodeW) -> &mut BitmodeW
    {
        let mut w = BitmodeW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BitmodeR {
    bits: u32,
}

impl BitmodeR {
    # [ doc = "Bits 0:1 - Sets timer behaviour ro be like the implementation of a timer with width as indicated." ]
    pub fn bitmode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BitmodeW {
    bits: u32,
}

impl BitmodeW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BitmodeW { bits: 0 }
    }
    # [ doc = "Bits 0:1 - Sets timer behaviour ro be like the implementation of a timer with width as indicated." ]
    pub fn bitmode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Prescaler {
    register: ::volatile_register::RW<u32>,
}

impl Prescaler {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PrescalerR, &'w mut PrescalerW)
                                -> &'w mut PrescalerW
    {
        let bits = self.register.read();
        let r = PrescalerR { bits: bits };
        let mut w = PrescalerW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PrescalerR {
        PrescalerR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PrescalerW) -> &mut PrescalerW
    {
        let mut w = PrescalerW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PrescalerR {
    bits: u32,
}

impl PrescalerR {
    # [ doc = "Bits 0:3 - Timer PRESCALER value. Max value is 9." ]
    pub fn prescaler(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PrescalerW {
    bits: u32,
}

impl PrescalerW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PrescalerW { bits: 4 }
    }
    # [ doc = "Bits 0:3 - Timer PRESCALER value. Max value is 9." ]
    pub fn prescaler(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cc {
    register: ::volatile_register::RW<u32>,
}

impl Cc {
    pub fn read(&self) -> u32 {
        self.register.read()
    }
    pub fn write(&mut self, value: u32) {
        self.register.write(value);
    }
}

# [ repr ( C ) ]
pub struct Power {
    register: ::volatile_register::RW<u32>,
}

impl Power {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PowerR, &'w mut PowerW) -> &'w mut PowerW
    {
        let bits = self.register.read();
        let r = PowerR { bits: bits };
        let mut w = PowerW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PowerR {
        PowerR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PowerW) -> &mut PowerW
    {
        let mut w = PowerW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PowerR {
    bits: u32,
}

impl PowerR {
    # [ doc = "Bit 0 - Peripheral power control." ]
    pub fn power(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PowerW {
    bits: u32,
}

impl PowerW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PowerW { bits: 0 }
    }
    # [ doc = "Bit 0 - Peripheral power control." ]
    pub fn power(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}
