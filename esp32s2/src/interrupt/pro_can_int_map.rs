#[doc = "Register `PRO_CAN_INT_MAP` reader"]
pub struct R(crate::R<PRO_CAN_INT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CAN_INT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CAN_INT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CAN_INT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CAN_INT_MAP` writer"]
pub struct W(crate::W<PRO_CAN_INT_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CAN_INT_MAP_SPEC>;
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
impl From<crate::W<PRO_CAN_INT_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CAN_INT_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_CAN_INT_MAP` reader - This register is used to map CAN_INT interrupt signal to one of the CPU interrupts."]
pub type PRO_CAN_INT_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRO_CAN_INT_MAP` writer - This register is used to map CAN_INT interrupt signal to one of the CPU interrupts."]
pub type PRO_CAN_INT_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_CAN_INT_MAP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - This register is used to map CAN_INT interrupt signal to one of the CPU interrupts."]
    #[inline(always)]
    pub fn pro_can_int_map(&self) -> PRO_CAN_INT_MAP_R {
        PRO_CAN_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register is used to map CAN_INT interrupt signal to one of the CPU interrupts."]
    #[inline(always)]
    pub fn pro_can_int_map(&mut self) -> PRO_CAN_INT_MAP_W<0> {
        PRO_CAN_INT_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_INT interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_can_int_map](index.html) module"]
pub struct PRO_CAN_INT_MAP_SPEC;
impl crate::RegisterSpec for PRO_CAN_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_can_int_map::R](R) reader structure"]
impl crate::Readable for PRO_CAN_INT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_can_int_map::W](W) writer structure"]
impl crate::Writable for PRO_CAN_INT_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_CAN_INT_MAP to value 0x10"]
impl crate::Resettable for PRO_CAN_INT_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
