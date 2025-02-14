#[doc = "Register `REDCY_SIG0_REG` reader"]
pub struct R(crate::R<REDCY_SIG0_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REDCY_SIG0_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REDCY_SIG0_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REDCY_SIG0_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REDCY_SIG0_REG` writer"]
pub struct W(crate::W<REDCY_SIG0_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REDCY_SIG0_REG_SPEC>;
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
impl From<crate::W<REDCY_SIG0_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REDCY_SIG0_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REDCY_SIG0` reader - reg_redcy_sig0"]
pub type REDCY_SIG0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REDCY_SIG0` writer - reg_redcy_sig0"]
pub type REDCY_SIG0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REDCY_SIG0_REG_SPEC, u32, u32, 31, O>;
#[doc = "Field `REDCY_ANDOR` reader - reg_redcy_andor"]
pub type REDCY_ANDOR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:30 - reg_redcy_sig0"]
    #[inline(always)]
    pub fn redcy_sig0(&self) -> REDCY_SIG0_R {
        REDCY_SIG0_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - reg_redcy_andor"]
    #[inline(always)]
    pub fn redcy_andor(&self) -> REDCY_ANDOR_R {
        REDCY_ANDOR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - reg_redcy_sig0"]
    #[inline(always)]
    pub fn redcy_sig0(&mut self) -> REDCY_SIG0_W<0> {
        REDCY_SIG0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_REDCY_SIG0_REG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [redcy_sig0_reg](index.html) module"]
pub struct REDCY_SIG0_REG_SPEC;
impl crate::RegisterSpec for REDCY_SIG0_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [redcy_sig0_reg::R](R) reader structure"]
impl crate::Readable for REDCY_SIG0_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [redcy_sig0_reg::W](W) writer structure"]
impl crate::Writable for REDCY_SIG0_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REDCY_SIG0_REG to value 0"]
impl crate::Resettable for REDCY_SIG0_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
