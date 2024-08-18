# Tandoor Importer

## Description
This is a nutrient importer for [Tandoor](https://tandoor.dev/), an open source recipe manager.
Tandoor has the possibility to add `Foods` which then can be used in recipes. For the different `Foods` properties like "Sugars", "Fats" or "Energy" can be added. 
These will be used to calculate the total nutrients of a recipe.
Because adding all the properties to a food item by hand is quite annoying I built this importer to import data provided by the [USDA Food Database](https://fdc.nal.usda.gov/fdc-app.html#/food-search?query=&type=Foundation) using the [API](https://fdc.nal.usda.gov/api-spec/fdc_api.html).

## How does it work?
1. The program reads the [appsettings.json](./appsettings.template.json) to find the Tandoor instance and to get the needed API-Keys for Tandoor and the USDA FDC database.
2. After that all properties and all foods are retrieved from the Tandoor instance.
3. For each food item retrieved from Tandoor the FDC ID of that food item (the ID is retrieved primarily from the "URL" field of a `Food`. If no URL is given the program tries to get it from the "FDC ID" field, or asks the user if the program is run in interactive mode) is used to query the FDC database for nutrients.
4. All nutrients of a food item are retrieved, then the nutrients that are not present in Tandoor are filtered out.
5. The data retrieved from the FDC database is added to the Tandoor food.
6. The updated food is pushed to the Tandoor database.

## Prerequisites
1. Create an API key for your Tandoor instance under `<your-tandoor-endpoint>/settings` in the API section. Make sure the token has `read write` as the scope.
2. Create an API key for the USDA FDC database by signing up [here](https://fdc.nal.usda.gov/api-key-signup.html). The API key will be sent to you via e-mail.
3. Open the FDC page for every food you want to update (e.g. [this](https://fdc.nal.usda.gov/fdc-app.html#/food-details/169661/nutrients) for maple syrup) 
   - Either copy the URL into the "URL" field of the food. The field can be found by editing a food item and going to the "More" section.
   - Or go to the admin view of your foods `<your-tandoor-endpoint>/admin/cookbook/food/` and edit the food there to enter the FDC ID.
   - Note that the URL will be preferred if URL and FDC ID are set.
   - You can also run the program in interactive mode (`./tandoor_importer --interactive`) to be asked for an FDC ID if your food does not have one assigned.
4. Make sure that every property you have created in Tandoor also has the corresponding FDC ID assigned so the matching can work.

## Usage
1. Copy [appsettings.template.json](./appsettings.template.json) to `appsettings.json` and add your Tandoor endpoint as well as the API key to it.
2. Run the program using `./tandoor_importer`. Refer to the [Parameters](#parameters) section for configuration.
3. All food items for which an FDC ID was assigned should now have values for all your properties.

## Parameters

### Flags
| Name          | short name | Description                                                                 | Required? | Default |
|---------------|------------|-----------------------------------------------------------------------------|-----------|---------|
| --interactive | -i         | When set the program asks the user to provide an FDC ID when none was found | No        | false   |
| --override    | -o         | When set the program overrides properties that are already present.         | No        | false   |


### Parameters with value
| Name        | short name | Description                                                     | Required? | Default |
|-------------|------------|-----------------------------------------------------------------|-----------|---------|
| --log-level | -l         | Sets the log level [One of: trace, debug, info, warning, error] | No        | info    |