import { useRoute } from "@react-navigation/native";
import { Text, View, StyleSheet, ScrollView, useWindowDimensions } from "react-native";
import { Image } from 'expo-image';
import RenderHTML from "react-native-render-html";

export default RecipeDetailsScreen = () => {
    const route = useRoute();
    const { width } = useWindowDimensions();
    const {
        image,
        title,
        nutrition,
        summary,
        servings,
        veryPopular,
        veryHealthy,
        vegetarian,
        dishTypes,
        diets,
        cuisines,
        instructions
    } = route.params;

    let subheader = '';
    let [carbCount, carbUnit] = ['', ''];
    let [proteinCount, proteinUnit] = ['', ''];
    let [fatCount, fatUnit] = ['', ''];

    const calories = nutrition.nutrients.find(nutrient => nutrient.name === "Calories")
    const [calCount, calUnit] = [calories.amount, calories.unit]
    subheader = `${calCount} ${calUnit}`

    const carbs = nutrition.nutrients.find(nutrient => nutrient.name === "Carbohydrates")
    carbs;
    [carbCount, carbUnit] = [carbs.amount, carbs.unit]

    const proteins = nutrition.nutrients.find(nutrient => nutrient.name === "Protein")
    proteins;
    [proteinCount, proteinUnit] = [proteins.amount, proteins.unit]

    const fats = nutrition.nutrients.find(nutrient => nutrient.name === "Fat")
    fats;
    [fatCount, fatUnit] = [fats?.amount || "", fats?.unit || ""]

    const isPopularIcon = require("../../assets/fire.png");
    const isPopular = veryPopular;

    const isHealthyIcon = require("../../assets/body.png");
    const isHealthy = veryHealthy;

    const isVegetarianIcon = require("../../assets/plant-based.png");
    const isVegetarian = vegetarian;

    const servingCountIcon = require("../../assets/food-serving.png");
    const servingCount = servings;

    const renderNutrientRow = (nutrient) => {
        return (
            <View style={styles.row}>
                <Text style={[styles.cell, { width: "50%", }]}>{nutrient.name}</Text>
                <Text style={[styles.cell, { width: "30%", }]}>{nutrient.amount} {nutrient.unit}</Text>
                <Text style={[styles.cell, { width: "20%" }]}>{nutrient.percentOfDailyNeeds}</Text>
            </View>
        )
    }

    return (
        <ScrollView style={[styles.card]}>
            <Image source={image} style={styles.image} />
            <Text style={styles.subheader}>{subheader}</Text>
            <Text style={styles.title}>{title}</Text>

            <View>
                <Text>Carbs: {carbCount}{carbUnit} ({nutrition.caloricBreakdown.percentCarbs}%)</Text>
                <Text>Protein: {proteinCount}{proteinUnit} ({nutrition.caloricBreakdown.percentProtein}%)</Text>
                <Text>Fat: {fatCount}{fatUnit} ({nutrition.caloricBreakdown.percentFat}%)</Text>
            </View>

            <View style={styles.icon}>
                <Image source={isPopularIcon} style={styles.iconImage} />
                <Text>{isPopular ? "Popular" : "Not Popular"}</Text>
            </View>
            <View style={styles.icon}>
                <Image source={isHealthyIcon} style={styles.iconImage} />
                <Text>{isHealthy ? "Healthy" : "Not Healthy"}</Text>
            </View>
            <View style={styles.icon}>
                <Image source={isVegetarianIcon} style={styles.iconImage} />
                <Text>{isVegetarian ? "Vegetarian" : "Not Vegetarian"}</Text>
            </View>
            <View style={styles.icon}>
                <Image source={servingCountIcon} style={styles.iconImage} />
                <Text>{servingCount} Servings</Text>
            </View>

            <View style={styles.separator} />
            {
                dishTypes.length > 0 && (
                    <View>
                        <Text style={{ fontWeight: "bold" }}>Dish Types</Text>
                        <Text>{dishTypes.map((dishType) => dishType + ", ")}</Text>
                        <View style={styles.separator} />
                    </View>
                )
            }

            {
                cuisines.length > 0 && (
                    <View>
                        <Text style={{ fontWeight: "bold" }}>Cuisines</Text>
                        <Text>{cuisines.map((cuisine) => cuisine + ", ")}</Text>
                        <View style={styles.separator} />
                    </View>
                )
            }

            {
                diets.length > 0 && (
                    <View>
                        <Text style={{ fontWeight: "bold" }}>Diets</Text>
                        <Text>{diets.map((diet) => diet + ", ")}</Text>
                        <View style={styles.separator} />
                    </View>
                )
            }

            {/* TODO: Display a nutrition table as well */}

            <View>
                <View style={styles.row}>
                    <Text style={[styles.cell, { width: "50%", fontWeight: "bold" }]}>Nutrients</Text>
                    <Text style={[styles.cell, { width: "30%", fontWeight: "bold" }]}>Amt.</Text>
                    <Text style={[styles.cell, { width: "20%", fontWeight: "bold" }]}>% daily needs</Text>

                </View>
                {
                    nutrition.nutrients.map((nutrient, key) => (
                        // <Text key={key}>{nutrient.name} {nutrient.amount}</Text>
                        renderNutrientRow(nutrient)
                    ))
                }
            </View>

            <View>
                <Text style={{ fontWeight: "bold" }}>Recipe Instructions</Text>
                {
                    instructions.map(({ name, steps }, index) => (
                        <View key={index}>
                            <Text>{name}</Text>
                            {
                                steps.map((step) => (
                                    <Text style={{ marginVertical: 10 }}>{step.number}) {step.step}</Text>
                                ))
                            }
                        </View>
                    ))
                }
            </View>

            <View style={styles.separator} />
            <Text style={{ fontWeight: "bold", marginBottom: 15 }}>Summary</Text>
            <RenderHTML source={{ html: summary }} contentWidth={width} />


        </ScrollView>
    );
}

const styles = StyleSheet.create({
    card: {
        borderBottomWidth: 1,
        borderColor: '#ccc',
        padding: 16,
        marginBottom: 16,
    },
    image: {
        width: "100%",
        height: 150,
        marginBottom: 8,
    },
    separator: {
        borderBottomColor: '#bcc',
        borderBottomWidth: StyleSheet.hairlineWidth,
        marginVertical: 10
    },
    icon: {
        flexDirection: "row",
        alignItems: "center",
        marginVertical: 10
    },
    iconImage: {
        width: 24,
        height: 24,
        marginRight: 8
    },
    subheader: {
        fontSize: 12,
        marginBottom: 4,
        color: '#888',
    },
    title: {
        fontSize: 18,
        fontWeight: 'bold',
        marginBottom: 8,
    },
    row: {
        flexDirection: "row",
        alignItems: "center",
        justifyContent: "flex-start",
    },
    cell: {
        borderWidth: 1,
        borderColor: '#ccc',
        padding: 8,
    }
});