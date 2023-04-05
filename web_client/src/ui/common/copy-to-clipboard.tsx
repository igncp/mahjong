import React, { useEffect, useState } from "react";

type TProps = {
  text: string;
};

const CopyToClipboard = ({ text }: TProps) => {
  const [copied, setCopied] = useState(false);

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
      <span>{copied ? " (copied)" : ""}</span>
    </>
  );
};

export default CopyToClipboard;
