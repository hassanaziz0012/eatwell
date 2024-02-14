import React, { useState } from 'react';
import { View, Text, TouchableOpacity, StyleSheet, ScrollView } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import sorts from '../../filters/sorts';
import DropdownModal from './DropdownModal';

const SortButton = ({ sortOptionState }) => {
    const [sortOption, setSortOption] = sortOptionState; // State to store selected option
    const [isSortModalVisible, setSortModalVisible] = useState(false);
    const [applied, setApplied] = useState(false);

    const sortOptions = sorts; // List of sorting options

    const openSortModal = () => {
        setSortModalVisible(true);
    };

    const closeSortModal = () => {
        setSortModalVisible(false);
    };

    const handleSortOptionSelect = (option) => {
        if (option === sortOption) {
            setSortOption("default");
            setApplied(false);
        } else {
            setSortOption(option);
            setApplied(true);
        }
        closeSortModal();
        // Perform actions based on the selected option
    };


    return (
        <View style={[styles.container, applied ? styles.containerApplied : null]}>
            {/* Sort button */}
            <TouchableOpacity onPress={openSortModal}>
                <Ionicons name="ios-options" size={20} color="black" />
            </TouchableOpacity>

            {/* Sort Modal */}
            <DropdownModal modalVisibleState={[isSortModalVisible, setSortModalVisible]} closeModal={closeSortModal}>
                <ScrollView style={styles.modalContent}>
                    {sortOptions.map((option, index) => (
                        <TouchableOpacity
                            key={index}
                            style={[styles.sortOption, sortOption === option ? styles.selectedOption : null]}
                            onPress={() => handleSortOptionSelect(option)}
                        >
                            <Text>{option}</Text>
                        </TouchableOpacity>
                    ))}
                </ScrollView>
            </DropdownModal>
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
    containerApplied: {
        backgroundColor: '#7794c9',
        borderColor: "#7794c9"
    },
    modalContent: {
        backgroundColor: '#fff',
        borderRadius: 20,
        paddingHorizontal: 30,
    },
    sortOption: {
        paddingVertical: 15,
        paddingHorizontal: 15,
        borderBottomWidth: 1,
        borderBottomColor: '#eee',
    },
    selectedOption: {
        backgroundColor: '#7794c9',
    }
});

export default SortButton;
