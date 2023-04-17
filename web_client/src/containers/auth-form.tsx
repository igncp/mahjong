import { useState } from "react";

import { tokenObserver } from "src/lib/auth";
import { HttpClient } from "src/lib/http-client";
import Button from "src/ui/common/button";
import Card from "src/ui/common/card";
import Input from "src/ui/common/input";
import PageContent from "src/ui/common/page-content";
import Space from "src/ui/common/space";
import Text from "src/ui/common/text";

const AuthForm = () => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  return (
    <PageContent style={{ marginTop: "20px" }}>
      <Text>Enter the username and password.</Text>
      <Text>If the user doesn{"'"}t exist, it will be created.</Text>
      <Space>
        <Card>
          <Space direction="vertical">
            <Text>Username:</Text>
            <Input
              onChange={(e) => setUsername(e.target.value)}
              type="text"
              value={username}
            />
            <Text>Password:</Text>
            <Input
              onChange={(e) => setPassword(e.target.value)}
              type="password"
              value={password}
            />
            <Button
              disabled={!username || !password}
              onClick={async () => {
                const response = await HttpClient.setAuth({
                  password,
                  username,
                });

                if (response.token) {
                  tokenObserver.next(response.token);
                }
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
