#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOTAL_TRANS_END_INT_CLR` writer - The clear bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt."]
pub type TOTAL_TRANS_END_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `ECC_ERR_INT_CLR` writer - The clear bit for SPI_MEM_ECC_ERR_INT interrupt. SPI_MEM_ECC_ERR_ADDR and SPI_MEM_ECC_ERR_CNT will be cleared by the pulse of this bit."]
pub type ECC_ERR_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 2 - The clear bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt."]
    #[inline(always)]
    pub fn total_trans_end_int_clr(&mut self) -> TOTAL_TRANS_END_INT_CLR_W<2> {
        TOTAL_TRANS_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - The clear bit for SPI_MEM_ECC_ERR_INT interrupt. SPI_MEM_ECC_ERR_ADDR and SPI_MEM_ECC_ERR_CNT will be cleared by the pulse of this bit."]
    #[inline(always)]
    pub fn ecc_err_int_clr(&mut self) -> ECC_ERR_INT_CLR_W<4> {
        ECC_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
