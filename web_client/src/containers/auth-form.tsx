import { useFormik } from "formik";
import { useRouter } from "next/router";
import { useCallback, useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { first } from "rxjs";
import z from "zod";

import { anonymousAuth, githubAuth } from "src/lib/auth";
import { SiteUrls } from "src/lib/site/urls";
import { tokenObserver } from "src/sdk/auth";
import { HttpClient } from "src/sdk/http-client";
import Alert from "src/ui/common/alert";
import Button from "src/ui/common/button";
import Card from "src/ui/common/card";
import Input from "src/ui/common/input";
import Space from "src/ui/common/space";
import Text from "src/ui/common/text";

import styles from "./auth-form.module.scss";
import PageContent from "./page-content";

type FormState = {
  password: string;
  username: string;
};

const AuthForm = () => {
  const { t } = useTranslation();
  const [error, setError] = useState<null | string>(null);
  const [showPasswordForm, setShowPasswordForm] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const router = useRouter();

  const onPlayAnonymously = useCallback(() => {
    setIsLoading(true);

    anonymousAuth
      .login()
      .pipe(first())
      .subscribe(({ token }) => {
        setIsLoading(false);

        if (token) {
          tokenObserver.next(token);
        }
      });
  }, []);

  const formSchema = useMemo(
    () =>
      z.object({
        password: z
          .string()
          .min(
            4,
            t(
              "auth.password.min",
              "Your password must have at least 4 characters",
            ) as string,
          ),
        username: z
          .string()
          .min(
            4,
            t(
              "auth.username.min",
              "Your username must have at least 4 characters",
            ) as string,
          ),
      }),
    [t],
  );

  const validate = (values: FormState) => {
    try {
      formSchema.parse(values);
    } catch (err: unknown) {
      return (err as z.ZodError).formErrors.fieldErrors;
    }
  };

  const formik = useFormik<FormState>({
    initialValues: {
      password: "",
      username: "",
    },
    onSubmit: (values, { setSubmitting }) => {
      const { password, username } = values;

      HttpClient.setAuth({
        password,
        username,
      })
        .pipe(first())
        .subscribe({
          error: (err) => {
            console.error("debug: auth-form.tsx: err", err);
            setError(t("auth.error.unknown"));
          },
          next: (response) => {
            setSubmitting(false);

            if (typeof response === "string") {
              const errorText =
                {
                  ["E_INVALID_USER_PASS"]: t("auth.error.invalidUserPass"),
                }[response] || t("auth.error.unknown");

              setError(errorText);

              return;
            }

            if (response.token) {
              tokenObserver.next(response.token);
            }
          },
        });
    },
    validate,
  });

  useEffect(() => {
    formik.validateForm();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [t]);

  const loginWithGithub = () => {
    githubAuth.login();
  };

  const trackOffscreenGame = () => {
    router.push(SiteUrls.offscreenGame);
  };

  return (
    <PageContent contentStyle={{ marginTop: "20px" }}>
      <div className={styles.waves}>
        <div className={styles.wave} />
        <div className={styles.wave} />
        <div className={styles.wave} />
      </div>
      <div className={styles.content}>
        {showPasswordForm && (
          <>
            <Text>{t("auth.intro1")}</Text>
            <Text>{t("auth.intro2")}</Text>
            <Space style={{ maxWidth: 500 }}>
              <Alert message={t("auth.warning")} type="info" />
            </Space>
          </>
        )}
        <div className={styles.formWrapper}>
          <Card style={{ width: "100%" }}>
            <form
              className={styles.form}
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  formik.submitForm();
                }
              }}
              onSubmit={formik.handleSubmit}
            >
              <Button
                data-name="auth-submit-password"
                disabled={formik.isSubmitting || isLoading}
                onClick={onPlayAnonymously}
                type="default"
              >
                {t("auth.button.playAnonymously")}
              </Button>
              {showPasswordForm ? (
                <>
                  {error && (
                    <Space>
                      <Alert message={error} type="error" />
                    </Space>
                  )}
                  <Text style={{ alignSelf: "flex-start" }}>
                    {t("auth.label.user")}
                  </Text>
                  <Input
                    data-name="username"
                    type="text"
                    {...formik.getFieldProps("username")}
                  />
                  {formik.touched.username && formik.errors.username?.length ? (
                    <Alert
                      message={formik.errors.username[0]}
                      style={{ alignSelf: "flex-start" }}
                      type="error"
                    />
                  ) : null}
                  <Text style={{ alignSelf: "flex-start" }}>
                    {t("auth.label.pass")}
                  </Text>
                  <Input
                    data-name="password"
                    type="password"
                    {...formik.getFieldProps("password")}
                  />
                  {formik.touched.password && formik.errors.password?.length ? (
                    <Alert
                      message={formik.errors.password[0]}
                      style={{ alignSelf: "flex-start" }}
                      type="error"
                    />
                  ) : null}
                  <Button
                    data-name="auth-submit"
                    disabled={
                      formik.isSubmitting || !formik.isValid || isLoading
                    }
                    onClick={formik.submitForm}
                    style={{ maxWidth: 150 }}
                    type="primary"
                  >
                    {t("auth.button.submit")}
                  </Button>
                </>
              ) : (
                <Button
                  data-name="auth-submit-password"
                  disabled={formik.isSubmitting}
                  onClick={() => {
                    setShowPasswordForm(true);
                  }}
                  type="primary"
                >
                  {t("auth.button.showUsernamePass")}
                </Button>
              )}
              <Button
                data-name="auth-submit-github"
                disabled={formik.isSubmitting}
                onClick={loginWithGithub}
                type="primary"
              >
                {t("auth.button.submitGithub")}
              </Button>
              <Button
                disabled={formik.isSubmitting}
                onClick={trackOffscreenGame}
                style={{ backgroundColor: "green" }}
                type="primary"
              >
                {t("auth.button.trackOffscreenGame")}
              </Button>
            </form>
          </Card>
        </div>
      </div>
    </PageContent>
  );
};

export default AuthForm;
