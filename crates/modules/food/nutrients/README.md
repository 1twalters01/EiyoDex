# Nutrients
Structs for nutrients

## Features
* Has comprehensive default nutrient profile
    * Nutrients can be toggled on/off, thus not counting them
    * Nutrients can be hidden, which does effect counting
    * Nothing can be moved, deleted or added on default profile
    * Equations for max and min value (or none)
        * Can be hidden for each nutrient (or all)
    * Equations for Top and Bottom ideal range (or none)
        * Can be hidden for each nutrient (or all)

* User can create their own nutrient profiles
    * These have the same basic rules as the default
    * User created nutrient profiles can be renamed (no duplicate names are allowed)
    * All fields can be renamed in a nutrient profile
    * Can add or remove new nutrients
    * User can add or remove guide bars (max min)
    * Allows user to move where nutrients are in hierarchy

## Default Fields
* Energy
    * Carbohydrates
        * Fiber
        * Starch
        * Sugar
            * Allulose
            * Fructose
            * Galactose
            * Glucose
            * Lactose
            * Maltose
            * Sucrose
        * Sugar Alcohols
            * Erythritol
            * Inositol (also a conditionally essential nutrient)
            * Mannitol
            * Sorbitol
            * Xylitol
        * Of Which Added
        * Of which net (not including Fiber or Sugar alcohols)
    * Proteins
        * Essential Amino Acids
            * Histidine
            * Isoleucine
            * Leucine
            * Lysine
            * Methionine
            * Phenylalanine
            * Threonine
            * Tryptophan
            * Valine
        * Conditionally Essential Amino Acids
            * Arginine
            * Cystine
            * Glutamine
            * Glycine
            * Ornithine
            * Proline
            * Serine
            * Tyrosine
        * Non-essential Amino Acids
            * Alanine
            * Aspartate
            * Glutamate
            * Hydroxyproline
    * Lipids
        * Fats
            * Monounsaturated
            * Polyunsaturated
                * Omega 3
                    * ALA
                    * DHA
                    * EPA
                * Omega 6
                * Omega 9
            * Saturated
        * Trans-Fats
            * Natural
            * Artificial
        * Cholesterol
        * Phytosterol
        * Phospholipids
    * Alcohol

* Water

* Vitamins
    * Vitamin A
        * Retinol
    * Vitamin B
        * Thiamine (B1)
        * Riboflavin (B2)
        * Niacin (B3)
        * Pantothenic Acid (B5)
        * Pyridoxine (B6)
        * Biotin (B7/H)
        * Folate (B9)
        * Cobalamin (B12)
    * Vitamin C
    * Vitamin D
        * Ergocalciferol (D2)
        * Cholecalciferol (D3)
    * Vitamin E
        * Tocopherols
            * Alpha-tocopherol
            * Beta-tocopherol
            * Gamma-tocopherol
            * Delta-tocopherol
        * Tocotrienols
            * Alpha-tocotrienol
            * Beta-tocotrienol
            * Gamma-tocotrienol
            * Delta-tocotrienol
    * Vitamin K
        * K1 (Phylloquinone)
        * K2 (Menaquinones)
            * MK-4
            * MK-7
            * MK-8
            * MK-9
        * K3 (Menadione)

* Minerals
    * Boron
    * Calcium
    * Chloride
    * Chromium
    * Copper
    * Fluoride
    * Iodide
    * Iron
    * Magnesium
    * Manganese
    * Molybdenum
    * Nickel
    * Phosphorus
    * Potassium
    * Selenium
    * Silicon
    * Sodium
    * Vanadium
    * Zinc

* Other Essential Nutrients
    * Choline

* Conditionally Essential Nutrients
    * Carnitine
    * Coenzyme Q10
    * Lipoic acid
    * Taurine
    * Inositol (Also a Sugar Alcohol)

* Phytonutrients
    * Carotenoids
        * Alpha-Carotene
        * Beta-Carotene
        * Lutein
        * Zeaxanthin
        * Lycopene
    * Polyphenols (may have antinutrient effects)
        * Flavonoids
        * Lignans
        * Tannin

* Antinutrients
    * Goitrogens
    * Glucosinolates
    * Lectins
    * Oxalate
    * Phytates
    * Trypsin inhibitors
    * Polyphenols