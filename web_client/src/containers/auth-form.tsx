import { tokenObserver } from "mahjong_sdk/dist/auth";
import { HttpClient } from "mahjong_sdk/dist/http-client";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { first } from "rxjs";

import Alert from "src/ui/common/alert";
import Button from "src/ui/common/button";
import Card from "src/ui/common/card";
import Input from "src/ui/common/input";
import Space from "src/ui/common/space";
import Text from "src/ui/common/text";

import styles from "./auth-form.module.scss";
import PageContent from "./page-content";

const AuthForm = () => {
  const { t } = useTranslation();
  const [error, setError] = useState<string | null>(null);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  return (
    <PageContent>
      <Text>{t("auth.intro1")}</Text>
      <Text>{t("auth.intro2")}</Text>
      <Space style={{ maxWidth: 500 }}>
        <Alert message={t("auth.warning")} type="info" />
      </Space>
      <div className={styles.formWrapper}>
        <Card>
          <Space direction="vertical">
            {error && (
              <Space>
                <Alert message={error} type="error" />
              </Space>
            )}
            <Text>{t("auth.label.user")}</Text>
            <Input
              data-name="username"
              onChange={(e) => {
                setError(null);
                setUsername(e.target.value);
              }}
              type="text"
              value={username}
            />
            <Text>{t("auth.label.pass")}</Text>
            <Input
              data-name="password"
              onChange={(e) => {
                setError(null);
                setPassword(e.target.value);
              }}
              type="password"
              value={password}
            />
            <Button
              data-name="auth-submit"
              disabled={!username || !password}
              onClick={() => {
                HttpClient.setAuth({
                  password,
                  username,
                })
                  .pipe(first())
                  .subscribe({
                    error: (err) => {
                      console.log("debug: auth-form.tsx: err", err);
                      setError(t("auth.error.unknown"));
                    },
                    next: (response) => {
                      if (typeof response === "string") {
                        const error =
                          {
                            ["E_INVALID_USER_PASS"]: t(
                              "auth.error.invalidUserPass"
                            ),
                          }[response] || t("auth.error.unknown");
                        setError(error);

                        return;
                      }

                      if (response.token) {
                        tokenObserver.next(response.token);
                      }
                    },
                  });
              }}
              type="primary"
            >
              {t("auth.button.submit")}
            </Button>
          </Space>
        </Card>
      </div>
    </PageContent>
  );
};

export default AuthForm;
