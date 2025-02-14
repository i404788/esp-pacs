#[doc = "Register `HCCHAR7` reader"]
pub struct R(crate::R<HCCHAR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCHAR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCHAR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCHAR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCHAR7` writer"]
pub struct W(crate::W<HCCHAR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCHAR7_SPEC>;
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
impl From<crate::W<HCCHAR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCCHAR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_MPS7` reader - "]
pub type H_MPS7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `H_MPS7` writer - "]
pub type H_MPS7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR7_SPEC, u16, u16, 11, O>;
#[doc = "Field `H_EPNUM7` reader - "]
pub type H_EPNUM7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `H_EPNUM7` writer - "]
pub type H_EPNUM7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR7_SPEC, u8, u8, 4, O>;
#[doc = "Field `H_EPDIR7` reader - "]
pub type H_EPDIR7_R = crate::BitReader<bool>;
#[doc = "Field `H_EPDIR7` writer - "]
pub type H_EPDIR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR7_SPEC, bool, O>;
#[doc = "Field `H_LSPDDEV7` reader - "]
pub type H_LSPDDEV7_R = crate::BitReader<bool>;
#[doc = "Field `H_LSPDDEV7` writer - "]
pub type H_LSPDDEV7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR7_SPEC, bool, O>;
#[doc = "Field `H_EPTYPE7` reader - "]
pub type H_EPTYPE7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `H_EPTYPE7` writer - "]
pub type H_EPTYPE7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR7_SPEC, u8, u8, 2, O>;
#[doc = "Field `H_EC7` reader - "]
pub type H_EC7_R = crate::BitReader<bool>;
#[doc = "Field `H_EC7` writer - "]
pub type H_EC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR7_SPEC, bool, O>;
#[doc = "Field `H_DEVADDR7` reader - "]
pub type H_DEVADDR7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `H_DEVADDR7` writer - "]
pub type H_DEVADDR7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR7_SPEC, u8, u8, 7, O>;
#[doc = "Field `H_ODDFRM7` reader - "]
pub type H_ODDFRM7_R = crate::BitReader<bool>;
#[doc = "Field `H_ODDFRM7` writer - "]
pub type H_ODDFRM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR7_SPEC, bool, O>;
#[doc = "Field `H_CHDIS7` reader - "]
pub type H_CHDIS7_R = crate::BitReader<bool>;
#[doc = "Field `H_CHDIS7` writer - "]
pub type H_CHDIS7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR7_SPEC, bool, O>;
#[doc = "Field `H_CHENA7` reader - "]
pub type H_CHENA7_R = crate::BitReader<bool>;
#[doc = "Field `H_CHENA7` writer - "]
pub type H_CHENA7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR7_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn h_mps7(&self) -> H_MPS7_R {
        H_MPS7_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn h_epnum7(&self) -> H_EPNUM7_R {
        H_EPNUM7_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn h_epdir7(&self) -> H_EPDIR7_R {
        H_EPDIR7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn h_lspddev7(&self) -> H_LSPDDEV7_R {
        H_LSPDDEV7_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn h_eptype7(&self) -> H_EPTYPE7_R {
        H_EPTYPE7_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn h_ec7(&self) -> H_EC7_R {
        H_EC7_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    pub fn h_devaddr7(&self) -> H_DEVADDR7_R {
        H_DEVADDR7_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn h_oddfrm7(&self) -> H_ODDFRM7_R {
        H_ODDFRM7_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn h_chdis7(&self) -> H_CHDIS7_R {
        H_CHDIS7_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_chena7(&self) -> H_CHENA7_R {
        H_CHENA7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn h_mps7(&mut self) -> H_MPS7_W<0> {
        H_MPS7_W::new(self)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn h_epnum7(&mut self) -> H_EPNUM7_W<11> {
        H_EPNUM7_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn h_epdir7(&mut self) -> H_EPDIR7_W<15> {
        H_EPDIR7_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn h_lspddev7(&mut self) -> H_LSPDDEV7_W<17> {
        H_LSPDDEV7_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn h_eptype7(&mut self) -> H_EPTYPE7_W<18> {
        H_EPTYPE7_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn h_ec7(&mut self) -> H_EC7_W<21> {
        H_EC7_W::new(self)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    pub fn h_devaddr7(&mut self) -> H_DEVADDR7_W<22> {
        H_DEVADDR7_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn h_oddfrm7(&mut self) -> H_ODDFRM7_W<29> {
        H_ODDFRM7_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn h_chdis7(&mut self) -> H_CHDIS7_W<30> {
        H_CHDIS7_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_chena7(&mut self) -> H_CHENA7_W<31> {
        H_CHENA7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar7](index.html) module"]
pub struct HCCHAR7_SPEC;
impl crate::RegisterSpec for HCCHAR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcchar7::R](R) reader structure"]
impl crate::Readable for HCCHAR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcchar7::W](W) writer structure"]
impl crate::Writable for HCCHAR7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCCHAR7 to value 0"]
impl crate::Resettable for HCCHAR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
