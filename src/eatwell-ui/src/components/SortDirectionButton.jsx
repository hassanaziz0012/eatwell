import React, { useState } from 'react';
import { View, TouchableOpacity, StyleSheet } from 'react-native';
import { Ionicons } from '@expo/vector-icons';

const SortDirectionButton = ({ directionState, onPress }) => {
    const [dir, setDir] = directionState;
    const iconName = dir === 'Ascending' ? 'arrow-up' : 'arrow-down';

    const sort = () => {
        if (dir === 'Ascending') {
            setDir('Descending');
        } else {
            setDir('Ascending');
        }
    }

    return (
        <View style={styles.container}>
            <TouchableOpacity onPress={sort}>
                <Ionicons name={iconName} size={24} color="black" />
            </TouchableOpacity>
        </View>
    );
};

const styles = StyleSheet.create({
    container: {
        alignItems: 'center',
        justifyContent: 'center',
        flexDirection: 'row',
        borderWidth: 1,
        borderColor: '#ccc',
        borderRadius: 5,
        padding: 8,
        marginRight: 8,
        alignItems: 'center',
    },
})

export default SortDirectionButton;
