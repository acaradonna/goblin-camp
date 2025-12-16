/// Recipe registry and crafting system for workshops
///
/// This module defines the core types and functionality for the recipe system,
/// which manages crafting recipes, ingredient specifications, and production chains.
/// Recipes define how raw materials are transformed into finished goods through
/// various workshop stations.
use crate::components::ItemType;
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Specifies an ingredient required for a recipe
/// Defines both the type of item needed and the quantity required
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IngredientSpec {
    /// The type of item required as an ingredient
    pub item: ItemType,
    /// The number of items required
    pub count: u32,
}

impl IngredientSpec {
    /// Create a new ingredient specification
    pub fn new(item: ItemType, count: u32) -> Self {
        Self { item, count }
    }
}

/// Specifies a product produced by a recipe
/// Defines both the type of item produced and the quantity created
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductSpec {
    /// The type of item produced
    pub item: ItemType,
    /// The number of items produced
    pub count: u32,
    /// Whether this is a byproduct (true) or main product (false)
    /// Byproducts may have different handling rules in the future
    #[serde(default)]
    pub byproduct: bool,
}

impl ProductSpec {
    /// Create a new product specification
    pub fn new(item: ItemType, count: u32) -> Self {
        Self {
            item,
            count,
            byproduct: false,
        }
    }

    /// Create a new byproduct specification
    pub fn new_byproduct(item: ItemType, count: u32) -> Self {
        Self {
            item,
            count,
            byproduct: true,
        }
    }
}

/// A complete recipe definition for crafting operations
/// Recipes define the transformation of input ingredients into output products
/// through specific workshop stations over a defined time period
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recipe {
    /// Unique identifier for this recipe
    pub id: String,
    /// List of workshop station types that can execute this recipe
    pub stations: Vec<String>,
    /// List of required ingredients with quantities
    pub inputs: Vec<IngredientSpec>,
    /// List of produced items with quantities
    pub outputs: Vec<ProductSpec>,
    /// Time required to complete the recipe in simulation ticks
    pub work_time_ticks: u32,
}

impl Recipe {
    /// Create a new recipe
    pub fn new(
        id: String,
        stations: Vec<String>,
        inputs: Vec<IngredientSpec>,
        outputs: Vec<ProductSpec>,
        work_time_ticks: u32,
    ) -> Self {
        Self {
            id,
            stations,
            inputs,
            outputs,
            work_time_ticks,
        }
    }

    /// Validate the recipe for basic consistency
    /// Returns true if the recipe is valid, false otherwise
    pub fn validate(&self) -> bool {
        // Recipe must have an ID
        if self.id.is_empty() {
            return false;
        }

        // Recipe must have at least one station
        if self.stations.is_empty() {
            return false;
        }

        // Recipe must have at least one input
        if self.inputs.is_empty() {
            return false;
        }

        // Recipe must have at least one output
        if self.outputs.is_empty() {
            return false;
        }

        // All ingredient counts must be positive
        if self.inputs.iter().any(|spec| spec.count == 0) {
            return false;
        }

        // All product counts must be positive
        if self.outputs.iter().any(|spec| spec.count == 0) {
            return false;
        }

        // Work time must be positive
        if self.work_time_ticks == 0 {
            return false;
        }

        true
    }
}

/// Error types for recipe registry operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeRegistryError {
    /// Recipe with duplicate ID was found
    DuplicateRecipeId(String),
    /// Recipe failed validation
    InvalidRecipe(String),
    /// JSON parsing failed
    ParseError(String),
    /// Unknown item type was referenced
    UnknownItemType(String),
}

impl std::fmt::Display for RecipeRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecipeRegistryError::DuplicateRecipeId(id) => write!(f, "Duplicate recipe ID: {}", id),
            RecipeRegistryError::InvalidRecipe(id) => write!(f, "Invalid recipe: {}", id),
            RecipeRegistryError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            RecipeRegistryError::UnknownItemType(item) => write!(f, "Unknown item type: {}", item),
        }
    }
}

impl std::error::Error for RecipeRegistryError {}

/// Registry containing all available crafting recipes
/// This is the central repository for recipe data that gets loaded at startup
/// and used throughout the simulation for crafting operations
#[derive(Debug, Clone, Resource)]
pub struct RecipeRegistry {
    /// Map of recipe ID to recipe data
    recipes: HashMap<String, Recipe>,
}

impl RecipeRegistry {
    /// Create a new empty recipe registry
    pub fn new() -> Self {
        Self {
            recipes: HashMap::new(),
        }
    }

