#[doc = "Register `ROM_MPU_TABLE3` reader"]
pub struct R(crate::R<ROM_MPU_TABLE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_MPU_TABLE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_MPU_TABLE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_MPU_TABLE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_MPU_TABLE3` writer"]
pub struct W(crate::W<ROM_MPU_TABLE3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_MPU_TABLE3_SPEC>;
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
impl From<crate::W<ROM_MPU_TABLE3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_MPU_TABLE3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_MPU_TABLE3` reader - "]
pub type ROM_MPU_TABLE3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROM_MPU_TABLE3` writer - "]
pub type ROM_MPU_TABLE3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROM_MPU_TABLE3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rom_mpu_table3(&self) -> ROM_MPU_TABLE3_R {
        ROM_MPU_TABLE3_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rom_mpu_table3(&mut self) -> ROM_MPU_TABLE3_W<0> {
        ROM_MPU_TABLE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_mpu_table3](index.html) module"]
pub struct ROM_MPU_TABLE3_SPEC;
impl crate::RegisterSpec for ROM_MPU_TABLE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_mpu_table3::R](R) reader structure"]
impl crate::Readable for ROM_MPU_TABLE3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_mpu_table3::W](W) writer structure"]
impl crate::Writable for ROM_MPU_TABLE3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_MPU_TABLE3 to value 0x01"]
impl crate::Resettable for ROM_MPU_TABLE3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
