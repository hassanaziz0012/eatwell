import React from 'react';
import { Image } from 'expo-image';
import { View, Text, TouchableOpacity, StyleSheet } from 'react-native';
import DarkButton from './DarkButton';
import { createStackNavigator } from '@react-navigation/stack';
import SearchBar from './SearchBar';
import { NavigationContainer, useRoute } from '@react-navigation/native';


const Stack = createStackNavigator();

const RecipeCard = ({ image, title, nutrition, summary, servings, veryPopular, veryHealthy, vegetarian, diets, cuisines, dishTypes, analyzedInstructions, navigation }) => {
    let subheader = '';
    let [carbCount, carbUnit] = ['', ''];
    let [proteinCount, proteinUnit] = ['', ''];
    let [fatCount, fatUnit] = ['', ''];
    if (veryPopular) {
        subheader += 'Very Popular, ';
    }
    if (veryHealthy) {
        subheader += 'Very Healthy, ';
    }
    if (vegetarian) {
        subheader += 'Vegetarian, ';
    }
    if (nutrition) {
        const calories = nutrition.nutrients.find(nutrient => nutrient.name === "Calories")
        const [calCount, calUnit] = [calories.amount, calories.unit]
        subheader += `${calCount} ${calUnit}`

        const carbs = nutrition.nutrients.find(nutrient => nutrient.name === "Carbohydrates")
        carbs;
        [carbCount, carbUnit] = [carbs.amount, carbs.unit]

        const proteins = nutrition.nutrients.find(nutrient => nutrient.name === "Protein")
        proteins;
        [proteinCount, proteinUnit] = [proteins.amount, proteins.unit]

        const fats = nutrition.nutrients.find(nutrient => nutrient.name === "Fat")
        fats;
        [fatCount, fatUnit] = [fats?.amount || "", fats?.unit || ""]
    }

    return (
        <View style={styles.card}>
            <Image source={image} style={styles.image} />
            <Text style={styles.subheader}>{subheader}</Text>
            <Text style={styles.title}>{title}</Text>

            {nutrition && (
                <View>
                    <Text>Carbs: {carbCount}{carbUnit} ({nutrition.caloricBreakdown.percentCarbs}%)</Text>
                    <Text>Protein: {proteinCount}{proteinUnit} ({nutrition.caloricBreakdown.percentProtein}%)</Text>
                    <Text>Fat: {fatCount}{fatUnit} ({nutrition.caloricBreakdown.percentFat}%)</Text>
                </View>
            )}
            <DarkButton text={"View Recipe"} onPress={() => {
                navigation.navigate("RecipeDetails", {
                    image, 
                    title, 
                    summary,
                    nutrition,
                    servings,
                    veryPopular,
                    veryHealthy,
                    vegetarian,
                    dishTypes,
                    cuisines,
                    diets,
                    instructions: analyzedInstructions
                })
                console.log("navigating...")
            }} />
        </View>
    );
};

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
});

export default RecipeCard;
