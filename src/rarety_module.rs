multiversx_sc::imports!();
multiversx_sc::derive_imports!();

const MAX_NFT_COUNTRY_BORDER: usize = 258;

use crate::storage;

#[derive(TopEncode, TopDecode, Clone, Copy)]
pub enum RarityProperties {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl RarityProperties {
    pub fn drop_rate(&self) -> usize {
        match self {
            RarityProperties::Common => 50,     // 50% de chance
            RarityProperties::Uncommon => 30,   // 30% de chance
            RarityProperties::Rare => 15,       // 15% de chance
            RarityProperties::Epic => 4,        // 4% de chance
            RarityProperties::Legendary => 1,   // 1% de chance
        }
    }
}

#[multiversx_sc::module]
pub trait RaretyModule: storage::StorageModule {

    // private

    fn get_storage_by_rarety(&self, rarety_storage: RarityProperties) -> VecMapper<usize> {
        match rarety_storage {
            RarityProperties::Common => self.common_items(),
            RarityProperties::Uncommon => self.uncommon_items(),
            RarityProperties::Rare => self.rare_items(),
            RarityProperties::Epic => self.epic_items(),
            RarityProperties::Legendary => self.legendary_items(),
        }
    }
    
    fn add_element_in_storage(&self, rarety_storage: RarityProperties, valeur: usize) {
        match rarety_storage {
            RarityProperties::Common => self.common_items().push(&valeur),
            RarityProperties::Uncommon => self.uncommon_items().push(&valeur),
            RarityProperties::Rare => self.rare_items().push(&valeur),
            RarityProperties::Epic => self.epic_items().push(&valeur),
            RarityProperties::Legendary => self.legendary_items().push(&valeur),
        };
    }

    fn pick_random_item(&self, rarety_storage: RarityProperties) -> usize {
        let source = self.get_storage_by_rarety(rarety_storage);
        let total_objects = source.len();
        
        require!(total_objects > 0, "Storage is empty");

        let mut rand_source = RandomnessSource::new();
        let rand_index = rand_source.next_usize_in_range(0, total_objects);

        source.get(rand_index)
    }

    fn get_percentage_count(&self, percentage: usize, total_objects: usize) -> usize {
        require!(percentage > 0 && percentage <= 100, "Invalid percentage");
        require!(total_objects > 0, "Total objects must be greater than zero");
    
        (total_objects * percentage as usize) / 100
    }
    
    fn fill_storage_randomly(&self, source_rarety_storage: RarityProperties, destination_rarety_storage: RarityProperties, count: usize) {
        let mut source = self.get_storage_by_rarety(source_rarety_storage);
        let mut destination = self.get_storage_by_rarety(destination_rarety_storage);
    
        let total_objects = source.len();
        require!(total_objects >= count, "Not enough elements in source storage");
    
        let mut rand_source = RandomnessSource::new();
    
        for _ in 0..count {
            let rand_index = rand_source.next_usize_in_range(0, source.len());
            let selected_item = source.get(rand_index);
    
            // Ajouter à `destination_rarety_storage`
            destination.push(&selected_item);
    
            // Supprimer de `source_rarety_storage`
            source.swap_remove(rand_index);
        }
    }

    fn fill_storage_with_max_elements(&self, rarety_storage: RarityProperties, max_value: usize) {
        for value in 1..=max_value {
            self.add_element_in_storage(rarety_storage.clone(), value);
        }
    }

    fn clean_all_sotrage(&self){
        self.common_items().clear();
        self.uncommon_items().clear();
        self.rare_items().clear();  
        self.epic_items().clear();
        self.legendary_items().clear();
    }


    fn fill_all_storage(&self, source_rarety_storage: RarityProperties) {      

        self.fill_storage_with_max_elements(source_rarety_storage.clone(), MAX_NFT_COUNTRY_BORDER);
        let total_objects = self.get_storage_by_rarety(source_rarety_storage).len();
        //let total_objects_communs = self.get_percentage_count(RarityProperties::Common.drop_rate(), total_objects);
        let total_objects_uncommons = self.get_percentage_count(RarityProperties::Uncommon.drop_rate(), total_objects);
        let total_objects_rares = self.get_percentage_count(RarityProperties::Rare.drop_rate(), total_objects);
        let total_objects_epics = self.get_percentage_count(RarityProperties::Epic.drop_rate(), total_objects);
        let total_objects_legendarys = self.get_percentage_count(RarityProperties::Legendary.drop_rate(), total_objects);

        //self.fill_storage_randomly(source_rarety_storage.clone(), RarityProperties::Common, total_objects_communs);
        self.fill_storage_randomly(source_rarety_storage.clone(), RarityProperties::Uncommon, total_objects_uncommons);
        self.fill_storage_randomly(source_rarety_storage.clone(), RarityProperties::Rare, total_objects_rares);
        self.fill_storage_randomly(source_rarety_storage.clone(), RarityProperties::Epic, total_objects_epics);
        self.fill_storage_randomly(source_rarety_storage.clone(), RarityProperties::Legendary, total_objects_legendarys);
    }

    fn drop_item(&self) -> usize {
        let mut rand_source = RandomnessSource::new();
        let random_percentage = rand_source.next_u64_in_range(0, 10000);
    
        let rarity  = match random_percentage {
            0..=4199 => RarityProperties::Common,
            4200..=7499 => RarityProperties::Uncommon,
            7500..=9089 => RarityProperties::Rare,
            9090..=9989 => RarityProperties::Epic,
            _ => RarityProperties::Legendary,
        };
    
        self.get_random_item(rarity)
    }

    fn get_random_item(&self, rarity: RarityProperties) -> usize {
        let items = self.get_storage_by_rarety(rarity);
        let mut rand_source = RandomnessSource::new();
        let random_index = rand_source.next_u64_in_range(0, items.len() as u64) as usize;
        items.get(random_index)
    }
    
    
}