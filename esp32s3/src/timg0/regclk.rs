#[doc = "Register `REGCLK` reader"]
pub struct R(crate::R<REGCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGCLK` writer"]
pub struct W(crate::W<REGCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGCLK_SPEC>;
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
impl From<crate::W<REGCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - Register clock gate signal. 1: The clock for software to read and write registers is always on. 0: The clock for software to read and write registers only exits when the operation happens."]
pub type CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_EN` writer - Register clock gate signal. 1: The clock for software to read and write registers is always on. 0: The clock for software to read and write registers only exits when the operation happens."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGCLK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Register clock gate signal. 1: The clock for software to read and write registers is always on. 0: The clock for software to read and write registers only exits when the operation happens."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Register clock gate signal. 1: The clock for software to read and write registers is always on. 0: The clock for software to read and write registers only exits when the operation happens."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<31> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer group clock gate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regclk](index.html) module"]
pub struct REGCLK_SPEC;
impl crate::RegisterSpec for REGCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [regclk::R](R) reader structure"]
impl crate::Readable for REGCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regclk::W](W) writer structure"]
impl crate::Writable for REGCLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REGCLK to value 0"]
impl crate::Resettable for REGCLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
