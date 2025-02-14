#[doc = "Register `TIMER1_CFG1` reader"]
pub struct R(crate::R<TIMER1_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1_CFG1` writer"]
pub struct W(crate::W<TIMER1_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_CFG1_SPEC>;
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
impl From<crate::W<TIMER1_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER1_START` reader - "]
pub type TIMER1_START_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER1_START` writer - "]
pub type TIMER1_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER1_CFG1_SPEC, u8, u8, 3, O>;
#[doc = "Field `TIMER1_MOD` reader - "]
pub type TIMER1_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER1_MOD` writer - "]
pub type TIMER1_MOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER1_CFG1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn timer1_start(&self) -> TIMER1_START_R {
        TIMER1_START_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn timer1_mod(&self) -> TIMER1_MOD_R {
        TIMER1_MOD_R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn timer1_start(&mut self) -> TIMER1_START_W<0> {
        TIMER1_START_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn timer1_mod(&mut self) -> TIMER1_MOD_W<3> {
        TIMER1_MOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1_cfg1](index.html) module"]
pub struct TIMER1_CFG1_SPEC;
impl crate::RegisterSpec for TIMER1_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1_cfg1::R](R) reader structure"]
impl crate::Readable for TIMER1_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1_cfg1::W](W) writer structure"]
impl crate::Writable for TIMER1_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER1_CFG1 to value 0"]
impl crate::Resettable for TIMER1_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
