import { useEffect, useState } from "react";

import AuthForm from "src/containers/auth-form";
import Dashboard from "src/containers/dashboard";
import { getIsLoggedIn } from "src/lib/auth";
import { tokenObserver } from "src/sdk/auth";

const Index = () => {
  const [isLoggedIn, setIsLoggedIn] = useState(getIsLoggedIn());

  useEffect(() => {
    const subscription = tokenObserver.subscribe(() => {
      setIsLoggedIn(getIsLoggedIn());
    });

    return () => subscription.unsubscribe();
  }, []);

  return <>{isLoggedIn ? <Dashboard /> : <AuthForm />}</>;
};

export default Index;
