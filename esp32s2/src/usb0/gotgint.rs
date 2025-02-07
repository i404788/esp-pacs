#[doc = "Register `GOTGINT` reader"]
pub struct R(crate::R<GOTGINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GOTGINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GOTGINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GOTGINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GOTGINT` writer"]
pub struct W(crate::W<GOTGINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GOTGINT_SPEC>;
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
impl From<crate::W<GOTGINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GOTGINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SESENDDET` reader - "]
pub type SESENDDET_R = crate::BitReader<bool>;
#[doc = "Field `SESENDDET` writer - "]
pub type SESENDDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
#[doc = "Field `SESREQSUCSTSCHNG` reader - "]
pub type SESREQSUCSTSCHNG_R = crate::BitReader<bool>;
#[doc = "Field `SESREQSUCSTSCHNG` writer - "]
pub type SESREQSUCSTSCHNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
#[doc = "Field `HSTNEGSUCSTSCHNG` reader - "]
pub type HSTNEGSUCSTSCHNG_R = crate::BitReader<bool>;
#[doc = "Field `HSTNEGSUCSTSCHNG` writer - "]
pub type HSTNEGSUCSTSCHNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
#[doc = "Field `HSTNEGDET` reader - "]
pub type HSTNEGDET_R = crate::BitReader<bool>;
#[doc = "Field `HSTNEGDET` writer - "]
pub type HSTNEGDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
#[doc = "Field `ADEVTOUTCHG` reader - "]
pub type ADEVTOUTCHG_R = crate::BitReader<bool>;
#[doc = "Field `ADEVTOUTCHG` writer - "]
pub type ADEVTOUTCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
#[doc = "Field `DBNCEDONE` reader - "]
pub type DBNCEDONE_R = crate::BitReader<bool>;
#[doc = "Field `DBNCEDONE` writer - "]
pub type DBNCEDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sesenddet(&self) -> SESENDDET_R {
        SESENDDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sesreqsucstschng(&self) -> SESREQSUCSTSCHNG_R {
        SESREQSUCSTSCHNG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn hstnegsucstschng(&self) -> HSTNEGSUCSTSCHNG_R {
        HSTNEGSUCSTSCHNG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hstnegdet(&self) -> HSTNEGDET_R {
        HSTNEGDET_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adevtoutchg(&self) -> ADEVTOUTCHG_R {
        ADEVTOUTCHG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dbncedone(&self) -> DBNCEDONE_R {
        DBNCEDONE_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sesenddet(&mut self) -> SESENDDET_W<2> {
        SESENDDET_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sesreqsucstschng(&mut self) -> SESREQSUCSTSCHNG_W<8> {
        SESREQSUCSTSCHNG_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn hstnegsucstschng(&mut self) -> HSTNEGSUCSTSCHNG_W<9> {
        HSTNEGSUCSTSCHNG_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hstnegdet(&mut self) -> HSTNEGDET_W<17> {
        HSTNEGDET_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adevtoutchg(&mut self) -> ADEVTOUTCHG_W<18> {
        ADEVTOUTCHG_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dbncedone(&mut self) -> DBNCEDONE_W<19> {
        DBNCEDONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gotgint](index.html) module"]
pub struct GOTGINT_SPEC;
impl crate::RegisterSpec for GOTGINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gotgint::R](R) reader structure"]
impl crate::Readable for GOTGINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gotgint::W](W) writer structure"]
impl crate::Writable for GOTGINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GOTGINT to value 0"]
impl crate::Resettable for GOTGINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
