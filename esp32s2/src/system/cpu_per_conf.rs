#[doc = "Register `CPU_PER_CONF` reader"]
pub struct R(crate::R<CPU_PER_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PER_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PER_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PER_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_PER_CONF` writer"]
pub struct W(crate::W<CPU_PER_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PER_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CPU_PER_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PER_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPUPERIOD_SEL` reader - This field is used to select the clock frequency of CPU or CPU period."]
pub type CPUPERIOD_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPUPERIOD_SEL` writer - This field is used to select the clock frequency of CPU or CPU period."]
pub type CPUPERIOD_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PER_CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLL_FREQ_SEL` reader - This field is used to select the PLL clock frequency based on CPU period."]
pub type PLL_FREQ_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PLL_FREQ_SEL` writer - This field is used to select the PLL clock frequency based on CPU period."]
pub type PLL_FREQ_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_PER_CONF_SPEC, bool, O>;
#[doc = "Field `CPU_WAIT_MODE_FORCE_ON` reader - Set this bit to force on CPU wait mode. In this mode, the clock gate of CPU is turned off until any interrupts happen. This mode could also be force on via WAITI instruction."]
pub type CPU_WAIT_MODE_FORCE_ON_R = crate::BitReader<bool>;
#[doc = "Field `CPU_WAIT_MODE_FORCE_ON` writer - Set this bit to force on CPU wait mode. In this mode, the clock gate of CPU is turned off until any interrupts happen. This mode could also be force on via WAITI instruction."]
pub type CPU_WAIT_MODE_FORCE_ON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PER_CONF_SPEC, bool, O>;
#[doc = "Field `CPU_WAITI_DELAY_NUM` reader - Sets the number of delay cycles to enter CPU wait mode after a WAITI instruction."]
pub type CPU_WAITI_DELAY_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU_WAITI_DELAY_NUM` writer - Sets the number of delay cycles to enter CPU wait mode after a WAITI instruction."]
pub type CPU_WAITI_DELAY_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PER_CONF_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - This field is used to select the clock frequency of CPU or CPU period."]
    #[inline(always)]
    pub fn cpuperiod_sel(&self) -> CPUPERIOD_SEL_R {
        CPUPERIOD_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This field is used to select the PLL clock frequency based on CPU period."]
    #[inline(always)]
    pub fn pll_freq_sel(&self) -> PLL_FREQ_SEL_R {
        PLL_FREQ_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force on CPU wait mode. In this mode, the clock gate of CPU is turned off until any interrupts happen. This mode could also be force on via WAITI instruction."]
    #[inline(always)]
    pub fn cpu_wait_mode_force_on(&self) -> CPU_WAIT_MODE_FORCE_ON_R {
        CPU_WAIT_MODE_FORCE_ON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Sets the number of delay cycles to enter CPU wait mode after a WAITI instruction."]
    #[inline(always)]
    pub fn cpu_waiti_delay_num(&self) -> CPU_WAITI_DELAY_NUM_R {
        CPU_WAITI_DELAY_NUM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field is used to select the clock frequency of CPU or CPU period."]
    #[inline(always)]
    pub fn cpuperiod_sel(&mut self) -> CPUPERIOD_SEL_W<0> {
        CPUPERIOD_SEL_W::new(self)
    }
    #[doc = "Bit 2 - This field is used to select the PLL clock frequency based on CPU period."]
    #[inline(always)]
    pub fn pll_freq_sel(&mut self) -> PLL_FREQ_SEL_W<2> {
        PLL_FREQ_SEL_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to force on CPU wait mode. In this mode, the clock gate of CPU is turned off until any interrupts happen. This mode could also be force on via WAITI instruction."]
    #[inline(always)]
    pub fn cpu_wait_mode_force_on(&mut self) -> CPU_WAIT_MODE_FORCE_ON_W<3> {
        CPU_WAIT_MODE_FORCE_ON_W::new(self)
    }
    #[doc = "Bits 4:7 - Sets the number of delay cycles to enter CPU wait mode after a WAITI instruction."]
    #[inline(always)]
    pub fn cpu_waiti_delay_num(&mut self) -> CPU_WAITI_DELAY_NUM_W<4> {
        CPU_WAITI_DELAY_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU peripheral clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_per_conf](index.html) module"]
pub struct CPU_PER_CONF_SPEC;
impl crate::RegisterSpec for CPU_PER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_per_conf::R](R) reader structure"]
impl crate::Readable for CPU_PER_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_per_conf::W](W) writer structure"]
impl crate::Writable for CPU_PER_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_PER_CONF to value 0x0c"]
impl crate::Resettable for CPU_PER_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c
    }
}
