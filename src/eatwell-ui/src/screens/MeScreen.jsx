import { ScrollView } from 'react-native';
import ButtonGroup from '../components/ButtonGroup';
import Button from '../components/Button';
import { useNavigation } from '@react-navigation/native';

export default MeScreen = ({ handleLogout }) => {
    const navigation = useNavigation();

    return (
        <ScrollView style={{ flexGrow: 1 }}>
            {/* <Text>Me Screen</Text> */}
            <ButtonGroup header={"Personal"}>
                <Button title={"Account"} onPress={() => {}} />
                <Button title={"Preferences"} onPress={() => {}} />
                <Button title={"Conditions"} onPress={() => {}} />
            </ButtonGroup>

            <ButtonGroup header={"Library"}>
                <Button title={"My Recipes"} onPress={() => navigation.navigate("MyRecipes")} />
                <Button title={"My Favorites"} onPress={() => {}} />
                <Button title={"My Shopping List"} onPress={() => {}} />
            </ButtonGroup>

            <ButtonGroup header={"Other"}>
                <Button title={"About"} onPress={() => navigation.navigate("About")} />
                <Button title={"Help"} onPress={() => navigation.navigate("Help")} />
                <Button title={"Log Out"} onPress={handleLogout} />
            </ButtonGroup>
        </ScrollView>
    );
};