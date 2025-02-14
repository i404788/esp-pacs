#[doc = "Register `CPU_PERIOD_CONF` reader"]
pub struct R(crate::R<CPU_PERIOD_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PERIOD_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PERIOD_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PERIOD_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_PERIOD_CONF` writer"]
pub struct W(crate::W<CPU_PERIOD_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PERIOD_CONF_SPEC>;
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
impl From<crate::W<CPU_PERIOD_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PERIOD_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPUSEL_CONF` reader - CPU sel option"]
pub type CPUSEL_CONF_R = crate::BitReader<bool>;
#[doc = "Field `CPUSEL_CONF` writer - CPU sel option"]
pub type CPUSEL_CONF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_PERIOD_CONF_SPEC, bool, O>;
#[doc = "Field `CPUPERIOD_SEL` reader - Need add desc"]
pub type CPUPERIOD_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPUPERIOD_SEL` writer - Need add desc"]
pub type CPUPERIOD_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PERIOD_CONF_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 29 - CPU sel option"]
    #[inline(always)]
    pub fn cpusel_conf(&self) -> CPUSEL_CONF_R {
        CPUSEL_CONF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Need add desc"]
    #[inline(always)]
    pub fn cpuperiod_sel(&self) -> CPUPERIOD_SEL_R {
        CPUPERIOD_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 29 - CPU sel option"]
    #[inline(always)]
    pub fn cpusel_conf(&mut self) -> CPUSEL_CONF_W<29> {
        CPUSEL_CONF_W::new(self)
    }
    #[doc = "Bits 30:31 - Need add desc"]
    #[inline(always)]
    pub fn cpuperiod_sel(&mut self) -> CPUPERIOD_SEL_W<30> {
        CPUPERIOD_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_period_conf](index.html) module"]
pub struct CPU_PERIOD_CONF_SPEC;
impl crate::RegisterSpec for CPU_PERIOD_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_period_conf::R](R) reader structure"]
impl crate::Readable for CPU_PERIOD_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_period_conf::W](W) writer structure"]
impl crate::Writable for CPU_PERIOD_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_PERIOD_CONF to value 0"]
impl crate::Resettable for CPU_PERIOD_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
