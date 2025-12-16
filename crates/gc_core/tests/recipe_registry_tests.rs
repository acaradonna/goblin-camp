/// Integration tests for recipe registry system
/// Tests that the recipe registry is properly integrated into the ECS world
use gc_core::prelude::*;

#[test]
fn recipe_registry_available_in_standard_world() {
    let world = build_standard_world(20, 20, 42, WorldOptions::default());

    // Recipe registry should be available as a resource
    let registry = world.resource::<RecipeRegistry>();

    // Should contain the default recipes
    assert_eq!(registry.len(), 2);
    assert!(registry.get_recipe("logs_to_planks").is_some());
    assert!(registry.get_recipe("stone_to_blocks").is_some());
}

#[test]
fn recipe_registry_validation_during_world_build() {
    // Should not panic during world creation even with complex recipes
    let _world = build_standard_world(
        50,
        50,
        123,
        WorldOptions {
            populate_demo_scene: true,
            tick_ms: 100,
        },
    );
    // If we get here without panicking, the registry loaded successfully
}

#[test]
fn recipe_registry_resource_usage() {
    let world = build_standard_world(10, 10, 42, WorldOptions::default());

    // Should be able to query recipes from within a system context
    let registry = world.resource::<RecipeRegistry>();

    // Test station filtering functionality
    let carpenter_recipes: Vec<_> = registry.recipes_for_station("carpenter").collect();
    assert_eq!(carpenter_recipes.len(), 1);
    assert_eq!(carpenter_recipes[0].id, "logs_to_planks");

    let mason_recipes: Vec<_> = registry.recipes_for_station("mason").collect();
    assert_eq!(mason_recipes.len(), 1);
    assert_eq!(mason_recipes[0].id, "stone_to_blocks");
}

#[test]
fn recipe_ingredients_and_products_valid() {
    let world = build_standard_world(10, 10, 42, WorldOptions::default());
    let registry = world.resource::<RecipeRegistry>();

    // Verify logs_to_planks recipe structure
    let recipe = registry.get_recipe("logs_to_planks").unwrap();
    assert_eq!(recipe.inputs.len(), 1);
    assert_eq!(recipe.inputs[0].item, ItemType::Log);
    assert_eq!(recipe.inputs[0].count, 1);

    assert_eq!(recipe.outputs.len(), 1);
    assert_eq!(recipe.outputs[0].item, ItemType::Plank);
    assert_eq!(recipe.outputs[0].count, 4);
    assert!(!recipe.outputs[0].byproduct);

    // Verify stone_to_blocks recipe structure
    let recipe = registry.get_recipe("stone_to_blocks").unwrap();
    assert_eq!(recipe.inputs.len(), 1);
    assert_eq!(recipe.inputs[0].item, ItemType::Stone);
    assert_eq!(recipe.inputs[0].count, 1);

    assert_eq!(recipe.outputs.len(), 1);
    assert_eq!(recipe.outputs[0].item, ItemType::Block);
    assert_eq!(recipe.outputs[0].count, 1);
    assert!(!recipe.outputs[0].byproduct);
}

#[test]
fn recipe_validation_all_items_known() {
    let world = build_standard_world(10, 10, 42, WorldOptions::default());
    let registry = world.resource::<RecipeRegistry>();

    // All item types in recipes should be valid ItemType variants
    for recipe in registry.recipes() {
        for input in &recipe.inputs {
            // These should all be valid ItemType variants
            match input.item {
                ItemType::Stone | ItemType::Log | ItemType::Plank | ItemType::Block => {
                    // If new item types are added, they should be handled
                }
            }
        }

        for output in &recipe.outputs {
            match output.item {
                ItemType::Stone | ItemType::Log | ItemType::Plank | ItemType::Block => {}
            }
        }

        // All recipes should be valid
        assert!(recipe.validate());
    }
}
