import { RefreshControl, ScrollView } from 'react-native';
import RecipeCard from '../components/RecipeCard';
import SearchBar from '../components/SearchBar';
import { useCallback, useEffect, useState } from 'react';
import { useNavigation } from '@react-navigation/native';

export default BrowseScreen = () => {
    const [refreshing, setRefreshing] = useState(false);
    const navigation = useNavigation();

    const onRefresh = useCallback(() => {
        setRefreshing(true);
        setTimeout(() => {
            setRefreshing(false);
        }, 3000);
        getInitialRecipes();
    }, []);

    useEffect(() => {
        console.log("rerender");
        getInitialRecipes();
    }, []);

    const getInitialRecipes = () => {
        const host = "http://localhost:8080"
        const url = host + "/search-recipes?" + new URLSearchParams({
            query: '',
            num: 12,
            sorting: "random"
        });
        console.log(url);
        fetch(url)
            .then((response) => response.json())
            .then((data) => {
                setRecipes(data);
                console.log(refreshing);
                if (refreshing === true) {
                    setRefreshing(false);
                }
            })
            .catch((error) => {
                console.error('Error:', error);
            })
    }

    const [recipes, setRecipes] = useState([]);

    return (
        <ScrollView contentContainerStyle={{ flexGrow: 1, marginHorizontal: 8, backgroundColor: "#fff" }} refreshControl={
            <RefreshControl refreshing={refreshing} onRefresh={onRefresh} />
        }>
            <SearchBar setRecipes={setRecipes} />
            {recipes.map((recipe, index) => (
                <RecipeCard key={index} {...recipe} navigation={navigation} />
            ))}
        </ScrollView>
    );
};