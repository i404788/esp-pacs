#[doc = "Register `CONF` reader"]
pub struct R(crate::R<CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF` writer"]
pub struct W(crate::W<CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SPEC>;
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
impl From<crate::W<CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OP_CODE` reader - efuse operation code"]
pub type OP_CODE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OP_CODE` writer - efuse operation code"]
pub type OP_CODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF_SPEC, u16, u16, 16, O>;
#[doc = "Field `FORCE_NO_WR_RD_DIS` reader - "]
pub type FORCE_NO_WR_RD_DIS_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_NO_WR_RD_DIS` writer - "]
pub type FORCE_NO_WR_RD_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - efuse operation code"]
    #[inline(always)]
    pub fn op_code(&self) -> OP_CODE_R {
        OP_CODE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn force_no_wr_rd_dis(&self) -> FORCE_NO_WR_RD_DIS_R {
        FORCE_NO_WR_RD_DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - efuse operation code"]
    #[inline(always)]
    pub fn op_code(&mut self) -> OP_CODE_W<0> {
        OP_CODE_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn force_no_wr_rd_dis(&mut self) -> FORCE_NO_WR_RD_DIS_W<16> {
        FORCE_NO_WR_RD_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](index.html) module"]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf::R](R) reader structure"]
impl crate::Readable for CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf::W](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF to value 0x0001_0000"]
impl crate::Resettable for CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0000
    }
}
