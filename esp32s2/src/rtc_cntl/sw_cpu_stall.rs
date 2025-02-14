#[doc = "Register `SW_CPU_STALL` reader"]
pub struct R(crate::R<SW_CPU_STALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_CPU_STALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_CPU_STALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_CPU_STALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_CPU_STALL` writer"]
pub struct W(crate::W<SW_CPU_STALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_CPU_STALL_SPEC>;
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
impl From<crate::W<SW_CPU_STALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_CPU_STALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_STALL_APPCPU_C1` reader - {reg_sw_stall_appcpu_c1\\[5:0\\] reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
pub type SW_STALL_APPCPU_C1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_STALL_APPCPU_C1` writer - {reg_sw_stall_appcpu_c1\\[5:0\\] reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
pub type SW_STALL_APPCPU_C1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SW_CPU_STALL_SPEC, u8, u8, 6, O>;
#[doc = "Field `SW_STALL_PROCPU_C1` reader - Set this bit to allow the SW to be able to send the CPU into stalling."]
pub type SW_STALL_PROCPU_C1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_STALL_PROCPU_C1` writer - Set this bit to allow the SW to be able to send the CPU into stalling."]
pub type SW_STALL_PROCPU_C1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SW_CPU_STALL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 20:25 - {reg_sw_stall_appcpu_c1\\[5:0\\] reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
    #[inline(always)]
    pub fn sw_stall_appcpu_c1(&self) -> SW_STALL_APPCPU_C1_R {
        SW_STALL_APPCPU_C1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:31 - Set this bit to allow the SW to be able to send the CPU into stalling."]
    #[inline(always)]
    pub fn sw_stall_procpu_c1(&self) -> SW_STALL_PROCPU_C1_R {
        SW_STALL_PROCPU_C1_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:25 - {reg_sw_stall_appcpu_c1\\[5:0\\] reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
    #[inline(always)]
    pub fn sw_stall_appcpu_c1(&mut self) -> SW_STALL_APPCPU_C1_W<20> {
        SW_STALL_APPCPU_C1_W::new(self)
    }
    #[doc = "Bits 26:31 - Set this bit to allow the SW to be able to send the CPU into stalling."]
    #[inline(always)]
    pub fn sw_stall_procpu_c1(&mut self) -> SW_STALL_PROCPU_C1_W<26> {
        SW_STALL_PROCPU_C1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU stall configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_cpu_stall](index.html) module"]
pub struct SW_CPU_STALL_SPEC;
impl crate::RegisterSpec for SW_CPU_STALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_cpu_stall::R](R) reader structure"]
impl crate::Readable for SW_CPU_STALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_cpu_stall::W](W) writer structure"]
impl crate::Writable for SW_CPU_STALL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_CPU_STALL to value 0"]
impl crate::Resettable for SW_CPU_STALL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
