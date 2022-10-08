use scrypto::prelude::*;

blueprint! {
    struct Jamon {
        // Define what resources and data will be managed by Jamon components
        sample_vault: Vault
    }

    impl Jamon {
        // This is a function, and can be called directly on the blueprint once deployed
        pub fn instantiate_jamon() -> ComponentAddress {
            // Create a new token called "Jamon," 
            let my_bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Jamon")
                .metadata("symbol", "JMN")
                .initial_supply(100000000);

            Self {
                sample_vault: Vault::with_bucket(my_bucket)
            }
            .instantiate()
            .globalize()
        }
     
        pub fn available_token(&mut self){
            info!("Tengo: {} Jamon Token Creados", self.sample_vault.amount());
            //Declario una variable para calcular el 20% de los token que se pueden reagalar.
            let gift = (self.sample_vault.amount())* 20/100;
            info!("Tengo: {} Jamon Token Disponibles para regalar", gift);
        }
    }
}