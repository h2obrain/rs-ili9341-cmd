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

    /**
        This command is an empty command; it does not have any effect on the display module. However it can be used to terminate
        Frame Memory Write or Read as described in RAMWR (Memory Write) and RAMRD (Memory Read) Commands.
    */
    pub fn no_operation(&mut self) -> Result<(), Iface::Error> {
        self.command(0x00)
    }
    /**
        When the Software Reset command is written, it causes a software reset. It resets the commands and parameters to their
        S/W Reset default values. (See default tables in each command description.)
        Note: The Frame Memory contents are unaffected by this command
    */
    pub fn software_reset(&mut self) -> Result<(), Iface::Error> {
        self.command(0x01)
    }
    /**
        This read byte returns 24 bits display identification information.
        The 1st parameter is dummy data.
        The 2nd parameter (ID1 [7:0]): LCD module’s manufacturer ID.
        The 3rd parameter (ID2 [7:0]): LCD module/driver version ID.
        The 4th parameter (ID3 [7:0]): LCD module/driver ID.
    */
    pub fn read_display_identification_information(
        &mut self,
    ) -> Result<
        read_display_identification_information::DisplayIdentificationInformation,
        Iface::Error,
    > {
        let mut r =
            read_display_identification_information::DisplayIdentificationInformation::default();
        self.read_parameters(0x04, &mut r.data)?;
        Ok(r)
    }
    /// Read Display Status
    pub fn read_display_status(
        &mut self,
    ) -> Result<read_display_status::DisplayStatus, Iface::Error> {
        let mut r = read_display_status::DisplayStatus::default();
        self.read_parameters(0x09, &mut r.data)?;
        Ok(r)
    }
    /// Read Display Power Mode
    pub fn read_display_power_mode(
        &mut self,
    ) -> Result<read_display_power_mode::DisplayPowerMode, Iface::Error> {
        let mut r = read_display_power_mode::DisplayPowerMode::default();
        self.read_parameters(0x0A, &mut r.data)?;
        Ok(r)
    }
    /// Read Display MADCTL
    pub fn read_display_madctl(
        &mut self,
    ) -> Result<read_display_madctl::DisplayMadctl, Iface::Error> {
        let mut r = read_display_madctl::DisplayMadctl::default();
        self.read_parameters(0x0B, &mut r.data)?;
        Ok(r)
    }
    /// Read Display Pixel Format
    pub fn read_display_pixel_format(
        &mut self,
    ) -> Result<read_display_pixel_format::DisplayPixelFormat, Iface::Error> {
        let mut r = read_display_pixel_format::DisplayPixelFormat::default();
        self.read_parameters(0x0C, &mut r.data)?;
        Ok(r)
    }
    /// Read Display Image Format
    pub fn read_display_image_format(
        &mut self,
    ) -> Result<read_display_image_format::DisplayImageFormat, Iface::Error> {
        let mut r = read_display_image_format::DisplayImageFormat::default();
        self.read_parameters(0x0D, &mut r.data)?;
        Ok(r)
    }
    /// Read Display Signal Mode
    pub fn read_display_signal_mode(
        &mut self,
    ) -> Result<read_display_signal_mode::DisplaySignalMode, Iface::Error> {
        let mut r = read_display_signal_mode::DisplaySignalMode::default();
        self.read_parameters(0x0E, &mut r.data)?;
        Ok(r)
    }
    /// Read Display Self-Diagnostic Result
    pub fn read_display_self_diagnostic_result(
        &mut self,
    ) -> Result<read_display_self_diagnostic_result::DisplaySelfDiagnosticResult, Iface::Error>
    {
        let mut r = read_display_self_diagnostic_result::DisplaySelfDiagnosticResult::default();
        self.read_parameters(0x0F, &mut r.data)?;
        Ok(r)
    }
    /**
        This command causes the LCD module to enter the minimum power consumption mode. In this mode e.g. the DC/DC
        converter is stopped, Internal oscillator is stopped, and panel scanning is stopped.

        MCU interface and memory are still working and the memory keeps its contents.
    */
    pub fn enter_sleep_mode(&mut self) -> Result<(), Iface::Error> {
        self.command(0x10)
    }
    /**
        This command turns off sleep mode.
        In this mode e.g. the DC/DC converter is enabled, Internal oscillator is started, and panel scanning is started.
    */
    pub fn sleep_out(&mut self) -> Result<(), Iface::Error> {
        self.command(0x11)
    }
    /**
        This command turns on partial mode The partial mode window is described by the Partial Area command (30H). To leave
        Partial mode, the Normal Display Mode On command (13H) should be written.
    */
    pub fn partial_mode_on(&mut self) -> Result<(), Iface::Error> {
        self.command(0x12)
    }
    /**
        This command returns the display to normal mode.
        Normal display mode on means Partial mode off.
        Exit from NORON by the Partial mode On command (12h)
    */
    pub fn normal_display_mode_on(&mut self) -> Result<(), Iface::Error> {
        self.command(0x13)
    }
    /**
        This command is used to recover from display inversion mode.
        This command makes no change of the content of frame memory.
        This command doesn’t change any other status.
    */
    pub fn display_inversion_off(&mut self) -> Result<(), Iface::Error> {
        self.command(0x20)
    }
    /**
        This command is used to enter into display inversion mode.
        This command makes no change of the content of frame memory. Every bit is inverted from the frame memory to the display.
        This command doesn’t change any other status.
        To exit Display inversion mode, the Display inversion OFF command (20h) should be written.
    */
    pub fn display_inversion_on(&mut self) -> Result<(), Iface::Error> {
        self.command(0x21)
    }
    /**
        This command is used to select the desired Gamma curve for the current display. A maximum of 4 fixed gamma curves can
        be selected. The curve is selected by setting the appropriate bit in the parameter.
        Note: All other values are undefined.
    */
    pub fn gamma_set<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(gamma::GammaSetWrite) -> gamma::GammaSetWrite,
    {
        self.send_parameters(0x26, &gamma::GammaSet::default().write(f).data)
    }
    /**
        This command is used to enter into DISPLAY OFF mode. In this mode, the output from Frame Memory is disabled and blank
        page inserted.
        This command makes no change of contents of frame memory.
        This command does not change any other status.
        There will be no abnormal visible effect on the display.
    */
    pub fn display_off(&mut self) -> Result<(), Iface::Error> {
        self.command(0x28)
    }
    /**
        This command is used to recover from DISPLAY OFF mode. Output from the Frame Memory is enabled.
        This command makes no change of contents of frame memory.
        This command does not change any other status
    */
    pub fn display_on(&mut self) -> Result<(), Iface::Error> {
        self.command(0x29)
    }
    /**
        This command is used to define area of frame memory where MCU can access. This command makes no change on the
        other driver status. The values of SC [15:0] and EC [15:0] are referred when RAMWR command comes. Each value
        represents one column line in the Frame Memory.
    */
    pub fn column_address_set<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(column_address::ColumnAddressSetWrite) -> column_address::ColumnAddressSetWrite,
    {
        self.send_parameters(
            0x2A,
            &column_address::ColumnAddressSet::default().write(f).data,
        )
    }
    /**
        This command is used to define area of frame memory where MCU can access. This command makes no change on the
        other driver status. The values of SP [15:0] and EP [15:0] are referred when RAMWR command comes. Each value
        represents one Page line in the Frame Memory.
    */
    pub fn page_address_set<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(page_address::PageAddressSetWrite) -> page_address::PageAddressSetWrite,
    {
        self.send_parameters(0x2B, &page_address::PageAddressSet::default().write(f).data)
    }
    /**
        This command is used to transfer data from MCU to frame memory. This command makes no change to the other driver
        status. When this command is accepted, the column register and the page register are reset to the Start Column/Start
        Page positions. The Start Column/Start Page positions are different in accordance with MADCTL setting.) Then D [17:0] is
        stored in frame memory and the column register and the page register incremented. Sending any other command can stop
        frame Write.
    */
    pub fn memory_write(&mut self, d: &[u8]) -> Result<(), Iface::Error> {
        self.send_parameters(0x2C, d)
    }
    /**
        This command is used to define the LUT for 16-bit to 18-bit color depth conversion.
        128 bytes must be written to the LUT regardless of the color mode. Only the values in Section 7.4 are referred.
        This command has no effect on other commands, parameter and contents of frame memory. Visible change takes effect
        next time the frame memory is written to.
    */
    pub fn color_set<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(color::ColorSetWrite) -> color::ColorSetWrite,
    {
        self.send_parameters(0x2D, &color::ColorSet::default().write(f).data)
    }
    /**
        This command transfers image data from ILI9341’s frame memory to the host processor starting at the pixel location
        specified by preceding set_column_address and set_page_address commands.

        If Memory Access control B5 = 0:
        The column and page registers are reset to the Start Column (SC) and Start Page (SP), respectively. Pixels are read from
        frame memory at (SC, SP). The column register is then incremented and pixels read from the frame memory until the
        column register equals the End Column (EC) value. The column register is then reset to SC and the page register is
        incremented. Pixels are read from the frame memory until the page register equals the End Page (EP) value or the host
        processor sends another command.

        If Memory Access Control B5 = 1:
        The column and page registers are reset to the Start Column (SC) and Start Page (SP), respectively. Pixels are read from
        frame memory at (SC, SP). The page register is then incremented and pixels read from the frame memory until the page
        register equals the End Page (EP) value. The page register is then reset to SP and the column register is incremented.
        Pixels are read from the frame memory until the column register equals the End Column (EC) value or the host processor
        sends another command.
    */
    pub fn memory_read<'l>(&mut self, d: &'l mut [u8]) -> Result<&'l mut [u8], Iface::Error> {
        self.read_parameters(0x2E, d)?;
        Ok(d)
    }
    /**
        This command defines the partial mode’s display area. There are 2 parameters associated with this command, the first
        defines the Start Row (SR) and the second the End Row (ER), as illustrated in the figures below. SR and ER refer to the
        Frame Memory Line Pointer.
    */
    pub fn partial_area<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(partial_area::PartialAreaWrite) -> partial_area::PartialAreaWrite,
    {
        self.send_parameters(0x30, &partial_area::PartialArea::default().write(f).data)
    }
    /// Vertical Scrolling Definition
    pub fn vertical_scrolling_definition<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            vertical_scrolling::VerticalScrollingDefinitionWrite,
        ) -> vertical_scrolling::VerticalScrollingDefinitionWrite,
    {
        self.send_parameters(
            0x33,
            &vertical_scrolling::VerticalScrollingDefinition::default()
                .write(f)
                .data,
        )
    }
    /// Tearing Effect Line OFF
    pub fn tearing_effect_line_off(&mut self) -> Result<(), Iface::Error> {
        self.command(0x34)
    }
    /**
        This command is used to turn ON the Tearing Effect output signal from the TE signal line. This output is not affected by
        changing MADCTL bit B4. The Tearing Effect Line On has one parameter which describes the mode of the Tearing Effect
        Output Line.
    */
    pub fn tearing_effect_line_on<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            tearing_effect_line_on::TearingEffectLineOnWrite,
        ) -> tearing_effect_line_on::TearingEffectLineOnWrite,
    {
        self.send_parameters(
            0x35,
            &tearing_effect_line_on::TearingEffectLineOn::default()
                .write(f)
                .data,
        )
    }
    /**
        This command defines read/write scanning direction of frame memory.
        This command makes no change on the other driver status.
        Note: When BGR bit is changed, the new setting is active immediately without update the content in Frame Memory again.
    */
    pub fn memory_access_control<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            memory_access_control::MemoryAccessControlWrite,
        ) -> memory_access_control::MemoryAccessControlWrite,
    {
        self.send_parameters(
            0x36,
            &memory_access_control::MemoryAccessControl::default()
                .write(f)
                .data,
        )
    }
    /**
        This command is used together with Vertical Scrolling Definition (33h). These two commands describe the scrolling area
        and the scrolling mode. The Vertical Scrolling Start Address command has one parameter which describes the address of
        the line in the Frame Memory that will be written as the first line after the last line of the Top Fixed Area
        on the display.
    */
    pub fn vertical_scrolling_start_address<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            vertical_scrolling_start_address::VerticalScrollingStartAddressWrite,
        ) -> vertical_scrolling_start_address::VerticalScrollingStartAddressWrite,
    {
        self.send_parameters(
            0x37,
            &vertical_scrolling_start_address::VerticalScrollingStartAddress::default()
                .write(f)
                .data,
        )
    }
    /**
        This command is used to recover from Idle mode on.
        In the idle off mode, LCD can display maximum 262,144 colors.
    */
    pub fn idle_mode_off(&mut self) -> Result<(), Iface::Error> {
        self.command(0x38)
    }
    /**
        This command is used to enter into Idle mode on.
        In the idle on mode, color expression is reduced. The primary and the secondary colors using MSB of each R, G and B in
        the
        Frame Memory, 8 color depth data is displayed.
    */
    pub fn idle_mode_on(&mut self) -> Result<(), Iface::Error> {
        self.command(0x39)
    }
    /**
        This command sets the pixel format for the RGB image data used by the interface. DPI [2:0] is the pixel format select
        of RGB
        interface and DBI [2:0] is the pixel format of MCU interface. If a particular interface, either RGB interface or MCU
        interface, is
        not used then the corresponding bits in the parameter are ignored..

        If using RGB Interface must selection serial interface.
    */
    pub fn pixel_format_set<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(pixel_format::PixelFormatSetWrite) -> pixel_format::PixelFormatSetWrite,
    {
        self.send_parameters(0x3A, &pixel_format::PixelFormatSet::default().write(f).data)
    }
    /**
        This command transfers image data from the host processor to the display module’s frame memory continuing from the
        pixel location following the previous write_memory_continue or write_memory_start command.

        If set_address_mode B5 = 0:
        Data  is  written  continuing  from  the  pixel  location  after  the  write  range  of  the  previous
        write_memory_start  or
        write_memory_continue. The column register is then incremented and pixels are written to the frame memory until the
        column register equals the End Column (EC) value. The column register is then reset to SC and the page register is
        incremented. Pixels are written to the frame memory until the page register equals the End Page (EP) value and the
        column register equals the EC value, or the host processor sends another command. If the number of pixels exceeds (EC –
        SC + 1) * (EP – SP + 1) the extra pixels are ignored.

        If set_address_mode B5 = 1:
        Data  is  written  continuing  from  the  pixel  location  after  the  write  range  of  the  previous
        write_memory_start  or
        write_memory_continue. The page register is then incremented and pixels are written to the frame memory until the page
        register equals the End Page (EP) value. The page register is then reset to SP and the column register is incremented.
        Pixels are written to the frame memory until the column register equals the End column (EC) value and the page register
        equals the EP value, or the host processor sends another command. If the number of pixels exceeds (EC – SC + 1) * (EP –
        SP + 1) the extra pixels are ignored.

        Sending any other command can stop frame Write.

        Frame Memory Access and Interface setting (B3h), WEMODE=0
        When the transfer number of data exceeds (EC-SC+1)*(EP-SP+1), the exceeding data will be ignored.

        Frame Memory Access and Interface setting (B3h), WEMODE=1
        When the transfer number of data exceeds (EC-SC+1)*(EP-SP+1), the column and page number will be reset, and the
        exceeding data will be written into the following column and page.
    */
    pub fn write_memory_continue(&mut self, d: &[u8]) -> Result<(), Iface::Error> {
        self.send_parameters(0x3C, d)
    }
    /**
        This command transfers image data from the display module’s frame memory to the host processor continuing from the
        location following the previous read_memory_continue (3Eh) or read_memory_start (2Eh) command.

        If set_address_mode B5 = 0:
        Pixels are read continuing from the pixel location after the read range of the previous read_memory_start or
        read_memory_continue. The column register is then incremented and pixels are read from the frame memory until the
        column register equals the End Column (EC) value. The column register is then reset to SC and the page register is
        incremented. Pixels are read from the frame memory until the page register equals the End Page (EP) value and the
        column register equals the EC value, or the host processor sends another command.

        If set_address_mode B5 = 1:
        Pixels are read continuing from the pixel location after the read range of the previous read_memory_start or
        read_memory_continue. The page register is then incremented and pixels are read from the frame memory until the page
        register equals the End Page (EP) value. The page register is then reset to SP and the column register is incremented.
        Pixels are read from the frame memory until the column register equals the End Column (EC) value and the page register
        equals the EP value, or the host processor sends another command.

        This command makes no change to the other driver status.
    */
    pub fn read_memory_continue<'l>(
        &mut self,
        d: &'l mut [u8],
    ) -> Result<&'l mut [u8], Iface::Error> {
        self.read_parameters(0x3E, d)?;
        Ok(d)
    }
    /**
        This command turns on the display Tearing Effect output signal on the TE signal line when the display reaches line STS.
        The TE signal is not affected by changing set_address_mode bit B4. The Tearing Effect Line On has one parameter that
        describes the Tearing Effect Output Line mode.
        tvdl tvdh
        Vertical Time Scale

        Note that set_tear_scanline with STS=0 is equivalent to set_tear_on with M=0.
        The Tearing Effect Output line shall be active low when the display module is in Sleep mode.
    */
    pub fn set_tear_scanline<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(tear_scanline::SetTearScanlineWrite) -> tear_scanline::SetTearScanlineWrite,
    {
        self.send_parameters(
            0x44,
            &tear_scanline::SetTearScanline::default().write(f).data,
        )
    }
    /**
        The display returns the current scan line, GTS, used to update the display device. The total number of scan lines on a
        display device is defined as VSYNC + VBP + VACT + VFP. The first scan line is defined as the first line of V-Sync and is
        denoted as Line 0.
        When in Sleep Mode, the value returned by get_scanline is undefined.
    */
    pub fn get_scanline(&mut self) -> Result<get_scanline::GetScanline, Iface::Error> {
        let mut r = get_scanline::GetScanline::default();
        self.read_parameters(0x45, &mut r.data)?;
        Ok(r)
    }
    /**
        This command is used to adjust the brightness value of the display.
        It should be checked what is the relationship between this written value and output brightness of the display. This
        relationship
        is defined on the display module specification.
        In principle relationship is that 00h value means the lowest brightness and FFh value means the highest brightness.
    */
    pub fn write_display_brightness<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            write_display_brightness::DisplayBrightnessWrite,
        ) -> write_display_brightness::DisplayBrightnessWrite,
    {
        self.send_parameters(
            0x51,
            &write_display_brightness::DisplayBrightness::default()
                .write(f)
                .data,
        )
    }
    /**
        This command returns the brightness value of the display.

        It should be checked what the relationship between this returned value and output brightness of the display. This
        relationship is defined on the display module specification.
        In principle the relationship is that 00h value means the lowest brightness and FFh value means the highest brightness.
    */
    pub fn read_display_brightness(
        &mut self,
    ) -> Result<read_display_brightness::DisplayBrightness, Iface::Error> {
        let mut r = read_display_brightness::DisplayBrightness::default();
        self.read_parameters(0x52, &mut r.data)?;
        Ok(r)
    }
    /**
        This command is used to control display brightness.
        BCTRL: Brightness Control Block On/Off, This bit is always used to switch brightness for display.
        0 = Off (Brightness registers are 00h, DBV[7..0])
        1 = On (Brightness registers are active, according to the other parameters.)

        DD: Display Dimming, only for manual brightness setting
        DD = 0: Display Dimming is off
        DD = 1: Display Dimming is on

        BL: Backlight Control On/Off
        0 = Off (Completely turn off backlight circuit. Control lines must be low. )
        1 = On
        Dimming function is adapted to the brightness registers for display when bit BCTRL is changed at DD=1, e.g. BCTRL: 0
        (cid:4)
        1 or 1(cid:4) 0.

        When BL bit change from “On” to “Off”, backlight is turned off without gradual dimming, even if dimming-on (DD=1) are
        selected.
    */
    pub fn write_ctrl_display<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(write_ctrl_display::CtrlDisplayWrite) -> write_ctrl_display::CtrlDisplayWrite,
    {
        self.send_parameters(
            0x53,
            &write_ctrl_display::CtrlDisplay::default().write(f).data,
        )
    }
    /**
        This command is used to return brightness setting.

        BCTRL: Brightness Control Block On/Off,
        ‘0’ = Off (Brightness registers are 00h)
        ‘1’ = On (Brightness registers are active, according to the DBV[7..0] parameters.)

        DD: Display Dimming
        ‘0’ = Display Dimming is off
        ‘1’ = Display Dimming is on

        BL: Backlight On/Off
        ‘0’ = Off (Completely turn off backlight circuit. Control lines must be low. )
        ‘1’ = On
    */
    pub fn read_ctrl_display(&mut self) -> Result<read_ctrl_display::CtrlDisplay, Iface::Error> {
        let mut r = read_ctrl_display::CtrlDisplay::default();
        self.read_parameters(0x54, &mut r.data)?;
        Ok(r)
    }
    /**
        This command is used to set parameters for image content based adaptive brightness control functionality.
        There is possible to use 4 different modes for content adaptive image functionality, which are defined on a table
        below.
    */
    pub fn write_content_adaptive_brightness_control<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(write_content_adaptive_brightness_control::ContentAdaptiveBrightnessControlWrite) -> write_content_adaptive_brightness_control::ContentAdaptiveBrightnessControlWrite
    {
        self.send_parameters(
            0x55,
            &write_content_adaptive_brightness_control::ContentAdaptiveBrightnessControl::default()
                .write(f)
                .data,
        )
    }
    /**
        This command is used to read the settings for image content based adaptive brightness control functionality.
        It is possible to use 4 different modes for content adaptive image functionality, which are defined on a table below.
    */
    pub fn read_content_adaptive_brightness_control(
        &mut self,
    ) -> Result<
        read_content_adaptive_brightness_control::ContentAdaptiveBrightnessControl,
        Iface::Error,
    > {
        let mut r =
            read_content_adaptive_brightness_control::ContentAdaptiveBrightnessControl::default();
        self.read_parameters(0x56, &mut r.data)?;
        Ok(r)
    }
    /**
        This command is used to set the minimum brightness value of the display for CABC function.
        CMB[7:0]: CABC minimum brightness control, this parameter is used to avoid too much brightness reduction.
        When CABC is active, CABC cannot reduce the display brightness to less than CABC minimum brightness setting. Image
        processing function is worked as normal, even if the brightness cannot be changed.
        This function does not affect to the other function, manual brightness setting. Manual brightness can be set the display
        brightness to less than CABC minimum brightness. Smooth transition and dimming function can be worked as normal.
        When display brightness is turned off (BCTRL=0 of “Write CTRL Display (53h)”), CABC minimum brightness setting is
        ignored.
        In principle relationship is that 00h value means the lowest brightness for CABC and FFh value means the highest
        brightness for CABC.
    */
    pub fn write_cabc_minimum_brightness<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            write_cabc_minimum_brightness::CabcMinimumBrightnessWrite,
        ) -> write_cabc_minimum_brightness::CabcMinimumBrightnessWrite,
    {
        self.send_parameters(
            0x5E,
            &write_cabc_minimum_brightness::CabcMinimumBrightness::default()
                .write(f)
                .data,
        )
    }
    /**
        This command returns the minimum brightness value of CABC function.
        In principle the relationship is that 00h value means the lowest brightness and FFh value means the highest brightness.
        CMB[7:0] is CABC minimum brightness specified with “Write CABC minimum brightness (5Eh)” command. In principle
        relationship is that 00h value means the lowest brightness for CABC and FFh value means the highest brightness for
        CABC.
    */
    pub fn read_cabc_minimum_brightness(
        &mut self,
    ) -> Result<read_cabc_minimum_brightness::CabcMinimumBrightness, Iface::Error> {
        let mut r = read_cabc_minimum_brightness::CabcMinimumBrightness::default();
        self.read_parameters(0x5F, &mut r.data)?;
        Ok(r)
    }
    /**
        This read byte identifies the LCD module’s manufacturer ID and it is specified by User
        The 1st parameter is dummy data.
        The 2nd parameter is LCD module’s manufacturer ID.
    */
    pub fn read_id1(&mut self) -> Result<read_id1::Id1, Iface::Error> {
        let mut r = read_id1::Id1::default();
        self.read_parameters(0xDA, &mut r.data)?;
        Ok(r)
    }
    /**
        This read byte is used to track the LCD module/driver version. It is defined by display supplier (with User’s
        agreement) and
        changes each time a revision is made to the display, material or construction specifications.
        The 1st parameter is dummy data.
        The 2nd parameter is LCD module/driver version ID and the ID parameter range is from 80h to FFh.
        The ID2 can be programmed by MTP function.
    */
    pub fn read_id2(&mut self) -> Result<read_id2::Id2, Iface::Error> {
        let mut r = read_id2::Id2::default();
        self.read_parameters(0xDB, &mut r.data)?;
        Ok(r)
    }
    /**
        This read byte identifies the LCD module/driver and It is specified by User.
        The 1st parameter is dummy data.
        The 2nd parameter is LCD module/driver ID.
        The ID3 can be programmed by MTP function.
    */
    pub fn read_id3(&mut self) -> Result<read_id3::Id3, Iface::Error> {
        let mut r = read_id3::Id3::default();
        self.read_parameters(0xDC, &mut r.data)?;
        Ok(r)
    }
    /**
        Sets the operation status of the display interface. The setting becomes effective as soon as the command is received.
        EPL: DE polarity (“0”= High enable for RGB interface, “1”= Low enable for RGB interface)
        DPL: DOTCLK polarity set (“0”= data fetched at the rising time, “1”= data fetched at the falling time)
        HSPL: HSYNC polarity (“0”= Low level sync clock, “1”= High level sync clock)
        VSPL: VSYNC polarity (“0”= Low level sync clock, “1”= High level sync clock)
        RCM [1:0]: RGB interface selection (refer to the RGB interface section).

        ByPass_MODE: Select display data path whether Memory or Direct to Shift register when RGB Interface is used.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn rgb_interface_signal_control<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            rgb_interface_signal_control::RgbInterfaceSignalControlWrite,
        ) -> rgb_interface_signal_control::RgbInterfaceSignalControlWrite,
    {
        self.send_parameters(
            0xB0,
            &rgb_interface_signal_control::RgbInterfaceSignalControl::default()
                .write(f)
                .data,
        )
    }
    /**
        Formula to calculate frame frequency: FrameRate = fosc / (ClocksPerLine x DivisionRatio x (Lines + VBP + VFP))
        Sets the division ratio for internal clocks of Normal mode at MCU interface.
        fosc : internal oscillator frequency
        Clocks per line : RTNA setting
        Division ratio : DIVA setting
        Lines : total driving line number
        VBP : back porch line number
        VFP : front porch line number

        DIVA [1:0] : division ratio for internal clocks when Normal mode.

        RTNA [4:0] : RTNA[4:0] is used to set 1H (line) period of Normal mode at MCU interface.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn frame_control_in_normal_mode<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            frame_control_in_normal_mode::FrameControlInNormalModeWrite,
        ) -> frame_control_in_normal_mode::FrameControlInNormalModeWrite,
    {
        self.send_parameters(
            0xB1,
            &frame_control_in_normal_mode::FrameControlInNormalMode::default()
                .write(f)
                .data,
        )
    }
    /**
        Formula to calculate frame frequency: FrameRate = fosc / (ClocksPerLine x DivisionRatio x (Lines + VBP + VFP))
        Sets the division ratio for internal clocks of Idle mode at MCU interface.
        fosc : internal oscillator frequency
        Clocks per line : RTNB setting
        Division ratio : DIVB setting
        Lines : total driving line number
        VBP : back porch line number
        VFP : front porch line number

        DIVB [1:0]: division ratio for internal clocks when Idle mode.

        RTNB [4:0]: RTNB[4:0] is used to set 1H (line) period of Idle mode at MCU interface.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn frame_control_in_idle_mode<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            frame_control_in_idle_mode::FrameControlInIdleModeWrite,
        ) -> frame_control_in_idle_mode::FrameControlInIdleModeWrite,
    {
        self.send_parameters(
            0xB2,
            &frame_control_in_idle_mode::FrameControlInIdleMode::default()
                .write(f)
                .data,
        )
    }
    /**
        Formula to calculate frame frequency: FrameRate = fosc / (ClocksPerLine x DivisionRatio x (Lines + VBP + VFP))
        Sets the division ratio for internal clocks of Partial mode (Idle mode off) at MCU interface.
        fosc : internal oscillator frequency
        Clocks per line : RTNC setting
        Division ratio : DIVC setting
        Lines : total driving line number
        VBP : back porch line number
        VFP : front porch line number

        DIVC [1:0]: division ratio for internal clocks when Partial mode.

        RTNC [4:0]: RTNC [4:0] is used to set 1H (line) period of Partial mode at MCU interface.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn frame_control_in_partial_mode<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            frame_control_in_partial_mode::FrameControlInPartialModeWrite,
        ) -> frame_control_in_partial_mode::FrameControlInPartialModeWrite,
    {
        self.send_parameters(
            0xB3,
            &frame_control_in_partial_mode::FrameControlInPartialMode::default()
                .write(f)
                .data,
        )
    }
    /**
        Display inversion mode set
        NLA: Inversion setting in full colors normal mode (Normal mode on)
        NLB: Inversion setting in Idle mode (Idle mode on)
        NLC: Inversion setting in full colors partial mode (Partial mode on / Idle mode off)
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn display_inversion_control<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            display_inversion_control::DisplayInversionControlWrite,
        ) -> display_inversion_control::DisplayInversionControlWrite,
    {
        self.send_parameters(
            0xB4,
            &display_inversion_control::DisplayInversionControl::default()
                .write(f)
                .data,
        )
    }
    /**
        VFP [6:0] / VBP [6:0]: The VFP [6:0] and VBP [6:0] bits specify the line number of vertical front and back porch period
        respectively.
        Note: VFP + VBP ≦ 254 HSYNC signals

        HFP [4:0] / HBP [4:0]: The HFP [4:0] and HBP [4:0] bits specify the line number of horizontal front and back porch
        period
        respectively.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn blanking_porch_control<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            blanking_porch_control::BlankingPorchControlWrite,
        ) -> blanking_porch_control::BlankingPorchControlWrite,
    {
        self.send_parameters(
            0xB5,
            &blanking_porch_control::BlankingPorchControl::default()
                .write(f)
                .data,
        )
    }
    /**
        PTG [1:0]: Set the scan mode in non-display area.

        PT [1:0]: Determine source/VCOM output in a non-display area in the partial display mode.

        SS: Select the shift direction of outputs from the source driver.
        In addition to the shift direction, the settings for both SS and BGR bits are required to change the assignment of R, G,
        and B dots to the source driver pins.
        To assign R, G, B dots to the source driver pins from S1 to S720, set SS = 0.
        To assign R, G, B dots to the source driver pins from S720 to S1, set SS = 1.

        REV: Select whether the liquid crystal type is normally white type or normally black type.

        ISC [3:0]: Specify the scan cycle interval of gate driver in non-display area when PTG [1:0] =”10” to select interval
        scan.
        Then scan cycle is set as odd number from 0~29 frame periods. The polarity is inverted every scan cycle.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn display_function_control<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            display_function_control::DisplayFunctionControlWrite,
        ) -> display_function_control::DisplayFunctionControlWrite,
    {
        self.send_parameters(
            0xB6,
            &display_function_control::DisplayFunctionControl::default()
                .write(f)
                .data,
        )
    }
    /**
        DSTB: The ILI9341 driver enters the Deep Standby Mode when DSTB is set to high (“1”). In Deep Standby mode, both
        internal logic power and SRAM power are turn off, the display data stored in the Frame Memory and the instructions are
        not
        saved. Rewrite Frame Memory content and instructions after the Deep Standby Mode is exited.
        Note:  ILI9341 provides two ways to exit the Deep Standby Mode:
        (1) Exit Deep Standby Mode by pull down CSX to low (“0”) 6 times.
        (2) Input a RESX pulse with effective low level duration to start up the inside logic regulator and makes a transition
        to the initial state.

        1 2 Wait 1ms or more 3 4 5 6
        CSX
        WRX “High”
        RDX “High”
        D/CX “Low” or “High”
        D[17:0] Don CareDon Care Don CareDon Care Don CareDon Care

        GAS: Low voltage detection control.

        GON/DTE: Set the output level of gate driver G1 ~ G320 as follows
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn entry_mode_set<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(entry_mode::EntryModeSetWrite) -> entry_mode::EntryModeSetWrite,
    {
        self.send_parameters(0xB7, &entry_mode::EntryModeSet::default().write(f).data)
    }
    /**
        TH_UI [3:0]: These bits are used to set the percentage of grayscale data accumulate histogram value in the user
        interface
        (UI) mode. This ratio of maximum number of pixels that makes display image white (=data “255”) to the total of
        pixels by image processing.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn backlight_control1<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            backlight_control1::BacklightControl1Write,
        ) -> backlight_control1::BacklightControl1Write,
    {
        self.send_parameters(
            0xB8,
            &backlight_control1::BacklightControl1::default()
                .write(f)
                .data,
        )
    }
    /**
        TH_ST [3:0]: These bits are used to set the percentage of grayscale data accumulate histogram value in the still picture
        mode. This ratio of maximum number of pixels that makes display image white (=data “255”) to the total of pixels
        by image processing.

        TH_MV [3:0]: These bits are used to set the percentage of grayscale data accumulate histogram value in the moving image
        mode. This ratio of maximum number of pixels that makes display image white (=data “255”) to the total of pixels
        by image processing.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn backlight_control2<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            backlight_control2::BacklightControl2Write,
        ) -> backlight_control2::BacklightControl2Write,
    {
        self.send_parameters(
            0xB9,
            &backlight_control2::BacklightControl2::default()
                .write(f)
                .data,
        )
    }
    /**
        DTH_UI [3:0]: This parameter is used set the minimum limitation of grayscale threshold value in User Icon (UI) image
        mode.
        This register setting will limit the minimum Dth value to prevent the display image from being too white and
        the display quality is not acceptable.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn backlight_control3<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            backlight_control3::BacklightControl3Write,
        ) -> backlight_control3::BacklightControl3Write,
    {
        self.send_parameters(
            0xBA,
            &backlight_control3::BacklightControl3::default()
                .write(f)
                .data,
        )
    }
    /**
        DTH_ST [3:0]/DTH_MV [3:0]: This parameter is used set the minimum limitation of grayscale threshold value. This register
        setting will limit the minimum Dth value to prevent the display image from being too white and the
        display quality is not acceptable.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn backlight_control4<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            backlight_control4::BacklightControl4Write,
        ) -> backlight_control4::BacklightControl4Write,
    {
        self.send_parameters(
            0xBB,
            &backlight_control4::BacklightControl4::default()
                .write(f)
                .data,
        )
    }
    /**
        DIM1 [2:0]: This parameter is used to set the transition time of brightness level to avoid the sharp brightness
        transition on
        vision.

        DIM2 [3:0]: This parameter is used to set the threshold of brightness change.
        When the brightness transition difference is smaller than DIM2 [3:0], the brightness transition will be ignored.
        For example:
        If | brightness B – brightness A| < DIM2 [2:0], the brightness transition will be ignored and keep the brightness A.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn backlight_control5<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            backlight_control5::BacklightControl5Write,
        ) -> backlight_control5::BacklightControl5Write,
    {
        self.send_parameters(
            0xBC,
            &backlight_control5::BacklightControl5::default()
                .write(f)
                .data,
        )
    }
    /**
        PWM_DIV [7:0]: PWM_OUT output frequency control. This command is used to adjust the PWM waveform frequency of
        PWM_OUT. The PWM frequency can be calculated by using the following equation.

        16MHz
        f  =
        PWM_OUT   (PWM_DIV[7:0]+1)×255

        Note: The output frequency tolerance of internal frequency divider in CABC is ±10%
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn backlight_control7<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            backlight_control7::BacklightControl7Write,
        ) -> backlight_control7::BacklightControl7Write,
    {
        self.send_parameters(
            0xBE,
            &backlight_control7::BacklightControl7::default()
                .write(f)
                .data,
        )
    }
    /**
        LEDPWMPOL: The bit is used to define polarity of LEDPWM signal.

        LEDONPOL: This bit is used to control LEDON pin.

        LEDONR: This bit is used to control LEDON pin.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn backlight_control8<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            backlight_control8::BacklightControl8Write,
        ) -> backlight_control8::BacklightControl8Write,
    {
        self.send_parameters(
            0xBF,
            &backlight_control8::BacklightControl8::default()
                .write(f)
                .data,
        )
    }
    /**
        VRH [5:0]: Set the GVDD level, which is a reference level for the VCOM level and the grayscale voltage level.

        Note1: Make sure that VC and VRH setting restriction: GVDD ≦ (AVDD - 0.5) V.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn power_control1<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(power_control1::PowerControl1Write) -> power_control1::PowerControl1Write,
    {
        self.send_parameters(
            0xC0,
            &power_control1::PowerControl1::default().write(f).data,
        )
    }
    /**
        BT [2:0]: Sets the factor used in the step-up circuits.
        Select the optimal step-up factor for the operating voltage. To reduce power consumption, set a smaller factor.
        Note1: Make sure that AVDD setting restriction: AVDD ≦ 5.5 V.
        2: Make sure that VGH and VGL setting restriction: VGH -VGL≦ 32 V.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn power_control2<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(power_control2::PowerControl2Write) -> power_control2::PowerControl2Write,
    {
        self.send_parameters(
            0xC1,
            &power_control2::PowerControl2::default().write(f).data,
        )
    }
    /**
        VMH [6:0] : Set the VCOMH voltage.

        VML [6:0] : Set the VCOML voltage
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn vcom_control1<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(vcom_control1::VcomControl1Write) -> vcom_control1::VcomControl1Write,
    {
        self.send_parameters(0xC5, &vcom_control1::VcomControl1::default().write(f).data)
    }
    /**
        nVM: nVM equals to “0” after power on reset and VCOM offset equals to program MTP value. When nVM set to “1”, setting
        of VMF [6:0] becomes valid and VCOMH/VCOML can be adjusted.

        VMF [6:0]: Set the VCOM offset voltage.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn vcom_control2<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(vcom_control2::VcomControl2Write) -> vcom_control2::VcomControl2Write,
    {
        self.send_parameters(0xC7, &vcom_control2::VcomControl2::default().write(f).data)
    }
    /**
        This command is used to program the NV memory data. After a successful MTP operation, the information of PGM_DATA
        [7:0] will programmed to NV memory.
        PGM_ADR [2:0]: The select bits of ID1, ID2, ID3 and VMF [6:0] programming.

        PGM_DATA [7:0]: The programmed data.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn nv_memory_write<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(nv_memory_write::NvMemoryWrite) -> nv_memory_write::NvMemoryWrite,
    {
        self.send_parameters(0xD0, &nv_memory_write::NvMemory::default().write(f).data)
    }
    /**
        KEY [23:0]: NV memory programming protection key. When writing MTP data to D1h, this register must be set to
        0x55AA66h to enable MTP programming. If D1h register is not written with 0x55AA66h, then NV memory programming will
        be aborted.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn nv_memory_protection_key<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            nv_memory_protection_key::NvMemoryProtectionKeyWrite,
        ) -> nv_memory_protection_key::NvMemoryProtectionKeyWrite,
    {
        self.send_parameters(
            0xD1,
            &nv_memory_protection_key::NvMemoryProtectionKey::default()
                .write(f)
                .data,
        )
    }
    /**
        ID1_CNT [2:0] / ID2_CNT [2:0] / ID3_CNT [2:0] / VMF_CNT [2:0]: NV memory program record. The bits will increase “+1”
        automatically after writing the PGM_DATA [7:0] to NV memory.

        BUSY: The status bit of NV memory programming.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn nv_memory_status_read(
        &mut self,
    ) -> Result<nv_memory_status_read::NvMemoryStatus, Iface::Error> {
        let mut r = nv_memory_status_read::NvMemoryStatus::default();
        self.read_parameters(0xD2, &mut r.data)?;
        Ok(r)
    }
    /**
        Read IC device code.
        The 1st parameter is dummy read period.
        The 2nd parameter means the IC version.
        The 3rd and 4th parameter mean the IC model name.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn read_id4(&mut self) -> Result<read_id4::Id4, Iface::Error> {
        let mut r = read_id4::Id4::default();
        self.read_parameters(0xD3, &mut r.data)?;
        Ok(r)
    }
    /// Positive Gamma Correction
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn positive_gamma_correction<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            positive_gamma_correction::PositiveGammaCorrectionWrite,
        ) -> positive_gamma_correction::PositiveGammaCorrectionWrite,
    {
        self.send_parameters(
            0xE0,
            &positive_gamma_correction::PositiveGammaCorrection::default()
                .write(f)
                .data,
        )
    }
    /// Negative Gamma Correction
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn negative_gamma_correction<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            negative_gamma_correction::NegativeGammaCorrectionWrite,
        ) -> negative_gamma_correction::NegativeGammaCorrectionWrite,
    {
        self.send_parameters(
            0xE1,
            &negative_gamma_correction::NegativeGammaCorrection::default()
                .write(f)
                .data,
        )
    }
    /**
        RCAx [3:0]: Gamma Macro-adjustment registers for red gamma curve.
        BCAx [3:0]: Gamma Macro-adjustment registers for blue gamma curve.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn digital_gamma_control1<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            digital_gamma_control1::DigitalGammaControl1Write,
        ) -> digital_gamma_control1::DigitalGammaControl1Write,
    {
        self.send_parameters(
            0xE2,
            &digital_gamma_control1::DigitalGammaControl1::default()
                .write(f)
                .data,
        )
    }
    /**
        RFAx [3:0]: Gamma Micro-adjustment register for red gamma curve.
        BFAx [3:0]: Gamma Micro-adjustment register for blue gamma curve.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn digital_gamma_control2<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            digital_gamma_control2::DigitalGammaControl2Write,
        ) -> digital_gamma_control2::DigitalGammaControl2Write,
    {
        self.send_parameters(
            0xE3,
            &digital_gamma_control2::DigitalGammaControl2::default()
                .write(f)
                .data,
        )
    }
    /**
        MY_EOR / MX_EOR / MV_EOR / BGR_EOR:
        The set value of MADCTL is used in the IC is derived as exclusive OR between 1st Parameter of IFCTL and MADCTL
        Parameter.

        MDT [1:0]: Select the method of display data transferring.

        WEMODE: Memory write control
        WEMODE=0: When the transfer number of data exceeds (EC-SC+1)*(EP-SP+1), the exceeding data will be ignored.
        WEMODE=1: When the transfer number of data exceeds (EC-SC+1)*(EP-SP+1), the column and page number will be
        reset, and the exceeding data will be written into the following column and page.

        ENDIAN: Select Little Endian Interface bit. At Little Endian mode, the host sends LSB data first.
        Note: Little Endian is valid on only 65K 8-bit and 9-bit MCU interface mode.

        DM [1:0]: Select the display operation mode.
        The DM [1:0] setting allows switching between internal clock operation mode and external display interface operation
        mode.
        However, switching between the RGB interface operation mode and the VSYNC interface operation mode is prohibited.
    */
    #[cfg(feature = "Ili9341ExtendedCommandSet")]
    pub fn interface_control<F>(&mut self, f: F) -> Result<(), Iface::Error>
    where
        F: FnOnce(
            interface_control::InterfaceControlWrite,
        ) -> interface_control::InterfaceControlWrite,
    {
        self.send_parameters(
            0xF6,
            &interface_control::InterfaceControl::default().write(f).data,
        )
    }
}

