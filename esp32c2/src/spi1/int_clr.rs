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
#[doc = "Field `PER_END_INT_CLR` writer - The clear bit for SPI_MEM_PER_END_INT interrupt."]
pub type PER_END_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `PES_END_INT_CLR` writer - The clear bit for SPI_MEM_PES_END_INT interrupt."]
pub type PES_END_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `WPE_END_INT_CLR` writer - The clear bit for SPI_MEM_WPE_END_INT interrupt."]
pub type WPE_END_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLV_ST_END_INT_CLR` writer - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SLV_ST_END_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `MST_ST_END_INT_CLR` writer - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type MST_ST_END_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `BROWN_OUT_INT_CLR` writer - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
pub type BROWN_OUT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - The clear bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    pub fn per_end_int_clr(&mut self) -> PER_END_INT_CLR_W<0> {
        PER_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The clear bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    pub fn pes_end_int_clr(&mut self) -> PES_END_INT_CLR_W<1> {
        PES_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - The clear bit for SPI_MEM_WPE_END_INT interrupt."]
    #[inline(always)]
    pub fn wpe_end_int_clr(&mut self) -> WPE_END_INT_CLR_W<2> {
        WPE_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn slv_st_end_int_clr(&mut self) -> SLV_ST_END_INT_CLR_W<3> {
        SLV_ST_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mst_st_end_int_clr(&mut self) -> MST_ST_END_INT_CLR_W<4> {
        MST_ST_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
    #[inline(always)]
    pub fn brown_out_int_clr(&mut self) -> BROWN_OUT_INT_CLR_W<5> {
        BROWN_OUT_INT_CLR_W::new(self)
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
