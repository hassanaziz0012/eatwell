import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import Button from './Button'; // Import your Button component

const ButtonGroup = ({ header, buttons, children }) => {
    return (
        <View style={styles.container}>
            <Text style={styles.header}>{header}</Text>
            <View style={styles.buttonsContainer}>
                {/* {buttons.map((button, index) => (
                    <Button key={index} title={button.title} onPress={button.onPress} />
                ))} */}
                { children }
            </View>
            <View style={styles.marginBottom} />
        </View>
    );
};

const styles = StyleSheet.create({
    container: {
        backgroundColor: '#fff',
        paddingHorizontal: 20,
        paddingTop: 20,
        paddingBottom: 10,
        marginBottom: 10,
    },
    header: {
        fontSize: 18,
        fontWeight: 'bold',
        marginBottom: 10,
    },
    buttonsContainer: {
        borderTopWidth: 1,
        borderTopColor: '#eee',
    },
    marginBottom: {
        marginBottom: 20,
    },
});

export default ButtonGroup;
