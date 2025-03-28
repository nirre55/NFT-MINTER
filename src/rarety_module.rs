multiversx_sc::imports!();
multiversx_sc::derive_imports!();

const MAX_NFT_COUNTRY_BORDER: usize = 258;

use crate::storage;

#[derive(TopEncode, TopDecode, Clone, Copy)]
pub enum RaretyProperties {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl RaretyProperties {
    pub fn drop_rate(&self) -> usize {
        match self {
            RaretyProperties::Common => 50,     // 50% de chance
            RaretyProperties::Uncommon => 30,   // 30% de chance
            RaretyProperties::Rare => 15,       // 15% de chance
            RaretyProperties::Epic => 4,        // 4% de chance
            RaretyProperties::Legendary => 1,   // 1% de chance
        }
    }
}

#[multiversx_sc::module]
pub trait RaretyModule: storage::StorageModule {

    // private

    fn get_storage_by_rarety(&self, rarety_storage: RaretyProperties) -> VecMapper<usize> {
        match rarety_storage {
            RaretyProperties::Common => self.common_index(),
            RaretyProperties::Uncommon => self.uncommon_index(),
            RaretyProperties::Rare => self.rare_index(),
            RaretyProperties::Epic => self.epic_index(),
            RaretyProperties::Legendary => self.legendary_index(),
        }
    }
    
    fn add_element_in_storage(&self, rarety_storage: RaretyProperties, valeur: usize) {
        match rarety_storage {
            RaretyProperties::Common => self.common_index().push(&valeur),
            RaretyProperties::Uncommon => self.uncommon_index().push(&valeur),
            RaretyProperties::Rare => self.rare_index().push(&valeur),
            RaretyProperties::Epic => self.epic_index().push(&valeur),
            RaretyProperties::Legendary => self.legendary_index().push(&valeur),
        };
    }

    fn pick_random_item(&self, rarety_storage: RaretyProperties) -> usize {
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
    
    fn fill_storage_randomly(&self, source_rarety_storage: RaretyProperties, destination_rarety_storage: RaretyProperties, count: usize) {
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

    fn fill_storage_with_max_elements(&self, rarety_storage: RaretyProperties, max_value: usize) {
        for value in 1..=max_value {
            self.add_element_in_storage(rarety_storage.clone(), value);
        }
    }

    fn clean_all_sotrage(&self){
        self.common_index().clear();
        self.uncommon_index().clear();
        self.rare_index().clear();  
        self.epic_index().clear();
        self.legendary_index().clear();
    }


    fn fill_all_storage(&self, source_rarety_storage: RaretyProperties) {      

        self.fill_storage_with_max_elements(source_rarety_storage.clone(), MAX_NFT_COUNTRY_BORDER);
        let total_objects = self.get_storage_by_rarety(source_rarety_storage).len();
        //let total_objects_communs = self.get_percentage_count(RaretyProperties::Common.drop_rate(), total_objects);
        let total_objects_uncommons = self.get_percentage_count(RaretyProperties::Uncommon.drop_rate(), total_objects);
        let total_objects_rares = self.get_percentage_count(RaretyProperties::Rare.drop_rate(), total_objects);
        let total_objects_epics = self.get_percentage_count(RaretyProperties::Epic.drop_rate(), total_objects);
        let total_objects_legendarys = self.get_percentage_count(RaretyProperties::Legendary.drop_rate(), total_objects);

        //self.fill_storage_randomly(source_rarety_storage.clone(), RaretyProperties::Common, total_objects_communs);
        self.fill_storage_randomly(source_rarety_storage.clone(), RaretyProperties::Uncommon, total_objects_uncommons);
        self.fill_storage_randomly(source_rarety_storage.clone(), RaretyProperties::Rare, total_objects_rares);
        self.fill_storage_randomly(source_rarety_storage.clone(), RaretyProperties::Epic, total_objects_epics);
        self.fill_storage_randomly(source_rarety_storage.clone(), RaretyProperties::Legendary, total_objects_legendarys);
    }


    
    
}