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
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
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

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum ShortsRCompare0Clear {
    # [ doc = "Shortcut disabled." ]
    Disabled,
    # [ doc = "Shortcut enabled." ]
    Enabled,
}
impl ShortsRCompare0Clear {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => ShortsRCompare0Clear::Disabled,
            1u32 => ShortsRCompare0Clear::Enabled,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum ShortsRCompare1Clear {
    # [ doc = "Shortcut disabled." ]
    Disabled,
    # [ doc = "Shortcut enabled." ]
    Enabled,
}
impl ShortsRCompare1Clear {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => ShortsRCompare1Clear::Disabled,
            1u32 => ShortsRCompare1Clear::Enabled,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum ShortsRCompare2Clear {
    # [ doc = "Shortcut disabled." ]
    Disabled,
    # [ doc = "Shortcut enabled." ]
    Enabled,
}
impl ShortsRCompare2Clear {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => ShortsRCompare2Clear::Disabled,
            1u32 => ShortsRCompare2Clear::Enabled,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum ShortsRCompare3Clear {
    # [ doc = "Shortcut disabled." ]
    Disabled,
    # [ doc = "Shortcut enabled." ]
    Enabled,
}
impl ShortsRCompare3Clear {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => ShortsRCompare3Clear::Disabled,
            1u32 => ShortsRCompare3Clear::Enabled,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum ShortsRCompare0Stop {
    # [ doc = "Shortcut disabled." ]
    Disabled,
    # [ doc = "Shortcut enabled." ]
    Enabled,
}
impl ShortsRCompare0Stop {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => ShortsRCompare0Stop::Disabled,
            1u32 => ShortsRCompare0Stop::Enabled,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum ShortsRCompare1Stop {
    # [ doc = "Shortcut disabled." ]
    Disabled,
    # [ doc = "Shortcut enabled." ]
    Enabled,
}
impl ShortsRCompare1Stop {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => ShortsRCompare1Stop::Disabled,
            1u32 => ShortsRCompare1Stop::Enabled,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum ShortsRCompare2Stop {
    # [ doc = "Shortcut disabled." ]
    Disabled,
    # [ doc = "Shortcut enabled." ]
    Enabled,
}
impl ShortsRCompare2Stop {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => ShortsRCompare2Stop::Disabled,
            1u32 => ShortsRCompare2Stop::Enabled,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum ShortsRCompare3Stop {
    # [ doc = "Shortcut disabled." ]
    Disabled,
    # [ doc = "Shortcut enabled." ]
    Enabled,
}
impl ShortsRCompare3Stop {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => ShortsRCompare3Stop::Disabled,
            1u32 => ShortsRCompare3Stop::Enabled,
            _ => unreachable!(),
        }
    }
}

impl ShortsR {
    # [ doc = "Bit 0 - Shortcut between CC[0] event and the CLEAR task." ]
    pub fn compare0_clear(&self) -> ShortsRCompare0Clear {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        ShortsRCompare0Clear::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 1 - Shortcut between CC[1] event and the CLEAR task." ]
    pub fn compare1_clear(&self) -> ShortsRCompare1Clear {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        ShortsRCompare1Clear::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 2 - Shortcut between CC[2] event and the CLEAR task." ]
    pub fn compare2_clear(&self) -> ShortsRCompare2Clear {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        ShortsRCompare2Clear::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 3 - Shortcut between CC[3] event and the CLEAR task." ]
    pub fn compare3_clear(&self) -> ShortsRCompare3Clear {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        ShortsRCompare3Clear::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 8 - Shortcut between CC[0] event and the STOP task." ]
    pub fn compare0_stop(&self) -> ShortsRCompare0Stop {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        ShortsRCompare0Stop::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 9 - Shortcut between CC[1] event and the STOP task." ]
    pub fn compare1_stop(&self) -> ShortsRCompare1Stop {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        ShortsRCompare1Stop::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 10 - Shortcut between CC[2] event and the STOP task." ]
    pub fn compare2_stop(&self) -> ShortsRCompare2Stop {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        ShortsRCompare2Stop::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 11 - Shortcut between CC[3] event and the STOP task." ]
    pub fn compare3_stop(&self) -> ShortsRCompare3Stop {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        ShortsRCompare3Stop::from((self.bits >> OFFSET) & MASK)
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ShortsW {
    bits: u32,
}

pub struct ShortsWCompare0Clear<'a> {
    register: &'a mut ShortsW,
}

impl<'a> ShortsWCompare0Clear<'a> {
    pub fn disabled(self) -> &'a mut ShortsW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn enabled(self) -> &'a mut ShortsW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct ShortsWCompare1Clear<'a> {
    register: &'a mut ShortsW,
}

impl<'a> ShortsWCompare1Clear<'a> {
    pub fn disabled(self) -> &'a mut ShortsW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn enabled(self) -> &'a mut ShortsW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct ShortsWCompare2Clear<'a> {
    register: &'a mut ShortsW,
}

impl<'a> ShortsWCompare2Clear<'a> {
    pub fn disabled(self) -> &'a mut ShortsW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn enabled(self) -> &'a mut ShortsW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct ShortsWCompare3Clear<'a> {
    register: &'a mut ShortsW,
}

impl<'a> ShortsWCompare3Clear<'a> {
    pub fn disabled(self) -> &'a mut ShortsW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn enabled(self) -> &'a mut ShortsW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct ShortsWCompare0Stop<'a> {
    register: &'a mut ShortsW,
}

impl<'a> ShortsWCompare0Stop<'a> {
    pub fn disabled(self) -> &'a mut ShortsW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn enabled(self) -> &'a mut ShortsW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct ShortsWCompare1Stop<'a> {
    register: &'a mut ShortsW,
}

impl<'a> ShortsWCompare1Stop<'a> {
    pub fn disabled(self) -> &'a mut ShortsW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn enabled(self) -> &'a mut ShortsW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct ShortsWCompare2Stop<'a> {
    register: &'a mut ShortsW,
}

impl<'a> ShortsWCompare2Stop<'a> {
    pub fn disabled(self) -> &'a mut ShortsW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn enabled(self) -> &'a mut ShortsW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct ShortsWCompare3Stop<'a> {
    register: &'a mut ShortsW,
}

impl<'a> ShortsWCompare3Stop<'a> {
    pub fn disabled(self) -> &'a mut ShortsW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn enabled(self) -> &'a mut ShortsW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

impl ShortsW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ShortsW { bits: 0 }
    }
    # [ doc = "Bit 0 - Shortcut between CC[0] event and the CLEAR task." ]
    pub fn compare0_clear(&mut self) -> ShortsWCompare0Clear {
        ShortsWCompare0Clear { register: self }
    }
    # [ doc = "Bit 1 - Shortcut between CC[1] event and the CLEAR task." ]
    pub fn compare1_clear(&mut self) -> ShortsWCompare1Clear {
        ShortsWCompare1Clear { register: self }
    }
    # [ doc = "Bit 2 - Shortcut between CC[2] event and the CLEAR task." ]
    pub fn compare2_clear(&mut self) -> ShortsWCompare2Clear {
        ShortsWCompare2Clear { register: self }
    }
    # [ doc = "Bit 3 - Shortcut between CC[3] event and the CLEAR task." ]
    pub fn compare3_clear(&mut self) -> ShortsWCompare3Clear {
        ShortsWCompare3Clear { register: self }
    }
    # [ doc = "Bit 8 - Shortcut between CC[0] event and the STOP task." ]
    pub fn compare0_stop(&mut self) -> ShortsWCompare0Stop {
        ShortsWCompare0Stop { register: self }
    }
    # [ doc = "Bit 9 - Shortcut between CC[1] event and the STOP task." ]
    pub fn compare1_stop(&mut self) -> ShortsWCompare1Stop {
        ShortsWCompare1Stop { register: self }
    }
    # [ doc = "Bit 10 - Shortcut between CC[2] event and the STOP task." ]
    pub fn compare2_stop(&mut self) -> ShortsWCompare2Stop {
        ShortsWCompare2Stop { register: self }
    }
    # [ doc = "Bit 11 - Shortcut between CC[3] event and the STOP task." ]
    pub fn compare3_stop(&mut self) -> ShortsWCompare3Stop {
        ShortsWCompare3Stop { register: self }
    }
}

# [ repr ( C ) ]
pub struct Intenset {
    register: ::volatile_register::RW<u32>,
}

impl Intenset {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
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

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum IntensetRCompare0 {
    # [ doc = "Interrupt disabled." ]
    Disabled,
    # [ doc = "Interrupt enabled." ]
    Enabled,
}
impl IntensetRCompare0 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => IntensetRCompare0::Disabled,
            1u32 => IntensetRCompare0::Enabled,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum IntensetRCompare1 {
    # [ doc = "Interrupt disabled." ]
    Disabled,
    # [ doc = "Interrupt enabled." ]
    Enabled,
}
impl IntensetRCompare1 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => IntensetRCompare1::Disabled,
            1u32 => IntensetRCompare1::Enabled,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum IntensetRCompare2 {
    # [ doc = "Interrupt disabled." ]
    Disabled,
    # [ doc = "Interrupt enabled." ]
    Enabled,
}
impl IntensetRCompare2 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => IntensetRCompare2::Disabled,
            1u32 => IntensetRCompare2::Enabled,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum IntensetRCompare3 {
    # [ doc = "Interrupt disabled." ]
    Disabled,
    # [ doc = "Interrupt enabled." ]
    Enabled,
}
impl IntensetRCompare3 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => IntensetRCompare3::Disabled,
            1u32 => IntensetRCompare3::Enabled,
            _ => unreachable!(),
        }
    }
}

impl IntensetR {
    # [ doc = "Bit 16 - Enable interrupt on COMPARE[0]" ]
    pub fn compare0(&self) -> IntensetRCompare0 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        IntensetRCompare0::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 17 - Enable interrupt on COMPARE[1]" ]
    pub fn compare1(&self) -> IntensetRCompare1 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        IntensetRCompare1::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 18 - Enable interrupt on COMPARE[2]" ]
    pub fn compare2(&self) -> IntensetRCompare2 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        IntensetRCompare2::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 19 - Enable interrupt on COMPARE[3]" ]
    pub fn compare3(&self) -> IntensetRCompare3 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        IntensetRCompare3::from((self.bits >> OFFSET) & MASK)
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IntensetW {
    bits: u32,
}

pub struct IntensetWCompare0<'a> {
    register: &'a mut IntensetW,
}

impl<'a> IntensetWCompare0<'a> {
    pub fn set(self) -> &'a mut IntensetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct IntensetWCompare1<'a> {
    register: &'a mut IntensetW,
}

impl<'a> IntensetWCompare1<'a> {
    pub fn set(self) -> &'a mut IntensetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct IntensetWCompare2<'a> {
    register: &'a mut IntensetW,
}

impl<'a> IntensetWCompare2<'a> {
    pub fn set(self) -> &'a mut IntensetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct IntensetWCompare3<'a> {
    register: &'a mut IntensetW,
}

impl<'a> IntensetWCompare3<'a> {
    pub fn set(self) -> &'a mut IntensetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

impl IntensetW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IntensetW { bits: 0 }
    }
    # [ doc = "Bit 16 - Enable interrupt on COMPARE[0]" ]
    pub fn compare0(&mut self) -> IntensetWCompare0 {
        IntensetWCompare0 { register: self }
    }
    # [ doc = "Bit 17 - Enable interrupt on COMPARE[1]" ]
    pub fn compare1(&mut self) -> IntensetWCompare1 {
        IntensetWCompare1 { register: self }
    }
    # [ doc = "Bit 18 - Enable interrupt on COMPARE[2]" ]
    pub fn compare2(&mut self) -> IntensetWCompare2 {
        IntensetWCompare2 { register: self }
    }
    # [ doc = "Bit 19 - Enable interrupt on COMPARE[3]" ]
    pub fn compare3(&mut self) -> IntensetWCompare3 {
        IntensetWCompare3 { register: self }
    }
}

# [ repr ( C ) ]
pub struct Intenclr {
    register: ::volatile_register::RW<u32>,
}

impl Intenclr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
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

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum IntenclrRCompare0 {
    # [ doc = "Interrupt disabled." ]
    Disabled,
    # [ doc = "Interrupt enabled." ]
    Enabled,
}
impl IntenclrRCompare0 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => IntenclrRCompare0::Disabled,
            1u32 => IntenclrRCompare0::Enabled,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum IntenclrRCompare1 {
    # [ doc = "Interrupt disabled." ]
    Disabled,
    # [ doc = "Interrupt enabled." ]
    Enabled,
}
impl IntenclrRCompare1 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => IntenclrRCompare1::Disabled,
            1u32 => IntenclrRCompare1::Enabled,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum IntenclrRCompare2 {
    # [ doc = "Interrupt disabled." ]
    Disabled,
    # [ doc = "Interrupt enabled." ]
    Enabled,
}
impl IntenclrRCompare2 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => IntenclrRCompare2::Disabled,
            1u32 => IntenclrRCompare2::Enabled,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum IntenclrRCompare3 {
    # [ doc = "Interrupt disabled." ]
    Disabled,
    # [ doc = "Interrupt enabled." ]
    Enabled,
}
impl IntenclrRCompare3 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => IntenclrRCompare3::Disabled,
            1u32 => IntenclrRCompare3::Enabled,
            _ => unreachable!(),
        }
    }
}

impl IntenclrR {
    # [ doc = "Bit 16 - Disable interrupt on COMPARE[0]" ]
    pub fn compare0(&self) -> IntenclrRCompare0 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        IntenclrRCompare0::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 17 - Disable interrupt on COMPARE[1]" ]
    pub fn compare1(&self) -> IntenclrRCompare1 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        IntenclrRCompare1::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 18 - Disable interrupt on COMPARE[2]" ]
    pub fn compare2(&self) -> IntenclrRCompare2 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        IntenclrRCompare2::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 19 - Disable interrupt on COMPARE[3]" ]
    pub fn compare3(&self) -> IntenclrRCompare3 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        IntenclrRCompare3::from((self.bits >> OFFSET) & MASK)
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IntenclrW {
    bits: u32,
}

pub struct IntenclrWCompare0<'a> {
    register: &'a mut IntenclrW,
}

impl<'a> IntenclrWCompare0<'a> {
    pub fn clear(self) -> &'a mut IntenclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct IntenclrWCompare1<'a> {
    register: &'a mut IntenclrW,
}

impl<'a> IntenclrWCompare1<'a> {
    pub fn clear(self) -> &'a mut IntenclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct IntenclrWCompare2<'a> {
    register: &'a mut IntenclrW,
}

impl<'a> IntenclrWCompare2<'a> {
    pub fn clear(self) -> &'a mut IntenclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct IntenclrWCompare3<'a> {
    register: &'a mut IntenclrW,
}

impl<'a> IntenclrWCompare3<'a> {
    pub fn clear(self) -> &'a mut IntenclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

impl IntenclrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IntenclrW { bits: 0 }
    }
    # [ doc = "Bit 16 - Disable interrupt on COMPARE[0]" ]
    pub fn compare0(&mut self) -> IntenclrWCompare0 {
        IntenclrWCompare0 { register: self }
    }
    # [ doc = "Bit 17 - Disable interrupt on COMPARE[1]" ]
    pub fn compare1(&mut self) -> IntenclrWCompare1 {
        IntenclrWCompare1 { register: self }
    }
    # [ doc = "Bit 18 - Disable interrupt on COMPARE[2]" ]
    pub fn compare2(&mut self) -> IntenclrWCompare2 {
        IntenclrWCompare2 { register: self }
    }
    # [ doc = "Bit 19 - Disable interrupt on COMPARE[3]" ]
    pub fn compare3(&mut self) -> IntenclrWCompare3 {
        IntenclrWCompare3 { register: self }
    }
}

# [ repr ( C ) ]
pub struct Mode {
    register: ::volatile_register::RW<u32>,
}

impl Mode {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
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

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum ModeRMode {
    # [ doc = "Timer in Normal mode." ]
    Timer,
    # [ doc = "Timer in Counter mode." ]
    Counter,
}
impl ModeRMode {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => ModeRMode::Timer,
            1u32 => ModeRMode::Counter,
            _ => unreachable!(),
        }
    }
}

impl ModeR {
    # [ doc = "Bit 0 - Select Normal or Counter mode." ]
    pub fn mode(&self) -> ModeRMode {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        ModeRMode::from((self.bits >> OFFSET) & MASK)
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ModeW {
    bits: u32,
}

pub struct ModeWMode<'a> {
    register: &'a mut ModeW,
}

impl<'a> ModeWMode<'a> {
    pub fn counter(self) -> &'a mut ModeW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
    pub fn timer(self) -> &'a mut ModeW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
}

impl ModeW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ModeW { bits: 0 }
    }
    # [ doc = "Bit 0 - Select Normal or Counter mode." ]
    pub fn mode(&mut self) -> ModeWMode {
        ModeWMode { register: self }
    }
}

# [ repr ( C ) ]
pub struct Bitmode {
    register: ::volatile_register::RW<u32>,
}

impl Bitmode {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
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

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum BitmodeRBitmode {
    # [ doc = "16-bit timer behaviour." ]
    _16Bit,
    # [ doc = "8-bit timer behaviour." ]
    _08Bit,
    # [ doc = "24-bit timer behaviour." ]
    _24Bit,
    # [ doc = "32-bit timer behaviour." ]
    _32Bit,
}
impl BitmodeRBitmode {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => BitmodeRBitmode::_16Bit,
            1u32 => BitmodeRBitmode::_08Bit,
            2u32 => BitmodeRBitmode::_24Bit,
            3u32 => BitmodeRBitmode::_32Bit,
            _ => unreachable!(),
        }
    }
}

impl BitmodeR {
    # [ doc = "Bits 0:1 - Sets timer behaviour ro be like the implementation of a timer with width as indicated." ]
    pub fn bitmode(&self) -> BitmodeRBitmode {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        BitmodeRBitmode::from((self.bits >> OFFSET) & MASK)
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BitmodeW {
    bits: u32,
}

pub struct BitmodeWBitmode<'a> {
    register: &'a mut BitmodeW,
}

impl<'a> BitmodeWBitmode<'a> {
    pub fn _16bit(self) -> &'a mut BitmodeW {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn _08bit(self) -> &'a mut BitmodeW {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
    pub fn _24bit(self) -> &'a mut BitmodeW {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 2u32 << OFFSET;
        self.register
    }
    pub fn _32bit(self) -> &'a mut BitmodeW {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 3u32 << OFFSET;
        self.register
    }
}

impl BitmodeW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BitmodeW { bits: 0 }
    }
    # [ doc = "Bits 0:1 - Sets timer behaviour ro be like the implementation of a timer with width as indicated." ]
    pub fn bitmode(&mut self) -> BitmodeWBitmode {
        BitmodeWBitmode { register: self }
    }
}

# [ repr ( C ) ]
pub struct Prescaler {
    register: ::volatile_register::RW<u32>,
}

impl Prescaler {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
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
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
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

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum PowerRPower {
    # [ doc = "Module power disabled." ]
    Disabled,
    # [ doc = "Module power enabled." ]
    Enabled,
}
impl PowerRPower {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => PowerRPower::Disabled,
            1u32 => PowerRPower::Enabled,
            _ => unreachable!(),
        }
    }
}

impl PowerR {
    # [ doc = "Bit 0 - Peripheral power control." ]
    pub fn power(&self) -> PowerRPower {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        PowerRPower::from((self.bits >> OFFSET) & MASK)
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PowerW {
    bits: u32,
}

pub struct PowerWPower<'a> {
    register: &'a mut PowerW,
}

impl<'a> PowerWPower<'a> {
    pub fn disabled(self) -> &'a mut PowerW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn enabled(self) -> &'a mut PowerW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

impl PowerW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PowerW { bits: 0 }
    }
    # [ doc = "Bit 0 - Peripheral power control." ]
    pub fn power(&mut self) -> PowerWPower {
        PowerWPower { register: self }
    }
}
