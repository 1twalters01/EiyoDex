# Profiles
User profiles crate

Modularise the parameters for health and nutrition and make them optional in the future?

## Define different user profiles
Takes in:
* Name
* Preferred Units
* Language
* Date of Birth
* Ethnicity (optional)
* Gender
* Height
* Weight
    * Current Weight Logs
    * Target Weight (Toggleable)
    * Desired Date (Toggleable)
* Waist?
* Hip?
* Body Fat Percentage (Estimate) Logs

* Energy & Macronutrient Targets (Optional, can be calculated)
* Basal Metabolic Rate (Can be Calculated)
* Thermic Effect of Food (Toggleable)
* Baseline Activity Enum (Toggleable)
* Custom Energy Target (Toggleable)
* Track time added (Toggleable)
    * Default to current time (Toggleable)
* Meal time profiles
    * Select default meal category loadout
        * Choose whose public/custom (shared with you) meal category loadouts you can choose from
    * Create meal category loadout (e.g [brunch, dinner] or [breakfast, lunch, dinner, snack])
    * Make Meal Category Loadout public to all profiles, private or customly seen (Toggleable)
        * Choose who can see your Meal Category Loadout
    * Select profiles whose Meal Category Loadouts
* Show Nutrient Balances (Which ones, Toggleable)
* Track Nutrient Targets (Select Which Are Wanted, Customise Ranges (Based on chosen range options), Toggle default values)
    * General
    * Vitamins
    * Minerals
    * Carbohydrates
    * Lipids
    * Proteins
    * Custom (if applicable)
* Show price on food (toggleable)

Calculates:
* BMI
* BRI
* Basal Metabolic Rate
* Macronutrient Ratios or Percentages
* Baseline Activity Level (From the Enum)
* Thermic Effect of Food (Toggleable) 
* Default Nutrient Values
