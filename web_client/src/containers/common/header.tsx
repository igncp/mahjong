import HeaderComp from "src/ui/common/header";
import React from "react";
import { SiteUrls } from "src/lib/site/urls";

const Header = () => {
  return <HeaderComp linkPath={SiteUrls.index} text="Mahjong Web Client" />;
};

export default Header;