    /// Create a recipe registry from JSON data
    /// Validates all recipes and ensures no duplicate IDs
    pub fn from_json(json_data: &str) -> Result<Self, RecipeRegistryError> {
        // Parse the JSON structure
        let parsed: serde_json::Value = serde_json::from_str(json_data)
            .map_err(|e| RecipeRegistryError::ParseError(e.to_string()))?;

        // Extract the recipes array
        let recipes_array = parsed
            .get("recipes")
            .and_then(|v| v.as_array())
            .ok_or_else(|| {
                RecipeRegistryError::ParseError("Missing 'recipes' array".to_string())
            })?;

        let mut registry = Self::new();

        // Parse each recipe
        for recipe_value in recipes_array {
            let recipe: Recipe = serde_json::from_value(recipe_value.clone())
                .map_err(|e| RecipeRegistryError::ParseError(e.to_string()))?;

            // Validate the recipe
            if !recipe.validate() {
                return Err(RecipeRegistryError::InvalidRecipe(recipe.id));
            }

            // Check for duplicate IDs
            if registry.recipes.contains_key(&recipe.id) {
                return Err(RecipeRegistryError::DuplicateRecipeId(recipe.id));
            }

            registry.recipes.insert(recipe.id.clone(), recipe);
        }

        Ok(registry)
    }

    /// Load the default embedded recipe registry
    /// This provides the example recipes specified in the requirements
    pub fn load_default() -> Result<Self, RecipeRegistryError> {
        const DEFAULT_RECIPES_JSON: &str = include_str!("../resources/recipes.json");
        Self::from_json(DEFAULT_RECIPES_JSON)
    }

    /// Get a recipe by ID
    pub fn get_recipe(&self, id: &str) -> Option<&Recipe> {
        self.recipes.get(id)
    }

    /// Get all recipe IDs
    pub fn recipe_ids(&self) -> impl Iterator<Item = &String> {
        self.recipes.keys()
    }

    /// Get all recipes
    pub fn recipes(&self) -> impl Iterator<Item = &Recipe> {
        self.recipes.values()
    }

    /// Get the number of recipes in the registry
    pub fn len(&self) -> usize {
        self.recipes.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.recipes.is_empty()
    }

    /// Find recipes that can be performed at a specific station type
    pub fn recipes_for_station<'a>(
        &'a self,
        station_type: &str,
    ) -> impl Iterator<Item = &'a Recipe> + 'a {
        let station_type_owned = station_type.to_string();
        self.recipes
            .values()
            .filter(move |recipe| recipe.stations.contains(&station_type_owned))
    }
}

