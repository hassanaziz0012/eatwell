import React from 'react';
import { View, Text, TouchableOpacity, Modal, StyleSheet } from 'react-native';

const DropdownModal = ({ modalVisibleState, closeModal, children }) => {
    const [isModalVisible, setModalVisible] = modalVisibleState;

    const handleBackdropPress = () => {
        closeModal();
    };

    return (
        <Modal
            animationType="slide"
            transparent={true}
            visible={isModalVisible}
            onRequestClose={closeModal}
        >
            <TouchableOpacity
                style={styles.backdrop}
                activeOpacity={1}
                onPress={handleBackdropPress}
                >
            </TouchableOpacity>

            <View style={styles.modalContainer}>
                {children}
                <TouchableOpacity style={styles.closeButton} onPress={closeModal}>
                    <Text>Close</Text>
                </TouchableOpacity>
            </View>
        </Modal>
    )
}

const styles = StyleSheet.create({
    backdrop: {
        flex: 1,
        justifyContent: 'flex-end',
        backgroundColor: 'rgba(0, 0, 0, 0.5)',
    },
    modalContainer: {
        flex: 5,
        justifyContent: 'space-between',
        backgroundColor: 'rgba(0, 0, 0, 0.5)',
    },
    closeButton: {
        backgroundColor: '#fff',
        alignItems: 'center',
        paddingVertical: 15,
        borderBottomLeftRadius: 20,
        borderBottomRightRadius: 20,
        marginTop: 10,
    },
})

export default DropdownModal;