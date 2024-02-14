import { RefreshControl, ScrollView } from 'react-native';
import RecipeCard from '../components/RecipeCard';
import SearchBar from '../components/SearchBar';
import { useCallback, useEffect, useState } from 'react';
import { createStackNavigator } from '@react-navigation/stack';
import { useNavigation } from '@react-navigation/native';
import RecipeDetailsScreen from '../screens/RecipeDetailsScreen';
import MeScreen from '../screens/MeScreen';
import AboutScreen from '../screens/AboutScreen';
import HelpScreen from '../screens/HelpScreen';
import MyRecipesScreen from '../screens/MyRecipesScreen';

const Stack = createStackNavigator();

const MeTab = ({ handleLogout }) => {
    return (
        <Stack.Navigator>
            <Stack.Screen
                name="MeScreen"
                children={() => <MeScreen handleLogout={handleLogout} />}
                options={{ headerShown: false }}
            />
            <Stack.Screen
                name="About"
                component={AboutScreen}
                options={{ title: 'About' }}
            />
            <Stack.Screen
                name='Help'
                component={HelpScreen}
                options={{ title: 'Help' }}
            />
            <Stack.Screen
                name='MyRecipes'
                component={MyRecipesScreen}
                options={{ title: 'My Recipes'}}
            />
            {/* Add more screens as needed */}
        </Stack.Navigator>
    )
}

export default MeTab;