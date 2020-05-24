#![no_std]

/// Trait representing the interface to the hardware.
/// Intended to abstract the various buses (SPI, MPU 8/9/16/18-bit) from the
/// Controller code.
/// TODO Add support for 16/32-bit words
pub trait Interface {
    /// An enumeration of Interface errors
    type Error;

    fn command(&mut self, command: u8) -> Result<(), Self::Error>;
    fn send_parameters(&mut self, command: u8, data: &[u8]) -> Result<(), Self::Error>;
    /// Read parameters
    /// Note: the implementation needs to add a dummy read between command send and data receive
    fn read_parameters(&mut self, command: u8, data: &mut [u8]) -> Result<(), Self::Error>;
}

/// Controller implements the LCD command set and calls on the Interface trait
/// to communicate with the LCD panel.
#[derive(Copy, Clone)]
pub struct Controller<Iface>
where
    Iface: Interface,
{
    /// Custom interface
    iface: Iface,
}

impl<Iface: Interface> Controller<Iface>
where
    Iface: Interface,
{
    pub fn new(iface: Iface) -> Controller<Iface> {
        Controller { iface }
    }

    #[inline(always)]
    fn command(&mut self, command: u8) -> Result<(), Iface::Error> {
        self.iface.command(command)
    }
    #[inline(always)]
    fn send_parameters(&mut self, command: u8, data: &[u8]) -> Result<(), Iface::Error> {
        self.iface.send_parameters(command, data)
    }
    #[inline(always)]
    fn read_parameters(&mut self, command: u8, data: &mut [u8]) -> Result<(), Iface::Error> {
        self.iface.read_parameters(command, data)
    }

    // >> paste functions ################################################
    // << paste functions ################################################
}

// >> paste types ################################################
// << paste types ################################################
