import React, { useState } from 'react';
import { View, Text, TouchableOpacity, StyleSheet } from 'react-native';

const Filter = ({ label, filters, selectedFiltersState, multiple }) => {
    const [selectedFilters, setSelectedFilters] = selectedFiltersState;

    const toggleFilter = (filter) => {
        if (selectedFilters.includes(filter)) {
            if (multiple === true) {
                setSelectedFilters(selectedFilters.filter((selected) => selected !== filter));
            }
            else {
                setSelectedFilters([]);
            }
        } else {
            if (multiple === true) {
                setSelectedFilters([...selectedFilters, filter]);
            }
            else {
                setSelectedFilters([filter]);
            }
        }
    };

    return (
        <View style={styles.container}>
            <Text style={styles.label}>{label}</Text>
            <View style={styles.filtersContainer}>
                {filters.map((filter, index) => (
                    <TouchableOpacity
                        key={index}
                        style={[
                            styles.filter,
                            selectedFilters.includes(filter) ? styles.selectedFilter : null,
                        ]}
                        onPress={() => toggleFilter(filter)}
                    >
                        <Text>{filter}</Text>
                    </TouchableOpacity>
                ))}
            </View>
        </View>
    );
};

const styles = StyleSheet.create({
    container: {
        marginBottom: 20,
    },
    label: {
        fontWeight: 'bold',
        marginBottom: 5,
    },
    filtersContainer: {
        flexDirection: 'row',
        flexWrap: 'wrap',
        width: '100%'
    },
    filter: {
        paddingHorizontal: 15,
        paddingVertical: 8,
        margin: 5,
        borderRadius: 20,
        borderWidth: 1,
        borderColor: '#ccc',
    },
    selectedFilter: {
        backgroundColor: '#007bff',
        borderColor: '#007bff',
        color: '#fff',
    },
});

export default Filter;
