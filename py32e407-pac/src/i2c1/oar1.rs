#[doc = "Register `OAR1` reader"]
pub type R = crate::R<Oar1Spec>;
#[doc = "Register `OAR1` writer"]
pub type W = crate::W<Oar1Spec>;
#[doc = "Field `ADD0` reader - desc ADD0"]
pub type Add0R = crate::BitReader;
#[doc = "Field `ADD0` writer - desc ADD0"]
pub type Add0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD1_7` reader - desc ADD"]
pub type Add1_7R = crate::FieldReader;
#[doc = "Field `ADD1_7` writer - desc ADD"]
pub type Add1_7W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADD8_9` reader - desc ADD"]
pub type Add8_9R = crate::FieldReader;
#[doc = "Field `ADD8_9` writer - desc ADD"]
pub type Add8_9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADDMODE` reader - desc ADDMODE"]
pub type AddmodeR = crate::BitReader;
#[doc = "Field `ADDMODE` writer - desc ADDMODE"]
pub type AddmodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc ADD0"]
    #[inline(always)]
    pub fn add0(&self) -> Add0R {
        Add0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - desc ADD"]
    #[inline(always)]
    pub fn add1_7(&self) -> Add1_7R {
        Add1_7R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - desc ADD"]
    #[inline(always)]
    pub fn add8_9(&self) -> Add8_9R {
        Add8_9R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - desc ADDMODE"]
    #[inline(always)]
    pub fn addmode(&self) -> AddmodeR {
        AddmodeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ADD0"]
    #[inline(always)]
    pub fn add0(&mut self) -> Add0W<'_, Oar1Spec> {
        Add0W::new(self, 0)
    }
    #[doc = "Bits 1:7 - desc ADD"]
    #[inline(always)]
    pub fn add1_7(&mut self) -> Add1_7W<'_, Oar1Spec> {
        Add1_7W::new(self, 1)
    }
    #[doc = "Bits 8:9 - desc ADD"]
    #[inline(always)]
    pub fn add8_9(&mut self) -> Add8_9W<'_, Oar1Spec> {
        Add8_9W::new(self, 8)
    }
    #[doc = "Bit 15 - desc ADDMODE"]
    #[inline(always)]
    pub fn addmode(&mut self) -> AddmodeW<'_, Oar1Spec> {
        AddmodeW::new(self, 15)
    }
}
#[doc = "desc OAR1\n\nYou can [`read`](crate::Reg::read) this register and get [`oar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oar1Spec;
impl crate::RegisterSpec for Oar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar1::R`](R) reader structure"]
impl crate::Readable for Oar1Spec {}
#[doc = "`write(|w| ..)` method takes [`oar1::W`](W) writer structure"]
impl crate::Writable for Oar1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OAR1 to value 0"]
impl crate::Resettable for Oar1Spec {}
