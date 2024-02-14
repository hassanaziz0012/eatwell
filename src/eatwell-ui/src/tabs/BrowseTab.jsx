import { createStackNavigator } from '@react-navigation/stack';
import BrowseScreen from '../screens/BrowseScreen';
import RecipeDetailsScreen from '../screens/RecipeDetailsScreen';

const Stack = createStackNavigator();

const BrowseTab = () => {
    return (
        <Stack.Navigator>
            <Stack.Screen
                name="BrowseScreen"
                component={BrowseScreen}
                options={{ headerShown: false }}
            />
            <Stack.Screen
                name="RecipeDetails"
                component={RecipeDetailsScreen}
                options={{ title: 'Recipe Details' }}
            />
            {/* Add more screens as needed */}
        </Stack.Navigator>
    )
}

export default BrowseTab;