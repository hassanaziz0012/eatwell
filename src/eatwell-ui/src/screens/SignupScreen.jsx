import { useNavigation } from '@react-navigation/native';
import React, { useState } from 'react';
import { View, Text, TextInput, TouchableOpacity, StyleSheet } from 'react-native';

const SignupScreen = () => {
    const [fullName, setFullName] = useState('');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');
    const [errorMsg, setErrorMsg] = useState('');

    const navigation = useNavigation();

    const handleSignup = () => {
        if (fullName.length < 3) {
            setErrorMsg("Please enter a valid full name.");
            return;
        }

        if (!validateEmail(email)) {
            setErrorMsg("Please enter a valid email address.");
            return;
        }

        const { passwordsMatch, isLengthValid } = validatePassword(password, confirmPassword);
        if (!isLengthValid) {
            setErrorMsg("Password must be at least 8 characters long.");
            return;
        }

        if (!passwordsMatch) {
            setErrorMsg("Passwords do not match.");
            return;
        }

        setErrorMsg("");
        
        const body = JSON.stringify({ name: fullName, email, password })
        console.log(body);
        fetch('http://localhost:8080/signup', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: body
        })
        .then(response => response.json())
        .then(data => {
            console.log(data);
            if (data.error) {
                setErrorMsg(data.error);
            } else {
                navigation.navigate('Login', { defaultEmail: email });
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

    const validatePassword = (password, confirmPassword) => {
        // Check if password and confirm password match
        const passwordsMatch = password === confirmPassword;

        // Check if password length is greater than 8 characters
        const isLengthValid = password.length >= 8;

        return { passwordsMatch, isLengthValid };
    };

    return (
        <View style={styles.container}>
            <Text style={styles.title}>Signup</Text>
            <TextInput
                style={styles.input}
                placeholder="Full Name"
                value={fullName}
                onChangeText={text => setFullName(text)}
            />
            <TextInput
                style={styles.input}
                placeholder="Email"
                value={email}
                onChangeText={text => setEmail(text)}
                keyboardType="email-address"
            />
            <TextInput
                style={styles.input}
                placeholder="Password"
                value={password}
                onChangeText={text => setPassword(text)}
                secureTextEntry
            />
            <TextInput
                style={[styles.input, { marginBottom: 0 }]}
                placeholder="Confirm Password"
                value={confirmPassword}
                onChangeText={text => setConfirmPassword(text)}
                secureTextEntry
            />
            <Text style={{ color: 'red', padding: 10 }}>{errorMsg}</Text>
            <TouchableOpacity style={styles.submitButton} onPress={handleSignup}>
                <Text style={styles.submitButtonText}>Submit</Text>
            </TouchableOpacity>
            <Text
                onPress={() => navigation.navigate('Login')}
                style={{ marginTop: 15, color: "blue", textDecorationLine: "underline" }}
            >
                Or log in?
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


export default SignupScreen