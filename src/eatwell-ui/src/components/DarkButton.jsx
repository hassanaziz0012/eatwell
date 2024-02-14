import { TouchableOpacity, Text, StyleSheet } from 'react-native';

const DarkButton = ({ text, onPress, style}) => {
    return (
        <TouchableOpacity style={[styles.button, style]} onPress={onPress}>
            <Text style={styles.buttonText}>{text}</Text>
        </TouchableOpacity>
    )
}

const styles = StyleSheet.create({
    button: {
        backgroundColor: '#000',
        borderWidth: 1,
        borderColor: '#000',
        paddingVertical: 8,
        paddingHorizontal: 12,
        alignItems: 'center',
        justifyContent: 'center',
        alignSelf: 'flex-start',
        transitionDuration: 300,
    },
    buttonText: {
        color: '#fff',
    },
})

export default DarkButton;