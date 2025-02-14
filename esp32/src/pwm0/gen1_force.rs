#[doc = "Register `GEN1_FORCE` reader"]
pub struct R(crate::R<GEN1_FORCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN1_FORCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN1_FORCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN1_FORCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN1_FORCE` writer"]
pub struct W(crate::W<GEN1_FORCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN1_FORCE_SPEC>;
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
impl From<crate::W<GEN1_FORCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN1_FORCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEN1_CNTUFORCE_UPMETHOD` reader - "]
pub type GEN1_CNTUFORCE_UPMETHOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GEN1_CNTUFORCE_UPMETHOD` writer - "]
pub type GEN1_CNTUFORCE_UPMETHOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GEN1_FORCE_SPEC, u8, u8, 6, O>;
#[doc = "Field `GEN1_A_CNTUFORCE_MODE` reader - "]
pub type GEN1_A_CNTUFORCE_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GEN1_A_CNTUFORCE_MODE` writer - "]
pub type GEN1_A_CNTUFORCE_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GEN1_FORCE_SPEC, u8, u8, 2, O>;
#[doc = "Field `GEN1_B_CNTUFORCE_MODE` reader - "]
pub type GEN1_B_CNTUFORCE_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GEN1_B_CNTUFORCE_MODE` writer - "]
pub type GEN1_B_CNTUFORCE_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GEN1_FORCE_SPEC, u8, u8, 2, O>;
#[doc = "Field `GEN1_A_NCIFORCE` reader - "]
pub type GEN1_A_NCIFORCE_R = crate::BitReader<bool>;
#[doc = "Field `GEN1_A_NCIFORCE` writer - "]
pub type GEN1_A_NCIFORCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GEN1_FORCE_SPEC, bool, O>;
#[doc = "Field `GEN1_A_NCIFORCE_MODE` reader - "]
pub type GEN1_A_NCIFORCE_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GEN1_A_NCIFORCE_MODE` writer - "]
pub type GEN1_A_NCIFORCE_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GEN1_FORCE_SPEC, u8, u8, 2, O>;
#[doc = "Field `GEN1_B_NCIFORCE` reader - "]
pub type GEN1_B_NCIFORCE_R = crate::BitReader<bool>;
#[doc = "Field `GEN1_B_NCIFORCE` writer - "]
pub type GEN1_B_NCIFORCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GEN1_FORCE_SPEC, bool, O>;
#[doc = "Field `GEN1_B_NCIFORCE_MODE` reader - "]
pub type GEN1_B_NCIFORCE_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GEN1_B_NCIFORCE_MODE` writer - "]
pub type GEN1_B_NCIFORCE_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GEN1_FORCE_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gen1_cntuforce_upmethod(&self) -> GEN1_CNTUFORCE_UPMETHOD_R {
        GEN1_CNTUFORCE_UPMETHOD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn gen1_a_cntuforce_mode(&self) -> GEN1_A_CNTUFORCE_MODE_R {
        GEN1_A_CNTUFORCE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gen1_b_cntuforce_mode(&self) -> GEN1_B_CNTUFORCE_MODE_R {
        GEN1_B_CNTUFORCE_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gen1_a_nciforce(&self) -> GEN1_A_NCIFORCE_R {
        GEN1_A_NCIFORCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn gen1_a_nciforce_mode(&self) -> GEN1_A_NCIFORCE_MODE_R {
        GEN1_A_NCIFORCE_MODE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gen1_b_nciforce(&self) -> GEN1_B_NCIFORCE_R {
        GEN1_B_NCIFORCE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gen1_b_nciforce_mode(&self) -> GEN1_B_NCIFORCE_MODE_R {
        GEN1_B_NCIFORCE_MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gen1_cntuforce_upmethod(&mut self) -> GEN1_CNTUFORCE_UPMETHOD_W<0> {
        GEN1_CNTUFORCE_UPMETHOD_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn gen1_a_cntuforce_mode(&mut self) -> GEN1_A_CNTUFORCE_MODE_W<6> {
        GEN1_A_CNTUFORCE_MODE_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gen1_b_cntuforce_mode(&mut self) -> GEN1_B_CNTUFORCE_MODE_W<8> {
        GEN1_B_CNTUFORCE_MODE_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gen1_a_nciforce(&mut self) -> GEN1_A_NCIFORCE_W<10> {
        GEN1_A_NCIFORCE_W::new(self)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn gen1_a_nciforce_mode(&mut self) -> GEN1_A_NCIFORCE_MODE_W<11> {
        GEN1_A_NCIFORCE_MODE_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gen1_b_nciforce(&mut self) -> GEN1_B_NCIFORCE_W<13> {
        GEN1_B_NCIFORCE_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gen1_b_nciforce_mode(&mut self) -> GEN1_B_NCIFORCE_MODE_W<14> {
        GEN1_B_NCIFORCE_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen1_force](index.html) module"]
pub struct GEN1_FORCE_SPEC;
impl crate::RegisterSpec for GEN1_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen1_force::R](R) reader structure"]
impl crate::Readable for GEN1_FORCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen1_force::W](W) writer structure"]
impl crate::Writable for GEN1_FORCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GEN1_FORCE to value 0x20"]
impl crate::Resettable for GEN1_FORCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
