#[doc = "Register `CPU_PERI_RST_EN` reader"]
pub struct R(crate::R<CPU_PERI_RST_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PERI_RST_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PERI_RST_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PERI_RST_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_PERI_RST_EN` writer"]
pub struct W(crate::W<CPU_PERI_RST_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PERI_RST_EN_SPEC>;
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
impl From<crate::W<CPU_PERI_RST_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PERI_RST_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RST_EN_ASSIST_DEBUG` reader - Set 1 to let assist_debug module reset"]
pub type RST_EN_ASSIST_DEBUG_R = crate::BitReader<bool>;
#[doc = "Field `RST_EN_ASSIST_DEBUG` writer - Set 1 to let assist_debug module reset"]
pub type RST_EN_ASSIST_DEBUG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PERI_RST_EN_SPEC, bool, O>;
#[doc = "Field `RST_EN_DEDICATED_GPIO` reader - Set 1 to let dedicated_gpio module reset"]
pub type RST_EN_DEDICATED_GPIO_R = crate::BitReader<bool>;
#[doc = "Field `RST_EN_DEDICATED_GPIO` writer - Set 1 to let dedicated_gpio module reset"]
pub type RST_EN_DEDICATED_GPIO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PERI_RST_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 6 - Set 1 to let assist_debug module reset"]
    #[inline(always)]
    pub fn rst_en_assist_debug(&self) -> RST_EN_ASSIST_DEBUG_R {
        RST_EN_ASSIST_DEBUG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set 1 to let dedicated_gpio module reset"]
    #[inline(always)]
    pub fn rst_en_dedicated_gpio(&self) -> RST_EN_DEDICATED_GPIO_R {
        RST_EN_DEDICATED_GPIO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Set 1 to let assist_debug module reset"]
    #[inline(always)]
    pub fn rst_en_assist_debug(&mut self) -> RST_EN_ASSIST_DEBUG_W<6> {
        RST_EN_ASSIST_DEBUG_W::new(self)
    }
    #[doc = "Bit 7 - Set 1 to let dedicated_gpio module reset"]
    #[inline(always)]
    pub fn rst_en_dedicated_gpio(&mut self) -> RST_EN_DEDICATED_GPIO_W<7> {
        RST_EN_DEDICATED_GPIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_peripheral reset configuration regsiter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_peri_rst_en](index.html) module"]
pub struct CPU_PERI_RST_EN_SPEC;
impl crate::RegisterSpec for CPU_PERI_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_peri_rst_en::R](R) reader structure"]
impl crate::Readable for CPU_PERI_RST_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_peri_rst_en::W](W) writer structure"]
impl crate::Writable for CPU_PERI_RST_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_PERI_RST_EN to value 0xc0"]
impl crate::Resettable for CPU_PERI_RST_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc0
    }
}
