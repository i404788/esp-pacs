#[doc = "Register `SLP_REJECT_CONF` reader"]
pub struct R(crate::R<SLP_REJECT_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_REJECT_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_REJECT_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_REJECT_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_REJECT_CONF` writer"]
pub struct W(crate::W<SLP_REJECT_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_REJECT_CONF_SPEC>;
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
impl From<crate::W<SLP_REJECT_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_REJECT_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEP_REJECT_ENA` reader - sleep reject enable"]
pub type SLEEP_REJECT_ENA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SLEEP_REJECT_ENA` writer - sleep reject enable"]
pub type SLEEP_REJECT_ENA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLP_REJECT_CONF_SPEC, u32, u32, 18, O>;
#[doc = "Field `LIGHT_SLP_REJECT_EN` reader - enable reject for light sleep"]
pub type LIGHT_SLP_REJECT_EN_R = crate::BitReader<bool>;
#[doc = "Field `LIGHT_SLP_REJECT_EN` writer - enable reject for light sleep"]
pub type LIGHT_SLP_REJECT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLP_REJECT_CONF_SPEC, bool, O>;
#[doc = "Field `DEEP_SLP_REJECT_EN` reader - enable reject for deep sleep"]
pub type DEEP_SLP_REJECT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DEEP_SLP_REJECT_EN` writer - enable reject for deep sleep"]
pub type DEEP_SLP_REJECT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLP_REJECT_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 12:29 - sleep reject enable"]
    #[inline(always)]
    pub fn sleep_reject_ena(&self) -> SLEEP_REJECT_ENA_R {
        SLEEP_REJECT_ENA_R::new(((self.bits >> 12) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 30 - enable reject for light sleep"]
    #[inline(always)]
    pub fn light_slp_reject_en(&self) -> LIGHT_SLP_REJECT_EN_R {
        LIGHT_SLP_REJECT_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable reject for deep sleep"]
    #[inline(always)]
    pub fn deep_slp_reject_en(&self) -> DEEP_SLP_REJECT_EN_R {
        DEEP_SLP_REJECT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 12:29 - sleep reject enable"]
    #[inline(always)]
    pub fn sleep_reject_ena(&mut self) -> SLEEP_REJECT_ENA_W<12> {
        SLEEP_REJECT_ENA_W::new(self)
    }
    #[doc = "Bit 30 - enable reject for light sleep"]
    #[inline(always)]
    pub fn light_slp_reject_en(&mut self) -> LIGHT_SLP_REJECT_EN_W<30> {
        LIGHT_SLP_REJECT_EN_W::new(self)
    }
    #[doc = "Bit 31 - enable reject for deep sleep"]
    #[inline(always)]
    pub fn deep_slp_reject_en(&mut self) -> DEEP_SLP_REJECT_EN_W<31> {
        DEEP_SLP_REJECT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_reject_conf](index.html) module"]
pub struct SLP_REJECT_CONF_SPEC;
impl crate::RegisterSpec for SLP_REJECT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_reject_conf::R](R) reader structure"]
impl crate::Readable for SLP_REJECT_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_reject_conf::W](W) writer structure"]
impl crate::Writable for SLP_REJECT_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLP_REJECT_CONF to value 0"]
impl crate::Resettable for SLP_REJECT_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
