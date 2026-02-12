use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub inputs: HashMap<String, u32>,
    pub outputs: HashMap<String, u32>,
    pub process_time_seconds: u32,
    pub energy_cost_kw: u32, 
}

pub struct SupplyChainGraph {
    recipes: HashMap<String, Recipe>,
    adjacency_list: HashMap<String, Vec<String>>, // Product -> Recipes that produce it
}

impl SupplyChainGraph {
    pub fn new() -> Self {
        let mut graph = SupplyChainGraph {
            recipes: HashMap::new(),
            adjacency_list: HashMap::new(),
        };

        // Hardcode "Iron -> Steel" Recipe (ADR-028)
        let mut inputs = HashMap::new();
        inputs.insert("Iron Ore".to_string(), 250);
        inputs.insert("Fuel".to_string(), 20); // Propellant/Fuel

        let mut outputs = HashMap::new();
        outputs.insert("Steel".to_string(), 100);

        let recipe = Recipe {
            inputs,
            outputs,
            process_time_seconds: 10, // Placeholder
            energy_cost_kw: 480, // ADR-024
        };

        graph.add_recipe("Refine Steel", recipe);

        graph
    }

    pub fn add_recipe(&mut self, name: &str, recipe: Recipe) {
        self.recipes.insert(name.to_string(), recipe.clone());
        for output in recipe.outputs.keys() {
            self.adjacency_list.entry(output.clone()).or_insert_with(Vec::new).push(name.to_string());
        }
    }

    pub fn find_production_path(&self, target_resource: &str) -> Option<Vec<String>> {
        if !self.adjacency_list.contains_key(target_resource) {
            return None;
        }
        self.adjacency_list.get(target_resource).cloned()
    }

    /// Calculate profitability of a recipe given current market prices.
    /// Formula: Profit = (Revenue) - (Cost of Goods + Energy + Fees)
    pub fn calculate_profitability(
        &self, 
        recipe_name: &str, 
        market_prices: &HashMap<String, f64>
    ) -> Result<f64> {
        let recipe = self.recipes.get(recipe_name)
            .ok_or_else(|| anyhow::anyhow!("Recipe not found: {}", recipe_name))?;

        let mut revenue = 0.0;
        for (product, query_qty) in &recipe.outputs {
            let price = market_prices.get(product).unwrap_or(&0.0);
            revenue += price * (*query_qty as f64);
        }

        let mut cost = 0.0;
        for (input, query_qty) in &recipe.inputs {
            let price = market_prices.get(input).unwrap_or(&0.0);
            cost += price * (*query_qty as f64);
        }
        
        // Add Energy Cost (Simplified: 1 kW = $0.001 for now, or just treated as 0 if not provided)
        // User didn't provide Energy Price in map, so we'll assume it's part of the 'Cost Energy' term
        // For now, we'll neglect explict energy price unless passed in map.
        
        Ok(revenue - cost)
    }
}
