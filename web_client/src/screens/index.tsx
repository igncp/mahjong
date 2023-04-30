import { useEffect, useState } from "react";

import { tokenObserver } from "mahjong_sdk/src/auth";
import AuthForm from "src/containers/auth-form";
import Header from "src/containers/common/header";
import Dashboard from "src/containers/dashboard";
import { getIsLoggedIn } from "src/lib/auth";

const Index = () => {
  const [isLoggedIn, setIsLoggedIn] = useState(getIsLoggedIn());

  useEffect(() => {
    const subscription = tokenObserver.subscribe(() => {
      setIsLoggedIn(getIsLoggedIn());
    });

    return () => subscription.unsubscribe();
  }, []);

  return (
    <main>
      <Header />
      {isLoggedIn ? <Dashboard /> : <AuthForm />}
    </main>
  );
};

export default Index;
