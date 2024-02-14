import React from 'react';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import { Ionicons } from '@expo/vector-icons';
import { GenerateScreen } from '../screens/GenerateScreen';
import BrowseTab from '../tabs/BrowseTab';
import MeTab from '../tabs/MeTab';

const Tab = createBottomTabNavigator();


const BottomNav = ({ handleLogout }) => {
    return (
        <Tab.Navigator>
            <Tab.Screen
                name="Browse"
                component={BrowseTab}
                options={{
                    tabBarIcon: ({ color, size }) => (
                        <Ionicons name="md-search" size={size} color={color} />
                    ),
                }}
            />
            <Tab.Screen
                name="Generate"
                component={GenerateScreen}
                options={{
                    tabBarIcon: ({ color, size }) => (
                        <Ionicons name="md-refresh" size={size} color={color} />
                    ),
                }}
            />
            <Tab.Screen
                name="Me"
                children={() => <MeTab handleLogout={handleLogout} />}
                options={{
                    tabBarIcon: ({ color, size }) => (
                        <Ionicons name="md-person" size={size} color={color} />
                    ),
                }}
            />
        </Tab.Navigator>
    );
};

export default BottomNav;
