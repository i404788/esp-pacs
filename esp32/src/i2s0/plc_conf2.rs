#[doc = "Register `PLC_CONF2` reader"]
pub struct R(crate::R<PLC_CONF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLC_CONF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLC_CONF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLC_CONF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLC_CONF2` writer"]
pub struct W(crate::W<PLC_CONF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLC_CONF2_SPEC>;
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
impl From<crate::W<PLC_CONF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLC_CONF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CVSD_SEG_MOD` reader - "]
pub type CVSD_SEG_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CVSD_SEG_MOD` writer - "]
pub type CVSD_SEG_MOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLC_CONF2_SPEC, u8, u8, 2, O>;
#[doc = "Field `MIN_PERIOD` reader - "]
pub type MIN_PERIOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN_PERIOD` writer - "]
pub type MIN_PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLC_CONF2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cvsd_seg_mod(&self) -> CVSD_SEG_MOD_R {
        CVSD_SEG_MOD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn min_period(&self) -> MIN_PERIOD_R {
        MIN_PERIOD_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cvsd_seg_mod(&mut self) -> CVSD_SEG_MOD_W<0> {
        CVSD_SEG_MOD_W::new(self)
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn min_period(&mut self) -> MIN_PERIOD_W<2> {
        MIN_PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plc_conf2](index.html) module"]
pub struct PLC_CONF2_SPEC;
impl crate::RegisterSpec for PLC_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plc_conf2::R](R) reader structure"]
impl crate::Readable for PLC_CONF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plc_conf2::W](W) writer structure"]
impl crate::Writable for PLC_CONF2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLC_CONF2 to value 0x28"]
impl crate::Resettable for PLC_CONF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x28
    }
}
