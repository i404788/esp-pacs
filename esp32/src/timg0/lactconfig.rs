#[doc = "Register `LACTCONFIG` reader"]
pub struct R(crate::R<LACTCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LACTCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LACTCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LACTCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LACTCONFIG` writer"]
pub struct W(crate::W<LACTCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LACTCONFIG_SPEC>;
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
impl From<crate::W<LACTCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LACTCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LACT_RTC_ONLY` reader - "]
pub type LACT_RTC_ONLY_R = crate::BitReader<bool>;
#[doc = "Field `LACT_RTC_ONLY` writer - "]
pub type LACT_RTC_ONLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, LACTCONFIG_SPEC, bool, O>;
#[doc = "Field `LACT_CPST_EN` reader - "]
pub type LACT_CPST_EN_R = crate::BitReader<bool>;
#[doc = "Field `LACT_CPST_EN` writer - "]
pub type LACT_CPST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LACTCONFIG_SPEC, bool, O>;
#[doc = "Field `LACT_LAC_EN` reader - "]
pub type LACT_LAC_EN_R = crate::BitReader<bool>;
#[doc = "Field `LACT_LAC_EN` writer - "]
pub type LACT_LAC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LACTCONFIG_SPEC, bool, O>;
#[doc = "Field `LACT_ALARM_EN` reader - "]
pub type LACT_ALARM_EN_R = crate::BitReader<bool>;
#[doc = "Field `LACT_ALARM_EN` writer - "]
pub type LACT_ALARM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LACTCONFIG_SPEC, bool, O>;
#[doc = "Field `LACT_LEVEL_INT_EN` reader - "]
pub type LACT_LEVEL_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `LACT_LEVEL_INT_EN` writer - "]
pub type LACT_LEVEL_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LACTCONFIG_SPEC, bool, O>;
#[doc = "Field `LACT_EDGE_INT_EN` reader - "]
pub type LACT_EDGE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `LACT_EDGE_INT_EN` writer - "]
pub type LACT_EDGE_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LACTCONFIG_SPEC, bool, O>;
#[doc = "Field `LACT_DIVIDER` reader - "]
pub type LACT_DIVIDER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LACT_DIVIDER` writer - "]
pub type LACT_DIVIDER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LACTCONFIG_SPEC, u16, u16, 16, O>;
#[doc = "Field `LACT_AUTORELOAD` reader - "]
pub type LACT_AUTORELOAD_R = crate::BitReader<bool>;
#[doc = "Field `LACT_AUTORELOAD` writer - "]
pub type LACT_AUTORELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, LACTCONFIG_SPEC, bool, O>;
#[doc = "Field `LACT_INCREASE` reader - "]
pub type LACT_INCREASE_R = crate::BitReader<bool>;
#[doc = "Field `LACT_INCREASE` writer - "]
pub type LACT_INCREASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LACTCONFIG_SPEC, bool, O>;
#[doc = "Field `LACT_EN` reader - "]
pub type LACT_EN_R = crate::BitReader<bool>;
#[doc = "Field `LACT_EN` writer - "]
pub type LACT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LACTCONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lact_rtc_only(&self) -> LACT_RTC_ONLY_R {
        LACT_RTC_ONLY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lact_cpst_en(&self) -> LACT_CPST_EN_R {
        LACT_CPST_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lact_lac_en(&self) -> LACT_LAC_EN_R {
        LACT_LAC_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn lact_alarm_en(&self) -> LACT_ALARM_EN_R {
        LACT_ALARM_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lact_level_int_en(&self) -> LACT_LEVEL_INT_EN_R {
        LACT_LEVEL_INT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lact_edge_int_en(&self) -> LACT_EDGE_INT_EN_R {
        LACT_EDGE_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:28"]
    #[inline(always)]
    pub fn lact_divider(&self) -> LACT_DIVIDER_R {
        LACT_DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lact_autoreload(&self) -> LACT_AUTORELOAD_R {
        LACT_AUTORELOAD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn lact_increase(&self) -> LACT_INCREASE_R {
        LACT_INCREASE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn lact_en(&self) -> LACT_EN_R {
        LACT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lact_rtc_only(&mut self) -> LACT_RTC_ONLY_W<7> {
        LACT_RTC_ONLY_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lact_cpst_en(&mut self) -> LACT_CPST_EN_W<8> {
        LACT_CPST_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lact_lac_en(&mut self) -> LACT_LAC_EN_W<9> {
        LACT_LAC_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn lact_alarm_en(&mut self) -> LACT_ALARM_EN_W<10> {
        LACT_ALARM_EN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lact_level_int_en(&mut self) -> LACT_LEVEL_INT_EN_W<11> {
        LACT_LEVEL_INT_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lact_edge_int_en(&mut self) -> LACT_EDGE_INT_EN_W<12> {
        LACT_EDGE_INT_EN_W::new(self)
    }
    #[doc = "Bits 13:28"]
    #[inline(always)]
    pub fn lact_divider(&mut self) -> LACT_DIVIDER_W<13> {
        LACT_DIVIDER_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lact_autoreload(&mut self) -> LACT_AUTORELOAD_W<29> {
        LACT_AUTORELOAD_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn lact_increase(&mut self) -> LACT_INCREASE_W<30> {
        LACT_INCREASE_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn lact_en(&mut self) -> LACT_EN_W<31> {
        LACT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lactconfig](index.html) module"]
pub struct LACTCONFIG_SPEC;
impl crate::RegisterSpec for LACTCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lactconfig::R](R) reader structure"]
impl crate::Readable for LACTCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lactconfig::W](W) writer structure"]
impl crate::Writable for LACTCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LACTCONFIG to value 0x6000_2300"]
impl crate::Resettable for LACTCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6000_2300
    }
}
