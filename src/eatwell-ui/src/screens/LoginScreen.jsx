import { useNavigation, useRoute } from '@react-navigation/native';
import React, { useState } from 'react';
import { View, Text, TextInput, TouchableOpacity, StyleSheet } from 'react-native';

const LoginScreen = ({ onLogin }) => {
    const route = useRoute();
    const { handleLogin, defaultEmail } = route.params || {};
    const [email, setEmail] = useState(defaultEmail || '');
    const [password, setPassword] = useState('');
    const [errorMsg, setErrorMsg] = useState('');

    const navigation = useNavigation();

    const login = () => {
        if (!validateEmail(email)) {
            setErrorMsg("Please enter a valid email address.");
            return;
        }

        const isLengthValid = validatePassword(password);
        if (!isLengthValid) {
            setErrorMsg("Password must be at least 8 characters long.");
            return;
        }

        setErrorMsg("");

        fetch('http://localhost:8080/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ email, password })
        })
        .then(response => response.json())
        .then(data => {
            if (data.error) {
                setErrorMsg(data.error);
            } else {
                onLogin();
                // navigation.navigate('BrowseScreen');
            }
        })
        .catch(error => {
            console.error('Error:', error);
        })
    };

    const validateEmail = (email) => {
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return emailRegex.test(email);
    };

    const validatePassword = (password) => {
        // Check if password length is greater than 8 characters
        const isLengthValid = password.length >= 8;

        return isLengthValid;
    };

    return (
        <View style={styles.container}>
            <Text style={styles.title}>Log In</Text>
            <TextInput
                style={styles.input}
                placeholder="Email"
                value={email}
                onChangeText={text => setEmail(text)}
                keyboardType="email-address"
            />
            <TextInput
                style={[styles.input, { marginBottom: 0 }]}
                placeholder="Password"
                value={password}
                onChangeText={text => setPassword(text)}
                secureTextEntry
            />
            <Text style={{ color: 'red', padding: 10 }}>{errorMsg}</Text>
            <TouchableOpacity style={styles.submitButton} onPress={login}>
                <Text style={styles.submitButtonText}>Submit</Text>
            </TouchableOpacity>
            <Text
                onPress={() => navigation.navigate('Signup')}
                style={{ marginTop: 15, color: "blue", textDecorationLine: "underline" }}
            >
                Or sign up?
            </Text>
        </View>
    );
};


const styles = StyleSheet.create({
    container: {
        flex: 1,
        justifyContent: 'center',
        alignItems: 'center',
        paddingHorizontal: 30,
    },
    title: {
        fontSize: 24,
        marginBottom: 20,
    },
    input: {
        height: 40,
        width: '100%',
        borderWidth: 1,
        borderColor: '#ccc',
        borderRadius: 5,
        paddingHorizontal: 10,
        marginBottom: 15,
    },
    submitButton: {
        backgroundColor: 'black',
        width: '100%',
        alignItems: 'center',
        padding: 15,
        borderRadius: 5,
    },
    submitButtonText: {
        color: 'white',
        fontSize: 18,
    },
});


export default LoginScreen