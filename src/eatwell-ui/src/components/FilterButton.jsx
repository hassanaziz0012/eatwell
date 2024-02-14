import { useState } from 'react';
import { TouchableOpacity, StyleSheet, View, ScrollView } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import DropdownModal from './DropdownModal';
import Filter from './Filter';
import diets from '../../filters/diets';
import meal_types from '../../filters/meal_types';
import cuisines from '../../filters/cuisines';
import DarkButton from './DarkButton';


const FilterButton = ({ selectedDietState, selectedMealTypeState, selectedCuisineState }) => {
    const [isModalVisible, setIsModalVisible] = useState(false);

    const dietFilters = diets;
    const [selectedDiets, setSelectedDiets] = selectedDietState;

    const mealTypeFilters = meal_types;
    const [selectedMealTypes, setSelectedMealTypes] = selectedMealTypeState;

    const cuisineFilters = cuisines;
    const [selectedCuisines, setSelectedCuisines] = selectedCuisineState;

    const [applied, setApplied] = useState(false);

    const openModal = () => {
        setIsModalVisible(true);
    };

    const closeModal = () => {
        if (selectedDiets.length > 0 || selectedMealTypes.length > 0 || selectedCuisines.length > 0) {
            setApplied(true);
        }
        else {
            setApplied(false);
        }
        setIsModalVisible(false);
    };

    return (
        <View style={[styles.container, applied ? styles.containerApplied : null]}>
            <TouchableOpacity onPress={openModal}>
                <Ionicons name="ios-funnel" size={20} color="black" />
            </TouchableOpacity>

            {/* filter modal */}
            <DropdownModal modalVisibleState={[isModalVisible, setIsModalVisible]} closeModal={closeModal}>
                <ScrollView style={styles.modalContent}>
                    <Filter 
                        label={"Diets"} 
                        filters={dietFilters} 
                        selectedFiltersState={[selectedDiets, setSelectedDiets]} 
                        multiple 
                    />
                    <Filter 
                        label={"Meal Types"} 
                        filters={mealTypeFilters} 
                        selectedFiltersState={[selectedMealTypes, setSelectedMealTypes]} 
                    />
                    <Filter 
                        label={"Cuisines"} 
                        filters={cuisineFilters} 
                        selectedFiltersState={[selectedCuisines, setSelectedCuisines]} 
                        multiple 
                    />
                    <DarkButton text={"Apply"} onPress={closeModal} style={{ marginBottom: 20 }} />
                </ScrollView>
            </DropdownModal>
        </View>
    )
}

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
        paddingVertical: 10,
        width: "100%"
    },
})

export default FilterButton;