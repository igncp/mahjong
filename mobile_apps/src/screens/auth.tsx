import { tokenObserver } from "mahjong_sdk/dist/auth";
import { HttpClient } from "mahjong_sdk/dist/http-client";
import React, { useCallback, useState } from "react";
import { useTranslation } from "react-i18next";
import { Button, Linking, Text, TextInput, View } from "react-native";
import { first } from "rxjs";

import LanguagePicker from "../containers/language-picker";
import { styles } from "./auth.styles";

export const AuthScreen = () => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const { t } = useTranslation();

  const onSubmit = useCallback(() => {
    HttpClient.setAuth({
      password,
      username,
    })
      .pipe(first())
      .subscribe({
        error: (err) => {
          console.log("debug: auth.tsx: err", err);
          setError(t("auth.unknownError", "Unknown error") as string);
        },
        next: (response) => {
          if (typeof response === "string") {
            switch (response) {
              case "E_INVALID_USER_PASS":
                setError(t("auth.invalidUserPass") as string);
                return;
              default:
                setError(response);
            }
            return;
          }

          if (response.token) {
            tokenObserver.next(response.token);
          }
        },
      });
  }, [username, password, t]);

  const onGithubClick = useCallback(() => {
    Linking.openURL("https://github.com/igncp/mahjong");
  }, []);

  return (
    <View style={styles.wrapper}>
      <View style={styles.title}>
        <Text style={styles.titleText}>
          {t("auth.mahjong", "Mahjong Rust")}
        </Text>
      </View>
      <View>
        <Text style={styles.introText}>
          {t(
            "auth.intro",
            "You can create a new account or use an existing one. Just add here you username and password. Keep in mind that the server will clear accounts periodically until it is more stable."
          )}
        </Text>
      </View>
      <View>
        <TextInput
          editable
          maxLength={40}
          numberOfLines={1}
          onChangeText={setUsername}
          placeholder={t("auth.username", "Username") as string}
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
          placeholder={t("auth.password", "Password") as string}
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
          title={t("auth.submit", "Submit")}
        />
      </View>
      <View>
        <Button
          onPress={onGithubClick}
          title={t("auth.openGH", "Open Github")}
        />
      </View>
      <LanguagePicker />
    </View>
  );
};
