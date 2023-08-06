import { tokenObserver } from "mahjong_sdk/dist/auth";
import { HttpClient } from "mahjong_sdk/dist/http-client";
import React, { useCallback, useState } from "react";
import {
  Button,
  SafeAreaView,
  ScrollView,
  StatusBar,
  StyleSheet,
  Text,
  TextInput,
  View,
  useColorScheme,
} from "react-native";
import { Colors } from "react-native/Libraries/NewAppScreen";
import { first } from "rxjs";

export const AuthScreen = () => {
  const isDarkMode = useColorScheme() === "dark";
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");

  const onSubmit = useCallback(() => {
    HttpClient.setAuth({
      password,
      username,
    })
      .pipe(first())
      .subscribe({
        error: (err) => {
          console.log("debug: auth.tsx: err", err);
          setError("Unknown error");
        },
        next: (response) => {
          if (typeof response === "string") {
            setError(response);
            return;
          }

          if (response.token) {
            tokenObserver.next(response.token);
          }
        },
      });
  }, [username, password]);

  const backgroundStyle = {
    backgroundColor: isDarkMode ? Colors.darker : Colors.lighter,
  };

  return (
    <SafeAreaView style={backgroundStyle}>
      <StatusBar
        backgroundColor={backgroundStyle.backgroundColor}
        barStyle={isDarkMode ? "light-content" : "dark-content"}
      />
      <ScrollView
        contentInsetAdjustmentBehavior="automatic"
        style={backgroundStyle}
      >
        <View
          style={{
            backgroundColor: isDarkMode ? Colors.black : Colors.white,
          }}
        >
          <View>
            <Text>Mahjong Rust</Text>
          </View>
          <View>
            <Text>Log in</Text>
          </View>
          <View>
            <TextInput
              editable
              maxLength={40}
              numberOfLines={1}
              onChangeText={setUsername}
              placeholder="Username"
              style={styles.input}
              value={username}
            />
          </View>
          <View>
            <TextInput
              editable
              maxLength={40}
              numberOfLines={1}
              onChangeText={setPassword}
              placeholder="Password"
              secureTextEntry
              style={styles.input}
              value={password}
            />
          </View>
          {error && (
            <View>
              <Text>{error}</Text>
            </View>
          )}
          <View>
            <Button
              disabled={username.length === 0 || password.length === 0}
              onPress={onSubmit}
              title="Submit"
            />
          </View>
        </View>
      </ScrollView>
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  input: {
    borderColor: "gray",
    borderWidth: 1,
  },
});
