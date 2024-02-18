//--------------------------    CAKE BLUEPRINT    --------------------------
// This is the main component of the cake module. It is responsible for
// creating the cake and managing the other components.
//--------------------------------------------------------------------------

use scrypto::prelude::*;

// Import the other components we need to create the cake
use crate::mixer::*;
use crate::oven::*;
use crate::recipe::*;

#[blueprint]
mod cake {
    struct Cake {
        // Define what resources and data will be managed by Cake components
        recipe: Global<recipe::Recipe>,
        mixer: Global<mixer::Mixer>,
        oven: Global<oven::Oven>,
    }

    impl Cake {
        
        pub fn instantiate_cake(is_vegan: bool) -> Global<Cake> {

            // Instantiate a Recipe component, and tell it how we want our cake
            let my_recipe = recipe::Recipe::instantiate_recipe(is_vegan);

            // Instantiate a Mixer component
            let my_mixer = mixer::Mixer::instantiate_mixer();

            // set the speed level and start the mixer
            my_mixer.adjust_speed(5);
            my_mixer.start();

            // Instantiate an Oven component and directly start it with the given temperature, duration and program
            // as we assume we know what we need to bake this cake
            let my_oven = oven::Oven::instantiate_oven(Decimal::from(180), 30, "top-bottom-heat".to_string());

            // Finally put everything together to build a cake
            Self {
                recipe: my_recipe,
                mixer: my_mixer,
                oven: my_oven,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        // Note: normally we would do all this in sequence, like
        // 1. Check the recipe and get all the ingredients
        // 2. Preheat the oven (with the values we get from the recipe)
        // 3. Start the mixer
        // 4. Stop the mixer
        // 5. Put the cake in the oven
        // 6. Stop the oven ...
        // But for the sake of this example we will keep it simple and
        // just bake the cake, assuming that the mixer and the oven stop by themselves

    }
}