impl Default for RecipeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ingredient_spec_creation() {
        let spec = IngredientSpec::new(ItemType::Stone, 5);
        assert_eq!(spec.item, ItemType::Stone);
        assert_eq!(spec.count, 5);
    }

    #[test]
    fn product_spec_creation() {
        let spec = ProductSpec::new(ItemType::Block, 3);
        assert_eq!(spec.item, ItemType::Block);
        assert_eq!(spec.count, 3);
        assert!(!spec.byproduct);

        let byproduct = ProductSpec::new_byproduct(ItemType::Stone, 1);
        assert!(byproduct.byproduct);
    }

    #[test]
    fn recipe_validation_success() {
        let recipe = Recipe::new(
            "test_recipe".to_string(),
            vec!["workshop".to_string()],
            vec![IngredientSpec::new(ItemType::Log, 1)],
            vec![ProductSpec::new(ItemType::Plank, 4)],
            50,
        );

        assert!(recipe.validate());
    }

    #[test]
    fn recipe_validation_empty_id() {
        let recipe = Recipe::new(
            "".to_string(),
            vec!["workshop".to_string()],
            vec![IngredientSpec::new(ItemType::Log, 1)],
            vec![ProductSpec::new(ItemType::Plank, 4)],
            50,
        );

        assert!(!recipe.validate());
    }

    #[test]
    fn recipe_validation_zero_count() {
        let recipe = Recipe::new(
            "test_recipe".to_string(),
            vec!["workshop".to_string()],
            vec![IngredientSpec::new(ItemType::Log, 0)], // Zero count
            vec![ProductSpec::new(ItemType::Plank, 4)],
            50,
        );

        assert!(!recipe.validate());
    }

    #[test]
    fn recipe_validation_zero_work_time() {
        let recipe = Recipe::new(
            "test_recipe".to_string(),
            vec!["workshop".to_string()],
            vec![IngredientSpec::new(ItemType::Log, 1)],
            vec![ProductSpec::new(ItemType::Plank, 4)],
            0, // Zero work time
        );

        assert!(!recipe.validate());
    }

    #[test]
    fn recipe_registry_creation() {
        let registry = RecipeRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn parse_valid_json() {
        let json = r#"
        {
          "recipes": [
            {
              "id": "logs_to_planks",
              "stations": ["carpenter"],
              "inputs": [{ "item": "Log", "count": 1 }],
              "outputs": [{ "item": "Plank", "count": 4 }],
              "work_time_ticks": 50
            }
          ]
        }
        "#;

        let registry = RecipeRegistry::from_json(json).expect("Should parse valid JSON");
        assert_eq!(registry.len(), 1);

        let recipe = registry
            .get_recipe("logs_to_planks")
            .expect("Should have recipe");
        assert_eq!(recipe.id, "logs_to_planks");
        assert_eq!(recipe.stations, vec!["carpenter"]);
        assert_eq!(recipe.inputs.len(), 1);
        assert_eq!(recipe.inputs[0].item, ItemType::Log);
        assert_eq!(recipe.inputs[0].count, 1);
        assert_eq!(recipe.outputs.len(), 1);
        assert_eq!(recipe.outputs[0].item, ItemType::Plank);
        assert_eq!(recipe.outputs[0].count, 4);
        assert_eq!(recipe.work_time_ticks, 50);
    }

    #[test]
    fn reject_duplicate_recipe_ids() {
        let json = r#"
        {
          "recipes": [
            {
              "id": "duplicate",
              "stations": ["carpenter"],
              "inputs": [{ "item": "Log", "count": 1 }],
              "outputs": [{ "item": "Plank", "count": 4 }],
              "work_time_ticks": 50
            },
            {
              "id": "duplicate",
              "stations": ["mason"],
              "inputs": [{ "item": "Stone", "count": 1 }],
              "outputs": [{ "item": "Block", "count": 1 }],
              "work_time_ticks": 30
            }
          ]
        }
        "#;

        let result = RecipeRegistry::from_json(json);
        assert!(matches!(
            result,
            Err(RecipeRegistryError::DuplicateRecipeId(_))
        ));
    }

    #[test]
    fn reject_invalid_recipe() {
        let json = r#"
        {
          "recipes": [
            {
              "id": "invalid",
              "stations": ["carpenter"],
              "inputs": [{ "item": "Log", "count": 0 }],
              "outputs": [{ "item": "Plank", "count": 4 }],
              "work_time_ticks": 50
            }
          ]
        }
        "#;

        let result = RecipeRegistry::from_json(json);
        assert!(matches!(result, Err(RecipeRegistryError::InvalidRecipe(_))));
    }

    #[test]
    fn recipes_for_station_filtering() {
        let json = r#"
        {
          "recipes": [
            {
              "id": "logs_to_planks",
              "stations": ["carpenter"],
              "inputs": [{ "item": "Log", "count": 1 }],
              "outputs": [{ "item": "Plank", "count": 4 }],
              "work_time_ticks": 50
            },
            {
              "id": "stone_to_blocks",
              "stations": ["mason"],
              "inputs": [{ "item": "Stone", "count": 1 }],
              "outputs": [{ "item": "Block", "count": 1 }],
              "work_time_ticks": 50
            },
            {
              "id": "multi_station",
              "stations": ["carpenter", "mason"],
              "inputs": [{ "item": "Log", "count": 1 }],
              "outputs": [{ "item": "Block", "count": 1 }],
              "work_time_ticks": 100
            }
          ]
        }
        "#;

        let registry = RecipeRegistry::from_json(json).expect("Should parse");

        let carpenter_recipes: Vec<_> = registry.recipes_for_station("carpenter").collect();
        assert_eq!(carpenter_recipes.len(), 2);

        let mason_recipes: Vec<_> = registry.recipes_for_station("mason").collect();
        assert_eq!(mason_recipes.len(), 2);

        let nonexistent_recipes: Vec<_> = registry.recipes_for_station("blacksmith").collect();
        assert_eq!(nonexistent_recipes.len(), 0);
    }

    #[test]
    fn load_default_registry() {
        let registry = RecipeRegistry::load_default().expect("Should load default registry");

        // Should have the two example recipes
        assert_eq!(registry.len(), 2);

        // Verify logs_to_planks recipe
        let logs_recipe = registry
            .get_recipe("logs_to_planks")
            .expect("Should have logs_to_planks");
        assert_eq!(logs_recipe.stations, vec!["carpenter"]);
        assert_eq!(logs_recipe.inputs.len(), 1);
        assert_eq!(logs_recipe.inputs[0].item, ItemType::Log);
        assert_eq!(logs_recipe.inputs[0].count, 1);
        assert_eq!(logs_recipe.outputs.len(), 1);
        assert_eq!(logs_recipe.outputs[0].item, ItemType::Plank);
        assert_eq!(logs_recipe.outputs[0].count, 4);
        assert_eq!(logs_recipe.work_time_ticks, 50);

        // Verify stone_to_blocks recipe
        let stone_recipe = registry
            .get_recipe("stone_to_blocks")
            .expect("Should have stone_to_blocks");
        assert_eq!(stone_recipe.stations, vec!["mason"]);
        assert_eq!(stone_recipe.inputs.len(), 1);
        assert_eq!(stone_recipe.inputs[0].item, ItemType::Stone);
        assert_eq!(stone_recipe.inputs[0].count, 1);
        assert_eq!(stone_recipe.outputs.len(), 1);
        assert_eq!(stone_recipe.outputs[0].item, ItemType::Block);
        assert_eq!(stone_recipe.outputs[0].count, 1);
        assert_eq!(stone_recipe.work_time_ticks, 50);
    }
}
