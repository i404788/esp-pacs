#[doc = "Register `PRO_AHB_2` reader"]
pub struct R(crate::R<PRO_AHB_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_AHB_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_AHB_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_AHB_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_AHB_2` writer"]
pub struct W(crate::W<PRO_AHB_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_AHB_2_SPEC>;
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
impl From<crate::W<PRO_AHB_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_AHB_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_AHB_RTCSLOW_1_SPLTADDR` reader - Configure the split address of RTCSlow_1 for PeriBus2 access."]
pub type PRO_AHB_RTCSLOW_1_SPLTADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_SPLTADDR` writer - Configure the split address of RTCSlow_1 for PeriBus2 access."]
pub type PRO_AHB_RTCSLOW_1_SPLTADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_AHB_2_SPEC, u16, u16, 11, O>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_L_F` reader - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_1 low address region."]
pub type PRO_AHB_RTCSLOW_1_L_F_R = crate::BitReader<bool>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_L_F` writer - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_1 low address region."]
pub type PRO_AHB_RTCSLOW_1_L_F_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_AHB_2_SPEC, bool, O>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_L_R` reader - Setting to 1 grants PeriBus2 permission to read RTCSlow_1 low address region."]
pub type PRO_AHB_RTCSLOW_1_L_R_R = crate::BitReader<bool>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_L_R` writer - Setting to 1 grants PeriBus2 permission to read RTCSlow_1 low address region."]
pub type PRO_AHB_RTCSLOW_1_L_R_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_AHB_2_SPEC, bool, O>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_L_W` reader - Setting to 1 grants PeriBus2 permission to write RTCSlow_1 low address region."]
pub type PRO_AHB_RTCSLOW_1_L_W_R = crate::BitReader<bool>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_L_W` writer - Setting to 1 grants PeriBus2 permission to write RTCSlow_1 low address region."]
pub type PRO_AHB_RTCSLOW_1_L_W_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_AHB_2_SPEC, bool, O>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_H_F` reader - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_1 high address region."]
pub type PRO_AHB_RTCSLOW_1_H_F_R = crate::BitReader<bool>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_H_F` writer - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_1 high address region."]
pub type PRO_AHB_RTCSLOW_1_H_F_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_AHB_2_SPEC, bool, O>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_H_R` reader - Setting to 1 grants PeriBus2 permission to read RTCSlow_1 high address region."]
pub type PRO_AHB_RTCSLOW_1_H_R_R = crate::BitReader<bool>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_H_R` writer - Setting to 1 grants PeriBus2 permission to read RTCSlow_1 high address region."]
pub type PRO_AHB_RTCSLOW_1_H_R_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_AHB_2_SPEC, bool, O>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_H_W` reader - Setting to 1 grants PeriBus2 permission to write RTCSlow_1 high address region."]
pub type PRO_AHB_RTCSLOW_1_H_W_R = crate::BitReader<bool>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_H_W` writer - Setting to 1 grants PeriBus2 permission to write RTCSlow_1 high address region."]
pub type PRO_AHB_RTCSLOW_1_H_W_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_AHB_2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10 - Configure the split address of RTCSlow_1 for PeriBus2 access."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_spltaddr(&self) -> PRO_AHB_RTCSLOW_1_SPLTADDR_R {
        PRO_AHB_RTCSLOW_1_SPLTADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_1 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_l_f(&self) -> PRO_AHB_RTCSLOW_1_L_F_R {
        PRO_AHB_RTCSLOW_1_L_F_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Setting to 1 grants PeriBus2 permission to read RTCSlow_1 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_l_r(&self) -> PRO_AHB_RTCSLOW_1_L_R_R {
        PRO_AHB_RTCSLOW_1_L_R_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Setting to 1 grants PeriBus2 permission to write RTCSlow_1 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_l_w(&self) -> PRO_AHB_RTCSLOW_1_L_W_R {
        PRO_AHB_RTCSLOW_1_L_W_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_1 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_h_f(&self) -> PRO_AHB_RTCSLOW_1_H_F_R {
        PRO_AHB_RTCSLOW_1_H_F_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Setting to 1 grants PeriBus2 permission to read RTCSlow_1 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_h_r(&self) -> PRO_AHB_RTCSLOW_1_H_R_R {
        PRO_AHB_RTCSLOW_1_H_R_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Setting to 1 grants PeriBus2 permission to write RTCSlow_1 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_h_w(&self) -> PRO_AHB_RTCSLOW_1_H_W_R {
        PRO_AHB_RTCSLOW_1_H_W_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Configure the split address of RTCSlow_1 for PeriBus2 access."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_spltaddr(&mut self) -> PRO_AHB_RTCSLOW_1_SPLTADDR_W<0> {
        PRO_AHB_RTCSLOW_1_SPLTADDR_W::new(self)
    }
    #[doc = "Bit 11 - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_1 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_l_f(&mut self) -> PRO_AHB_RTCSLOW_1_L_F_W<11> {
        PRO_AHB_RTCSLOW_1_L_F_W::new(self)
    }
    #[doc = "Bit 12 - Setting to 1 grants PeriBus2 permission to read RTCSlow_1 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_l_r(&mut self) -> PRO_AHB_RTCSLOW_1_L_R_W<12> {
        PRO_AHB_RTCSLOW_1_L_R_W::new(self)
    }
    #[doc = "Bit 13 - Setting to 1 grants PeriBus2 permission to write RTCSlow_1 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_l_w(&mut self) -> PRO_AHB_RTCSLOW_1_L_W_W<13> {
        PRO_AHB_RTCSLOW_1_L_W_W::new(self)
    }
    #[doc = "Bit 14 - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_1 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_h_f(&mut self) -> PRO_AHB_RTCSLOW_1_H_F_W<14> {
        PRO_AHB_RTCSLOW_1_H_F_W::new(self)
    }
    #[doc = "Bit 15 - Setting to 1 grants PeriBus2 permission to read RTCSlow_1 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_h_r(&mut self) -> PRO_AHB_RTCSLOW_1_H_R_W<15> {
        PRO_AHB_RTCSLOW_1_H_R_W::new(self)
    }
    #[doc = "Bit 16 - Setting to 1 grants PeriBus2 permission to write RTCSlow_1 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_h_w(&mut self) -> PRO_AHB_RTCSLOW_1_H_W_W<16> {
        PRO_AHB_RTCSLOW_1_H_W_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PeriBus2 permission control register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_ahb_2](index.html) module"]
pub struct PRO_AHB_2_SPEC;
impl crate::RegisterSpec for PRO_AHB_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_ahb_2::R](R) reader structure"]
impl crate::Readable for PRO_AHB_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_ahb_2::W](W) writer structure"]
impl crate::Writable for PRO_AHB_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_AHB_2 to value 0x0001_f800"]
impl crate::Resettable for PRO_AHB_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_f800
    }
}