macro_rules! enum_with_from {
    ($($name:ident ( $repr_type:ident ) => {
        $($entry:ident = $value:tt),+$(,)?
    }),+$(,)?) => {
        $(
            #[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
            #[repr($repr_type)]
            pub enum $name {
                $($entry = $value,)+
            }
            impl From<$repr_type> for $name {
                fn from(v: $repr_type) -> Self {
                    match v {
                        $($value => Self::$entry,)+
                        _ => panic!("Invalid input value {} for type $name", v)
                    }
                }
            }
        )+
    };
}
pub mod read_display_identification_information {
    #[derive(Copy, Clone, Debug)]
    pub struct DisplayIdentificationInformation {
        pub(super) data: [u8; 3],
    }
    impl DisplayIdentificationInformation {
        pub fn read(&self) -> DisplayIdentificationInformationRead {
            DisplayIdentificationInformationRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(
                DisplayIdentificationInformationWrite,
            ) -> DisplayIdentificationInformationWrite,
        {
            f(DisplayIdentificationInformationWrite { d: self }).d
        }
    }
    pub struct DisplayIdentificationInformationRead<'l> {
        d: &'l DisplayIdentificationInformation,
    }
    impl<'l> DisplayIdentificationInformationRead<'l> {
        /// lcd_modules_manufacturer_id
        #[inline(always)]
        pub fn lcd_modules_manufacturer_id(&self) -> u8 {
            self.d.data[0]
        }
        /// lcd_module_driver_version_id
        #[inline(always)]
        pub fn lcd_module_driver_version_id(&self) -> u8 {
            self.d.data[1]
        }
        /// lcd_module_driver_id
        #[inline(always)]
        pub fn lcd_module_driver_id(&self) -> u8 {
            self.d.data[2]
        }
    }
    pub struct DisplayIdentificationInformationWrite<'l> {
        d: &'l mut DisplayIdentificationInformation,
    }
    impl<'l> DisplayIdentificationInformationWrite<'l> {
        /// lcd_modules_manufacturer_id
        #[inline(always)]
        pub fn lcd_modules_manufacturer_id(self, w: u8) -> Self {
            self.d.data[0] = w;
            self
        }
        /// lcd_module_driver_version_id
        #[inline(always)]
        pub fn lcd_module_driver_version_id(self, w: u8) -> Self {
            self.d.data[1] = w;
            self
        }
        /// lcd_module_driver_id
        #[inline(always)]
        pub fn lcd_module_driver_id(self, w: u8) -> Self {
            self.d.data[2] = w;
            self
        }
    }
    impl Default for DisplayIdentificationInformation {
        fn default() -> Self {
            DisplayIdentificationInformation {
                data: [0x00, 0x00, 0x00],
            }
        }
    }
}
pub mod read_display_status {
    enum_with_from! {
        BoosterVoltageStatus(u8) => { BoosterOff = 0x00, BoosterOn = 0x01 },
        RowAddressOrder(u8) => { TopToBottomWhenMadctlB7Eq0 = 0x00, BottomToTopWhenMadctlB7Eq1 = 0x01 },
        ColumnAddressOrder(u8) => { LeftToRightWhenMadctlB6Eq0 = 0x00, RightToLeftWhenMadctlB6Eq1 = 0x01 },
        RowColumnExchange(u8) => { NormalModeWhenMadctlB5Eq0 = 0x00, ReverseModeWhenMadctlB5Eq1 = 0x01 },
        VerticalRefresh(u8) => { LcdRefreshTopToBottomWhenMadctlB4Eq0 = 0x00, LcdRefreshBottomToTopWhenMadctlB4Eq1 = 0x01 },
        RgbBgrOrder(u8) => { RgbWhenMadctlB3Eq0 = 0x00, BgrWhenMadctlB3Eq1 = 0x01 },
        HorizontalRefreshOrder(u8) => { LcdRefreshLeftToRightWhenMadctlB2Eq0 = 0x00, LcdRefreshRightToLeftWhenMadctlB2Eq1 = 0x01 },
        InterfaceColorPixelFormat(u8) => { N16BitPerPixel = 0x05, N18BitPerPixel = 0x06 },
        IdleMode(u8) => { IdleModeOff = 0x00, IdleModeOn = 0x01 },
        PartialMode(u8) => { PartialModeOff = 0x00, PartialModeOn = 0x01 },
        Sleep(u8) => { SleepInMode = 0x00, SleepOutMode = 0x01 },
        DisplayNormalMode(u8) => { DisplayNormalModeOff = 0x00, DisplayNormalModeOn = 0x01 },
        VerticalScrollingStatus(u8) => { ScrollOff = 0x00 },
        Display(u8) => { DisplayIsOff = 0x00, DisplayIsOn = 0x01 },
        TearingEffectLine(u8) => { TearingEffectLineOff = 0x00, TearingEffectOn = 0x01 },
        GammaCurveSelection(u8) => { Gc0 = 0x00 },
        TearingEffectLineMode(u8) => { Mode1VBlankingOnly = 0x00, Mode2BothHBlankingAndVBlanking = 0x01 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct DisplayStatus {
        pub(super) data: [u8; 4],
    }
    impl DisplayStatus {
        pub fn read(&self) -> DisplayStatusRead {
            DisplayStatusRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(DisplayStatusWrite) -> DisplayStatusWrite,
        {
            f(DisplayStatusWrite { d: self }).d
        }
    }
    pub struct DisplayStatusRead<'l> {
        d: &'l DisplayStatus,
    }
    impl<'l> DisplayStatusRead<'l> {
        /// booster_voltage_status
        #[inline(always)]
        pub fn booster_voltage_status(&self) -> BoosterVoltageStatus {
            BoosterVoltageStatus::from((self.d.data[0] >> 7) & 0x01)
        }
        /// row_address_order
        #[inline(always)]
        pub fn row_address_order(&self) -> RowAddressOrder {
            RowAddressOrder::from((self.d.data[0] >> 6) & 0x01)
        }
        /// column_address_order
        #[inline(always)]
        pub fn column_address_order(&self) -> ColumnAddressOrder {
            ColumnAddressOrder::from((self.d.data[0] >> 5) & 0x01)
        }
        /// row_column_exchange
        #[inline(always)]
        pub fn row_column_exchange(&self) -> RowColumnExchange {
            RowColumnExchange::from((self.d.data[0] >> 4) & 0x01)
        }
        /// vertical_refresh
        #[inline(always)]
        pub fn vertical_refresh(&self) -> VerticalRefresh {
            VerticalRefresh::from((self.d.data[0] >> 3) & 0x01)
        }
        /// rgb_bgr_order
        #[inline(always)]
        pub fn rgb_bgr_order(&self) -> RgbBgrOrder {
            RgbBgrOrder::from((self.d.data[0] >> 2) & 0x01)
        }
        /// horizontal_refresh_order
        #[inline(always)]
        pub fn horizontal_refresh_order(&self) -> HorizontalRefreshOrder {
            HorizontalRefreshOrder::from((self.d.data[0] >> 1) & 0x01)
        }
        /// interface_color_pixel_format
        #[inline(always)]
        pub fn interface_color_pixel_format(&self) -> InterfaceColorPixelFormat {
            InterfaceColorPixelFormat::from((self.d.data[1] >> 4) & 0x07)
        }
        /// idle_mode
        #[inline(always)]
        pub fn idle_mode(&self) -> IdleMode {
            IdleMode::from((self.d.data[1] >> 3) & 0x01)
        }
        /// partial_mode
        #[inline(always)]
        pub fn partial_mode(&self) -> PartialMode {
            PartialMode::from((self.d.data[1] >> 2) & 0x01)
        }
        /// sleep
        #[inline(always)]
        pub fn sleep(&self) -> Sleep {
            Sleep::from((self.d.data[1] >> 1) & 0x01)
        }
        /// display_normal_mode
        #[inline(always)]
        pub fn display_normal_mode(&self) -> DisplayNormalMode {
            DisplayNormalMode::from(self.d.data[1] & 0x01)
        }
        /// vertical_scrolling_status
        #[inline(always)]
        pub fn vertical_scrolling_status(&self) -> VerticalScrollingStatus {
            VerticalScrollingStatus::from((self.d.data[2] >> 7) & 0x01)
        }
        /// display
        #[inline(always)]
        pub fn display(&self) -> Display {
            Display::from((self.d.data[2] >> 2) & 0x01)
        }
        /// tearing_effect_line
        #[inline(always)]
        pub fn tearing_effect_line(&self) -> TearingEffectLine {
            TearingEffectLine::from((self.d.data[2] >> 1) & 0x01)
        }
        /// gamma_curve_selection
        #[inline(always)]
        pub fn gamma_curve_selection(&self) -> GammaCurveSelection {
            GammaCurveSelection::from((self.d.data[2] & 0x01) | ((self.d.data[3] >> 6) & 0x03))
        }
        /// tearing_effect_line_mode
        #[inline(always)]
        pub fn tearing_effect_line_mode(&self) -> TearingEffectLineMode {
            TearingEffectLineMode::from((self.d.data[3] >> 5) & 0x01)
        }
    }
    pub struct DisplayStatusWrite<'l> {
        d: &'l mut DisplayStatus,
    }
    impl<'l> DisplayStatusWrite<'l> {
        /// booster_voltage_status
        #[inline(always)]
        pub fn booster_voltage_status(self, w: BoosterVoltageStatus) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 7);
            self.d.data[0] |= (w & 0x01) << 7;
            self
        }
        /// row_address_order
        #[inline(always)]
        pub fn row_address_order(self, w: RowAddressOrder) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 6);
            self.d.data[0] |= (w & 0x01) << 6;
            self
        }
        /// column_address_order
        #[inline(always)]
        pub fn column_address_order(self, w: ColumnAddressOrder) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 5);
            self.d.data[0] |= (w & 0x01) << 5;
            self
        }
        /// row_column_exchange
        #[inline(always)]
        pub fn row_column_exchange(self, w: RowColumnExchange) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 4);
            self.d.data[0] |= (w & 0x01) << 4;
            self
        }
        /// vertical_refresh
        #[inline(always)]
        pub fn vertical_refresh(self, w: VerticalRefresh) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 3);
            self.d.data[0] |= (w & 0x01) << 3;
            self
        }
        /// rgb_bgr_order
        #[inline(always)]
        pub fn rgb_bgr_order(self, w: RgbBgrOrder) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 2);
            self.d.data[0] |= (w & 0x01) << 2;
            self
        }
        /// horizontal_refresh_order
        #[inline(always)]
        pub fn horizontal_refresh_order(self, w: HorizontalRefreshOrder) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 1);
            self.d.data[0] |= (w & 0x01) << 1;
            self
        }
        /// interface_color_pixel_format
        #[inline(always)]
        pub fn interface_color_pixel_format(self, w: InterfaceColorPixelFormat) -> Self {
            let w = w as u8;
            self.d.data[1] &= !(0x07 << 4);
            self.d.data[1] |= (w & 0x07) << 4;
            self
        }
        /// idle_mode
        #[inline(always)]
        pub fn idle_mode(self, w: IdleMode) -> Self {
            let w = w as u8;
            self.d.data[1] &= !(0x01 << 3);
            self.d.data[1] |= (w & 0x01) << 3;
            self
        }
        /// partial_mode
        #[inline(always)]
        pub fn partial_mode(self, w: PartialMode) -> Self {
            let w = w as u8;
            self.d.data[1] &= !(0x01 << 2);
            self.d.data[1] |= (w & 0x01) << 2;
            self
        }
        /// sleep
        #[inline(always)]
        pub fn sleep(self, w: Sleep) -> Self {
            let w = w as u8;
            self.d.data[1] &= !(0x01 << 1);
            self.d.data[1] |= (w & 0x01) << 1;
            self
        }
        /// display_normal_mode
        #[inline(always)]
        pub fn display_normal_mode(self, w: DisplayNormalMode) -> Self {
            let w = w as u8;
            self.d.data[1] &= !(0x01);
            self.d.data[1] |= w & 0x01;
            self
        }
        /// vertical_scrolling_status
        #[inline(always)]
        pub fn vertical_scrolling_status(self, w: VerticalScrollingStatus) -> Self {
            let w = w as u8;
            self.d.data[2] &= !(0x01 << 7);
            self.d.data[2] |= (w & 0x01) << 7;
            self
        }
        /// display
        #[inline(always)]
        pub fn display(self, w: Display) -> Self {
            let w = w as u8;
            self.d.data[2] &= !(0x01 << 2);
            self.d.data[2] |= (w & 0x01) << 2;
            self
        }
        /// tearing_effect_line
        #[inline(always)]
        pub fn tearing_effect_line(self, w: TearingEffectLine) -> Self {
            let w = w as u8;
            self.d.data[2] &= !(0x01 << 1);
            self.d.data[2] |= (w & 0x01) << 1;
            self
        }
        /// gamma_curve_selection
        #[inline(always)]
        pub fn gamma_curve_selection(self, w: GammaCurveSelection) -> Self {
            let w = w as u8;
            self.d.data[2] &= !(0x01);
            self.d.data[2] |= w & 0x01;
            self.d.data[3] &= !(0x03 << 6);
            self.d.data[3] |= (w & 0x03) << 6;
            self
        }
        /// tearing_effect_line_mode
        #[inline(always)]
        pub fn tearing_effect_line_mode(self, w: TearingEffectLineMode) -> Self {
            let w = w as u8;
            self.d.data[3] &= !(0x01 << 5);
            self.d.data[3] |= (w & 0x01) << 5;
            self
        }
    }
    impl Default for DisplayStatus {
        fn default() -> Self {
            DisplayStatus {
                data: [0x00, 0x61, 0x00, 0x00],
            }
        }
    }
}
pub mod read_display_power_mode {
    enum_with_from! {
        Booster(u8) => { BoosterOffOrHasAFault = 0x00, BoosterOnAndWorkingOk = 0x01 },
        IdleMode(u8) => { IdleModeOff = 0x00, IdleModeOn = 0x01 },
        PartialMode(u8) => { PartialModeOff = 0x00, PartialModeOn = 0x01 },
        Sleep(u8) => { SleepInMode = 0x00, SleepOutMode = 0x01 },
        DisplayNormalMode(u8) => { DisplayNormalModeOff = 0x00, DisplayNormalModeOn = 0x01 },
        DisplayIs(u8) => { DisplayIsOff = 0x00, DisplayIsOn = 0x01 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct DisplayPowerMode {
        pub(super) data: [u8; 1],
    }
    impl DisplayPowerMode {
        pub fn read(&self) -> DisplayPowerModeRead {
            DisplayPowerModeRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(DisplayPowerModeWrite) -> DisplayPowerModeWrite,
        {
            f(DisplayPowerModeWrite { d: self }).d
        }
    }
    pub struct DisplayPowerModeRead<'l> {
        d: &'l DisplayPowerMode,
    }
    impl<'l> DisplayPowerModeRead<'l> {
        /// booster
        #[inline(always)]
        pub fn booster(&self) -> Booster {
            Booster::from((self.d.data[0] >> 7) & 0x01)
        }
        /// idle_mode
        #[inline(always)]
        pub fn idle_mode(&self) -> IdleMode {
            IdleMode::from((self.d.data[0] >> 6) & 0x01)
        }
        /// partial_mode
        #[inline(always)]
        pub fn partial_mode(&self) -> PartialMode {
            PartialMode::from((self.d.data[0] >> 5) & 0x01)
        }
        /// sleep
        #[inline(always)]
        pub fn sleep(&self) -> Sleep {
            Sleep::from((self.d.data[0] >> 4) & 0x01)
        }
        /// display_normal_mode
        #[inline(always)]
        pub fn display_normal_mode(&self) -> DisplayNormalMode {
            DisplayNormalMode::from((self.d.data[0] >> 3) & 0x01)
        }
        /// display_is
        #[inline(always)]
        pub fn display_is(&self) -> DisplayIs {
            DisplayIs::from((self.d.data[0] >> 2) & 0x01)
        }
    }
    pub struct DisplayPowerModeWrite<'l> {
        d: &'l mut DisplayPowerMode,
    }
    impl<'l> DisplayPowerModeWrite<'l> {
        /// booster
        #[inline(always)]
        pub fn booster(self, w: Booster) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 7);
            self.d.data[0] |= (w & 0x01) << 7;
            self
        }
        /// idle_mode
        #[inline(always)]
        pub fn idle_mode(self, w: IdleMode) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 6);
            self.d.data[0] |= (w & 0x01) << 6;
            self
        }
        /// partial_mode
        #[inline(always)]
        pub fn partial_mode(self, w: PartialMode) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 5);
            self.d.data[0] |= (w & 0x01) << 5;
            self
        }
        /// sleep
        #[inline(always)]
        pub fn sleep(self, w: Sleep) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 4);
            self.d.data[0] |= (w & 0x01) << 4;
            self
        }
        /// display_normal_mode
        #[inline(always)]
        pub fn display_normal_mode(self, w: DisplayNormalMode) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 3);
            self.d.data[0] |= (w & 0x01) << 3;
            self
        }
        /// display_is
        #[inline(always)]
        pub fn display_is(self, w: DisplayIs) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 2);
            self.d.data[0] |= (w & 0x01) << 2;
            self
        }
    }
    impl Default for DisplayPowerMode {
        fn default() -> Self {
            DisplayPowerMode { data: [0x08] }
        }
    }
}
pub mod read_display_madctl {
    enum_with_from! {
        RowAddressOrder(u8) => { TopToBottomWhenMadctlB7Eq0 = 0x00, BottomToTopWhenMadctlB7Eq1 = 0x01 },
        ColumnAddressOrder(u8) => { LeftToRightWhenMadctlB6Eq0 = 0x00, RightToLeftWhenMadctlB6Eq1 = 0x01 },
        RowColumnExchange(u8) => { NormalModeWhenMadctlB5Eq0 = 0x00, ReverseModeWhenMadctlB5Eq1 = 0x01 },
        VerticalRefresh(u8) => { LcdRefreshTopToBottomWhenMadctlB4Eq0 = 0x00, LcdRefreshBottomToTopWhenMadctlB4Eq1 = 0x01 },
        RgbBgrOrder(u8) => { RgbWhenMadctlB3Eq0 = 0x00, BgrWhenMadctlB3Eq1 = 0x01 },
        HorizontalRefreshOrder(u8) => { LcdRefreshLeftToRightWhenMadctlB2Eq0 = 0x00, LcdRefreshRightToLeftWhenMadctlB2Eq1 = 0x01 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct DisplayMadctl {
        pub(super) data: [u8; 1],
    }
    impl DisplayMadctl {
        pub fn read(&self) -> DisplayMadctlRead {
            DisplayMadctlRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(DisplayMadctlWrite) -> DisplayMadctlWrite,
        {
            f(DisplayMadctlWrite { d: self }).d
        }
    }
    pub struct DisplayMadctlRead<'l> {
        d: &'l DisplayMadctl,
    }
    impl<'l> DisplayMadctlRead<'l> {
        /// row_address_order
        #[inline(always)]
        pub fn row_address_order(&self) -> RowAddressOrder {
            RowAddressOrder::from((self.d.data[0] >> 7) & 0x01)
        }
        /// column_address_order
        #[inline(always)]
        pub fn column_address_order(&self) -> ColumnAddressOrder {
            ColumnAddressOrder::from((self.d.data[0] >> 6) & 0x01)
        }
        /// row_column_exchange
        #[inline(always)]
        pub fn row_column_exchange(&self) -> RowColumnExchange {
            RowColumnExchange::from((self.d.data[0] >> 5) & 0x01)
        }
        /// vertical_refresh
        #[inline(always)]
        pub fn vertical_refresh(&self) -> VerticalRefresh {
            VerticalRefresh::from((self.d.data[0] >> 4) & 0x01)
        }
        /// rgb_bgr_order
        #[inline(always)]
        pub fn rgb_bgr_order(&self) -> RgbBgrOrder {
            RgbBgrOrder::from((self.d.data[0] >> 3) & 0x01)
        }
        /// horizontal_refresh_order
        #[inline(always)]
        pub fn horizontal_refresh_order(&self) -> HorizontalRefreshOrder {
            HorizontalRefreshOrder::from((self.d.data[0] >> 2) & 0x01)
        }
    }
    pub struct DisplayMadctlWrite<'l> {
        d: &'l mut DisplayMadctl,
    }
    impl<'l> DisplayMadctlWrite<'l> {
        /// row_address_order
        #[inline(always)]
        pub fn row_address_order(self, w: RowAddressOrder) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 7);
            self.d.data[0] |= (w & 0x01) << 7;
            self
        }
        /// column_address_order
        #[inline(always)]
        pub fn column_address_order(self, w: ColumnAddressOrder) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 6);
            self.d.data[0] |= (w & 0x01) << 6;
            self
        }
        /// row_column_exchange
        #[inline(always)]
        pub fn row_column_exchange(self, w: RowColumnExchange) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 5);
            self.d.data[0] |= (w & 0x01) << 5;
            self
        }
        /// vertical_refresh
        #[inline(always)]
        pub fn vertical_refresh(self, w: VerticalRefresh) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 4);
            self.d.data[0] |= (w & 0x01) << 4;
            self
        }
        /// rgb_bgr_order
        #[inline(always)]
        pub fn rgb_bgr_order(self, w: RgbBgrOrder) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 3);
            self.d.data[0] |= (w & 0x01) << 3;
            self
        }
        /// horizontal_refresh_order
        #[inline(always)]
        pub fn horizontal_refresh_order(self, w: HorizontalRefreshOrder) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 2);
            self.d.data[0] |= (w & 0x01) << 2;
            self
        }
    }
    impl Default for DisplayMadctl {
        fn default() -> Self {
            DisplayMadctl { data: [0x00] }
        }
    }
}
pub mod read_display_pixel_format {
    enum_with_from! {
        RgbInterfaceFormat(u8) => { N16Bits = 0x05, N18Bits = 0x06, N16Bits6Bit3TimesDataTransfer = 0x0D, N18Bits6Bit3TimesDataTransfer = 0x0E },
        McuInterfaceFormat(u8) => { N16Bits = 0x05, N18Bits = 0x06 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct DisplayPixelFormat {
        pub(super) data: [u8; 1],
    }
    impl DisplayPixelFormat {
        pub fn read(&self) -> DisplayPixelFormatRead {
            DisplayPixelFormatRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(DisplayPixelFormatWrite) -> DisplayPixelFormatWrite,
        {
            f(DisplayPixelFormatWrite { d: self }).d
        }
    }
    pub struct DisplayPixelFormatRead<'l> {
        d: &'l DisplayPixelFormat,
    }
    impl<'l> DisplayPixelFormatRead<'l> {
        /// rgb_interface_format
        #[inline(always)]
        pub fn rgb_interface_format(&self) -> RgbInterfaceFormat {
            RgbInterfaceFormat::from((self.d.data[0] >> 4) & 0x0F)
        }
        /// mcu_interface_format
        #[inline(always)]
        pub fn mcu_interface_format(&self) -> McuInterfaceFormat {
            McuInterfaceFormat::from(self.d.data[0] & 0x07)
        }
    }
    pub struct DisplayPixelFormatWrite<'l> {
        d: &'l mut DisplayPixelFormat,
    }
    impl<'l> DisplayPixelFormatWrite<'l> {
        /// rgb_interface_format
        #[inline(always)]
        pub fn rgb_interface_format(self, w: RgbInterfaceFormat) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x0F << 4);
            self.d.data[0] |= (w & 0x0F) << 4;
            self
        }
        /// mcu_interface_format
        #[inline(always)]
        pub fn mcu_interface_format(self, w: McuInterfaceFormat) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x07);
            self.d.data[0] |= w & 0x07;
            self
        }
    }
    impl Default for DisplayPixelFormat {
        fn default() -> Self {
            DisplayPixelFormat { data: [0x06] }
        }
    }
}
pub mod read_display_image_format {
    enum_with_from! {
        GammaCurveSelection(u8) => { GammaCurve1G2o2 = 0x00 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct DisplayImageFormat {
        pub(super) data: [u8; 1],
    }
    impl DisplayImageFormat {
        pub fn read(&self) -> DisplayImageFormatRead {
            DisplayImageFormatRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(DisplayImageFormatWrite) -> DisplayImageFormatWrite,
        {
            f(DisplayImageFormatWrite { d: self }).d
        }
    }
    pub struct DisplayImageFormatRead<'l> {
        d: &'l DisplayImageFormat,
    }
    impl<'l> DisplayImageFormatRead<'l> {
        /// gamma_curve_selection
        #[inline(always)]
        pub fn gamma_curve_selection(&self) -> GammaCurveSelection {
            GammaCurveSelection::from(self.d.data[0] & 0x07)
        }
    }
    pub struct DisplayImageFormatWrite<'l> {
        d: &'l mut DisplayImageFormat,
    }
    impl<'l> DisplayImageFormatWrite<'l> {
        /// gamma_curve_selection
        #[inline(always)]
        pub fn gamma_curve_selection(self, w: GammaCurveSelection) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x07);
            self.d.data[0] |= w & 0x07;
            self
        }
    }
    impl Default for DisplayImageFormat {
        fn default() -> Self {
            DisplayImageFormat { data: [0x00] }
        }
    }
}
pub mod read_display_signal_mode {
    enum_with_from! {
        TearingEffectLine(u8) => { TearingEffectLineOff = 0x00, TearingEffectLineOn = 0x01 },
        TearingEffectLineMode(u8) => { TearingEffectLineMode1 = 0x00, TearingEffectLineMode2 = 0x01 },
        HorizontalSync(u8) => { HorizontalSyncRgbInterfaceOff = 0x00, HorizontalSyncRgbInterfaceOn = 0x01 },
        VerticalSync(u8) => { VerticalSyncRgbInterfaceOff = 0x00, VerticalSyncRgbInterfaceOn = 0x01 },
        PixelClock(u8) => { PixelClockDotclkRgbInterfaceOff = 0x00, PixelClockDotclkRgbInterfaceOn = 0x01 },
        DataEnable(u8) => { DataEnableDeRgbInterfaceOff = 0x00, DataEnableDeRgbInterfaceOn = 0x01 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct DisplaySignalMode {
        pub(super) data: [u8; 1],
    }
    impl DisplaySignalMode {
        pub fn read(&self) -> DisplaySignalModeRead {
            DisplaySignalModeRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(DisplaySignalModeWrite) -> DisplaySignalModeWrite,
        {
            f(DisplaySignalModeWrite { d: self }).d
        }
    }
    pub struct DisplaySignalModeRead<'l> {
        d: &'l DisplaySignalMode,
    }
    impl<'l> DisplaySignalModeRead<'l> {
        /// tearing_effect_line
        #[inline(always)]
        pub fn tearing_effect_line(&self) -> TearingEffectLine {
            TearingEffectLine::from((self.d.data[0] >> 7) & 0x01)
        }
        /// tearing_effect_line_mode
        #[inline(always)]
        pub fn tearing_effect_line_mode(&self) -> TearingEffectLineMode {
            TearingEffectLineMode::from((self.d.data[0] >> 6) & 0x01)
        }
        /// horizontal_sync
        #[inline(always)]
        pub fn horizontal_sync(&self) -> HorizontalSync {
            HorizontalSync::from((self.d.data[0] >> 5) & 0x01)
        }
        /// vertical_sync
        #[inline(always)]
        pub fn vertical_sync(&self) -> VerticalSync {
            VerticalSync::from((self.d.data[0] >> 4) & 0x01)
        }
        /// pixel_clock
        #[inline(always)]
        pub fn pixel_clock(&self) -> PixelClock {
            PixelClock::from((self.d.data[0] >> 3) & 0x01)
        }
        /// data_enable
        #[inline(always)]
        pub fn data_enable(&self) -> DataEnable {
            DataEnable::from((self.d.data[0] >> 2) & 0x01)
        }
    }
    pub struct DisplaySignalModeWrite<'l> {
        d: &'l mut DisplaySignalMode,
    }
    impl<'l> DisplaySignalModeWrite<'l> {
        /// tearing_effect_line
        #[inline(always)]
        pub fn tearing_effect_line(self, w: TearingEffectLine) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 7);
            self.d.data[0] |= (w & 0x01) << 7;
            self
        }
        /// tearing_effect_line_mode
        #[inline(always)]
        pub fn tearing_effect_line_mode(self, w: TearingEffectLineMode) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 6);
            self.d.data[0] |= (w & 0x01) << 6;
            self
        }
        /// horizontal_sync
        #[inline(always)]
        pub fn horizontal_sync(self, w: HorizontalSync) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 5);
            self.d.data[0] |= (w & 0x01) << 5;
            self
        }
        /// vertical_sync
        #[inline(always)]
        pub fn vertical_sync(self, w: VerticalSync) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 4);
            self.d.data[0] |= (w & 0x01) << 4;
            self
        }
        /// pixel_clock
        #[inline(always)]
        pub fn pixel_clock(self, w: PixelClock) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 3);
            self.d.data[0] |= (w & 0x01) << 3;
            self
        }
        /// data_enable
        #[inline(always)]
        pub fn data_enable(self, w: DataEnable) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 2);
            self.d.data[0] |= (w & 0x01) << 2;
            self
        }
    }
    impl Default for DisplaySignalMode {
        fn default() -> Self {
            DisplaySignalMode { data: [0x00] }
        }
    }
}
pub mod read_display_self_diagnostic_result {
    #[derive(Copy, Clone, Debug)]
    pub struct DisplaySelfDiagnosticResult {
        pub(super) data: [u8; 1],
    }
    impl DisplaySelfDiagnosticResult {
        pub fn read(&self) -> DisplaySelfDiagnosticResultRead {
            DisplaySelfDiagnosticResultRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(DisplaySelfDiagnosticResultWrite) -> DisplaySelfDiagnosticResultWrite,
        {
            f(DisplaySelfDiagnosticResultWrite { d: self }).d
        }
    }
    pub struct DisplaySelfDiagnosticResultRead<'l> {
        d: &'l DisplaySelfDiagnosticResult,
    }
    impl<'l> DisplaySelfDiagnosticResultRead<'l> {
        /// d
        #[inline(always)]
        pub fn d(&self) -> u8 {
            ((self.d.data[0] >> 6) & 0x03) << 6
        }
    }
    pub struct DisplaySelfDiagnosticResultWrite<'l> {
        d: &'l mut DisplaySelfDiagnosticResult,
    }
    impl<'l> DisplaySelfDiagnosticResultWrite<'l> {
        /// d
        #[inline(always)]
        pub fn d(self, w: u8) -> Self {
            self.d.data[0] &= !(0x03 << 6);
            self.d.data[0] |= ((w >> 6) & 0x03) << 6;
            self
        }
    }
    impl Default for DisplaySelfDiagnosticResult {
        fn default() -> Self {
            DisplaySelfDiagnosticResult { data: [0x00] }
        }
    }
}
pub mod gamma {
    enum_with_from! {
        CurveSelected(u8) => { GammaCurve1G2o2 = 0x01 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct GammaSet {
        pub(super) data: [u8; 1],
    }
    impl GammaSet {
        pub fn read(&self) -> GammaSetRead {
            GammaSetRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(GammaSetWrite) -> GammaSetWrite,
        {
            f(GammaSetWrite { d: self }).d
        }
    }
    pub struct GammaSetRead<'l> {
        d: &'l GammaSet,
    }
    impl<'l> GammaSetRead<'l> {
        /// curve_selected
        #[inline(always)]
        pub fn curve_selected(&self) -> CurveSelected {
            CurveSelected::from(self.d.data[0])
        }
    }
    pub struct GammaSetWrite<'l> {
        d: &'l mut GammaSet,
    }
    impl<'l> GammaSetWrite<'l> {
        /// curve_selected
        #[inline(always)]
        pub fn curve_selected(self, w: CurveSelected) -> Self {
            let w = w as u8;
            self.d.data[0] = w;
            self
        }
    }
    impl Default for GammaSet {
        fn default() -> Self {
            GammaSet { data: [0x01] }
        }
    }
}
pub mod column_address {
    #[derive(Copy, Clone, Debug)]
    pub struct ColumnAddressSet {
        pub(super) data: [u8; 4],
    }
    impl ColumnAddressSet {
        pub fn read(&self) -> ColumnAddressSetRead {
            ColumnAddressSetRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(ColumnAddressSetWrite) -> ColumnAddressSetWrite,
        {
            f(ColumnAddressSetWrite { d: self }).d
        }
    }
    pub struct ColumnAddressSetRead<'l> {
        d: &'l ColumnAddressSet,
    }
    impl<'l> ColumnAddressSetRead<'l> {
        /// sc
        #[inline(always)]
        pub fn sc(&self) -> u16 {
            ((self.d.data[0] as u16) << 8) | (self.d.data[1] as u16)
        }
        /// ec
        #[inline(always)]
        pub fn ec(&self) -> u16 {
            ((self.d.data[2] as u16) << 8) | (self.d.data[3] as u16)
        }
    }
    pub struct ColumnAddressSetWrite<'l> {
        d: &'l mut ColumnAddressSet,
    }
    impl<'l> ColumnAddressSetWrite<'l> {
        /// sc
        #[inline(always)]
        pub fn sc(self, w: u16) -> Self {
            self.d.data[0] = (w >> 8) as u8;
            self.d.data[1] = w as u8;
            self
        }
        /// ec
        #[inline(always)]
        pub fn ec(self, w: u16) -> Self {
            self.d.data[2] = (w >> 8) as u8;
            self.d.data[3] = w as u8;
            self
        }
    }
    impl Default for ColumnAddressSet {
        fn default() -> Self {
            ColumnAddressSet {
                data: [0x00, 0x00, 0x00, 0xEF],
            }
        }
    }
}
pub mod page_address {
    #[derive(Copy, Clone, Debug)]
    pub struct PageAddressSet {
        pub(super) data: [u8; 4],
    }
    impl PageAddressSet {
        pub fn read(&self) -> PageAddressSetRead {
            PageAddressSetRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(PageAddressSetWrite) -> PageAddressSetWrite,
        {
            f(PageAddressSetWrite { d: self }).d
        }
    }
    pub struct PageAddressSetRead<'l> {
        d: &'l PageAddressSet,
    }
    impl<'l> PageAddressSetRead<'l> {
        /// sp
        #[inline(always)]
        pub fn sp(&self) -> u16 {
            ((self.d.data[0] as u16) << 8) | (self.d.data[1] as u16)
        }
        /// ep
        #[inline(always)]
        pub fn ep(&self) -> u16 {
            ((self.d.data[2] as u16) << 8) | (self.d.data[3] as u16)
        }
    }
    pub struct PageAddressSetWrite<'l> {
        d: &'l mut PageAddressSet,
    }
    impl<'l> PageAddressSetWrite<'l> {
        /// sp
        #[inline(always)]
        pub fn sp(self, w: u16) -> Self {
            self.d.data[0] = (w >> 8) as u8;
            self.d.data[1] = w as u8;
            self
        }
        /// ep
        #[inline(always)]
        pub fn ep(self, w: u16) -> Self {
            self.d.data[2] = (w >> 8) as u8;
            self.d.data[3] = w as u8;
            self
        }
    }
    impl Default for PageAddressSet {
        fn default() -> Self {
            PageAddressSet {
                data: [0x00, 0x00, 0x01, 0x3F],
            }
        }
    }
}
pub mod color {
    #[derive(Copy, Clone, Debug)]
    pub struct ColorSet {
        pub(super) data: [u8; 128],
    }
    impl ColorSet {
        pub fn read(&self) -> ColorSetRead {
            ColorSetRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(ColorSetWrite) -> ColorSetWrite,
        {
            f(ColorSetWrite { d: self }).d
        }
    }
    pub struct ColorSetRead<'l> {
        d: &'l ColorSet,
    }
    impl<'l> ColorSetRead<'l> {
        /// r
        #[inline(always)]
        pub fn r(&self) -> &'l [u8] {
            &self.d.data[0..32]
            // self.d.data[0..32].iter().map(|rr| rr & 0x3F).collect(somehow)
        }
        /// g
        #[inline(always)]
        pub fn g(&self) -> &'l [u8] {
            &self.d.data[32..96]
            // self.d.data[32..96].iter().map(|rr| rr & 0x3F).collect(somehow)
        }
        /// b
        #[inline(always)]
        pub fn b(&self) -> &'l [u8] {
            &self.d.data[96..128]
            // self.d.data[96..128].iter().map(|rr| rr & 0x3F).collect(somehow)
        }
    }
    pub struct ColorSetWrite<'l> {
        d: &'l mut ColorSet,
    }
    impl<'l> ColorSetWrite<'l> {
        /// r
        #[inline(always)]
        pub fn r(self, w: &'l [u8]) -> Self {
            self.d.data[0..32]
                .iter_mut()
                .zip(w.iter())
                .for_each(|(dd, ww)| *dd = *ww & 0x3F);
            self
        }
        /// g
        #[inline(always)]
        pub fn g(self, w: &'l [u8]) -> Self {
            self.d.data[32..96]
                .iter_mut()
                .zip(w.iter())
                .for_each(|(dd, ww)| *dd = *ww & 0x3F);
            self
        }
        /// b
        #[inline(always)]
        pub fn b(self, w: &'l [u8]) -> Self {
            self.d.data[96..128]
                .iter_mut()
                .zip(w.iter())
                .for_each(|(dd, ww)| *dd = *ww & 0x3F);
            self
        }
    }
    impl Default for ColorSet {
        fn default() -> Self {
            ColorSet {
                data: [
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
            }
        }
    }
}
pub mod partial_area {
    #[derive(Copy, Clone, Debug)]
    pub struct PartialArea {
        pub(super) data: [u8; 4],
    }
    impl PartialArea {
        pub fn read(&self) -> PartialAreaRead {
            PartialAreaRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(PartialAreaWrite) -> PartialAreaWrite,
        {
            f(PartialAreaWrite { d: self }).d
        }
    }
    pub struct PartialAreaRead<'l> {
        d: &'l PartialArea,
    }
    impl<'l> PartialAreaRead<'l> {
        /// sr
        #[inline(always)]
        pub fn sr(&self) -> u16 {
            ((self.d.data[0] as u16) << 8) | (self.d.data[1] as u16)
        }
        /// er
        #[inline(always)]
        pub fn er(&self) -> u16 {
            ((self.d.data[2] as u16) << 8) | (self.d.data[3] as u16)
        }
    }
    pub struct PartialAreaWrite<'l> {
        d: &'l mut PartialArea,
    }
    impl<'l> PartialAreaWrite<'l> {
        /// sr
        #[inline(always)]
        pub fn sr(self, w: u16) -> Self {
            self.d.data[0] = (w >> 8) as u8;
            self.d.data[1] = w as u8;
            self
        }
        /// er
        #[inline(always)]
        pub fn er(self, w: u16) -> Self {
            self.d.data[2] = (w >> 8) as u8;
            self.d.data[3] = w as u8;
            self
        }
    }
    impl Default for PartialArea {
        fn default() -> Self {
            PartialArea {
                data: [0x00, 0x00, 0x01, 0x3F],
            }
        }
    }
}
pub mod vertical_scrolling {
    #[derive(Copy, Clone, Debug)]
    pub struct VerticalScrollingDefinition {
        pub(super) data: [u8; 6],
    }
    impl VerticalScrollingDefinition {
        pub fn read(&self) -> VerticalScrollingDefinitionRead {
            VerticalScrollingDefinitionRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(VerticalScrollingDefinitionWrite) -> VerticalScrollingDefinitionWrite,
        {
            f(VerticalScrollingDefinitionWrite { d: self }).d
        }
    }
    pub struct VerticalScrollingDefinitionRead<'l> {
        d: &'l VerticalScrollingDefinition,
    }
    impl<'l> VerticalScrollingDefinitionRead<'l> {
        /// tfa
        #[inline(always)]
        pub fn tfa(&self) -> u16 {
            ((self.d.data[0] as u16) << 8) | (self.d.data[1] as u16)
        }
        /// vsa
        #[inline(always)]
        pub fn vsa(&self) -> u16 {
            ((self.d.data[2] as u16) << 8) | (self.d.data[3] as u16)
        }
        /// bfa
        #[inline(always)]
        pub fn bfa(&self) -> u16 {
            ((self.d.data[4] as u16) << 8) | (self.d.data[5] as u16)
        }
    }
    pub struct VerticalScrollingDefinitionWrite<'l> {
        d: &'l mut VerticalScrollingDefinition,
    }
    impl<'l> VerticalScrollingDefinitionWrite<'l> {
        /// tfa
        #[inline(always)]
        pub fn tfa(self, w: u16) -> Self {
            self.d.data[0] = (w >> 8) as u8;
            self.d.data[1] = w as u8;
            self
        }
        /// vsa
        #[inline(always)]
        pub fn vsa(self, w: u16) -> Self {
            self.d.data[2] = (w >> 8) as u8;
            self.d.data[3] = w as u8;
            self
        }
        /// bfa
        #[inline(always)]
        pub fn bfa(self, w: u16) -> Self {
            self.d.data[4] = (w >> 8) as u8;
            self.d.data[5] = w as u8;
            self
        }
    }
    impl Default for VerticalScrollingDefinition {
        fn default() -> Self {
            VerticalScrollingDefinition {
                data: [0x00, 0x00, 0x01, 0x40, 0x00, 0x00],
            }
        }
    }
}
pub mod tearing_effect_line_on {
    #[derive(Copy, Clone, Debug)]
    pub struct TearingEffectLineOn {
        pub(super) data: [u8; 1],
    }
    impl TearingEffectLineOn {
        pub fn read(&self) -> TearingEffectLineOnRead {
            TearingEffectLineOnRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(TearingEffectLineOnWrite) -> TearingEffectLineOnWrite,
        {
            f(TearingEffectLineOnWrite { d: self }).d
        }
    }
    pub struct TearingEffectLineOnRead<'l> {
        d: &'l TearingEffectLineOn,
    }
    impl<'l> TearingEffectLineOnRead<'l> {
        /// m
        #[inline(always)]
        pub fn m(&self) -> bool {
            (self.d.data[0] & 0x01) != 0
        }
    }
    pub struct TearingEffectLineOnWrite<'l> {
        d: &'l mut TearingEffectLineOn,
    }
    impl<'l> TearingEffectLineOnWrite<'l> {
        /// m
        #[inline(always)]
        pub fn m(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01);
            self.d.data[0] |= (w) as u8;
            self
        }
    }
    impl Default for TearingEffectLineOn {
        fn default() -> Self {
            TearingEffectLineOn { data: [0x00] }
        }
    }
}
pub mod memory_access_control {
    #[derive(Copy, Clone, Debug)]
    pub struct MemoryAccessControl {
        pub(super) data: [u8; 1],
    }
    impl MemoryAccessControl {
        pub fn read(&self) -> MemoryAccessControlRead {
            MemoryAccessControlRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(MemoryAccessControlWrite) -> MemoryAccessControlWrite,
        {
            f(MemoryAccessControlWrite { d: self }).d
        }
    }
    pub struct MemoryAccessControlRead<'l> {
        d: &'l MemoryAccessControl,
    }
    impl<'l> MemoryAccessControlRead<'l> {
        /// row_address_order
        #[inline(always)]
        pub fn row_address_order(&self) -> bool {
            ((self.d.data[0] >> 7) & 0x01) != 0
        }
        /// column_address_order
        #[inline(always)]
        pub fn column_address_order(&self) -> bool {
            ((self.d.data[0] >> 6) & 0x01) != 0
        }
        /// row_column_exchange
        #[inline(always)]
        pub fn row_column_exchange(&self) -> bool {
            ((self.d.data[0] >> 5) & 0x01) != 0
        }
        /// vertical_refresh_order
        #[inline(always)]
        pub fn vertical_refresh_order(&self) -> bool {
            ((self.d.data[0] >> 4) & 0x01) != 0
        }
        /// rgb_bgr_order
        #[inline(always)]
        pub fn rgb_bgr_order(&self) -> bool {
            ((self.d.data[0] >> 3) & 0x01) != 0
        }
        /// horizontal_refresh_order
        #[inline(always)]
        pub fn horizontal_refresh_order(&self) -> bool {
            ((self.d.data[0] >> 2) & 0x01) != 0
        }
    }
    pub struct MemoryAccessControlWrite<'l> {
        d: &'l mut MemoryAccessControl,
    }
    impl<'l> MemoryAccessControlWrite<'l> {
        /// row_address_order
        #[inline(always)]
        pub fn row_address_order(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 7);
            self.d.data[0] |= ((w) as u8) << 7;
            self
        }
        /// column_address_order
        #[inline(always)]
        pub fn column_address_order(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 6);
            self.d.data[0] |= ((w) as u8) << 6;
            self
        }
        /// row_column_exchange
        #[inline(always)]
        pub fn row_column_exchange(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 5);
            self.d.data[0] |= ((w) as u8) << 5;
            self
        }
        /// vertical_refresh_order
        #[inline(always)]
        pub fn vertical_refresh_order(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 4);
            self.d.data[0] |= ((w) as u8) << 4;
            self
        }
        /// rgb_bgr_order
        #[inline(always)]
        pub fn rgb_bgr_order(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 3);
            self.d.data[0] |= ((w) as u8) << 3;
            self
        }
        /// horizontal_refresh_order
        #[inline(always)]
        pub fn horizontal_refresh_order(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 2);
            self.d.data[0] |= ((w) as u8) << 2;
            self
        }
    }
    impl Default for MemoryAccessControl {
        fn default() -> Self {
            MemoryAccessControl { data: [0x00] }
        }
    }
}
pub mod vertical_scrolling_start_address {
    #[derive(Copy, Clone, Debug)]
    pub struct VerticalScrollingStartAddress {
        pub(super) data: [u8; 2],
    }
    impl VerticalScrollingStartAddress {
        pub fn read(&self) -> VerticalScrollingStartAddressRead {
            VerticalScrollingStartAddressRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(VerticalScrollingStartAddressWrite) -> VerticalScrollingStartAddressWrite,
        {
            f(VerticalScrollingStartAddressWrite { d: self }).d
        }
    }
    pub struct VerticalScrollingStartAddressRead<'l> {
        d: &'l VerticalScrollingStartAddress,
    }
    impl<'l> VerticalScrollingStartAddressRead<'l> {
        /// vsp
        #[inline(always)]
        pub fn vsp(&self) -> u16 {
            ((self.d.data[0] as u16) << 8) | (self.d.data[1] as u16)
        }
    }
    pub struct VerticalScrollingStartAddressWrite<'l> {
        d: &'l mut VerticalScrollingStartAddress,
    }
    impl<'l> VerticalScrollingStartAddressWrite<'l> {
        /// vsp
        #[inline(always)]
        pub fn vsp(self, w: u16) -> Self {
            self.d.data[0] = (w >> 8) as u8;
            self.d.data[1] = w as u8;
            self
        }
    }
    impl Default for VerticalScrollingStartAddress {
        fn default() -> Self {
            VerticalScrollingStartAddress { data: [0x00, 0x00] }
        }
    }
}
pub mod pixel_format {
    enum_with_from! {
        RgbInterfaceFormat(u8) => { N16Bits = 0x05, N18Bits = 0x06 },
        McuInterfaceFormat(u8) => { N16Bits = 0x05, N18Bits = 0x06 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct PixelFormatSet {
        pub(super) data: [u8; 1],
    }
    impl PixelFormatSet {
        pub fn read(&self) -> PixelFormatSetRead {
            PixelFormatSetRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(PixelFormatSetWrite) -> PixelFormatSetWrite,
        {
            f(PixelFormatSetWrite { d: self }).d
        }
    }
    pub struct PixelFormatSetRead<'l> {
        d: &'l PixelFormatSet,
    }
    impl<'l> PixelFormatSetRead<'l> {
        /// rgb_interface_format
        #[inline(always)]
        pub fn rgb_interface_format(&self) -> RgbInterfaceFormat {
            RgbInterfaceFormat::from((self.d.data[0] >> 4) & 0x07)
        }
        /// mcu_interface_format
        #[inline(always)]
        pub fn mcu_interface_format(&self) -> McuInterfaceFormat {
            McuInterfaceFormat::from(self.d.data[0] & 0x07)
        }
    }
    pub struct PixelFormatSetWrite<'l> {
        d: &'l mut PixelFormatSet,
    }
    impl<'l> PixelFormatSetWrite<'l> {
        /// rgb_interface_format
        #[inline(always)]
        pub fn rgb_interface_format(self, w: RgbInterfaceFormat) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x07 << 4);
            self.d.data[0] |= (w & 0x07) << 4;
            self
        }
        /// mcu_interface_format
        #[inline(always)]
        pub fn mcu_interface_format(self, w: McuInterfaceFormat) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x07);
            self.d.data[0] |= w & 0x07;
            self
        }
    }
    impl Default for PixelFormatSet {
        fn default() -> Self {
            PixelFormatSet { data: [0x66] }
        }
    }
}
pub mod tear_scanline {
    #[derive(Copy, Clone, Debug)]
    pub struct SetTearScanline {
        pub(super) data: [u8; 2],
    }
    impl SetTearScanline {
        pub fn read(&self) -> SetTearScanlineRead {
            SetTearScanlineRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(SetTearScanlineWrite) -> SetTearScanlineWrite,
        {
            f(SetTearScanlineWrite { d: self }).d
        }
    }
    pub struct SetTearScanlineRead<'l> {
        d: &'l SetTearScanline,
    }
    impl<'l> SetTearScanlineRead<'l> {
        /// sts
        #[inline(always)]
        pub fn sts(&self) -> u16 {
            (((self.d.data[0] & 0x01) as u16) << 8) | (self.d.data[1] as u16)
        }
    }
    pub struct SetTearScanlineWrite<'l> {
        d: &'l mut SetTearScanline,
    }
    impl<'l> SetTearScanlineWrite<'l> {
        /// sts
        #[inline(always)]
        pub fn sts(self, w: u16) -> Self {
            self.d.data[0] &= !(0x01);
            self.d.data[0] |= ((w >> 8) & 0x01) as u8;
            self.d.data[1] = w as u8;
            self
        }
    }
    impl Default for SetTearScanline {
        fn default() -> Self {
            SetTearScanline { data: [0x00, 0x00] }
        }
    }
}
pub mod get_scanline {
    #[derive(Copy, Clone, Debug)]
    pub struct GetScanline {
        pub(super) data: [u8; 2],
    }
    impl GetScanline {
        pub fn read(&self) -> GetScanlineRead {
            GetScanlineRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(GetScanlineWrite) -> GetScanlineWrite,
        {
            f(GetScanlineWrite { d: self }).d
        }
    }
    pub struct GetScanlineRead<'l> {
        d: &'l GetScanline,
    }
    impl<'l> GetScanlineRead<'l> {
        /// gts
        #[inline(always)]
        pub fn gts(&self) -> u16 {
            (((self.d.data[0] & 0x03) as u16) << 8) | (self.d.data[1] as u16)
        }
    }
    pub struct GetScanlineWrite<'l> {
        d: &'l mut GetScanline,
    }
    impl<'l> GetScanlineWrite<'l> {
        /// gts
        #[inline(always)]
        pub fn gts(self, w: u16) -> Self {
            self.d.data[0] &= !(0x03);
            self.d.data[0] |= ((w >> 8) & 0x03) as u8;
            self.d.data[1] = w as u8;
            self
        }
    }
    impl Default for GetScanline {
        fn default() -> Self {
            GetScanline { data: [0x00, 0x00] }
        }
    }
}
pub mod write_display_brightness {
    #[derive(Copy, Clone, Debug)]
    pub struct DisplayBrightness {
        pub(super) data: [u8; 1],
    }
    impl DisplayBrightness {
        pub fn read(&self) -> DisplayBrightnessRead {
            DisplayBrightnessRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(DisplayBrightnessWrite) -> DisplayBrightnessWrite,
        {
            f(DisplayBrightnessWrite { d: self }).d
        }
    }
    pub struct DisplayBrightnessRead<'l> {
        d: &'l DisplayBrightness,
    }
    impl<'l> DisplayBrightnessRead<'l> {
        /// dbv
        #[inline(always)]
        pub fn dbv(&self) -> u8 {
            self.d.data[0]
        }
    }
    pub struct DisplayBrightnessWrite<'l> {
        d: &'l mut DisplayBrightness,
    }
    impl<'l> DisplayBrightnessWrite<'l> {
        /// dbv
        #[inline(always)]
        pub fn dbv(self, w: u8) -> Self {
            self.d.data[0] = w;
            self
        }
    }
    impl Default for DisplayBrightness {
        fn default() -> Self {
            DisplayBrightness { data: [0x00] }
        }
    }
}
pub mod read_display_brightness {
    #[derive(Copy, Clone, Debug)]
    pub struct DisplayBrightness {
        pub(super) data: [u8; 1],
    }
    impl DisplayBrightness {
        pub fn read(&self) -> DisplayBrightnessRead {
            DisplayBrightnessRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(DisplayBrightnessWrite) -> DisplayBrightnessWrite,
        {
            f(DisplayBrightnessWrite { d: self }).d
        }
    }
    pub struct DisplayBrightnessRead<'l> {
        d: &'l DisplayBrightness,
    }
    impl<'l> DisplayBrightnessRead<'l> {
        /// dbv
        #[inline(always)]
        pub fn dbv(&self) -> u8 {
            self.d.data[0]
        }
    }
    pub struct DisplayBrightnessWrite<'l> {
        d: &'l mut DisplayBrightness,
    }
    impl<'l> DisplayBrightnessWrite<'l> {
        /// dbv
        #[inline(always)]
        pub fn dbv(self, w: u8) -> Self {
            self.d.data[0] = w;
            self
        }
    }
    impl Default for DisplayBrightness {
        fn default() -> Self {
            DisplayBrightness { data: [0x00] }
        }
    }
}
pub mod write_ctrl_display {
    #[derive(Copy, Clone, Debug)]
    pub struct CtrlDisplay {
        pub(super) data: [u8; 1],
    }
    impl CtrlDisplay {
        pub fn read(&self) -> CtrlDisplayRead {
            CtrlDisplayRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(CtrlDisplayWrite) -> CtrlDisplayWrite,
        {
            f(CtrlDisplayWrite { d: self }).d
        }
    }
    pub struct CtrlDisplayRead<'l> {
        d: &'l CtrlDisplay,
    }
    impl<'l> CtrlDisplayRead<'l> {
        /// brightness_control_block
        #[inline(always)]
        pub fn brightness_control_block(&self) -> bool {
            ((self.d.data[0] >> 5) & 0x01) != 0
        }
        /// display_dimming
        #[inline(always)]
        pub fn display_dimming(&self) -> bool {
            ((self.d.data[0] >> 3) & 0x01) != 0
        }
        /// backlight_control
        #[inline(always)]
        pub fn backlight_control(&self) -> bool {
            ((self.d.data[0] >> 2) & 0x01) != 0
        }
    }
    pub struct CtrlDisplayWrite<'l> {
        d: &'l mut CtrlDisplay,
    }
    impl<'l> CtrlDisplayWrite<'l> {
        /// brightness_control_block
        #[inline(always)]
        pub fn brightness_control_block(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 5);
            self.d.data[0] |= ((w) as u8) << 5;
            self
        }
        /// display_dimming
        #[inline(always)]
        pub fn display_dimming(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 3);
            self.d.data[0] |= ((w) as u8) << 3;
            self
        }
        /// backlight_control
        #[inline(always)]
        pub fn backlight_control(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 2);
            self.d.data[0] |= ((w) as u8) << 2;
            self
        }
    }
    impl Default for CtrlDisplay {
        fn default() -> Self {
            CtrlDisplay { data: [0x00] }
        }
    }
}
pub mod read_ctrl_display {
    #[derive(Copy, Clone, Debug)]
    pub struct CtrlDisplay {
        pub(super) data: [u8; 1],
    }
    impl CtrlDisplay {
        pub fn read(&self) -> CtrlDisplayRead {
            CtrlDisplayRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(CtrlDisplayWrite) -> CtrlDisplayWrite,
        {
            f(CtrlDisplayWrite { d: self }).d
        }
    }
    pub struct CtrlDisplayRead<'l> {
        d: &'l CtrlDisplay,
    }
    impl<'l> CtrlDisplayRead<'l> {
        /// brightness_control_block
        #[inline(always)]
        pub fn brightness_control_block(&self) -> bool {
            ((self.d.data[0] >> 5) & 0x01) != 0
        }
        /// display_dimming
        #[inline(always)]
        pub fn display_dimming(&self) -> bool {
            ((self.d.data[0] >> 3) & 0x01) != 0
        }
        /// backlight
        #[inline(always)]
        pub fn backlight(&self) -> bool {
            ((self.d.data[0] >> 2) & 0x01) != 0
        }
    }
    pub struct CtrlDisplayWrite<'l> {
        d: &'l mut CtrlDisplay,
    }
    impl<'l> CtrlDisplayWrite<'l> {
        /// brightness_control_block
        #[inline(always)]
        pub fn brightness_control_block(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 5);
            self.d.data[0] |= ((w) as u8) << 5;
            self
        }
        /// display_dimming
        #[inline(always)]
        pub fn display_dimming(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 3);
            self.d.data[0] |= ((w) as u8) << 3;
            self
        }
        /// backlight
        #[inline(always)]
        pub fn backlight(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 2);
            self.d.data[0] |= ((w) as u8) << 2;
            self
        }
    }
    impl Default for CtrlDisplay {
        fn default() -> Self {
            CtrlDisplay { data: [0x00] }
        }
    }
}
pub mod write_content_adaptive_brightness_control {
    enum_with_from! {
        AdaptiveBrightnessControlMode(u8) => { Off = 0x00, UserInterfaceImage = 0x01, StillPicture = 0x02, MovingImage = 0x03 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct ContentAdaptiveBrightnessControl {
        pub(super) data: [u8; 1],
    }
    impl ContentAdaptiveBrightnessControl {
        pub fn read(&self) -> ContentAdaptiveBrightnessControlRead {
            ContentAdaptiveBrightnessControlRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(
                ContentAdaptiveBrightnessControlWrite,
            ) -> ContentAdaptiveBrightnessControlWrite,
        {
            f(ContentAdaptiveBrightnessControlWrite { d: self }).d
        }
    }
    pub struct ContentAdaptiveBrightnessControlRead<'l> {
        d: &'l ContentAdaptiveBrightnessControl,
    }
    impl<'l> ContentAdaptiveBrightnessControlRead<'l> {
        /// adaptive_brightness_control_mode
        #[inline(always)]
        pub fn adaptive_brightness_control_mode(&self) -> AdaptiveBrightnessControlMode {
            AdaptiveBrightnessControlMode::from(self.d.data[0] & 0x03)
        }
    }
    pub struct ContentAdaptiveBrightnessControlWrite<'l> {
        d: &'l mut ContentAdaptiveBrightnessControl,
    }
    impl<'l> ContentAdaptiveBrightnessControlWrite<'l> {
        /// adaptive_brightness_control_mode
        #[inline(always)]
        pub fn adaptive_brightness_control_mode(self, w: AdaptiveBrightnessControlMode) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x03);
            self.d.data[0] |= w & 0x03;
            self
        }
    }
    impl Default for ContentAdaptiveBrightnessControl {
        fn default() -> Self {
            ContentAdaptiveBrightnessControl { data: [0x00] }
        }
    }
}
pub mod read_content_adaptive_brightness_control {
    enum_with_from! {
        AdaptiveBrightnessControlMode(u8) => { Off = 0x00, UserInterfaceImage = 0x01, StillPicture = 0x02, MovingImage = 0x03 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct ContentAdaptiveBrightnessControl {
        pub(super) data: [u8; 1],
    }
    impl ContentAdaptiveBrightnessControl {
        pub fn read(&self) -> ContentAdaptiveBrightnessControlRead {
            ContentAdaptiveBrightnessControlRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(
                ContentAdaptiveBrightnessControlWrite,
            ) -> ContentAdaptiveBrightnessControlWrite,
        {
            f(ContentAdaptiveBrightnessControlWrite { d: self }).d
        }
    }
    pub struct ContentAdaptiveBrightnessControlRead<'l> {
        d: &'l ContentAdaptiveBrightnessControl,
    }
    impl<'l> ContentAdaptiveBrightnessControlRead<'l> {
        /// adaptive_brightness_control_mode
        #[inline(always)]
        pub fn adaptive_brightness_control_mode(&self) -> AdaptiveBrightnessControlMode {
            AdaptiveBrightnessControlMode::from(self.d.data[0] & 0x03)
        }
    }
    pub struct ContentAdaptiveBrightnessControlWrite<'l> {
        d: &'l mut ContentAdaptiveBrightnessControl,
    }
    impl<'l> ContentAdaptiveBrightnessControlWrite<'l> {
        /// adaptive_brightness_control_mode
        #[inline(always)]
        pub fn adaptive_brightness_control_mode(self, w: AdaptiveBrightnessControlMode) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x03);
            self.d.data[0] |= w & 0x03;
            self
        }
    }
    impl Default for ContentAdaptiveBrightnessControl {
        fn default() -> Self {
            ContentAdaptiveBrightnessControl { data: [0x00] }
        }
    }
}
pub mod write_cabc_minimum_brightness {
    #[derive(Copy, Clone, Debug)]
    pub struct CabcMinimumBrightness {
        pub(super) data: [u8; 1],
    }
    impl CabcMinimumBrightness {
        pub fn read(&self) -> CabcMinimumBrightnessRead {
            CabcMinimumBrightnessRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(CabcMinimumBrightnessWrite) -> CabcMinimumBrightnessWrite,
        {
            f(CabcMinimumBrightnessWrite { d: self }).d
        }
    }
    pub struct CabcMinimumBrightnessRead<'l> {
        d: &'l CabcMinimumBrightness,
    }
    impl<'l> CabcMinimumBrightnessRead<'l> {
        /// cabc_minimum_brightness
        #[inline(always)]
        pub fn cabc_minimum_brightness(&self) -> u8 {
            self.d.data[0]
        }
    }
    pub struct CabcMinimumBrightnessWrite<'l> {
        d: &'l mut CabcMinimumBrightness,
    }
    impl<'l> CabcMinimumBrightnessWrite<'l> {
        /// cabc_minimum_brightness
        #[inline(always)]
        pub fn cabc_minimum_brightness(self, w: u8) -> Self {
            self.d.data[0] = w;
            self
        }
    }
    impl Default for CabcMinimumBrightness {
        fn default() -> Self {
            CabcMinimumBrightness { data: [0x00] }
        }
    }
}
pub mod read_cabc_minimum_brightness {
    #[derive(Copy, Clone, Debug)]
    pub struct CabcMinimumBrightness {
        pub(super) data: [u8; 1],
    }
    impl CabcMinimumBrightness {
        pub fn read(&self) -> CabcMinimumBrightnessRead {
            CabcMinimumBrightnessRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(CabcMinimumBrightnessWrite) -> CabcMinimumBrightnessWrite,
        {
            f(CabcMinimumBrightnessWrite { d: self }).d
        }
    }
    pub struct CabcMinimumBrightnessRead<'l> {
        d: &'l CabcMinimumBrightness,
    }
    impl<'l> CabcMinimumBrightnessRead<'l> {
        /// cabc_minimum_brightness
        #[inline(always)]
        pub fn cabc_minimum_brightness(&self) -> u8 {
            self.d.data[0]
        }
    }
    pub struct CabcMinimumBrightnessWrite<'l> {
        d: &'l mut CabcMinimumBrightness,
    }
    impl<'l> CabcMinimumBrightnessWrite<'l> {
        /// cabc_minimum_brightness
        #[inline(always)]
        pub fn cabc_minimum_brightness(self, w: u8) -> Self {
            self.d.data[0] = w;
            self
        }
    }
    impl Default for CabcMinimumBrightness {
        fn default() -> Self {
            CabcMinimumBrightness { data: [0x00] }
        }
    }
}
pub mod read_id1 {
    #[derive(Copy, Clone, Debug)]
    pub struct Id1 {
        pub(super) data: [u8; 1],
    }
    impl Id1 {
        pub fn read(&self) -> Id1Read {
            Id1Read { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(Id1Write) -> Id1Write,
        {
            f(Id1Write { d: self }).d
        }
    }
    pub struct Id1Read<'l> {
        d: &'l Id1,
    }
    impl<'l> Id1Read<'l> {
        /// id1
        #[inline(always)]
        pub fn id1(&self) -> u8 {
            self.d.data[0]
        }
    }
    pub struct Id1Write<'l> {
        d: &'l mut Id1,
    }
    impl<'l> Id1Write<'l> {
        /// id1
        #[inline(always)]
        pub fn id1(self, w: u8) -> Self {
            self.d.data[0] = w;
            self
        }
    }
    impl Default for Id1 {
        fn default() -> Self {
            Id1 { data: [0x00] }
        }
    }
}
pub mod read_id2 {
    #[derive(Copy, Clone, Debug)]
    pub struct Id2 {
        pub(super) data: [u8; 1],
    }
    impl Id2 {
        pub fn read(&self) -> Id2Read {
            Id2Read { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(Id2Write) -> Id2Write,
        {
            f(Id2Write { d: self }).d
        }
    }
    pub struct Id2Read<'l> {
        d: &'l Id2,
    }
    impl<'l> Id2Read<'l> {
        /// id2
        #[inline(always)]
        pub fn id2(&self) -> u8 {
            self.d.data[0] & 0x7F
        }
    }
    pub struct Id2Write<'l> {
        d: &'l mut Id2,
    }
    impl<'l> Id2Write<'l> {
        /// id2
        #[inline(always)]
        pub fn id2(self, w: u8) -> Self {
            self.d.data[0] &= !(0x7F);
            self.d.data[0] |= w & 0x7F;
            self
        }
    }
    impl Default for Id2 {
        fn default() -> Self {
            Id2 { data: [0x80] }
        }
    }
}
pub mod read_id3 {
    #[derive(Copy, Clone, Debug)]
    pub struct Id3 {
        pub(super) data: [u8; 1],
    }
    impl Id3 {
        pub fn read(&self) -> Id3Read {
            Id3Read { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(Id3Write) -> Id3Write,
        {
            f(Id3Write { d: self }).d
        }
    }
    pub struct Id3Read<'l> {
        d: &'l Id3,
    }
    impl<'l> Id3Read<'l> {
        /// id3
        #[inline(always)]
        pub fn id3(&self) -> u8 {
            self.d.data[0]
        }
    }
    pub struct Id3Write<'l> {
        d: &'l mut Id3,
    }
    impl<'l> Id3Write<'l> {
        /// id3
        #[inline(always)]
        pub fn id3(self, w: u8) -> Self {
            self.d.data[0] = w;
            self
        }
    }
    impl Default for Id3 {
        fn default() -> Self {
            Id3 { data: [0x00] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod rgb_interface_signal_control {
    enum_with_from! {
        DisplayDataPath(u8) => { DirectToShiftRegisterDefault = 0x00, Memory = 0x01 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct RgbInterfaceSignalControl {
        pub(super) data: [u8; 1],
    }
    impl RgbInterfaceSignalControl {
        pub fn read(&self) -> RgbInterfaceSignalControlRead {
            RgbInterfaceSignalControlRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(RgbInterfaceSignalControlWrite) -> RgbInterfaceSignalControlWrite,
        {
            f(RgbInterfaceSignalControlWrite { d: self }).d
        }
    }
    pub struct RgbInterfaceSignalControlRead<'l> {
        d: &'l RgbInterfaceSignalControl,
    }
    impl<'l> RgbInterfaceSignalControlRead<'l> {
        /// display_data_path
        #[inline(always)]
        pub fn display_data_path(&self) -> DisplayDataPath {
            DisplayDataPath::from((self.d.data[0] >> 7) & 0x01)
        }
        /// rgb_interface_selection
        #[inline(always)]
        pub fn rgb_interface_selection(&self) -> u8 {
            (self.d.data[0] >> 5) & 0x03
        }
        /// vsync_polarity
        #[inline(always)]
        pub fn vsync_polarity(&self) -> bool {
            ((self.d.data[0] >> 3) & 0x01) != 0
        }
        /// hsync_polarity
        #[inline(always)]
        pub fn hsync_polarity(&self) -> bool {
            ((self.d.data[0] >> 2) & 0x01) != 0
        }
        /// dotclk_polarity
        #[inline(always)]
        pub fn dotclk_polarity(&self) -> bool {
            ((self.d.data[0] >> 1) & 0x01) != 0
        }
        /// de_polarity
        #[inline(always)]
        pub fn de_polarity(&self) -> bool {
            (self.d.data[0] & 0x01) != 0
        }
    }
    pub struct RgbInterfaceSignalControlWrite<'l> {
        d: &'l mut RgbInterfaceSignalControl,
    }
    impl<'l> RgbInterfaceSignalControlWrite<'l> {
        /// display_data_path
        #[inline(always)]
        pub fn display_data_path(self, w: DisplayDataPath) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 7);
            self.d.data[0] |= (w & 0x01) << 7;
            self
        }
        /// rgb_interface_selection
        #[inline(always)]
        pub fn rgb_interface_selection(self, w: u8) -> Self {
            self.d.data[0] &= !(0x03 << 5);
            self.d.data[0] |= (w & 0x03) << 5;
            self
        }
        /// vsync_polarity
        #[inline(always)]
        pub fn vsync_polarity(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 3);
            self.d.data[0] |= ((w) as u8) << 3;
            self
        }
        /// hsync_polarity
        #[inline(always)]
        pub fn hsync_polarity(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 2);
            self.d.data[0] |= ((w) as u8) << 2;
            self
        }
        /// dotclk_polarity
        #[inline(always)]
        pub fn dotclk_polarity(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 1);
            self.d.data[0] |= ((w) as u8) << 1;
            self
        }
        /// de_polarity
        #[inline(always)]
        pub fn de_polarity(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01);
            self.d.data[0] |= (w) as u8;
            self
        }
    }
    impl Default for RgbInterfaceSignalControl {
        fn default() -> Self {
            RgbInterfaceSignalControl { data: [0x41] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod frame_control_in_normal_mode {
    enum_with_from! {
        DivisionRatio(u8) => { Fosc = 0x00, FoscDiv2 = 0x01, FoscDiv4 = 0x02, FoscDiv8 = 0x03 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct FrameControlInNormalMode {
        pub(super) data: [u8; 2],
    }
    impl FrameControlInNormalMode {
        pub fn read(&self) -> FrameControlInNormalModeRead {
            FrameControlInNormalModeRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(FrameControlInNormalModeWrite) -> FrameControlInNormalModeWrite,
        {
            f(FrameControlInNormalModeWrite { d: self }).d
        }
    }
    pub struct FrameControlInNormalModeRead<'l> {
        d: &'l FrameControlInNormalMode,
    }
    impl<'l> FrameControlInNormalModeRead<'l> {
        /// division_ratio
        #[inline(always)]
        pub fn division_ratio(&self) -> DivisionRatio {
            DivisionRatio::from(self.d.data[0] & 0x03)
        }
        /// clock_per_line
        #[inline(always)]
        pub fn clock_per_line(&self) -> u8 {
            (((self.d.data[1] & 0x1F) - 16) as u8) + 16
        }
    }
    pub struct FrameControlInNormalModeWrite<'l> {
        d: &'l mut FrameControlInNormalMode,
    }
    impl<'l> FrameControlInNormalModeWrite<'l> {
        /// division_ratio
        #[inline(always)]
        pub fn division_ratio(self, w: DivisionRatio) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x03);
            self.d.data[0] |= w & 0x03;
            self
        }
        /// clock_per_line
        #[inline(always)]
        pub fn clock_per_line(self, w: u8) -> Self {
            let w = ((w - 16) as u8) + 16;
            self.d.data[1] &= !(0x1F);
            self.d.data[1] |= w & 0x1F;
            self
        }
    }
    impl Default for FrameControlInNormalMode {
        fn default() -> Self {
            FrameControlInNormalMode { data: [0x00, 0x1B] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod frame_control_in_idle_mode {
    enum_with_from! {
        DivisionRatio(u8) => { Fosc = 0x00, FoscDiv2 = 0x01, FoscDiv4 = 0x02, FoscDiv8 = 0x03 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct FrameControlInIdleMode {
        pub(super) data: [u8; 2],
    }
    impl FrameControlInIdleMode {
        pub fn read(&self) -> FrameControlInIdleModeRead {
            FrameControlInIdleModeRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(FrameControlInIdleModeWrite) -> FrameControlInIdleModeWrite,
        {
            f(FrameControlInIdleModeWrite { d: self }).d
        }
    }
    pub struct FrameControlInIdleModeRead<'l> {
        d: &'l FrameControlInIdleMode,
    }
    impl<'l> FrameControlInIdleModeRead<'l> {
        /// division_ratio
        #[inline(always)]
        pub fn division_ratio(&self) -> DivisionRatio {
            DivisionRatio::from(self.d.data[0] & 0x03)
        }
        /// clock_per_line
        #[inline(always)]
        pub fn clock_per_line(&self) -> u8 {
            (((self.d.data[1] & 0x1F) - 16) as u8) + 16
        }
    }
    pub struct FrameControlInIdleModeWrite<'l> {
        d: &'l mut FrameControlInIdleMode,
    }
    impl<'l> FrameControlInIdleModeWrite<'l> {
        /// division_ratio
        #[inline(always)]
        pub fn division_ratio(self, w: DivisionRatio) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x03);
            self.d.data[0] |= w & 0x03;
            self
        }
        /// clock_per_line
        #[inline(always)]
        pub fn clock_per_line(self, w: u8) -> Self {
            let w = ((w - 16) as u8) + 16;
            self.d.data[1] &= !(0x1F);
            self.d.data[1] |= w & 0x1F;
            self
        }
    }
    impl Default for FrameControlInIdleMode {
        fn default() -> Self {
            FrameControlInIdleMode { data: [0x00, 0x1B] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod frame_control_in_partial_mode {
    enum_with_from! {
        DivisionRatio(u8) => { Fosc = 0x00, FoscDiv2 = 0x01, FoscDiv4 = 0x02, FoscDiv8 = 0x03 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct FrameControlInPartialMode {
        pub(super) data: [u8; 2],
    }
    impl FrameControlInPartialMode {
        pub fn read(&self) -> FrameControlInPartialModeRead {
            FrameControlInPartialModeRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(FrameControlInPartialModeWrite) -> FrameControlInPartialModeWrite,
        {
            f(FrameControlInPartialModeWrite { d: self }).d
        }
    }
    pub struct FrameControlInPartialModeRead<'l> {
        d: &'l FrameControlInPartialMode,
    }
    impl<'l> FrameControlInPartialModeRead<'l> {
        /// division_ratio
        #[inline(always)]
        pub fn division_ratio(&self) -> DivisionRatio {
            DivisionRatio::from(self.d.data[0] & 0x03)
        }
        /// clock_per_line
        #[inline(always)]
        pub fn clock_per_line(&self) -> u8 {
            (((self.d.data[1] & 0x1F) - 16) as u8) + 16
        }
    }
    pub struct FrameControlInPartialModeWrite<'l> {
        d: &'l mut FrameControlInPartialMode,
    }
    impl<'l> FrameControlInPartialModeWrite<'l> {
        /// division_ratio
        #[inline(always)]
        pub fn division_ratio(self, w: DivisionRatio) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x03);
            self.d.data[0] |= w & 0x03;
            self
        }
        /// clock_per_line
        #[inline(always)]
        pub fn clock_per_line(self, w: u8) -> Self {
            let w = ((w - 16) as u8) + 16;
            self.d.data[1] &= !(0x1F);
            self.d.data[1] |= w & 0x1F;
            self
        }
    }
    impl Default for FrameControlInPartialMode {
        fn default() -> Self {
            FrameControlInPartialMode { data: [0x00, 0x1B] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod display_inversion_control {
    enum_with_from! {
        InversionSettingInFullColorsNormalMode(u8) => { LineInversion = 0x00, FrameInversion = 0x01 },
        InversionSettingInIdleMode(u8) => { LineInversion = 0x00, FrameInversion = 0x01 },
        InversionSettingInFullColorsPartialMode(u8) => { LineInversion = 0x00, FrameInversion = 0x01 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct DisplayInversionControl {
        pub(super) data: [u8; 1],
    }
    impl DisplayInversionControl {
        pub fn read(&self) -> DisplayInversionControlRead {
            DisplayInversionControlRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(DisplayInversionControlWrite) -> DisplayInversionControlWrite,
        {
            f(DisplayInversionControlWrite { d: self }).d
        }
    }
    pub struct DisplayInversionControlRead<'l> {
        d: &'l DisplayInversionControl,
    }
    impl<'l> DisplayInversionControlRead<'l> {
        /// inversion_setting_in_full_colors_normal_mode
        #[inline(always)]
        pub fn inversion_setting_in_full_colors_normal_mode(
            &self,
        ) -> InversionSettingInFullColorsNormalMode {
            InversionSettingInFullColorsNormalMode::from((self.d.data[0] >> 2) & 0x01)
        }
        /// inversion_setting_in_idle_mode
        #[inline(always)]
        pub fn inversion_setting_in_idle_mode(&self) -> InversionSettingInIdleMode {
            InversionSettingInIdleMode::from((self.d.data[0] >> 1) & 0x01)
        }
        /// inversion_setting_in_full_colors_partial_mode
        #[inline(always)]
        pub fn inversion_setting_in_full_colors_partial_mode(
            &self,
        ) -> InversionSettingInFullColorsPartialMode {
            InversionSettingInFullColorsPartialMode::from(self.d.data[0] & 0x01)
        }
    }
    pub struct DisplayInversionControlWrite<'l> {
        d: &'l mut DisplayInversionControl,
    }
    impl<'l> DisplayInversionControlWrite<'l> {
        /// inversion_setting_in_full_colors_normal_mode
        #[inline(always)]
        pub fn inversion_setting_in_full_colors_normal_mode(
            self,
            w: InversionSettingInFullColorsNormalMode,
        ) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 2);
            self.d.data[0] |= (w & 0x01) << 2;
            self
        }
        /// inversion_setting_in_idle_mode
        #[inline(always)]
        pub fn inversion_setting_in_idle_mode(self, w: InversionSettingInIdleMode) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 1);
            self.d.data[0] |= (w & 0x01) << 1;
            self
        }
        /// inversion_setting_in_full_colors_partial_mode
        #[inline(always)]
        pub fn inversion_setting_in_full_colors_partial_mode(
            self,
            w: InversionSettingInFullColorsPartialMode,
        ) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01);
            self.d.data[0] |= w & 0x01;
            self
        }
    }
    impl Default for DisplayInversionControl {
        fn default() -> Self {
            DisplayInversionControl { data: [0x02] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod blanking_porch_control {
    #[derive(Copy, Clone, Debug)]
    pub struct BlankingPorchControl {
        pub(super) data: [u8; 4],
    }
    impl BlankingPorchControl {
        pub fn read(&self) -> BlankingPorchControlRead {
            BlankingPorchControlRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(BlankingPorchControlWrite) -> BlankingPorchControlWrite,
        {
            f(BlankingPorchControlWrite { d: self }).d
        }
    }
    pub struct BlankingPorchControlRead<'l> {
        d: &'l BlankingPorchControl,
    }
    impl<'l> BlankingPorchControlRead<'l> {
        /// vfp
        #[inline(always)]
        pub fn vfp(&self) -> u8 {
            (((self.d.data[0] & 0x7F) - 2) as u8) + 2
        }
        /// vbp
        #[inline(always)]
        pub fn vbp(&self) -> u8 {
            (((self.d.data[1] & 0x7F) - 2) as u8) + 2
        }
        /// hfp
        #[inline(always)]
        pub fn hfp(&self) -> u8 {
            (((self.d.data[2] & 0x1F) - 2) as u8) + 2
        }
        /// hbp
        #[inline(always)]
        pub fn hbp(&self) -> u8 {
            (((self.d.data[3] & 0x1F) - 2) as u8) + 2
        }
    }
    pub struct BlankingPorchControlWrite<'l> {
        d: &'l mut BlankingPorchControl,
    }
    impl<'l> BlankingPorchControlWrite<'l> {
        /// vfp
        #[inline(always)]
        pub fn vfp(self, w: u8) -> Self {
            let w = ((w - 2) as u8) + 2;
            self.d.data[0] &= !(0x7F);
            self.d.data[0] |= w & 0x7F;
            self
        }
        /// vbp
        #[inline(always)]
        pub fn vbp(self, w: u8) -> Self {
            let w = ((w - 2) as u8) + 2;
            self.d.data[1] &= !(0x7F);
            self.d.data[1] |= w & 0x7F;
            self
        }
        /// hfp
        #[inline(always)]
        pub fn hfp(self, w: u8) -> Self {
            let w = ((w - 2) as u8) + 2;
            self.d.data[2] &= !(0x1F);
            self.d.data[2] |= w & 0x1F;
            self
        }
        /// hbp
        #[inline(always)]
        pub fn hbp(self, w: u8) -> Self {
            let w = ((w - 2) as u8) + 2;
            self.d.data[3] &= !(0x1F);
            self.d.data[3] |= w & 0x1F;
            self
        }
    }
    impl Default for BlankingPorchControl {
        fn default() -> Self {
            BlankingPorchControl {
                data: [0x02, 0x02, 0x0A, 0x14],
            }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod display_function_control {
    enum_with_from! {
        GateOutputsInNonDisplayArea(u8) => { NormalScan = 0x00, IntervalScan = 0x02 },
        LiquidCrystalType(u8) => { NormallyBlack = 0x00, NormallyWhite = 0x01 },
        GateOutputScanDirection(u8) => { G1Cid4G320 = 0x00, G320Cid4G1 = 0x01 },
        SourceOutputScanDirection(u8) => { S1Cid4S720 = 0x00, S720Cid4S1 = 0x01 },
        ScanCycle(u8) => { N1Frame = 0x00, N3Frames = 0x01, N5Frames = 0x02, N7Frames = 0x03, N9Frames = 0x04, N11Frames = 0x05, N13Frames = 0x06, N15Frames = 0x07, N17Frames = 0x08, N19Frames = 0x09, N21Frames = 0x0A, N23Frames = 0x0B, N25Frames = 0x0C, N27Frames = 0x0D, N29Frames = 0x0E, N31Frames = 0x0F },
        LcdDriverLine(u8) => { N16Lines = 0x01, N24Lines = 0x02, N32Lines = 0x03, N40Lines = 0x04, N48Lines = 0x05, N56Lines = 0x06, N64Lines = 0x07, N72Lines = 0x08, N80Lines = 0x09, N88Lines = 0x0A, N96Lines = 0x0B, N104Lines = 0x0C, N112Lines = 0x0D, N120Lines = 0x0E, N128Lines = 0x0F, N136Lines = 0x10, N144Lines = 0x11, N152Lines = 0x12, N160Lines = 0x13, N168Lines = 0x14, N176Lines = 0x15, N184Lines = 0x16, N192Lines = 0x17, N200Lines = 0x18, N208Lines = 0x19, N216Lines = 0x1A, N224Lines = 0x1B, N232Lines = 0x1C, N240Lines = 0x1D, N248Lines = 0x1E, N256Lines = 0x1F, N264Lines = 0x20, N272Lines = 0x21, N280Lines = 0x22, N288Lines = 0x23, N296Lines = 0x24, N304Lines = 0x25, N312Lines = 0x26, N320Lines = 0x27 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct DisplayFunctionControl {
        pub(super) data: [u8; 4],
    }
    impl DisplayFunctionControl {
        pub fn read(&self) -> DisplayFunctionControlRead {
            DisplayFunctionControlRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(DisplayFunctionControlWrite) -> DisplayFunctionControlWrite,
        {
            f(DisplayFunctionControlWrite { d: self }).d
        }
    }
    pub struct DisplayFunctionControlRead<'l> {
        d: &'l DisplayFunctionControl,
    }
    impl<'l> DisplayFunctionControlRead<'l> {
        /// gate_outputs_in_non_display_area
        #[inline(always)]
        pub fn gate_outputs_in_non_display_area(&self) -> GateOutputsInNonDisplayArea {
            GateOutputsInNonDisplayArea::from((self.d.data[0] >> 2) & 0x03)
        }
        /// determine_source_and_vcom_output_in_an_on_display_area_in_the_partial_display_mode
        #[inline(always)]
        pub fn determine_source_and_vcom_output_in_an_on_display_area_in_the_partial_display_mode(
            &self,
        ) -> u8 {
            self.d.data[0] & 0x03
        }
        /// liquid_crystal_type
        #[inline(always)]
        pub fn liquid_crystal_type(&self) -> LiquidCrystalType {
            LiquidCrystalType::from((self.d.data[1] >> 7) & 0x01)
        }
        /// gate_output_scan_direction
        #[inline(always)]
        pub fn gate_output_scan_direction(&self) -> GateOutputScanDirection {
            GateOutputScanDirection::from((self.d.data[1] >> 6) & 0x01)
        }
        /// source_output_scan_direction
        #[inline(always)]
        pub fn source_output_scan_direction(&self) -> SourceOutputScanDirection {
            SourceOutputScanDirection::from((self.d.data[1] >> 5) & 0x01)
        }
        /// sm
        #[inline(always)]
        pub fn sm(&self) -> bool {
            ((self.d.data[1] >> 4) & 0x01) != 0
        }
        /// scan_cycle
        #[inline(always)]
        pub fn scan_cycle(&self) -> ScanCycle {
            ScanCycle::from(self.d.data[1] & 0x0F)
        }
        /// lcd_driver_line
        #[inline(always)]
        pub fn lcd_driver_line(&self) -> LcdDriverLine {
            LcdDriverLine::from(self.d.data[2] & 0x3F)
        }
        /// pcdiv
        #[inline(always)]
        pub fn pcdiv(&self) -> u8 {
            self.d.data[3] & 0x3F
        }
    }
    pub struct DisplayFunctionControlWrite<'l> {
        d: &'l mut DisplayFunctionControl,
    }
    impl<'l> DisplayFunctionControlWrite<'l> {
        /// gate_outputs_in_non_display_area
        #[inline(always)]
        pub fn gate_outputs_in_non_display_area(self, w: GateOutputsInNonDisplayArea) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x03 << 2);
            self.d.data[0] |= (w & 0x03) << 2;
            self
        }
        /// determine_source_and_vcom_output_in_an_on_display_area_in_the_partial_display_mode
        #[inline(always)]
        pub fn determine_source_and_vcom_output_in_an_on_display_area_in_the_partial_display_mode(
            self,
            w: u8,
        ) -> Self {
            self.d.data[0] &= !(0x03);
            self.d.data[0] |= w & 0x03;
            self
        }
        /// liquid_crystal_type
        #[inline(always)]
        pub fn liquid_crystal_type(self, w: LiquidCrystalType) -> Self {
            let w = w as u8;
            self.d.data[1] &= !(0x01 << 7);
            self.d.data[1] |= (w & 0x01) << 7;
            self
        }
        /// gate_output_scan_direction
        #[inline(always)]
        pub fn gate_output_scan_direction(self, w: GateOutputScanDirection) -> Self {
            let w = w as u8;
            self.d.data[1] &= !(0x01 << 6);
            self.d.data[1] |= (w & 0x01) << 6;
            self
        }
        /// source_output_scan_direction
        #[inline(always)]
        pub fn source_output_scan_direction(self, w: SourceOutputScanDirection) -> Self {
            let w = w as u8;
            self.d.data[1] &= !(0x01 << 5);
            self.d.data[1] |= (w & 0x01) << 5;
            self
        }
        /// sm
        #[inline(always)]
        pub fn sm(self, w: bool) -> Self {
            self.d.data[1] &= !(0x01 << 4);
            self.d.data[1] |= ((w) as u8) << 4;
            self
        }
        /// scan_cycle
        #[inline(always)]
        pub fn scan_cycle(self, w: ScanCycle) -> Self {
            let w = w as u8;
            self.d.data[1] &= !(0x0F);
            self.d.data[1] |= w & 0x0F;
            self
        }
        /// lcd_driver_line
        #[inline(always)]
        pub fn lcd_driver_line(self, w: LcdDriverLine) -> Self {
            let w = w as u8;
            self.d.data[2] &= !(0x3F);
            self.d.data[2] |= w & 0x3F;
            self
        }
        /// pcdiv
        #[inline(always)]
        pub fn pcdiv(self, w: u8) -> Self {
            self.d.data[3] &= !(0x3F);
            self.d.data[3] |= w & 0x3F;
            self
        }
    }
    impl Default for DisplayFunctionControl {
        fn default() -> Self {
            DisplayFunctionControl {
                data: [0x0A, 0x82, 0x27, 0x00],
            }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod entry_mode {
    enum_with_from! {
        G1G320GateOutput(u8) => { Vgh = 0x01, Vgl = 0x02, NormalDisplay = 0x03 },
        LowVoltageDetection(u8) => { Enable = 0x00, Disable = 0x01 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct EntryModeSet {
        pub(super) data: [u8; 1],
    }
    impl EntryModeSet {
        pub fn read(&self) -> EntryModeSetRead {
            EntryModeSetRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(EntryModeSetWrite) -> EntryModeSetWrite,
        {
            f(EntryModeSetWrite { d: self }).d
        }
    }
    pub struct EntryModeSetRead<'l> {
        d: &'l EntryModeSet,
    }
    impl<'l> EntryModeSetRead<'l> {
        /// deep_standby_mode
        #[inline(always)]
        pub fn deep_standby_mode(&self) -> bool {
            ((self.d.data[0] >> 3) & 0x01) != 0
        }
        /// g1_g320_gate_output
        #[inline(always)]
        pub fn g1_g320_gate_output(&self) -> G1G320GateOutput {
            G1G320GateOutput::from((self.d.data[0] >> 1) & 0x03)
        }
        /// low_voltage_detection
        #[inline(always)]
        pub fn low_voltage_detection(&self) -> LowVoltageDetection {
            LowVoltageDetection::from(self.d.data[0] & 0x01)
        }
    }
    pub struct EntryModeSetWrite<'l> {
        d: &'l mut EntryModeSet,
    }
    impl<'l> EntryModeSetWrite<'l> {
        /// deep_standby_mode
        #[inline(always)]
        pub fn deep_standby_mode(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 3);
            self.d.data[0] |= ((w) as u8) << 3;
            self
        }
        /// g1_g320_gate_output
        #[inline(always)]
        pub fn g1_g320_gate_output(self, w: G1G320GateOutput) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x03 << 1);
            self.d.data[0] |= (w & 0x03) << 1;
            self
        }
        /// low_voltage_detection
        #[inline(always)]
        pub fn low_voltage_detection(self, w: LowVoltageDetection) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01);
            self.d.data[0] |= w & 0x01;
            self
        }
    }
    impl Default for EntryModeSet {
        fn default() -> Self {
            EntryModeSet { data: [0x06] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod backlight_control1 {
    enum_with_from! {
        HistogramThresholdInUserInterfaceMode(u8) => { N99 = 0x00, N98 = 0x01, N96 = 0x02, N94 = 0x03, N92 = 0x04, N90 = 0x05, N88 = 0x06, N86 = 0x07, N84 = 0x08, N82 = 0x09, N80 = 0x0A, N78 = 0x0B, N76 = 0x0C, N74 = 0x0D, N72 = 0x0E, N70 = 0x0F },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct BacklightControl1 {
        pub(super) data: [u8; 1],
    }
    impl BacklightControl1 {
        pub fn read(&self) -> BacklightControl1Read {
            BacklightControl1Read { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(BacklightControl1Write) -> BacklightControl1Write,
        {
            f(BacklightControl1Write { d: self }).d
        }
    }
    pub struct BacklightControl1Read<'l> {
        d: &'l BacklightControl1,
    }
    impl<'l> BacklightControl1Read<'l> {
        /// histogram_threshold_in_user_interface_mode
        #[inline(always)]
        pub fn histogram_threshold_in_user_interface_mode(
            &self,
        ) -> HistogramThresholdInUserInterfaceMode {
            HistogramThresholdInUserInterfaceMode::from(self.d.data[0] & 0x0F)
        }
    }
    pub struct BacklightControl1Write<'l> {
        d: &'l mut BacklightControl1,
    }
    impl<'l> BacklightControl1Write<'l> {
        /// histogram_threshold_in_user_interface_mode
        #[inline(always)]
        pub fn histogram_threshold_in_user_interface_mode(
            self,
            w: HistogramThresholdInUserInterfaceMode,
        ) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x0F);
            self.d.data[0] |= w & 0x0F;
            self
        }
    }
    impl Default for BacklightControl1 {
        fn default() -> Self {
            BacklightControl1 { data: [0x06] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod backlight_control2 {
    enum_with_from! {
        HistogramThresholdInMovingImageMode(u8) => { N99 = 0x00, N98 = 0x01, N96 = 0x02, N94 = 0x03, N92 = 0x04, N90 = 0x05, N88 = 0x06, N86 = 0x07, N84 = 0x08, N82 = 0x09, N80 = 0x0A, N78 = 0x0B, N76 = 0x0C, N74 = 0x0D, N72 = 0x0E, N70 = 0x0F },
        HistogramThresholdInStillPictureMode(u8) => { N99 = 0x00, N98 = 0x01, N96 = 0x02, N94 = 0x03, N92 = 0x04, N90 = 0x05, N88 = 0x06, N86 = 0x07, N84 = 0x08, N82 = 0x09, N80 = 0x0A, N78 = 0x0B, N76 = 0x0C, N74 = 0x0D, N72 = 0x0E, N70 = 0x0F },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct BacklightControl2 {
        pub(super) data: [u8; 1],
    }
    impl BacklightControl2 {
        pub fn read(&self) -> BacklightControl2Read {
            BacklightControl2Read { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(BacklightControl2Write) -> BacklightControl2Write,
        {
            f(BacklightControl2Write { d: self }).d
        }
    }
    pub struct BacklightControl2Read<'l> {
        d: &'l BacklightControl2,
    }
    impl<'l> BacklightControl2Read<'l> {
        /// histogram_threshold_in_moving_image_mode
        #[inline(always)]
        pub fn histogram_threshold_in_moving_image_mode(
            &self,
        ) -> HistogramThresholdInMovingImageMode {
            HistogramThresholdInMovingImageMode::from((self.d.data[0] >> 4) & 0x0F)
        }
        /// histogram_threshold_in_still_picture_mode
        #[inline(always)]
        pub fn histogram_threshold_in_still_picture_mode(
            &self,
        ) -> HistogramThresholdInStillPictureMode {
            HistogramThresholdInStillPictureMode::from(self.d.data[0] & 0x0F)
        }
    }
    pub struct BacklightControl2Write<'l> {
        d: &'l mut BacklightControl2,
    }
    impl<'l> BacklightControl2Write<'l> {
        /// histogram_threshold_in_moving_image_mode
        #[inline(always)]
        pub fn histogram_threshold_in_moving_image_mode(
            self,
            w: HistogramThresholdInMovingImageMode,
        ) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x0F << 4);
            self.d.data[0] |= (w & 0x0F) << 4;
            self
        }
        /// histogram_threshold_in_still_picture_mode
        #[inline(always)]
        pub fn histogram_threshold_in_still_picture_mode(
            self,
            w: HistogramThresholdInStillPictureMode,
        ) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x0F);
            self.d.data[0] |= w & 0x0F;
            self
        }
    }
    impl Default for BacklightControl2 {
        fn default() -> Self {
            BacklightControl2 { data: [0xCC] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod backlight_control3 {
    enum_with_from! {
        PixelThresholdInUserInterfaceMode(u8) => { N252 = 0x00, N248 = 0x01, N244 = 0x02, N240 = 0x03, N236 = 0x04, N232 = 0x05, N228 = 0x06, N224 = 0x07, N220 = 0x08, N216 = 0x09, N212 = 0x0A, N208 = 0x0B, N204 = 0x0C, N200 = 0x0D, N196 = 0x0E, N192 = 0x0F },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct BacklightControl3 {
        pub(super) data: [u8; 1],
    }
    impl BacklightControl3 {
        pub fn read(&self) -> BacklightControl3Read {
            BacklightControl3Read { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(BacklightControl3Write) -> BacklightControl3Write,
        {
            f(BacklightControl3Write { d: self }).d
        }
    }
    pub struct BacklightControl3Read<'l> {
        d: &'l BacklightControl3,
    }
    impl<'l> BacklightControl3Read<'l> {
        /// pixel_threshold_in_user_interface_mode
        #[inline(always)]
        pub fn pixel_threshold_in_user_interface_mode(&self) -> PixelThresholdInUserInterfaceMode {
            PixelThresholdInUserInterfaceMode::from(self.d.data[0] & 0x0F)
        }
    }
    pub struct BacklightControl3Write<'l> {
        d: &'l mut BacklightControl3,
    }
    impl<'l> BacklightControl3Write<'l> {
        /// pixel_threshold_in_user_interface_mode
        #[inline(always)]
        pub fn pixel_threshold_in_user_interface_mode(
            self,
            w: PixelThresholdInUserInterfaceMode,
        ) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x0F);
            self.d.data[0] |= w & 0x0F;
            self
        }
    }
    impl Default for BacklightControl3 {
        fn default() -> Self {
            BacklightControl3 { data: [0x04] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod backlight_control4 {
    enum_with_from! {
        PixelThresholdInMovingImageMode(u8) => { N224 = 0x00, N220 = 0x01, N216 = 0x02, N212 = 0x03, N208 = 0x04, N204 = 0x05, N200 = 0x06, N196 = 0x07, N192 = 0x08, N188 = 0x09, N184 = 0x0A, N180 = 0x0B, N176 = 0x0C, N172 = 0x0D, N168 = 0x0E, N164 = 0x0F },
        PixelThresholdInStillPictureMode(u8) => { N224 = 0x00, N220 = 0x01, N216 = 0x02, N212 = 0x03, N208 = 0x04, N204 = 0x05, N200 = 0x06, N196 = 0x07, N192 = 0x08, N188 = 0x09, N184 = 0x0A, N180 = 0x0B, N176 = 0x0C, N172 = 0x0D, N168 = 0x0E, N164 = 0x0F },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct BacklightControl4 {
        pub(super) data: [u8; 1],
    }
    impl BacklightControl4 {
        pub fn read(&self) -> BacklightControl4Read {
            BacklightControl4Read { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(BacklightControl4Write) -> BacklightControl4Write,
        {
            f(BacklightControl4Write { d: self }).d
        }
    }
    pub struct BacklightControl4Read<'l> {
        d: &'l BacklightControl4,
    }
    impl<'l> BacklightControl4Read<'l> {
        /// pixel_threshold_in_moving_image_mode
        #[inline(always)]
        pub fn pixel_threshold_in_moving_image_mode(&self) -> PixelThresholdInMovingImageMode {
            PixelThresholdInMovingImageMode::from((self.d.data[0] >> 4) & 0x0F)
        }
        /// pixel_threshold_in_still_picture_mode
        #[inline(always)]
        pub fn pixel_threshold_in_still_picture_mode(&self) -> PixelThresholdInStillPictureMode {
            PixelThresholdInStillPictureMode::from(self.d.data[0] & 0x0F)
        }
    }
    pub struct BacklightControl4Write<'l> {
        d: &'l mut BacklightControl4,
    }
    impl<'l> BacklightControl4Write<'l> {
        /// pixel_threshold_in_moving_image_mode
        #[inline(always)]
        pub fn pixel_threshold_in_moving_image_mode(
            self,
            w: PixelThresholdInMovingImageMode,
        ) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x0F << 4);
            self.d.data[0] |= (w & 0x0F) << 4;
            self
        }
        /// pixel_threshold_in_still_picture_mode
        #[inline(always)]
        pub fn pixel_threshold_in_still_picture_mode(
            self,
            w: PixelThresholdInStillPictureMode,
        ) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x0F);
            self.d.data[0] |= w & 0x0F;
            self
        }
    }
    impl Default for BacklightControl4 {
        fn default() -> Self {
            BacklightControl4 { data: [0x65] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod backlight_control5 {
    enum_with_from! {
        TransitionTime(u8) => { N1Frame = 0x01, N2Frames = 0x02, N4Frames = 0x03, N8Frames = 0x04, N16Frames = 0x05, N32Frames = 0x06, N64Frames = 0x07 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct BacklightControl5 {
        pub(super) data: [u8; 1],
    }
    impl BacklightControl5 {
        pub fn read(&self) -> BacklightControl5Read {
            BacklightControl5Read { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(BacklightControl5Write) -> BacklightControl5Write,
        {
            f(BacklightControl5Write { d: self }).d
        }
    }
    pub struct BacklightControl5Read<'l> {
        d: &'l BacklightControl5,
    }
    impl<'l> BacklightControl5Read<'l> {
        /// brightness_change_threshold
        #[inline(always)]
        pub fn brightness_change_threshold(&self) -> u8 {
            (self.d.data[0] >> 4) & 0x0F
        }
        /// transition_time
        #[inline(always)]
        pub fn transition_time(&self) -> TransitionTime {
            TransitionTime::from(self.d.data[0] & 0x07)
        }
    }
    pub struct BacklightControl5Write<'l> {
        d: &'l mut BacklightControl5,
    }
    impl<'l> BacklightControl5Write<'l> {
        /// brightness_change_threshold
        #[inline(always)]
        pub fn brightness_change_threshold(self, w: u8) -> Self {
            self.d.data[0] &= !(0x0F << 4);
            self.d.data[0] |= (w & 0x0F) << 4;
            self
        }
        /// transition_time
        #[inline(always)]
        pub fn transition_time(self, w: TransitionTime) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x07);
            self.d.data[0] |= w & 0x07;
            self
        }
    }
    impl Default for BacklightControl5 {
        fn default() -> Self {
            BacklightControl5 { data: [0x44] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod backlight_control7 {
    enum_with_from! {
        FpWmOut(u8) => { Pwm62745Hz = 0x00, Pwm31373Hz = 0x01, Pwm20915Hz = 0x02, Pwm15686Hz = 0x03, Pwm12549Hz = 0x04, Pwm10458Hz = 0x05, Pwm8964Hz = 0x06, Pwm7843Hz = 0x07, Pwm6972Hz = 0x08, Pwm6275Hz = 0x09, Pwm5704Hz = 0x0A, Pwm5229Hz = 0x0B, Pwm4827Hz = 0x0C, Pwm4482Hz = 0x0D, Pwm4183Hz = 0x0E, Pwm3922Hz = 0x0F, Pwm3691Hz = 0x10, Pwm3486Hz = 0x11, Pwm3302Hz = 0x12, Pwm3137Hz = 0x13, Pwm2988Hz = 0x14, Pwm2852Hz = 0x15, Pwm2728Hz = 0x16, Pwm2614Hz = 0x17, Pwm2510Hz = 0x18, Pwm2413Hz = 0x19, Pwm2324Hz = 0x1A, Pwm2241Hz = 0x1B, Pwm2164Hz = 0x1C, Pwm2092Hz = 0x1D, Pwm2024Hz = 0x1E, Pwm1961Hz = 0x1F, Pwm1901Hz = 0x20, Pwm1845Hz = 0x21, Pwm1793Hz = 0x22, Pwm1743Hz = 0x23, Pwm1696Hz = 0x24, Pwm1651Hz = 0x25, Pwm1609Hz = 0x26, Pwm1569Hz = 0x27, Pwm1530Hz = 0x28, Pwm1494Hz = 0x29, Pwm1459Hz = 0x2A, Pwm1426Hz = 0x2B, Pwm1394Hz = 0x2C, Pwm1364Hz = 0x2D, Pwm1335Hz = 0x2E, Pwm1307Hz = 0x2F, Pwm1281Hz = 0x30, Pwm1255Hz = 0x31, Pwm1230Hz = 0x32, Pwm1207Hz = 0x33, Pwm1184Hz = 0x34, Pwm1162Hz = 0x35, Pwm1141Hz = 0x36, Pwm1120Hz = 0x37, Pwm1101Hz = 0x38, Pwm1082Hz = 0x39, Pwm1063Hz = 0x3A, Pwm1046Hz = 0x3B, Pwm1029Hz = 0x3C, Pwm1012Hz = 0x3D, Pwm996Hz = 0x3E, Pwm980Hz = 0x3F, Pwm965Hz = 0x40, Pwm951Hz = 0x41, Pwm936Hz = 0x42, Pwm923Hz = 0x43, Pwm909Hz = 0x44, Pwm896Hz = 0x45, Pwm884Hz = 0x46, Pwm871Hz = 0x47, Pwm860Hz = 0x48, Pwm848Hz = 0x49, Pwm837Hz = 0x4A, Pwm826Hz = 0x4B, Pwm815Hz = 0x4C, Pwm804Hz = 0x4D, Pwm794Hz = 0x4E, Pwm784Hz = 0x4F, Pwm775Hz = 0x50, Pwm765Hz = 0x51, Pwm756Hz = 0x52, Pwm747Hz = 0x53, Pwm738Hz = 0x54, Pwm730Hz = 0x55, Pwm721Hz = 0x56, Pwm713Hz = 0x57, Pwm705Hz = 0x58, Pwm697Hz = 0x59, Pwm690Hz = 0x5A, Pwm682Hz = 0x5B, Pwm675Hz = 0x5C, Pwm668Hz = 0x5D, Pwm660Hz = 0x5E, Pwm654Hz = 0x5F, Pwm647Hz = 0x60, Pwm640Hz = 0x61, Pwm634Hz = 0x62, Pwm627Hz = 0x63, Pwm621Hz = 0x64, Pwm615Hz = 0x65, Pwm609Hz = 0x66, Pwm603Hz = 0x67, Pwm598Hz = 0x68, Pwm592Hz = 0x69, Pwm586Hz = 0x6A, Pwm581Hz = 0x6B, Pwm576Hz = 0x6C, Pwm570Hz = 0x6D, Pwm565Hz = 0x6E, Pwm560Hz = 0x6F, Pwm555Hz = 0x70, Pwm550Hz = 0x71, Pwm546Hz = 0x72, Pwm541Hz = 0x73, Pwm536Hz = 0x74, Pwm532Hz = 0x75, Pwm527Hz = 0x76, Pwm523Hz = 0x77, Pwm519Hz = 0x78, Pwm514Hz = 0x79, Pwm510Hz = 0x7A, Pwm506Hz = 0x7B, Pwm502Hz = 0x7C, Pwm498Hz = 0x7D, Pwm494Hz = 0x7E, Pwm490Hz = 0x7F, Pwm486Hz = 0x80, Pwm483Hz = 0x81, Pwm479Hz = 0x82, Pwm475Hz = 0x83, Pwm472Hz = 0x84, Pwm468Hz = 0x85, Pwm465Hz = 0x86, Pwm461Hz = 0x87, Pwm458Hz = 0x88, Pwm455Hz = 0x89, Pwm451Hz = 0x8A, Pwm448Hz = 0x8B, Pwm445Hz = 0x8C, Pwm442Hz = 0x8D, Pwm439Hz = 0x8E, Pwm436Hz = 0x8F, Pwm433Hz = 0x90, Pwm430Hz = 0x91, Pwm427Hz = 0x92, Pwm424Hz = 0x93, Pwm421Hz = 0x94, Pwm418Hz = 0x95, Pwm416Hz = 0x96, Pwm413Hz = 0x97, Pwm410Hz = 0x98, Pwm407Hz = 0x99, Pwm405Hz = 0x9A, Pwm402Hz = 0x9B, Pwm400Hz = 0x9C, Pwm397Hz = 0x9D, Pwm395Hz = 0x9E, Pwm392Hz = 0x9F, Pwm390Hz = 0xA0, Pwm387Hz = 0xA1, Pwm385Hz = 0xA2, Pwm383Hz = 0xA3, Pwm380Hz = 0xA4, Pwm378Hz = 0xA5, Pwm376Hz = 0xA6, Pwm373Hz = 0xA7, Pwm371Hz = 0xA8, Pwm369Hz = 0xA9, Pwm367Hz = 0xAA, Pwm365Hz = 0xAB, Pwm363Hz = 0xAC, Pwm361Hz = 0xAD, Pwm359Hz = 0xAE, Pwm357Hz = 0xAF, Pwm354Hz = 0xB0, Pwm353Hz = 0xB1, Pwm351Hz = 0xB2, Pwm349Hz = 0xB3, Pwm347Hz = 0xB4, Pwm345Hz = 0xB5, Pwm343Hz = 0xB6, Pwm341Hz = 0xB7, Pwm339Hz = 0xB8, Pwm337Hz = 0xB9, Pwm336Hz = 0xBA, Pwm334Hz = 0xBB, Pwm332Hz = 0xBC, Pwm330Hz = 0xBD, Pwm329Hz = 0xBE, Pwm327Hz = 0xBF, Pwm325Hz = 0xC0, Pwm323Hz = 0xC1, Pwm322Hz = 0xC2, Pwm320Hz = 0xC3, Pwm319Hz = 0xC4, Pwm317Hz = 0xC5, Pwm315Hz = 0xC6, Pwm314Hz = 0xC7, Pwm312Hz = 0xC8, Pwm311Hz = 0xC9, Pwm309Hz = 0xCA, Pwm308Hz = 0xCB, Pwm306Hz = 0xCC, Pwm305Hz = 0xCD, Pwm303Hz = 0xCE, Pwm302Hz = 0xCF, Pwm300Hz = 0xD0, Pwm299Hz = 0xD1, Pwm297Hz = 0xD2, Pwm296Hz = 0xD3, Pwm295Hz = 0xD4, Pwm293Hz = 0xD5, Pwm292Hz = 0xD6, Pwm290Hz = 0xD7, Pwm289Hz = 0xD8, Pwm288Hz = 0xD9, Pwm287Hz = 0xDA, Pwm285Hz = 0xDB, Pwm284Hz = 0xDC, Pwm283Hz = 0xDD, Pwm281Hz = 0xDE, Pwm280Hz = 0xDF, Pwm279Hz = 0xE0, Pwm278Hz = 0xE1, Pwm276Hz = 0xE2, Pwm275Hz = 0xE3, Pwm274Hz = 0xE4, Pwm273Hz = 0xE5, Pwm272Hz = 0xE6, Pwm270Hz = 0xE7, Pwm269Hz = 0xE8, Pwm268Hz = 0xE9, Pwm267Hz = 0xEA, Pwm266Hz = 0xEB, Pwm265Hz = 0xEC, Pwm264Hz = 0xED, Pwm263Hz = 0xEE, Pwm261Hz = 0xEF, Pwm260Hz = 0xF0, Pwm259Hz = 0xF1, Pwm258Hz = 0xF2, Pwm257Hz = 0xF3, Pwm256Hz = 0xF4, Pwm255Hz = 0xF5, Pwm254Hz = 0xF6, Pwm253Hz = 0xF7, Pwm252Hz = 0xF8, Pwm251Hz = 0xF9, Pwm250Hz = 0xFA, Pwm249Hz = 0xFB, Pwm248Hz = 0xFC, Pwm247Hz = 0xFD, Pwm246Hz = 0xFE, Pwm245Hz = 0xFF },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct BacklightControl7 {
        pub(super) data: [u8; 1],
    }
    impl BacklightControl7 {
        pub fn read(&self) -> BacklightControl7Read {
            BacklightControl7Read { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(BacklightControl7Write) -> BacklightControl7Write,
        {
            f(BacklightControl7Write { d: self }).d
        }
    }
    pub struct BacklightControl7Read<'l> {
        d: &'l BacklightControl7,
    }
    impl<'l> BacklightControl7Read<'l> {
        /// fp_wm_out
        #[inline(always)]
        pub fn fp_wm_out(&self) -> FpWmOut {
            FpWmOut::from(self.d.data[0])
        }
    }
    pub struct BacklightControl7Write<'l> {
        d: &'l mut BacklightControl7,
    }
    impl<'l> BacklightControl7Write<'l> {
        /// fp_wm_out
        #[inline(always)]
        pub fn fp_wm_out(self, w: FpWmOut) -> Self {
            let w = w as u8;
            self.d.data[0] = w;
            self
        }
    }
    impl Default for BacklightControl7 {
        fn default() -> Self {
            BacklightControl7 { data: [0x0F] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod backlight_control8 {
    enum_with_from! {
        Polarity(u8) => { Low = 0x00, High = 0x01 },
        LedonPin(u8) => { Ledonr = 0x00, InversedLedonr = 0x01 },
        LedpwmPin(u8) => { OriginalPolarityOfPwmSignal = 0x00, InversedPolarityOfPwmSignal = 0x01 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct BacklightControl8 {
        pub(super) data: [u8; 1],
    }
    impl BacklightControl8 {
        pub fn read(&self) -> BacklightControl8Read {
            BacklightControl8Read { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(BacklightControl8Write) -> BacklightControl8Write,
        {
            f(BacklightControl8Write { d: self }).d
        }
    }
    pub struct BacklightControl8Read<'l> {
        d: &'l BacklightControl8,
    }
    impl<'l> BacklightControl8Read<'l> {
        /// polarity
        #[inline(always)]
        pub fn polarity(&self) -> Polarity {
            Polarity::from((self.d.data[0] >> 2) & 0x01)
        }
        /// ledon_pin
        #[inline(always)]
        pub fn ledon_pin(&self) -> LedonPin {
            LedonPin::from((self.d.data[0] >> 1) & 0x01)
        }
        /// ledpwm_pin
        #[inline(always)]
        pub fn ledpwm_pin(&self) -> LedpwmPin {
            LedpwmPin::from(self.d.data[0] & 0x01)
        }
    }
    pub struct BacklightControl8Write<'l> {
        d: &'l mut BacklightControl8,
    }
    impl<'l> BacklightControl8Write<'l> {
        /// polarity
        #[inline(always)]
        pub fn polarity(self, w: Polarity) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 2);
            self.d.data[0] |= (w & 0x01) << 2;
            self
        }
        /// ledon_pin
        #[inline(always)]
        pub fn ledon_pin(self, w: LedonPin) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01 << 1);
            self.d.data[0] |= (w & 0x01) << 1;
            self
        }
        /// ledpwm_pin
        #[inline(always)]
        pub fn ledpwm_pin(self, w: LedpwmPin) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x01);
            self.d.data[0] |= w & 0x01;
            self
        }
    }
    impl Default for BacklightControl8 {
        fn default() -> Self {
            BacklightControl8 { data: [0x00] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod power_control1 {
    enum_with_from! {
        Gvdd(u8) => { N3o00V = 0x03, N3o05V = 0x04, N3o10V = 0x05, N3o15V = 0x06, N3o20V = 0x07, N3o25V = 0x08, N3o30V = 0x09, N3o35V = 0x0A, N3o40V = 0x0B, N3o45V = 0x0C, N3o50V = 0x0D, N3o55V = 0x0E, N3o60V = 0x0F, N3o65V = 0x10, N3o70V = 0x11, N3o75V = 0x12, N3o80V = 0x13, N3o85V = 0x14, N3o90V = 0x15, N3o95V = 0x16, N4o00V = 0x17, N4o05V = 0x18, N4o10V = 0x19, N4o15V = 0x1A, N4o20V = 0x1B, N4o25V = 0x1C, N4o30V = 0x1D, N4o35V = 0x1E, N4o40V = 0x1F, N4o45V = 0x20, N4o50V = 0x21, N4o55V = 0x22, N4o60V = 0x23, N4o65V = 0x24, N4o70V = 0x25, N4o75V = 0x26, N4o80V = 0x27, N4o85V = 0x28, N4o90V = 0x29, N4o95V = 0x2A, N5o00V = 0x2B, N5o05V = 0x2C, N5o10V = 0x2D, N5o15V = 0x2E, N5o20V = 0x2F, N5o25V = 0x30, N5o30V = 0x31, N5o35V = 0x32, N5o40V = 0x33, N5o45V = 0x34, N5o50V = 0x35, N5o55V = 0x36, N5o60V = 0x37, N5o65V = 0x38, N5o70V = 0x39, N5o75V = 0x3A, N5o80V = 0x3B, N5o85V = 0x3C, N5o90V = 0x3D, N5o95V = 0x3E, N6o00V = 0x3F },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct PowerControl1 {
        pub(super) data: [u8; 1],
    }
    impl PowerControl1 {
        pub fn read(&self) -> PowerControl1Read {
            PowerControl1Read { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(PowerControl1Write) -> PowerControl1Write,
        {
            f(PowerControl1Write { d: self }).d
        }
    }
    pub struct PowerControl1Read<'l> {
        d: &'l PowerControl1,
    }
    impl<'l> PowerControl1Read<'l> {
        /// gvdd
        #[inline(always)]
        pub fn gvdd(&self) -> Gvdd {
            Gvdd::from(self.d.data[0] & 0x3F)
        }
    }
    pub struct PowerControl1Write<'l> {
        d: &'l mut PowerControl1,
    }
    impl<'l> PowerControl1Write<'l> {
        /// gvdd
        #[inline(always)]
        pub fn gvdd(self, w: Gvdd) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x3F);
            self.d.data[0] |= w & 0x3F;
            self
        }
    }
    impl Default for PowerControl1 {
        fn default() -> Self {
            PowerControl1 { data: [0x21] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod power_control2 {
    enum_with_from! {
        Avdd(u8) => { VciX2VciX7VciX4 = 0x00, VciX2VciX7VciX3 = 0x01, VciX2VciX6VciX4 = 0x02, VciX2VciX6VciX3 = 0x03 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct PowerControl2 {
        pub(super) data: [u8; 1],
    }
    impl PowerControl2 {
        pub fn read(&self) -> PowerControl2Read {
            PowerControl2Read { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(PowerControl2Write) -> PowerControl2Write,
        {
            f(PowerControl2Write { d: self }).d
        }
    }
    pub struct PowerControl2Read<'l> {
        d: &'l PowerControl2,
    }
    impl<'l> PowerControl2Read<'l> {
        /// avdd
        #[inline(always)]
        pub fn avdd(&self) -> Avdd {
            Avdd::from(self.d.data[0] & 0x07)
        }
    }
    pub struct PowerControl2Write<'l> {
        d: &'l mut PowerControl2,
    }
    impl<'l> PowerControl2Write<'l> {
        /// avdd
        #[inline(always)]
        pub fn avdd(self, w: Avdd) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x07);
            self.d.data[0] |= w & 0x07;
            self
        }
    }
    impl Default for PowerControl2 {
        fn default() -> Self {
            PowerControl2 { data: [0x10] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod vcom_control1 {
    enum_with_from! {
        VcomhV(u8) => { N2o700 = 0x00, N2o725 = 0x01, N2o750 = 0x02, N2o775 = 0x03, N2o800 = 0x04, N2o825 = 0x05, N2o850 = 0x06, N2o875 = 0x07, N2o900 = 0x08, N2o925 = 0x09, N2o950 = 0x0A, N2o975 = 0x0B, N3o000 = 0x0C, N3o025 = 0x0D, N3o050 = 0x0E, N3o075 = 0x0F, N3o100 = 0x10, N3o125 = 0x11, N3o150 = 0x12, N3o175 = 0x13, N3o200 = 0x14, N3o225 = 0x15, N3o250 = 0x16, N3o275 = 0x17, N3o300 = 0x18, N3o325 = 0x19, N3o350 = 0x1A, N3o375 = 0x1B, N3o400 = 0x1C, N3o425 = 0x1D, N3o450 = 0x1E, N3o475 = 0x1F, N3o500 = 0x20, N3o525 = 0x21, N3o550 = 0x22, N3o575 = 0x23, N3o600 = 0x24, N3o625 = 0x25, N3o650 = 0x26, N3o675 = 0x27, N3o700 = 0x28, N3o725 = 0x29, N3o750 = 0x2A, N3o775 = 0x2B, N3o800 = 0x2C, N3o825 = 0x2D, N3o850 = 0x2E, N3o875 = 0x2F, N3o900 = 0x30, N3o925 = 0x31, N3o950 = 0x32, N3o975 = 0x33, N4o000 = 0x34, N4o025 = 0x35, N4o050 = 0x36, N4o075 = 0x37, N4o100 = 0x38, N4o125 = 0x39, N4o150 = 0x3A, N4o175 = 0x3B, N4o200 = 0x3C, N4o225 = 0x3D, N4o250 = 0x3E, N4o275 = 0x3F, N4o300 = 0x40, N4o325 = 0x41, N4o350 = 0x42, N4o375 = 0x43, N4o400 = 0x44, N4o425 = 0x45, N4o450 = 0x46, N4o475 = 0x47, N4o500 = 0x48, N4o525 = 0x49, N4o550 = 0x4A, N4o575 = 0x4B, N4o600 = 0x4C, N4o625 = 0x4D, N4o650 = 0x4E, N4o675 = 0x4F, N4o700 = 0x50, N4o725 = 0x51, N4o750 = 0x52, N4o775 = 0x53, N4o800 = 0x54, N4o825 = 0x55, N4o850 = 0x56, N4o875 = 0x57, N4o900 = 0x58, N4o925 = 0x59, N4o950 = 0x5A, N4o975 = 0x5B, N5o000 = 0x5C, N5o025 = 0x5D, N5o050 = 0x5E, N5o075 = 0x5F, N5o100 = 0x60, N5o125 = 0x61, N5o150 = 0x62, N5o175 = 0x63, N5o200 = 0x64, N5o225 = 0x65, N5o250 = 0x66, N5o275 = 0x67, N5o300 = 0x68, N5o325 = 0x69, N5o350 = 0x6A, N5o375 = 0x6B, N5o400 = 0x6C, N5o425 = 0x6D, N5o450 = 0x6E, N5o475 = 0x6F, N5o500 = 0x70, N5o525 = 0x71, N5o550 = 0x72, N5o575 = 0x73, N5o600 = 0x74, N5o625 = 0x75, N5o650 = 0x76, N5o675 = 0x77, N5o700 = 0x78, N5o725 = 0x79, N5o750 = 0x7A, N5o775 = 0x7B, N5o800 = 0x7C, N5o825 = 0x7D, N5o850 = 0x7E, N5o875 = 0x7F },
        VcomlV(u8) => { NNeg2o500 = 0x00, NNeg2o475 = 0x01, NNeg2o450 = 0x02, NNeg2o425 = 0x03, NNeg2o400 = 0x04, NNeg2o375 = 0x05, NNeg2o350 = 0x06, NNeg2o325 = 0x07, NNeg2o300 = 0x08, NNeg2o275 = 0x09, NNeg2o250 = 0x0A, NNeg2o225 = 0x0B, NNeg2o200 = 0x0C, NNeg2o175 = 0x0D, NNeg2o150 = 0x0E, NNeg2o125 = 0x0F, NNeg2o100 = 0x10, NNeg2o075 = 0x11, NNeg2o050 = 0x12, NNeg2o025 = 0x13, NNeg2o000 = 0x14, NNeg1o975 = 0x15, NNeg1o950 = 0x16, NNeg1o925 = 0x17, NNeg1o900 = 0x18, NNeg1o875 = 0x19, NNeg1o850 = 0x1A, NNeg1o825 = 0x1B, NNeg1o800 = 0x1C, NNeg1o775 = 0x1D, NNeg1o750 = 0x1E, NNeg1o725 = 0x1F, NNeg1o700 = 0x20, NNeg1o675 = 0x21, NNeg1o650 = 0x22, NNeg1o625 = 0x23, NNeg1o600 = 0x24, NNeg1o575 = 0x25, NNeg1o550 = 0x26, NNeg1o525 = 0x27, NNeg1o500 = 0x28, NNeg1o475 = 0x29, NNeg1o450 = 0x2A, NNeg1o425 = 0x2B, NNeg1o400 = 0x2C, NNeg1o375 = 0x2D, NNeg1o350 = 0x2E, NNeg1o325 = 0x2F, NNeg1o300 = 0x30, NNeg1o275 = 0x31, NNeg1o250 = 0x32, NNeg1o225 = 0x33, NNeg1o200 = 0x34, NNeg1o175 = 0x35, NNeg1o150 = 0x36, NNeg1o125 = 0x37, NNeg1o100 = 0x38, NNeg1o075 = 0x39, NNeg1o050 = 0x3A, NNeg1o025 = 0x3B, NNeg1o000 = 0x3C, NNeg0o975 = 0x3D, NNeg0o950 = 0x3E, NNeg0o925 = 0x3F, NNeg0o900 = 0x40, NNeg0o875 = 0x41, NNeg0o850 = 0x42, NNeg0o825 = 0x43, NNeg0o800 = 0x44, NNeg0o775 = 0x45, NNeg0o750 = 0x46, NNeg0o725 = 0x47, NNeg0o700 = 0x48, NNeg0o675 = 0x49, NNeg0o650 = 0x4A, NNeg0o625 = 0x4B, NNeg0o600 = 0x4C, NNeg0o575 = 0x4D, NNeg0o550 = 0x4E, NNeg0o525 = 0x4F, NNeg0o500 = 0x50, NNeg0o475 = 0x51, NNeg0o450 = 0x52, NNeg0o425 = 0x53, NNeg0o400 = 0x54, NNeg0o375 = 0x55, NNeg0o350 = 0x56, NNeg0o325 = 0x57, NNeg0o300 = 0x58, NNeg0o275 = 0x59, NNeg0o250 = 0x5A, NNeg0o225 = 0x5B, NNeg0o200 = 0x5C, NNeg0o175 = 0x5D, NNeg0o150 = 0x5E, NNeg0o125 = 0x5F, NNeg0o100 = 0x60, NNeg0o075 = 0x61, NNeg0o050 = 0x62, NNeg0o025 = 0x63, N0 = 0x64 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct VcomControl1 {
        pub(super) data: [u8; 2],
    }
    impl VcomControl1 {
        pub fn read(&self) -> VcomControl1Read {
            VcomControl1Read { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(VcomControl1Write) -> VcomControl1Write,
        {
            f(VcomControl1Write { d: self }).d
        }
    }
    pub struct VcomControl1Read<'l> {
        d: &'l VcomControl1,
    }
    impl<'l> VcomControl1Read<'l> {
        /// vcomh_v
        #[inline(always)]
        pub fn vcomh_v(&self) -> VcomhV {
            VcomhV::from(self.d.data[0] & 0x7F)
        }
        /// vcoml_v
        #[inline(always)]
        pub fn vcoml_v(&self) -> VcomlV {
            VcomlV::from(self.d.data[1] & 0x7F)
        }
    }
    pub struct VcomControl1Write<'l> {
        d: &'l mut VcomControl1,
    }
    impl<'l> VcomControl1Write<'l> {
        /// vcomh_v
        #[inline(always)]
        pub fn vcomh_v(self, w: VcomhV) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x7F);
            self.d.data[0] |= w & 0x7F;
            self
        }
        /// vcoml_v
        #[inline(always)]
        pub fn vcoml_v(self, w: VcomlV) -> Self {
            let w = w as u8;
            self.d.data[1] &= !(0x7F);
            self.d.data[1] |= w & 0x7F;
            self
        }
    }
    impl Default for VcomControl1 {
        fn default() -> Self {
            VcomControl1 { data: [0x31, 0x3C] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod vcom_control2 {
    #[derive(Copy, Clone, Debug)]
    pub struct VcomControl2 {
        pub(super) data: [u8; 1],
    }
    impl VcomControl2 {
        pub fn read(&self) -> VcomControl2Read {
            VcomControl2Read { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(VcomControl2Write) -> VcomControl2Write,
        {
            f(VcomControl2Write { d: self }).d
        }
    }
    pub struct VcomControl2Read<'l> {
        d: &'l VcomControl2,
    }
    impl<'l> VcomControl2Read<'l> {
        /// vcom_setting_source
        #[inline(always)]
        pub fn vcom_setting_source(&self) -> bool {
            ((self.d.data[0] >> 7) & 0x01) != 0
        }
        /// vcomh
        #[inline(always)]
        pub fn vcomh(&self) -> i8 {
            (((self.d.data[0] & 0x7F) - 1) as i8) + -63
        }
    }
    pub struct VcomControl2Write<'l> {
        d: &'l mut VcomControl2,
    }
    impl<'l> VcomControl2Write<'l> {
        /// vcom_setting_source
        #[inline(always)]
        pub fn vcom_setting_source(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 7);
            self.d.data[0] |= ((w) as u8) << 7;
            self
        }
        /// vcomh
        #[inline(always)]
        pub fn vcomh(self, w: i8) -> Self {
            let w = ((w - -63) as u8) + 1;
            self.d.data[0] &= !(0x7F);
            self.d.data[0] |= w & 0x7F;
            self
        }
    }
    impl Default for VcomControl2 {
        fn default() -> Self {
            VcomControl2 { data: [0xC0] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod nv_memory_write {
    enum_with_from! {
        ProgrammedNvMemorySelection(u8) => { Id1Programming = 0x00, Id2Programming = 0x01, Id3Programming = 0x02, Vmf60Programming = 0x04 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct NvMemory {
        pub(super) data: [u8; 2],
    }
    impl NvMemory {
        pub fn read(&self) -> NvMemoryRead {
            NvMemoryRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(NvMemoryWrite) -> NvMemoryWrite,
        {
            f(NvMemoryWrite { d: self }).d
        }
    }
    pub struct NvMemoryRead<'l> {
        d: &'l NvMemory,
    }
    impl<'l> NvMemoryRead<'l> {
        /// programmed_nv_memory_selection
        #[inline(always)]
        pub fn programmed_nv_memory_selection(&self) -> ProgrammedNvMemorySelection {
            ProgrammedNvMemorySelection::from(self.d.data[0] & 0x07)
        }
        /// the_programmed_data
        #[inline(always)]
        pub fn the_programmed_data(&self) -> u8 {
            self.d.data[1]
        }
    }
    pub struct NvMemoryWrite<'l> {
        d: &'l mut NvMemory,
    }
    impl<'l> NvMemoryWrite<'l> {
        /// programmed_nv_memory_selection
        #[inline(always)]
        pub fn programmed_nv_memory_selection(self, w: ProgrammedNvMemorySelection) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x07);
            self.d.data[0] |= w & 0x07;
            self
        }
        /// the_programmed_data
        #[inline(always)]
        pub fn the_programmed_data(self, w: u8) -> Self {
            self.d.data[1] = w;
            self
        }
    }
    impl Default for NvMemory {
        fn default() -> Self {
            NvMemory { data: [0x00, 0x00] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod nv_memory_protection_key {
    #[derive(Copy, Clone, Debug)]
    pub struct NvMemoryProtectionKey {
        pub(super) data: [u8; 3],
    }
    impl NvMemoryProtectionKey {
        pub fn read(&self) -> NvMemoryProtectionKeyRead {
            NvMemoryProtectionKeyRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(NvMemoryProtectionKeyWrite) -> NvMemoryProtectionKeyWrite,
        {
            f(NvMemoryProtectionKeyWrite { d: self }).d
        }
    }
    pub struct NvMemoryProtectionKeyRead<'l> {
        d: &'l NvMemoryProtectionKey,
    }
    impl<'l> NvMemoryProtectionKeyRead<'l> {
        /// nv_memory_programming_protection_key
        #[inline(always)]
        pub fn nv_memory_programming_protection_key(&self) -> u32 {
            ((self.d.data[0] as u32) << 16)
                | ((self.d.data[1] as u32) << 8)
                | (self.d.data[2] as u32)
        }
    }
    pub struct NvMemoryProtectionKeyWrite<'l> {
        d: &'l mut NvMemoryProtectionKey,
    }
    impl<'l> NvMemoryProtectionKeyWrite<'l> {
        /// nv_memory_programming_protection_key
        #[inline(always)]
        pub fn nv_memory_programming_protection_key(self, w: u32) -> Self {
            self.d.data[0] = (w >> 16) as u8;
            self.d.data[1] = (w >> 8) as u8;
            self.d.data[2] = w as u8;
            self
        }
    }
    impl Default for NvMemoryProtectionKey {
        fn default() -> Self {
            NvMemoryProtectionKey {
                data: [0x55, 0xAA, 0x66],
            }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod nv_memory_status_read {
    enum_with_from! {
        Id2WriteCount(u8) => { NoProgrammed = 0x00, Programmed1Time = 0x01, Programmed2Times = 0x03, Programmed3Times = 0x07 },
        Id1WriteCount(u8) => { NoProgrammed = 0x00, Programmed1Time = 0x01, Programmed2Times = 0x03, Programmed3Times = 0x07 },
        TheStatusOfNvMemory(u8) => { Busy = 0x01 },
        VmfWriteCount(u8) => { NoProgrammed = 0x00, Programmed1Time = 0x01, Programmed2Times = 0x03, Programmed3Times = 0x07 },
        Id3WriteCount(u8) => { NoProgrammed = 0x00, Programmed1Time = 0x01, Programmed2Times = 0x03, Programmed3Times = 0x07 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct NvMemoryStatus {
        pub(super) data: [u8; 2],
    }
    impl NvMemoryStatus {
        pub fn read(&self) -> NvMemoryStatusRead {
            NvMemoryStatusRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(NvMemoryStatusWrite) -> NvMemoryStatusWrite,
        {
            f(NvMemoryStatusWrite { d: self }).d
        }
    }
    pub struct NvMemoryStatusRead<'l> {
        d: &'l NvMemoryStatus,
    }
    impl<'l> NvMemoryStatusRead<'l> {
        /// id2_write_count
        #[inline(always)]
        pub fn id2_write_count(&self) -> Id2WriteCount {
            Id2WriteCount::from((self.d.data[0] >> 4) & 0x07)
        }
        /// id1_write_count
        #[inline(always)]
        pub fn id1_write_count(&self) -> Id1WriteCount {
            Id1WriteCount::from(self.d.data[0] & 0x07)
        }
        /// the_status_of_nv_memory
        #[inline(always)]
        pub fn the_status_of_nv_memory(&self) -> TheStatusOfNvMemory {
            TheStatusOfNvMemory::from((self.d.data[1] >> 7) & 0x01)
        }
        /// vmf_write_count
        #[inline(always)]
        pub fn vmf_write_count(&self) -> VmfWriteCount {
            VmfWriteCount::from((self.d.data[1] >> 4) & 0x07)
        }
        /// id3_write_count
        #[inline(always)]
        pub fn id3_write_count(&self) -> Id3WriteCount {
            Id3WriteCount::from(self.d.data[1] & 0x07)
        }
    }
    pub struct NvMemoryStatusWrite<'l> {
        d: &'l mut NvMemoryStatus,
    }
    impl<'l> NvMemoryStatusWrite<'l> {
        /// id2_write_count
        #[inline(always)]
        pub fn id2_write_count(self, w: Id2WriteCount) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x07 << 4);
            self.d.data[0] |= (w & 0x07) << 4;
            self
        }
        /// id1_write_count
        #[inline(always)]
        pub fn id1_write_count(self, w: Id1WriteCount) -> Self {
            let w = w as u8;
            self.d.data[0] &= !(0x07);
            self.d.data[0] |= w & 0x07;
            self
        }
        /// the_status_of_nv_memory
        #[inline(always)]
        pub fn the_status_of_nv_memory(self, w: TheStatusOfNvMemory) -> Self {
            let w = w as u8;
            self.d.data[1] &= !(0x01 << 7);
            self.d.data[1] |= (w & 0x01) << 7;
            self
        }
        /// vmf_write_count
        #[inline(always)]
        pub fn vmf_write_count(self, w: VmfWriteCount) -> Self {
            let w = w as u8;
            self.d.data[1] &= !(0x07 << 4);
            self.d.data[1] |= (w & 0x07) << 4;
            self
        }
        /// id3_write_count
        #[inline(always)]
        pub fn id3_write_count(self, w: Id3WriteCount) -> Self {
            let w = w as u8;
            self.d.data[1] &= !(0x07);
            self.d.data[1] |= w & 0x07;
            self
        }
    }
    impl Default for NvMemoryStatus {
        fn default() -> Self {
            NvMemoryStatus { data: [0x00, 0x00] }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod read_id4 {
    #[derive(Copy, Clone, Debug)]
    pub struct Id4 {
        pub(super) data: [u8; 3],
    }
    impl Id4 {
        pub fn read(&self) -> Id4Read {
            Id4Read { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(Id4Write) -> Id4Write,
        {
            f(Id4Write { d: self }).d
        }
    }
    pub struct Id4Read<'l> {
        d: &'l Id4,
    }
    impl<'l> Id4Read<'l> {}
    pub struct Id4Write<'l> {
        d: &'l mut Id4,
    }
    impl<'l> Id4Write<'l> {}
    impl Default for Id4 {
        fn default() -> Self {
            Id4 {
                data: [0x00, 0x93, 0x41],
            }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod positive_gamma_correction {
    #[derive(Copy, Clone, Debug)]
    pub struct PositiveGammaCorrection {
        pub(super) data: [u8; 15],
    }
    impl PositiveGammaCorrection {
        pub fn read(&self) -> PositiveGammaCorrectionRead {
            PositiveGammaCorrectionRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(PositiveGammaCorrectionWrite) -> PositiveGammaCorrectionWrite,
        {
            f(PositiveGammaCorrectionWrite { d: self }).d
        }
    }
    pub struct PositiveGammaCorrectionRead<'l> {
        d: &'l PositiveGammaCorrection,
    }
    impl<'l> PositiveGammaCorrectionRead<'l> {
        /// vp63
        #[inline(always)]
        pub fn vp63(&self) -> u8 {
            self.d.data[0] & 0x0F
        }
        /// vp62
        #[inline(always)]
        pub fn vp62(&self) -> u8 {
            self.d.data[1] & 0x3F
        }
        /// vp61
        #[inline(always)]
        pub fn vp61(&self) -> u8 {
            self.d.data[2] & 0x3F
        }
        /// vp59
        #[inline(always)]
        pub fn vp59(&self) -> u8 {
            self.d.data[3] & 0x0F
        }
        /// vp57
        #[inline(always)]
        pub fn vp57(&self) -> u8 {
            self.d.data[4] & 0x1F
        }
        /// vp50
        #[inline(always)]
        pub fn vp50(&self) -> u8 {
            self.d.data[5] & 0x0F
        }
        /// vp43
        #[inline(always)]
        pub fn vp43(&self) -> u8 {
            self.d.data[6] & 0x7F
        }
        /// vp27
        #[inline(always)]
        pub fn vp27(&self) -> u8 {
            (self.d.data[7] >> 4) & 0x0F
        }
        /// vp36
        #[inline(always)]
        pub fn vp36(&self) -> u8 {
            self.d.data[7] & 0x0F
        }
        /// vp20
        #[inline(always)]
        pub fn vp20(&self) -> u8 {
            self.d.data[8] & 0x7F
        }
        /// vp13
        #[inline(always)]
        pub fn vp13(&self) -> u8 {
            self.d.data[9] & 0x0F
        }
        /// vp6
        #[inline(always)]
        pub fn vp6(&self) -> u8 {
            self.d.data[10] & 0x1F
        }
        /// vp4
        #[inline(always)]
        pub fn vp4(&self) -> u8 {
            self.d.data[11] & 0x0F
        }
        /// vp2
        #[inline(always)]
        pub fn vp2(&self) -> u8 {
            self.d.data[12] & 0x3F
        }
        /// vp1
        #[inline(always)]
        pub fn vp1(&self) -> u8 {
            self.d.data[13] & 0x3F
        }
        /// vp0
        #[inline(always)]
        pub fn vp0(&self) -> u8 {
            self.d.data[14] & 0x0F
        }
    }
    pub struct PositiveGammaCorrectionWrite<'l> {
        d: &'l mut PositiveGammaCorrection,
    }
    impl<'l> PositiveGammaCorrectionWrite<'l> {
        /// vp63
        #[inline(always)]
        pub fn vp63(self, w: u8) -> Self {
            self.d.data[0] &= !(0x0F);
            self.d.data[0] |= w & 0x0F;
            self
        }
        /// vp62
        #[inline(always)]
        pub fn vp62(self, w: u8) -> Self {
            self.d.data[1] &= !(0x3F);
            self.d.data[1] |= w & 0x3F;
            self
        }
        /// vp61
        #[inline(always)]
        pub fn vp61(self, w: u8) -> Self {
            self.d.data[2] &= !(0x3F);
            self.d.data[2] |= w & 0x3F;
            self
        }
        /// vp59
        #[inline(always)]
        pub fn vp59(self, w: u8) -> Self {
            self.d.data[3] &= !(0x0F);
            self.d.data[3] |= w & 0x0F;
            self
        }
        /// vp57
        #[inline(always)]
        pub fn vp57(self, w: u8) -> Self {
            self.d.data[4] &= !(0x1F);
            self.d.data[4] |= w & 0x1F;
            self
        }
        /// vp50
        #[inline(always)]
        pub fn vp50(self, w: u8) -> Self {
            self.d.data[5] &= !(0x0F);
            self.d.data[5] |= w & 0x0F;
            self
        }
        /// vp43
        #[inline(always)]
        pub fn vp43(self, w: u8) -> Self {
            self.d.data[6] &= !(0x7F);
            self.d.data[6] |= w & 0x7F;
            self
        }
        /// vp27
        #[inline(always)]
        pub fn vp27(self, w: u8) -> Self {
            self.d.data[7] &= !(0x0F << 4);
            self.d.data[7] |= (w & 0x0F) << 4;
            self
        }
        /// vp36
        #[inline(always)]
        pub fn vp36(self, w: u8) -> Self {
            self.d.data[7] &= !(0x0F);
            self.d.data[7] |= w & 0x0F;
            self
        }
        /// vp20
        #[inline(always)]
        pub fn vp20(self, w: u8) -> Self {
            self.d.data[8] &= !(0x7F);
            self.d.data[8] |= w & 0x7F;
            self
        }
        /// vp13
        #[inline(always)]
        pub fn vp13(self, w: u8) -> Self {
            self.d.data[9] &= !(0x0F);
            self.d.data[9] |= w & 0x0F;
            self
        }
        /// vp6
        #[inline(always)]
        pub fn vp6(self, w: u8) -> Self {
            self.d.data[10] &= !(0x1F);
            self.d.data[10] |= w & 0x1F;
            self
        }
        /// vp4
        #[inline(always)]
        pub fn vp4(self, w: u8) -> Self {
            self.d.data[11] &= !(0x0F);
            self.d.data[11] |= w & 0x0F;
            self
        }
        /// vp2
        #[inline(always)]
        pub fn vp2(self, w: u8) -> Self {
            self.d.data[12] &= !(0x3F);
            self.d.data[12] |= w & 0x3F;
            self
        }
        /// vp1
        #[inline(always)]
        pub fn vp1(self, w: u8) -> Self {
            self.d.data[13] &= !(0x3F);
            self.d.data[13] |= w & 0x3F;
            self
        }
        /// vp0
        #[inline(always)]
        pub fn vp0(self, w: u8) -> Self {
            self.d.data[14] &= !(0x0F);
            self.d.data[14] |= w & 0x0F;
            self
        }
    }
    impl Default for PositiveGammaCorrection {
        fn default() -> Self {
            PositiveGammaCorrection {
                data: [
                    0x08, 0x00, 0x00, 0x05, 0x00, 0x09, 0x00, 0x00, 0x00, 0x0B, 0x00, 0x00, 0x00,
                    0x00, 0x00,
                ],
            }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod negative_gamma_correction {
    #[derive(Copy, Clone, Debug)]
    pub struct NegativeGammaCorrection {
        pub(super) data: [u8; 15],
    }
    impl NegativeGammaCorrection {
        pub fn read(&self) -> NegativeGammaCorrectionRead {
            NegativeGammaCorrectionRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(NegativeGammaCorrectionWrite) -> NegativeGammaCorrectionWrite,
        {
            f(NegativeGammaCorrectionWrite { d: self }).d
        }
    }
    pub struct NegativeGammaCorrectionRead<'l> {
        d: &'l NegativeGammaCorrection,
    }
    impl<'l> NegativeGammaCorrectionRead<'l> {
        /// vn63
        #[inline(always)]
        pub fn vn63(&self) -> u8 {
            self.d.data[0] & 0x0F
        }
        /// vn62
        #[inline(always)]
        pub fn vn62(&self) -> u8 {
            self.d.data[1] & 0x3F
        }
        /// vn61
        #[inline(always)]
        pub fn vn61(&self) -> u8 {
            self.d.data[2] & 0x3F
        }
        /// vn59
        #[inline(always)]
        pub fn vn59(&self) -> u8 {
            self.d.data[3] & 0x0F
        }
        /// vn57
        #[inline(always)]
        pub fn vn57(&self) -> u8 {
            self.d.data[4] & 0x1F
        }
        /// vn50
        #[inline(always)]
        pub fn vn50(&self) -> u8 {
            self.d.data[5] & 0x0F
        }
        /// vn43
        #[inline(always)]
        pub fn vn43(&self) -> u8 {
            self.d.data[6] & 0x7F
        }
        /// vn36
        #[inline(always)]
        pub fn vn36(&self) -> u8 {
            (self.d.data[7] >> 4) & 0x0F
        }
        /// vn27
        #[inline(always)]
        pub fn vn27(&self) -> u8 {
            self.d.data[7] & 0x0F
        }
        /// vn20
        #[inline(always)]
        pub fn vn20(&self) -> u8 {
            self.d.data[8] & 0x7F
        }
        /// vn13
        #[inline(always)]
        pub fn vn13(&self) -> u8 {
            self.d.data[9] & 0x0F
        }
        /// vn6
        #[inline(always)]
        pub fn vn6(&self) -> u8 {
            self.d.data[10] & 0x1F
        }
        /// vn4
        #[inline(always)]
        pub fn vn4(&self) -> u8 {
            self.d.data[11] & 0x0F
        }
        /// vn2
        #[inline(always)]
        pub fn vn2(&self) -> u8 {
            self.d.data[12] & 0x3F
        }
        /// vn1
        #[inline(always)]
        pub fn vn1(&self) -> u8 {
            self.d.data[13] & 0x3F
        }
        /// vn0
        #[inline(always)]
        pub fn vn0(&self) -> u8 {
            self.d.data[14] & 0x0F
        }
    }
    pub struct NegativeGammaCorrectionWrite<'l> {
        d: &'l mut NegativeGammaCorrection,
    }
    impl<'l> NegativeGammaCorrectionWrite<'l> {
        /// vn63
        #[inline(always)]
        pub fn vn63(self, w: u8) -> Self {
            self.d.data[0] &= !(0x0F);
            self.d.data[0] |= w & 0x0F;
            self
        }
        /// vn62
        #[inline(always)]
        pub fn vn62(self, w: u8) -> Self {
            self.d.data[1] &= !(0x3F);
            self.d.data[1] |= w & 0x3F;
            self
        }
        /// vn61
        #[inline(always)]
        pub fn vn61(self, w: u8) -> Self {
            self.d.data[2] &= !(0x3F);
            self.d.data[2] |= w & 0x3F;
            self
        }
        /// vn59
        #[inline(always)]
        pub fn vn59(self, w: u8) -> Self {
            self.d.data[3] &= !(0x0F);
            self.d.data[3] |= w & 0x0F;
            self
        }
        /// vn57
        #[inline(always)]
        pub fn vn57(self, w: u8) -> Self {
            self.d.data[4] &= !(0x1F);
            self.d.data[4] |= w & 0x1F;
            self
        }
        /// vn50
        #[inline(always)]
        pub fn vn50(self, w: u8) -> Self {
            self.d.data[5] &= !(0x0F);
            self.d.data[5] |= w & 0x0F;
            self
        }
        /// vn43
        #[inline(always)]
        pub fn vn43(self, w: u8) -> Self {
            self.d.data[6] &= !(0x7F);
            self.d.data[6] |= w & 0x7F;
            self
        }
        /// vn36
        #[inline(always)]
        pub fn vn36(self, w: u8) -> Self {
            self.d.data[7] &= !(0x0F << 4);
            self.d.data[7] |= (w & 0x0F) << 4;
            self
        }
        /// vn27
        #[inline(always)]
        pub fn vn27(self, w: u8) -> Self {
            self.d.data[7] &= !(0x0F);
            self.d.data[7] |= w & 0x0F;
            self
        }
        /// vn20
        #[inline(always)]
        pub fn vn20(self, w: u8) -> Self {
            self.d.data[8] &= !(0x7F);
            self.d.data[8] |= w & 0x7F;
            self
        }
        /// vn13
        #[inline(always)]
        pub fn vn13(self, w: u8) -> Self {
            self.d.data[9] &= !(0x0F);
            self.d.data[9] |= w & 0x0F;
            self
        }
        /// vn6
        #[inline(always)]
        pub fn vn6(self, w: u8) -> Self {
            self.d.data[10] &= !(0x1F);
            self.d.data[10] |= w & 0x1F;
            self
        }
        /// vn4
        #[inline(always)]
        pub fn vn4(self, w: u8) -> Self {
            self.d.data[11] &= !(0x0F);
            self.d.data[11] |= w & 0x0F;
            self
        }
        /// vn2
        #[inline(always)]
        pub fn vn2(self, w: u8) -> Self {
            self.d.data[12] &= !(0x3F);
            self.d.data[12] |= w & 0x3F;
            self
        }
        /// vn1
        #[inline(always)]
        pub fn vn1(self, w: u8) -> Self {
            self.d.data[13] &= !(0x3F);
            self.d.data[13] |= w & 0x3F;
            self
        }
        /// vn0
        #[inline(always)]
        pub fn vn0(self, w: u8) -> Self {
            self.d.data[14] &= !(0x0F);
            self.d.data[14] |= w & 0x0F;
            self
        }
    }
    impl Default for NegativeGammaCorrection {
        fn default() -> Self {
            NegativeGammaCorrection {
                data: [
                    0x08, 0x00, 0x00, 0x07, 0x00, 0x05, 0x00, 0x00, 0x00, 0x04, 0x00, 0x0F, 0x00,
                    0x00, 0x0F,
                ],
            }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod digital_gamma_control1 {
    #[derive(Copy, Clone, Debug)]
    pub struct DigitalGammaControl1 {
        pub(super) data: [u8; 16],
    }
    impl DigitalGammaControl1 {
        pub fn read(&self) -> DigitalGammaControl1Read {
            DigitalGammaControl1Read { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(DigitalGammaControl1Write) -> DigitalGammaControl1Write,
        {
            f(DigitalGammaControl1Write { d: self }).d
        }
    }
    pub struct DigitalGammaControl1Read<'l> {
        d: &'l DigitalGammaControl1,
    }
    impl<'l> DigitalGammaControl1Read<'l> {
        /// rca
        #[inline(always)]
        pub fn rca(&self) -> &'l [u8] {
            &self.d.data[0..16]
            // self.d.data[0..16].iter().map(|rr| (rr >> 4) & 0x0F).collect(somehow)
        }
        /// bca
        #[inline(always)]
        pub fn bca(&self) -> &'l [u8] {
            &self.d.data[0..16]
            // self.d.data[0..16].iter().map(|rr| rr & 0x0F).collect(somehow)
        }
    }
    pub struct DigitalGammaControl1Write<'l> {
        d: &'l mut DigitalGammaControl1,
    }
    impl<'l> DigitalGammaControl1Write<'l> {
        /// rca
        #[inline(always)]
        pub fn rca(self, w: &'l [u8]) -> Self {
            self.d.data[0..16]
                .iter_mut()
                .zip(w.iter())
                .for_each(|(dd, ww)| *dd = (*ww & 0x0F) << 4);
            self
        }
        /// bca
        #[inline(always)]
        pub fn bca(self, w: &'l [u8]) -> Self {
            self.d.data[0..16]
                .iter_mut()
                .zip(w.iter())
                .for_each(|(dd, ww)| *dd = *ww & 0x0F);
            self
        }
    }
    impl Default for DigitalGammaControl1 {
        fn default() -> Self {
            DigitalGammaControl1 {
                data: [
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00,
                ],
            }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod digital_gamma_control2 {
    #[derive(Copy, Clone, Debug)]
    pub struct DigitalGammaControl2 {
        pub(super) data: [u8; 64],
    }
    impl DigitalGammaControl2 {
        pub fn read(&self) -> DigitalGammaControl2Read {
            DigitalGammaControl2Read { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(DigitalGammaControl2Write) -> DigitalGammaControl2Write,
        {
            f(DigitalGammaControl2Write { d: self }).d
        }
    }
    pub struct DigitalGammaControl2Read<'l> {
        d: &'l DigitalGammaControl2,
    }
    impl<'l> DigitalGammaControl2Read<'l> {
        /// rfa
        #[inline(always)]
        pub fn rfa(&self) -> &'l [u8] {
            &self.d.data[0..64]
            // self.d.data[0..64].iter().map(|rr| (rr >> 4) & 0x0F).collect(somehow)
        }
        /// bfa
        #[inline(always)]
        pub fn bfa(&self) -> &'l [u8] {
            &self.d.data[0..64]
            // self.d.data[0..64].iter().map(|rr| rr & 0x0F).collect(somehow)
        }
    }
    pub struct DigitalGammaControl2Write<'l> {
        d: &'l mut DigitalGammaControl2,
    }
    impl<'l> DigitalGammaControl2Write<'l> {
        /// rfa
        #[inline(always)]
        pub fn rfa(self, w: &'l [u8]) -> Self {
            self.d.data[0..64]
                .iter_mut()
                .zip(w.iter())
                .for_each(|(dd, ww)| *dd = (*ww & 0x0F) << 4);
            self
        }
        /// bfa
        #[inline(always)]
        pub fn bfa(self, w: &'l [u8]) -> Self {
            self.d.data[0..64]
                .iter_mut()
                .zip(w.iter())
                .for_each(|(dd, ww)| *dd = *ww & 0x0F);
            self
        }
    }
    impl Default for DigitalGammaControl2 {
        fn default() -> Self {
            DigitalGammaControl2 {
                data: [
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
            }
        }
    }
}
#[cfg(feature = "Ili9341ExtendedCommandSet")]
pub mod interface_control {
    enum_with_from! {
        Expand16BbpRgbtO18BbpRgb(u8) => { MsbIsInputtedToLsbR50EqR40R4G50EqG50B50EqB40B4 = 0x00, N0IsInputtedToLsbR50EqR400G50EqG50B50EqB400ExceptionR40B40Eq5H1FR50B50Eq6H3F = 0x01, N1IsInputtedToLsbR50EqR401G50EqG50B50EqB401ExceptionR40B40Eq5H00R50B50Eq6H00 = 0x02, CompareR40G51B40CaseCase1REqGEqBR50EqR40G0G50EqG50B50EqB40G0Case2REqBgR50EqR40R4G50EqG50B50EqB40B0Case3REqGbR50EqR40G0G50EqG50B50EqB40B0Case4BEqGrR50EqR40R4G50EqG50B50EqB40G0 = 0x03 },
        DataTransferMode(u8) => { NormalMsbFirstDefault = 0x00, LittleEndianLsbFirst = 0x01 },
        DisplayOperationMode(u8) => { InternalClockOperation = 0x00, RgbInterfaceMode = 0x01, VsyncInterfaceMode = 0x02, SettingDisabled = 0x03 },
        InterfaceForRamAccess(u8) => { SystemInterfaceOrVsyncInterface = 0x00, RgbInterface = 0x01 },
        RgbInterfaceMode(u8) => { N16Or18BitRgbInterface1TransferPerPixel = 0x00, N6BitRgbInterface3TransferPerPixel = 0x01 },
    }
    #[derive(Copy, Clone, Debug)]
    pub struct InterfaceControl {
        pub(super) data: [u8; 3],
    }
    impl InterfaceControl {
        pub fn read(&self) -> InterfaceControlRead {
            InterfaceControlRead { d: self }
        }
        pub fn write<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(InterfaceControlWrite) -> InterfaceControlWrite,
        {
            f(InterfaceControlWrite { d: self }).d
        }
    }
    pub struct InterfaceControlRead<'l> {
        d: &'l InterfaceControl,
    }
    impl<'l> InterfaceControlRead<'l> {
        /// my_eor
        #[inline(always)]
        pub fn my_eor(&self) -> bool {
            ((self.d.data[0] >> 7) & 0x01) != 0
        }
        /// mx_eor
        #[inline(always)]
        pub fn mx_eor(&self) -> bool {
            ((self.d.data[0] >> 6) & 0x01) != 0
        }
        /// mv_eor
        #[inline(always)]
        pub fn mv_eor(&self) -> bool {
            ((self.d.data[0] >> 5) & 0x01) != 0
        }
        /// bgr_eor
        #[inline(always)]
        pub fn bgr_eor(&self) -> bool {
            ((self.d.data[0] >> 3) & 0x01) != 0
        }
        /// memory_write_control
        #[inline(always)]
        pub fn memory_write_control(&self) -> bool {
            (self.d.data[0] & 0x01) != 0
        }
        /// expand16_bbp_rgbt_o18_bbp_rgb
        #[inline(always)]
        pub fn expand16_bbp_rgbt_o18_bbp_rgb(&self) -> Expand16BbpRgbtO18BbpRgb {
            Expand16BbpRgbtO18BbpRgb::from((self.d.data[1] >> 4) & 0x03)
        }
        /// select_the_method_of_display_data_transferring
        #[inline(always)]
        pub fn select_the_method_of_display_data_transferring(&self) -> u8 {
            self.d.data[1] & 0x03
        }
        /// data_transfer_mode
        #[inline(always)]
        pub fn data_transfer_mode(&self) -> DataTransferMode {
            DataTransferMode::from((self.d.data[2] >> 5) & 0x01)
        }
        /// display_operation_mode
        #[inline(always)]
        pub fn display_operation_mode(&self) -> DisplayOperationMode {
            DisplayOperationMode::from((self.d.data[2] >> 2) & 0x03)
        }
        /// interface_for_ram_access
        #[inline(always)]
        pub fn interface_for_ram_access(&self) -> InterfaceForRamAccess {
            InterfaceForRamAccess::from((self.d.data[2] >> 1) & 0x01)
        }
        /// rgb_interface_mode
        #[inline(always)]
        pub fn rgb_interface_mode(&self) -> RgbInterfaceMode {
            RgbInterfaceMode::from(self.d.data[2] & 0x01)
        }
    }
    pub struct InterfaceControlWrite<'l> {
        d: &'l mut InterfaceControl,
    }
    impl<'l> InterfaceControlWrite<'l> {
        /// my_eor
        #[inline(always)]
        pub fn my_eor(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 7);
            self.d.data[0] |= ((w) as u8) << 7;
            self
        }
        /// mx_eor
        #[inline(always)]
        pub fn mx_eor(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 6);
            self.d.data[0] |= ((w) as u8) << 6;
            self
        }
        /// mv_eor
        #[inline(always)]
        pub fn mv_eor(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 5);
            self.d.data[0] |= ((w) as u8) << 5;
            self
        }
        /// bgr_eor
        #[inline(always)]
        pub fn bgr_eor(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01 << 3);
            self.d.data[0] |= ((w) as u8) << 3;
            self
        }
        /// memory_write_control
        #[inline(always)]
        pub fn memory_write_control(self, w: bool) -> Self {
            self.d.data[0] &= !(0x01);
            self.d.data[0] |= (w) as u8;
            self
        }
        /// expand16_bbp_rgbt_o18_bbp_rgb
        #[inline(always)]
        pub fn expand16_bbp_rgbt_o18_bbp_rgb(self, w: Expand16BbpRgbtO18BbpRgb) -> Self {
            let w = w as u8;
            self.d.data[1] &= !(0x03 << 4);
            self.d.data[1] |= (w & 0x03) << 4;
            self
        }
        /// select_the_method_of_display_data_transferring
        #[inline(always)]
        pub fn select_the_method_of_display_data_transferring(self, w: u8) -> Self {
            self.d.data[1] &= !(0x03);
            self.d.data[1] |= w & 0x03;
            self
        }
        /// data_transfer_mode
        #[inline(always)]
        pub fn data_transfer_mode(self, w: DataTransferMode) -> Self {
            let w = w as u8;
            self.d.data[2] &= !(0x01 << 5);
            self.d.data[2] |= (w & 0x01) << 5;
            self
        }
        /// display_operation_mode
        #[inline(always)]
        pub fn display_operation_mode(self, w: DisplayOperationMode) -> Self {
            let w = w as u8;
            self.d.data[2] &= !(0x03 << 2);
            self.d.data[2] |= (w & 0x03) << 2;
            self
        }
        /// interface_for_ram_access
        #[inline(always)]
        pub fn interface_for_ram_access(self, w: InterfaceForRamAccess) -> Self {
            let w = w as u8;
            self.d.data[2] &= !(0x01 << 1);
            self.d.data[2] |= (w & 0x01) << 1;
            self
        }
        /// rgb_interface_mode
        #[inline(always)]
        pub fn rgb_interface_mode(self, w: RgbInterfaceMode) -> Self {
            let w = w as u8;
            self.d.data[2] &= !(0x01);
            self.d.data[2] |= w & 0x01;
            self
        }
    }
    impl Default for InterfaceControl {
        fn default() -> Self {
            InterfaceControl {
                data: [0x01, 0x00, 0x00],
            }
        }
    }
}
