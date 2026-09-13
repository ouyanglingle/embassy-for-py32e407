#[doc = "Register `FS_CID` reader"]
pub type R = crate::R<FsCidSpec>;
#[doc = "Register `FS_CID` writer"]
pub type W = crate::W<FsCidSpec>;
#[doc = "Field `PRODUCT_ID` reader - Product ID field"]
pub type ProductIdR = crate::FieldReader<u32>;
#[doc = "Field `PRODUCT_ID` writer - Product ID field"]
pub type ProductIdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Product ID field"]
    #[inline(always)]
    pub fn product_id(&self) -> ProductIdR {
        ProductIdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Product ID field"]
    #[inline(always)]
    pub fn product_id(&mut self) -> ProductIdW<'_, FsCidSpec> {
        ProductIdW::new(self, 0)
    }
}
#[doc = "core ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_cid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_cid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsCidSpec;
impl crate::RegisterSpec for FsCidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_cid::R`](R) reader structure"]
impl crate::Readable for FsCidSpec {}
#[doc = "`write(|w| ..)` method takes [`fs_cid::W`](W) writer structure"]
impl crate::Writable for FsCidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FS_CID to value 0"]
impl crate::Resettable for FsCidSpec {}
