import { useRouter } from "next/router";
import React from "react";

import { tokenObserver } from "mahjong_sdk/src/auth";
import { getIsLoggedIn } from "src/lib/auth";
import { SiteUrls } from "src/lib/site/urls";
import Button from "src/ui/common/button";
import HeaderComp from "src/ui/common/header";

const Header = () => {
  const isLoggedIn = getIsLoggedIn();
  const router = useRouter();

  return (
    <HeaderComp linkPath={SiteUrls.index} text="Mahjong">
      {isLoggedIn && (
        <span
          style={{
            display: "inline-block",
            flex: 1,
            textAlign: "right",
          }}
        >
          <Button
            onClick={() => {
              tokenObserver.next("");
              router.replace(SiteUrls.index);
            }}
          >
            Log out
          </Button>
        </span>
      )}
    </HeaderComp>
  );
};

export default Header;
