import React, { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

type TProps = {
  text: string;
};

const CopyToClipboard = ({ text }: TProps) => {
  const [copied, setCopied] = useState(false);
  const { t } = useTranslation();

  useEffect(() => {
    if (copied) {
      const timeout = setTimeout(() => {
        setCopied(false);
      }, 1000);

      return () => {
        clearTimeout(timeout);
      };
    }
  }, [copied]);

  return (
    <>
      <span
        onClick={() => {
          navigator.clipboard.writeText(text);
          setCopied(true);
        }}
        style={{
          cursor: "pointer",
          textDecoration: "underline",
        }}
      >
        {text}
      </span>
      <span>{copied ? ` (${t("ui.copied", "copied")})` : ""}</span>
    </>
  );
};

export default CopyToClipboard;
