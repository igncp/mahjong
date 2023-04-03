import React from "react";

import { SiteUrls } from "src/lib/site/urls";
import HeaderComp from "src/ui/common/header";

const Header = () => (
  <HeaderComp linkPath={SiteUrls.index} text="Mahjong Web Client" />
);

export default Header;
