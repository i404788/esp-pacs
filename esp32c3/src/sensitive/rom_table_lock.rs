#[doc = "Register `ROM_TABLE_LOCK` reader"]
pub struct R(crate::R<ROM_TABLE_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_TABLE_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_TABLE_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_TABLE_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_TABLE_LOCK` writer"]
pub struct W(crate::W<ROM_TABLE_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_TABLE_LOCK_SPEC>;
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
impl From<crate::W<ROM_TABLE_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_TABLE_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_TABLE_LOCK` reader - rom_table_lock"]
pub type ROM_TABLE_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `ROM_TABLE_LOCK` writer - rom_table_lock"]
pub type ROM_TABLE_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ROM_TABLE_LOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - rom_table_lock"]
    #[inline(always)]
    pub fn rom_table_lock(&self) -> ROM_TABLE_LOCK_R {
        ROM_TABLE_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - rom_table_lock"]
    #[inline(always)]
    pub fn rom_table_lock(&mut self) -> ROM_TABLE_LOCK_W<0> {
        ROM_TABLE_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_ROM_TABLE_LOCK_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_table_lock](index.html) module"]
pub struct ROM_TABLE_LOCK_SPEC;
impl crate::RegisterSpec for ROM_TABLE_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_table_lock::R](R) reader structure"]
impl crate::Readable for ROM_TABLE_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_table_lock::W](W) writer structure"]
impl crate::Writable for ROM_TABLE_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_TABLE_LOCK to value 0"]
impl crate::Resettable for ROM_TABLE_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
