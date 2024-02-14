import React, { useRef } from 'react'
import { View, Text, StyleSheet, FlatList, ScrollView, SafeAreaView } from 'react-native'

const HelpScreen = () => {
    const tableOfContents = [
        "About Eatwell",
        "Browse and Search",
        "Generate meal plans",
        "Eating Preferences, Diets, Conditions",
        "Other"
    ]
    const contentsRef = useRef(null);

    return (
        <ScrollView style={styles.container} ref={contentsRef}>
            <SafeAreaView style={styles.tableOfContentsContainer}>
                <Text style={styles.header}>Table of Contents</Text>
                {/* <FlatList data={tableOfContents} renderItem={({ item, index }) => (
                    <Text
                        style={[styles.link, { fontWeight: index === 0 ? 'bold' : 'normal' }]}
                        onPress={() => contentsRef.current.scrollTo({ x: 0, y: 400 * index, animated: true })}
                    >
                        {index + 1}) {item}
                    </Text>
                )}
                /> */}

                {
                    tableOfContents.map((item, index) => (
                        <Text
                        style={[styles.link, { fontWeight: index === 0 ? 'bold' : 'normal' }]}
                        onPress={() => contentsRef.current.scrollTo({ x: 0, y: 400 * index, animated: true })}
                        key={index}
                        >
                            {index + 1}) {item}
                        </Text>
                    ))
                }
            </SafeAreaView>

            <Text style={styles.header}>About Eatwell</Text>
            <Text>
                Eatwell is a little hobby project of mine. A simple nutrition app for finding and building a good, healthy diet, based
                on your own unique conditions, eating preferences, and budget.
            </Text>

            <View style={styles.separator} />

            <Text style={styles.header}>Browse and Search</Text>
            <Text>
                Lorem ipsum dolor sit amet, consectetur adipisicing elit. Consequuntur sint quidem repellat alias sequi doloribus
                necessitatibus voluptatem qui blanditiis commodi adipisci voluptates nulla fuga harum facere, amet atque ipsa
                aspernatur! Voluptas excepturi, enim sit doloribus error, qui nostrum officiis amet architecto saepe ab facere nihil
                eveniet. Totam suscipit quisquam, corrupti ducimus commodi aperiam fugit iste dolorum, nulla eligendi voluptas sequi
                reiciendis nobis error, harum inventore ullam architecto rem. Harum, deserunt! Quibusdam tempora commodi voluptatum
                molestiae laboriosam officiis. Illum totam nam adipisci veritatis hic impedit exercitationem necessitatibus, et
                nesciunt provident ullam voluptatem pariatur tempora debitis quam accusantium inventore natus nostrum ab.
            </Text>
            <View style={styles.separator} />

            <Text style={styles.header}>Generate meal plans</Text>
            <Text>
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Vel minus eligendi explicabo rerum distinctio voluptatem, 
                blanditiis illum consequatur, accusantium animi molestias cupiditate perferendis totam! Impedit eos dolore voluptas 
                minima nihil, reprehenderit quia nam omnis aperiam illo porro repellat hic repudiandae voluptatibus illum soluta 
                saepe magni at consequatur aspernatur tempora deleniti.
            </Text>

            <Text style={styles.header}>Eating Preferences, Diets, Conditions</Text>
            <Text>
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Explicabo officia eos consequuntur enim neque sit voluptate 
                doloremque, dolore odio magni obcaecati quos ipsum dolor ullam nisi exercitationem, quis suscipit eveniet 
                reprehenderit? Asperiores repudiandae fugiat quaerat in suscipit commodi quasi dolore iste atque dolor! Minus neque 
                vitae nemo, quia impedit unde?
            </Text>
            <Text>
                Lorem, ipsum dolor sit amet consectetur adipisicing elit. Itaque maiores velit iste optio voluptatem porro facilis 
                doloremque rerum aliquam assumenda, ex ducimus modi nisi perferendis similique, placeat repellat. Porro atque maiores 
                dolor, dolore aliquam nostrum rem amet corporis quo? Ex amet repudiandae sit debitis eos quos recusandae facilis 
                adipisci voluptas!
            </Text>

            <Text style={styles.header}>Other</Text>
            <Text>
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Quam accusantium eum, nam expedita vel eligendi quibusdam 
                quisquam veniam facilis in iste error quaerat fuga eaque neque deleniti numquam excepturi illo porro repudiandae? 
                Natus officia quisquam eaque repellendus nam omnis sapiente quis architecto at laborum officiis, minus hic error 
                accusamus? Fugiat.
            </Text>

        </ScrollView>
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
    tableOfContentsContainer: {
        marginVertical: 10,
        padding: 10,
        borderRadius: 20,
        borderStyle: "solid",
        borderWidth: 1
    },
    link: {
        color: 'blue',
        textDecorationLine: 'underline',
    }
})

export default HelpScreen