import { useState } from "react";

import { tokenObserver } from "src/lib/auth";
import { HttpClient } from "src/lib/http-client";
import Button from "src/ui/common/button";

const AuthForm = () => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  return (
    <div>
      <div>Enter the username and password.</div>
      <div>If the user doesn{"'"}t exist, it will be created.</div>
      <div>Username:</div>
      <div>
        <input
          onChange={(e) => setUsername(e.target.value)}
          type="text"
          value={username}
        />
      </div>
      <div>Password:</div>
      <div>
        <input
          onChange={(e) => setPassword(e.target.value)}
          type="password"
          value={password}
        />
      </div>
      <Button
        onClick={async () => {
          const response = await HttpClient.setAuth({
            password,
            username,
          });

          if (response.token) {
            tokenObserver.next(response.token);
          }
        }}
      >
        Submit
      </Button>
    </div>
  );
};

export default AuthForm;
