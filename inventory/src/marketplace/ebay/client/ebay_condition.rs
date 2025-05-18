use crate::item::ItemCondition;

#[allow(unused)]
pub enum Condition {
    New,
    LikeNew,
    NewOther,
    NewWithDefects,
    CertifiedRefurbished,
    ExcellentRefurbished,
    VeryGoodRefurbished,
    GoodRefurbished,
    SellerRefurbished,
    UsedExcellent,
    UsedVeryGood,
    UsedGood,
    UsedAcceptable,
    ForPartsOrNotWorking,
    PreOwnedExcellent,
    PreOwnedFair,
}

impl Condition {
    pub fn to_serial(&self) -> &'static str {
        match self {
            Condition::New => "NEW",
            Condition::LikeNew => "LIKE_NEW",
            Condition::NewOther => "NEW_OTHER",
            Condition::NewWithDefects => "NEW_WITH_DEFECTS",
            Condition::CertifiedRefurbished => "CERTIFIED_REFURBISHED",
            Condition::ExcellentRefurbished => "EXCELLENT_REFURBISHED",
            Condition::VeryGoodRefurbished => "VERY_GOOD_REFURBISHED",
            Condition::GoodRefurbished => "GOOD_REFURBISHED",
            Condition::SellerRefurbished => "SELLER_REFURBISHED",
            Condition::UsedExcellent => "USED_EXCELLENT",
            Condition::UsedVeryGood => "USED_VERY_GOOD",
            Condition::UsedGood => "USED_GOOD",
            Condition::UsedAcceptable => "USED_ACCEPTABLE",
            Condition::ForPartsOrNotWorking => "FOR_PARTS_OR_NOT_WORKING",
            Condition::PreOwnedExcellent => "PRE_OWNED_EXCELLENT",
            Condition::PreOwnedFair => "PRE_OWNED_FAIR",
        }
    }
}

impl From<&ItemCondition> for Condition {
    fn from(value: &ItemCondition) -> Self {
        match value {
            ItemCondition::Inapplicable => Condition::New,
            ItemCondition::BrandNew => Condition::New,
            ItemCondition::LikeNew => Condition::LikeNew,
            ItemCondition::VeryGood => Condition::UsedVeryGood,
            ItemCondition::Good => Condition::UsedGood,
            ItemCondition::Acceptable => Condition::UsedAcceptable,
            ItemCondition::Digital => Condition::NewOther,
        }
    }
}
