use core::fmt::{self, Display};

pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Self {
        Self {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    #[expect(
        clippy::min_ident_chars,
        reason = "https://github.com/rust-lang/rust-clippy/issues/13396"
    )]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    const STANDARD_DECAY_RATE: i32 = 1;

    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }

    fn set_quality(item: &mut Item, value: i32) {
        if item.name == "Sulfuras, Hand of Ragnaros" {
            return;
        }
        match value {
            50.. => item.quality = 50,
            ..=0 => item.quality = 0,
            value => item.quality = value,
        }
    }

    fn increase_quality(item: &mut Item, amount: i32) {
        Self::set_quality(item, item.quality.saturating_add(amount));
    }

    fn decrease_quality(item: &mut Item, amount: i32) {
        Self::set_quality(item, item.quality.saturating_sub(amount));
    }

    fn expire(item: &mut Item) {
        match item.name.as_str() {
            "Aged Brie" => Self::increase_quality(item, 1),
            "Backstage passes to a TAFKAL80ETC concert" => {
                Self::set_quality(item, 0);
            }
            "Conjured Mana Cake" => Self::decrease_quality(
                item,
                Self::STANDARD_DECAY_RATE.saturating_mul(2),
            ),
            _ => Self::decrease_quality(item, Self::STANDARD_DECAY_RATE),
        }
    }

    fn process_day(item: &mut Item) {
        if item.name != "Sulfuras, Hand of Ragnaros" {
            match item.name.as_str() {
                "Aged Brie" => Self::increase_quality(item, 1),
                "Backstage passes to a TAFKAL80ETC concert" => {
                    let amount = match item.sell_in {
                        0..=5 => 3,
                        6..=10 => 2,
                        _ => 1,
                    };
                    Self::increase_quality(item, amount);
                }
                "Conjured Mana Cake" => Self::decrease_quality(
                    item,
                    Self::STANDARD_DECAY_RATE.saturating_mul(2),
                ),
                _ => Self::decrease_quality(item, Self::STANDARD_DECAY_RATE),
            }

            item.sell_in = item.sell_in.saturating_sub(1);
        }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            Self::process_day(item);
            if item.sell_in < 0 {
                Self::expire(item);
            }
        }
    }
}

#[cfg(test)]
#[expect(clippy::missing_panics_doc, reason = "tests")]
mod tests {
    use super::*;

    fn test_item(
        item_name: &str,
        sell_in: i32,
        quality: i32,
        new_sell_in: i32,
        new_quality: i32,
    ) {
        let items: Vec<Item> = vec![Item::new(item_name, sell_in, quality)];

        let mut rose: GildedRose = GildedRose::new(items);
        rose.update_quality();
        let rose: GildedRose = rose;

        assert_eq!(item_name, rose.items[0].name);
        assert_eq!(new_sell_in, rose.items[0].sell_in);
        assert_eq!(new_quality, rose.items[0].quality);
    }

    #[test]
    pub fn generic_item() {
        const ITEM_NAME: &str = "Foo's Bar Buz";
        const SELL_IN: i32 = 5;
        const QUALITY: i32 = 3;
        const NEW_SELL_IN: i32 = 4;
        const NEW_QUALITY: i32 = 2;

        test_item(ITEM_NAME, SELL_IN, QUALITY, NEW_SELL_IN, NEW_QUALITY);
    }

    #[test]
    pub fn generic_item_expired() {
        const ITEM_NAME: &str = "Foo's Bar Buz";
        const SELL_IN: i32 = 0;
        const QUALITY: i32 = 4;
        const NEW_SELL_IN: i32 = -1;
        const NEW_QUALITY: i32 = 2;

        test_item(ITEM_NAME, SELL_IN, QUALITY, NEW_SELL_IN, NEW_QUALITY);
    }

    #[test]
    pub fn aged_item() {
        const ITEM_NAME: &str = "Aged Brie";
        const SELL_IN: i32 = 5;
        const QUALITY: i32 = 3;
        const NEW_SELL_IN: i32 = 4;
        const NEW_QUALITY: i32 = 4;

        test_item(ITEM_NAME, SELL_IN, QUALITY, NEW_SELL_IN, NEW_QUALITY);
    }

