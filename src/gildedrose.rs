use crate::item::Item;

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in self.items.iter_mut() {
            if item.name.starts_with("Sulfuras") {
                item.quality = 80;
                continue;
            }
            item.sell_in -= 1;
            let quality_factor = GildedRose::set_quality_factor(item);
            if item.quality < 50 || item.quality > 0 {
                item.quality -= quality_factor;
            }
        }
    }

    fn set_quality_factor(current_item: &Item) -> i32 {
        if current_item.name.starts_with("Aged") {
            return -1;
        }

        if current_item.name.starts_with("Backstage passes") {
            match current_item.sell_in as u32 {
                10 | 9 | 8 | 7 | 6 => return -2,
                5 | 4 | 3 | 2 | 1 => return -3,
                0 => return current_item.quality,
                _ => return -1,
            }
        }
        let quality_factor = if current_item.name.starts_with("Conjured") {
            2
        } else {
            1
        };

        if current_item.sell_in < 0 {
            return quality_factor * 2;
        }
        quality_factor
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};

    #[test]
    pub fn foo() {
        let items = vec![Item::new("foo", 0, 0)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!("fixme", rose.items[0].name);
    }
}
