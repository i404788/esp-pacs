#[doc = "Register `RTCCALICFG` reader"]
pub struct R(crate::R<RTCCALICFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCALICFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCALICFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCALICFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCALICFG` writer"]
pub struct W(crate::W<RTCCALICFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCALICFG_SPEC>;
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
impl From<crate::W<RTCCALICFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCALICFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_CALI_START_CYCLING` reader - "]
pub type RTC_CALI_START_CYCLING_R = crate::BitReader<bool>;
#[doc = "Field `RTC_CALI_START_CYCLING` writer - "]
pub type RTC_CALI_START_CYCLING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RTCCALICFG_SPEC, bool, O>;
#[doc = "Field `RTC_CALI_CLK_SEL` reader - "]
pub type RTC_CALI_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_CALI_CLK_SEL` writer - "]
pub type RTC_CALI_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTCCALICFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `RTC_CALI_RDY` reader - "]
pub type RTC_CALI_RDY_R = crate::BitReader<bool>;
#[doc = "Field `RTC_CALI_MAX` reader - "]
pub type RTC_CALI_MAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RTC_CALI_MAX` writer - "]
pub type RTC_CALI_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTCCALICFG_SPEC, u16, u16, 15, O>;
#[doc = "Field `RTC_CALI_START` reader - "]
pub type RTC_CALI_START_R = crate::BitReader<bool>;
#[doc = "Field `RTC_CALI_START` writer - "]
pub type RTC_CALI_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCCALICFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtc_cali_start_cycling(&self) -> RTC_CALI_START_CYCLING_R {
        RTC_CALI_START_CYCLING_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn rtc_cali_clk_sel(&self) -> RTC_CALI_CLK_SEL_R {
        RTC_CALI_CLK_SEL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc_cali_rdy(&self) -> RTC_CALI_RDY_R {
        RTC_CALI_RDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:30"]
    #[inline(always)]
    pub fn rtc_cali_max(&self) -> RTC_CALI_MAX_R {
        RTC_CALI_MAX_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cali_start(&self) -> RTC_CALI_START_R {
        RTC_CALI_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtc_cali_start_cycling(&mut self) -> RTC_CALI_START_CYCLING_W<12> {
        RTC_CALI_START_CYCLING_W::new(self)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn rtc_cali_clk_sel(&mut self) -> RTC_CALI_CLK_SEL_W<13> {
        RTC_CALI_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 16:30"]
    #[inline(always)]
    pub fn rtc_cali_max(&mut self) -> RTC_CALI_MAX_W<16> {
        RTC_CALI_MAX_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cali_start(&mut self) -> RTC_CALI_START_W<31> {
        RTC_CALI_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccalicfg](index.html) module"]
pub struct RTCCALICFG_SPEC;
impl crate::RegisterSpec for RTCCALICFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtccalicfg::R](R) reader structure"]
impl crate::Readable for RTCCALICFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtccalicfg::W](W) writer structure"]
impl crate::Writable for RTCCALICFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCCALICFG to value 0x0001_3000"]
impl crate::Resettable for RTCCALICFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_3000
    }
}
