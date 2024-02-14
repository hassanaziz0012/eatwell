import { StyleSheet, Text, View } from 'react-native';
import BottomNav from './src/components/BottomNav';
import { NavigationContainer } from '@react-navigation/native';
import { createStackNavigator } from '@react-navigation/stack';
import SignupScreen from './src/screens/SignupScreen';
import LoginScreen from './src/screens/LoginScreen';
import { useState } from 'react';
import AsyncStorage from '@react-native-async-storage/async-storage';
import { get, set } from './src/utils/storage';

const Stack = createStackNavigator();


export default function App() {
    const [loggedIn, setLoggedIn] = useState(false);;

    const handleLogin = () => {
        setLoggedIn(true);
        set('loggedIn', 'true');
        get("loggedIn").then((value) => {
            console.log("value", k);
        })
    }

    const handleLogout = () => {
        setLoggedIn(false);
        set('loggedIn', 'false');
        get("loggedIn").then((value) => {
            console.log("value", value);
        })
    }

    return (
        <NavigationContainer>
            <View style={styles.container}>
                {
                    loggedIn === true ?
                        <BottomNav handleLogout={handleLogout} />
                    :
                    (
                        <Stack.Navigator>
                            <Stack.Screen
                                name="Signup"
                                component={SignupScreen}
                                options={{ headerShown: false }}
                            />
                            <Stack.Screen
                                name="Login"
                                // component={LoginScreen}
                                children={(props) => <LoginScreen {...props} onLogin={handleLogin} />}
                                options={{ headerShown: false }}
                            />
                            {/* Add more screens as needed */}
                        </Stack.Navigator>
                    )
                }
            </View>
        </NavigationContainer>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: '#fff',
    },
});
