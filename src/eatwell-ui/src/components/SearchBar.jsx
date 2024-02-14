import React, { useState } from 'react';
import { View, TextInput, TouchableOpacity, Text, StyleSheet } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import SortButton from './SortButton';
import FilterButton from './FilterButton';
import SortDirectionButton from './SortDirectionButton';

const SearchBar = ({ setRecipes }) => {
    const [searchText, setSearchText] = useState('');
    const [sortDirection, setSortDirection] = useState("Ascending");
    const [sortOption, setSortOption] = useState("Default");
    const [selectedDiets, setSelectedDiets] = useState([]);
    const [selectedMealTypes, setSelectedMealTypes] = useState([]);
    const [selectedCuisines, setSelectedCuisines] = useState([]);

    const handleSearch = () => {
        // TODO: Configure diets, sorts, meal types, etc. all these filters.
        const host = "http://localhost:8080"
        const url = host + "/search-recipes?" + new URLSearchParams({
            query: searchText,
            num: 12,
            sort_direction: sortDirection,
            sorting: sortOption,
            diet: selectedDiets,
            meal_types: selectedMealTypes,
            cuisines: selectedCuisines
        });
        console.log(url);
        fetch(url)
        .then((response) => {
            return response.json();
        })
        .then((data) => {
            setRecipes(data);
        })
        .catch((error) => {
            console.error('Error:', error);
        })
    }

    return (
        <View style={styles.searchContainer}>
            <View style={styles.searchBar}>
                <TextInput
                    placeholder="Search..."
                    style={styles.searchInput}
                    onChangeText={(text) => setSearchText(text)}
                // Add functionality for handling text input
                />
                <TouchableOpacity style={styles.searchIcon}>
                    <Ionicons name="ios-search" size={24} color="black" />
                </TouchableOpacity>
            </View>
            <View style={styles.buttonsContainer}>
                <View style={{ display: "flex", flexDirection: "row" }}>
                    <FilterButton
                        selectedCuisineState={[selectedCuisines, setSelectedCuisines]}
                        selectedDietState={[selectedDiets, setSelectedDiets]}
                        selectedMealTypeState={[selectedMealTypes, setSelectedMealTypes]}
                    />
                    <SortButton sortOptionState={[sortOption, setSortOption]} />
                    <SortDirectionButton directionState={[sortDirection, setSortDirection]} />
                </View>
                <TouchableOpacity style={styles.searchButton} onPress={handleSearch}>
                    <Ionicons name="ios-search" size={20} color="black" />
                    <Text>Search</Text>
                </TouchableOpacity>
            </View>
        </View>
    );
};

const styles = StyleSheet.create({
    searchContainer: {
        flexDirection: 'column',
        alignItems: 'stretch',
        display: 'block',
        paddingTop: 20
    },
    searchBar: {
        flexDirection: 'row',
        borderWidth: 1,
        borderColor: '#ccc',
        borderRadius: 5,
        overflow: 'hidden',
        alignItems: 'center',
        marginBottom: 10,
    },
    searchInput: {
        flex: 1,
        padding: 8,
    },
    searchIcon: {
        backgroundColor: '#ccc',
        padding: 8,
    },
    buttonsContainer: {
        flexDirection: 'row',
        alignItems: 'center',
        justifyContent: "space-between",
    },
    searchButton: {
        flexDirection: 'row',
        borderWidth: 1,
        borderColor: '#ccc',
        borderRadius: 5,
        padding: 8,
        alignItems: 'center',
    },
});

export default SearchBar;
