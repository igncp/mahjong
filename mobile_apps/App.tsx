import AsyncStorage from "@react-native-async-storage/async-storage";
import React, { useCallback, useEffect, useState } from "react";
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
import { first, from, tap } from "rxjs";

import { tokenObserver } from "mahjong_sdk/src/auth";
import { HttpClient, setBaseUrl } from "mahjong_sdk/src/http-server";

const TOKEN_KEY = "mahjong_rust_token";

const setupApp = () => {
  setBaseUrl("https://mahjong-rust.com/api");

  from(AsyncStorage.getItem(TOKEN_KEY))
    .pipe(
      first(),
      tap((token) => {
        if (token) {
          tokenObserver.next(token);
        }

        tokenObserver.subscribe((newToken) => {
          if (newToken) {
            AsyncStorage.setItem(TOKEN_KEY, newToken);
          } else {
            AsyncStorage.removeItem(TOKEN_KEY);
          }
        });
      })
    )
    .subscribe();
};

const AuthForm = () => {
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
          console.log("debug: auth-form.tsx: err", err);
          setError("Unknown error");
        },
        next: (response) => {
          if (typeof response === "string") {
            setError(response);
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

const LoadingScreen = () => <Text>Loading</Text>;

const Dashboard = () => <Text>Dashboard</Text>;

const App = () => {
  const isDarkMode = useColorScheme() === "dark";
  const [isLoggedIn, setIsLoggedIn] = useState<boolean | null>(null);

  useEffect(() => {
    setupApp();

    tokenObserver.subscribe({
      next: (newToken) => {
        setIsLoggedIn(!!newToken);
      },
    });
  }, []);

  const backgroundStyle = {
    backgroundColor: isDarkMode ? Colors.darker : Colors.lighter,
  };

  if (isLoggedIn === null) {
    return <LoadingScreen />;
  }

  return (
    <SafeAreaView style={backgroundStyle}>
      <StatusBar
        backgroundColor={backgroundStyle.backgroundColor}
        barStyle={isDarkMode ? "light-content" : "dark-content"}
      />
      {isLoggedIn ? <Dashboard /> : <AuthForm />}
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  input: {
    borderColor: "gray",
    borderWidth: 1,
  },
});

export default App;
