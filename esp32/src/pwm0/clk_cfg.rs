#[doc = "Register `CLK_CFG` reader"]
pub struct R(crate::R<CLK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CFG` writer"]
pub struct W(crate::W<CLK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CFG_SPEC>;
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
impl From<crate::W<CLK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_PRESCALE` reader - "]
pub type CLK_PRESCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLK_PRESCALE` writer - "]
pub type CLK_PRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn clk_prescale(&self) -> CLK_PRESCALE_R {
        CLK_PRESCALE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn clk_prescale(&mut self) -> CLK_PRESCALE_W<0> {
        CLK_PRESCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg](index.html) module"]
pub struct CLK_CFG_SPEC;
impl crate::RegisterSpec for CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cfg::R](R) reader structure"]
impl crate::Readable for CLK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cfg::W](W) writer structure"]
impl crate::Writable for CLK_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CFG to value 0"]
impl crate::Resettable for CLK_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}