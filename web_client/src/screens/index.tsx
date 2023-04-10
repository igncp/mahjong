import { useEffect, useState } from "react";

import AuthForm from "src/containers/auth-form";
import Header from "src/containers/common/header";
import Dashboard from "src/containers/dashboard";
import { getIsLoggedIn, tokenObserver } from "src/lib/auth";

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
