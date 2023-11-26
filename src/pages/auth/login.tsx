import { Box, Button, Center, Container, PasswordInput, TextInput, Title, Paper } from "@mantine/core";
import { useForm } from "@mantine/form";
import api from "@api/index";
import { notifications } from "@mantine/notifications";
import { useNavigate } from "react-router-dom";
import { useMutation } from "@tanstack/react-query";
import i18next from "i18next";
import { useTranslatePage } from "@hooks/index";
import { Wfm } from "../../types";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faExclamationTriangle } from "@fortawesome/free-solid-svg-icons";

export default function LoginPage() {
  const useTraLogin = (key: string, context?: { [key: string]: any }) => useTranslatePage(`auth.${key}`, { ...context })

  const navigate = useNavigate();
  const logInMutation = useMutation((data: { email: string, password: string }) => api.auth.login(data.email, data.password), {
    onSuccess: async (data: Wfm.UserDto) => {
      notifications.show({
        title: i18next.t('success.auth.login_title'),
        message: i18next.t('success.auth.login_message', { name: data.ingame_name }),
        color: 'success',
        autoClose: 5000,
      });
      api.auction.refresh();
      api.orders.refresh();
      navigate('/')
    },
    onError: () => {
      notifications.show({
        title: i18next.t('error.auth.login_title'),
        message: i18next.t('error.auth.login_message', { name: "" }),
        color: 'red',
        icon: <FontAwesomeIcon icon={faExclamationTriangle} />,
        autoClose: 5000,
      });
    },
  })




  const form = useForm({
    initialValues: {
      email: '',
      password: '',
      rememberMe: false,
    },
    validate: {
      // email: (val) => (/^\S+@\S+$/.test(val) ? null : 'Invalid email'),
      // password: (val) => (val.length <= 6 ? 'Password should include at least 6 characters' : null),
    },
  });

  return (
    <Center w={"100%"} h={"100%"}>
      <Box>
        <form onSubmit={form.onSubmit(async () => {
          await logInMutation.mutateAsync(form.values)
        })}>
          <Container >
            <Title
              align="center"
              sx={(theme) => ({
                fontFamily: `Greycliff CF, ${theme.fontFamily}`,
                fontWeight: 900,
              })}
            >
              {useTraLogin('login.title')}
            </Title>

            <Paper withBorder shadow="md" p={30} mt={30} radius="md">
              <TextInput
                required
                label={useTraLogin('login.email')}
                placeholder="Your email"
                value={form.values.email}
                onChange={(event) => form.setFieldValue('email', event.currentTarget.value)}
                error={form.errors.email && i18next.t('error.invalid_email')}
                radius="md" />
              <PasswordInput
                required
                label={useTraLogin('login.password')}
                placeholder="Your password"
                value={form.values.password}
                onChange={(event) => form.setFieldValue('password', event.currentTarget.value)}
                error={form.errors.password && i18next.t('error.auth.password_invalid')}
                radius="md"
              />
              <Button loading={logInMutation.isLoading} type="submit" fullWidth mt="xl">
                {useTraLogin('login.submit')}
              </Button>
            </Paper>
          </Container>
        </form>
      </Box>
    </Center>
  );
}
