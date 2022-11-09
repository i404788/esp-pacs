#[doc = "Register `UART1_PD_CTRL` reader"]
pub struct R(crate::R<UART1_PD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART1_PD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART1_PD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART1_PD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART1_PD_CTRL` writer"]
pub struct W(crate::W<UART1_PD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART1_PD_CTRL_SPEC>;
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
impl From<crate::W<UART1_PD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART1_PD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART1_MEM_FORCE_PU` reader - Set this bit to force power down UART1 memory."]
pub type UART1_MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `UART1_MEM_FORCE_PU` writer - Set this bit to force power down UART1 memory."]
pub type UART1_MEM_FORCE_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART1_PD_CTRL_SPEC, bool, O>;
#[doc = "Field `UART1_MEM_FORCE_PD` reader - Set this bit to force power up UART1 memory."]
pub type UART1_MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `UART1_MEM_FORCE_PD` writer - Set this bit to force power up UART1 memory."]
pub type UART1_MEM_FORCE_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART1_PD_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Set this bit to force power down UART1 memory."]
    #[inline(always)]
    pub fn uart1_mem_force_pu(&self) -> UART1_MEM_FORCE_PU_R {
        UART1_MEM_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force power up UART1 memory."]
    #[inline(always)]
    pub fn uart1_mem_force_pd(&self) -> UART1_MEM_FORCE_PD_R {
        UART1_MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to force power down UART1 memory."]
    #[inline(always)]
    #[must_use]
    pub fn uart1_mem_force_pu(&mut self) -> UART1_MEM_FORCE_PU_W<1> {
        UART1_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to force power up UART1 memory."]
    #[inline(always)]
    #[must_use]
    pub fn uart1_mem_force_pd(&mut self) -> UART1_MEM_FORCE_PD_W<2> {
        UART1_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART1 power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_pd_ctrl](index.html) module"]
pub struct UART1_PD_CTRL_SPEC;
impl crate::RegisterSpec for UART1_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart1_pd_ctrl::R](R) reader structure"]
impl crate::Readable for UART1_PD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart1_pd_ctrl::W](W) writer structure"]
impl crate::Writable for UART1_PD_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART1_PD_CTRL to value 0x02"]
impl crate::Resettable for UART1_PD_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}