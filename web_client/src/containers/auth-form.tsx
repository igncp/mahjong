import { useState } from "react";
import { first } from "rxjs";

import { tokenObserver } from "mahjong_sdk/src/auth";
import { HttpClient } from "mahjong_sdk/src/http-server";
import Alert from "src/ui/common/alert";
import Button from "src/ui/common/button";
import Card from "src/ui/common/card";
import Input from "src/ui/common/input";
import PageContent from "src/ui/common/page-content";
import Space from "src/ui/common/space";
import Text from "src/ui/common/text";

const AuthForm = () => {
  const [error, setError] = useState<string | null>(null);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  return (
    <PageContent style={{ marginTop: "20px" }}>
      <Text>Enter the username and password.</Text>
      <Text>If the user doesn{"'"}t exist, it will be created.</Text>
      <Space style={{ maxWidth: 500 }}>
        <Alert
          message="Keep in mind that this application is still in early development and the database will be cleared periodically."
          type="info"
        />
      </Space>
      <Space>
        <Card>
          <Space direction="vertical">
            {error && (
              <Space>
                <Alert message={error} type="error" />
              </Space>
            )}
            <Text>Username:</Text>
            <Input
              onChange={(e) => {
                setError(null);
                setUsername(e.target.value);
              }}
              type="text"
              value={username}
            />
            <Text>Password:</Text>
            <Input
              onChange={(e) => {
                setError(null);
                setPassword(e.target.value);
              }}
              type="password"
              value={password}
            />
            <Button
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
              }}
              type="primary"
            >
              Submit
            </Button>
          </Space>
        </Card>
      </Space>
    </PageContent>
  );
};

export default AuthForm;
