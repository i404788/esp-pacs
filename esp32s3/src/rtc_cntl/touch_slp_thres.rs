#[doc = "Register `TOUCH_SLP_THRES` reader"]
pub struct R(crate::R<TOUCH_SLP_THRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_SLP_THRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_SLP_THRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_SLP_THRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_SLP_THRES` writer"]
pub struct W(crate::W<TOUCH_SLP_THRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_SLP_THRES_SPEC>;
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
impl From<crate::W<TOUCH_SLP_THRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_SLP_THRES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_SLP_TH` reader - the threshold for sleep touch pad"]
pub type TOUCH_SLP_TH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TOUCH_SLP_TH` writer - the threshold for sleep touch pad"]
pub type TOUCH_SLP_TH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOUCH_SLP_THRES_SPEC, u32, u32, 22, O>;
#[doc = "Field `TOUCH_SLP_APPROACH_EN` reader - sleep pad approach function enable"]
pub type TOUCH_SLP_APPROACH_EN_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_SLP_APPROACH_EN` writer - sleep pad approach function enable"]
pub type TOUCH_SLP_APPROACH_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TOUCH_SLP_THRES_SPEC, bool, O>;
#[doc = "Field `TOUCH_SLP_PAD` reader - configure which pad as slp pad"]
pub type TOUCH_SLP_PAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOUCH_SLP_PAD` writer - configure which pad as slp pad"]
pub type TOUCH_SLP_PAD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOUCH_SLP_THRES_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:21 - the threshold for sleep touch pad"]
    #[inline(always)]
    pub fn touch_slp_th(&self) -> TOUCH_SLP_TH_R {
        TOUCH_SLP_TH_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 26 - sleep pad approach function enable"]
    #[inline(always)]
    pub fn touch_slp_approach_en(&self) -> TOUCH_SLP_APPROACH_EN_R {
        TOUCH_SLP_APPROACH_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - configure which pad as slp pad"]
    #[inline(always)]
    pub fn touch_slp_pad(&self) -> TOUCH_SLP_PAD_R {
        TOUCH_SLP_PAD_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:21 - the threshold for sleep touch pad"]
    #[inline(always)]
    pub fn touch_slp_th(&mut self) -> TOUCH_SLP_TH_W<0> {
        TOUCH_SLP_TH_W::new(self)
    }
    #[doc = "Bit 26 - sleep pad approach function enable"]
    #[inline(always)]
    pub fn touch_slp_approach_en(&mut self) -> TOUCH_SLP_APPROACH_EN_W<26> {
        TOUCH_SLP_APPROACH_EN_W::new(self)
    }
    #[doc = "Bits 27:31 - configure which pad as slp pad"]
    #[inline(always)]
    pub fn touch_slp_pad(&mut self) -> TOUCH_SLP_PAD_W<27> {
        TOUCH_SLP_PAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure touch controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_slp_thres](index.html) module"]
pub struct TOUCH_SLP_THRES_SPEC;
impl crate::RegisterSpec for TOUCH_SLP_THRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_slp_thres::R](R) reader structure"]
impl crate::Readable for TOUCH_SLP_THRES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_slp_thres::W](W) writer structure"]
impl crate::Writable for TOUCH_SLP_THRES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOUCH_SLP_THRES to value 0x7800_0000"]
impl crate::Resettable for TOUCH_SLP_THRES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7800_0000
    }
}
