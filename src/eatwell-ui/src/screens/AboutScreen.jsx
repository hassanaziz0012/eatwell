import React from 'react'
import { StyleSheet, View, Text, Linking } from 'react-native'

const AboutScreen = () => {
    return (
        <View style={styles.container}>
            <Text style={styles.header}>
                About Eatwell
            </Text>
            <Text>
                Eatwell is a little hobby project of mine. A simple nutrition app for finding and building a good, healthy diet, based 
                on your own unique conditions, eating preferences, and budget.
            </Text>
            
            <View style={styles.separator} />
            
            <Text style={styles.header}>
                About Me
            </Text>
            <Text>
                I'm a freelance developer and like building things. I've been coding since my pre-teen days, and it's still just as 
                exciting as it was back then. Aside from coding, I waste my youth on video games and manga. Sometimes, I touch grass (daring, I know).
            </Text>
            
            <View style={styles.separator} />

            <Text>
                Find me here:
            </Text>
            <Text style={styles.link} onPress={() => {
                Linking.openURL('https://github.com/hassanaziz0012')
            }}>
                Github
            </Text>

            <Text style={styles.link} onPress={() => {
                Linking.openURL('https://hassan-aziz-zeta.vercel.app')
            }}>
                Personal Website
            </Text>

            <View style={styles.separator} />
            <Text>
                Thanks for using Eatwell! If you have something to share, please lemme know.
            </Text>
        </View>
    )
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        marginHorizontal: 10,
        marginTop: 10,
    },
    header: {
        fontSize: 20,
        fontWeight: 'bold',
    },
    separator: {
        borderBottomColor: '#bcc',
        borderBottomWidth: StyleSheet.hairlineWidth,
        marginVertical: 10
    },
    link: {
        color: 'blue',
        textDecorationLine: 'underline',
    }
})

export default AboutScreen