    #[test]
    pub fn aged_item_expired() {
        const ITEM_NAME: &str = "Aged Brie";
        const SELL_IN: i32 = 0;
        const QUALITY: i32 = 6;
        const NEW_SELL_IN: i32 = -1;
        const NEW_QUALITY: i32 = 8;

        test_item(ITEM_NAME, SELL_IN, QUALITY, NEW_SELL_IN, NEW_QUALITY);
    }

    #[test]
    pub fn backstage_passes() {
        const ITEM_NAME: &str = "Backstage passes to a TAFKAL80ETC concert";
        const SELL_IN: i32 = 11;
        const QUALITY: i32 = 7;
        const NEW_SELL_IN: i32 = 10;
        const NEW_QUALITY: i32 = 8;

        test_item(ITEM_NAME, SELL_IN, QUALITY, NEW_SELL_IN, NEW_QUALITY);
    }

    #[test]
    pub fn backstage_passes_ten_days() {
        const ITEM_NAME: &str = "Backstage passes to a TAFKAL80ETC concert";
        const SELL_IN: i32 = 10;
        const QUALITY: i32 = 8;
        const NEW_SELL_IN: i32 = 9;
        const NEW_QUALITY: i32 = 10;

        test_item(ITEM_NAME, SELL_IN, QUALITY, NEW_SELL_IN, NEW_QUALITY);
    }

    #[test]
    pub fn backstage_passes_five_days() {
        const ITEM_NAME: &str = "Backstage passes to a TAFKAL80ETC concert";
        const SELL_IN: i32 = 5;
        const QUALITY: i32 = 10;
        const NEW_SELL_IN: i32 = 4;
        const NEW_QUALITY: i32 = 13;

        test_item(ITEM_NAME, SELL_IN, QUALITY, NEW_SELL_IN, NEW_QUALITY);
    }

    #[test]
    pub fn backstage_passes_expired() {
        const ITEM_NAME: &str = "Backstage passes to a TAFKAL80ETC concert";
        const SELL_IN: i32 = 0;
        const QUALITY: i32 = 50;
        const NEW_SELL_IN: i32 = -1;
        const NEW_QUALITY: i32 = 0;

        test_item(ITEM_NAME, SELL_IN, QUALITY, NEW_SELL_IN, NEW_QUALITY);
    }

    #[test]
    pub fn legendary_item() {
        const ITEM_NAME: &str = "Sulfuras, Hand of Ragnaros";
        const SELL_IN: i32 = -1;
        const QUALITY: i32 = 80;

        let items: Vec<Item> = vec![Item::new(ITEM_NAME, SELL_IN, QUALITY)];

        let mut rose: GildedRose = GildedRose::new(items);
        for _ in 0..100 {
            rose.update_quality();
        }
        let rose: GildedRose = rose;

        assert_eq!(ITEM_NAME, rose.items[0].name);
        assert_eq!(SELL_IN, rose.items[0].sell_in);
        assert_eq!(QUALITY, rose.items[0].quality);
    }

    #[test]
    pub fn conjured_item() {
        const ITEM_NAME: &str = "Conjured Mana Cake";
        const GENERIC_ITEM_NAME: &str = "Baguette";
        const SELL_IN: i32 = 1;
        const QUALITY: i32 = 50;

        let items: Vec<Item> = vec![
            Item::new(ITEM_NAME, SELL_IN, QUALITY),
            Item::new(GENERIC_ITEM_NAME, SELL_IN, QUALITY),
        ];

        let mut rose: GildedRose = GildedRose::new(items);
        rose.update_quality();
        let rose: GildedRose = rose;
        assert_eq!(
            QUALITY.saturating_sub(rose.items[0].quality),
            QUALITY
                .saturating_sub(rose.items[1].quality)
                .saturating_mul(2)
        );
    }
}
