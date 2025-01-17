#![no_std]
#![no_main]

use defmt::unwrap;
use defmt_rtt as _; // global logger
use nucleo_wl55jc_bsp::hal::{
    cortex_m,
    info::{self, Core},
    pac::{self, DWT},
    rcc,
    util::reset_cycle_count,
};
use panic_probe as _;

const FREQ: u32 = 48_000_000;
const CYC_PER_MICRO: u32 = FREQ / 1000 / 1000;

// WARNING will wrap-around eventually, use this for relative timing only
defmt::timestamp!("{=u32:us}", DWT::get_cycle_count() / CYC_PER_MICRO);

#[defmt_test::tests]
mod tests {
    use super::*;

    #[init]
    fn init() {
        let mut cp: pac::CorePeripherals = unwrap!(pac::CorePeripherals::take());
        let mut dp: pac::Peripherals = unwrap!(pac::Peripherals::take());

        cortex_m::interrupt::free(|cs| unsafe {
            rcc::set_sysclk_msi_max(&mut dp.FLASH, &mut dp.PWR, &mut dp.RCC, cs)
        });

        cp.DCB.enable_trace();
        cp.DWT.enable_cycle_counter();
        reset_cycle_count(&mut cp.DWT);
    }

    #[test]
    fn core() {
        defmt::assert_eq!(info::CORE, Core::Cm4);
        defmt::assert_eq!(info::core(), Core::Cm4);
    }

    #[test]
    fn flash_size() {
        defmt::assert_eq!(info::flash_size_kibibyte(), 256);
        defmt::assert_eq!(info::flash_size(), 256 * 1024);
    }

    #[test]
    fn uid64() {
        defmt::assert_eq!(info::uid64().dev_id(), 0x15);
        defmt::assert_eq!(info::uid64().company_id(), 0x0080E1);
    }
}
