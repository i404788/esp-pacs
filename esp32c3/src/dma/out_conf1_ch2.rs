#[doc = "Register `OUT_CONF1_CH2` reader"]
pub struct R(crate::R<OUT_CONF1_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_CONF1_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_CONF1_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_CONF1_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_CONF1_CH2` writer"]
pub struct W(crate::W<OUT_CONF1_CH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_CONF1_CH2_SPEC>;
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
impl From<crate::W<OUT_CONF1_CH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_CONF1_CH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_CHECK_OWNER` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type OUT_CHECK_OWNER_R = crate::BitReader<bool>;
#[doc = "Field `OUT_CHECK_OWNER` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type OUT_CHECK_OWNER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_CONF1_CH2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn out_check_owner(&self) -> OUT_CHECK_OWNER_R {
        OUT_CHECK_OWNER_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn out_check_owner(&mut self) -> OUT_CHECK_OWNER_W<12> {
        OUT_CHECK_OWNER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_OUT_CONF1_CH2_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_conf1_ch2](index.html) module"]
pub struct OUT_CONF1_CH2_SPEC;
impl crate::RegisterSpec for OUT_CONF1_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_conf1_ch2::R](R) reader structure"]
impl crate::Readable for OUT_CONF1_CH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_conf1_ch2::W](W) writer structure"]
impl crate::Writable for OUT_CONF1_CH2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_CONF1_CH2 to value 0"]
impl crate::Resettable for OUT_CONF1_CH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
