import { Alert, Center, Text } from "@mantine/core";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faExclamationTriangle } from "@fortawesome/free-solid-svg-icons";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { useAuthContext } from "@contexts/auth.context";
import { Countdown } from "@components/Shared/Countdown";

export default function BannedPage() {
  const { user } = useAuthContext();
  const navigate = useNavigate();

  // State
  const [endData, setEndData] = useState<Date | undefined>(undefined);
  useEffect(() => {
    if (user?.qf_banned_until) setEndData(new Date(user.qf_banned_until));
    if (user?.wfm_banned_until) setEndData(new Date(user.wfm_banned_until));
    if (!user?.qf_banned && !user?.wfm_banned) navigate("/");
  }, [user]);

  // Translate general
  const useTranslatePage = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`banned.${key}`, { ...context }, i18Key);

  return (
    <Center w={"100%"} h={"92vh"}>
      {user?.wfm_banned && (
        <Alert color="red" title={useTranslatePage("wfm.title")} icon={<FontAwesomeIcon icon={faExclamationTriangle} />} maw={"75%"}>
          <Text>{useTranslatePage("wfm.reason", { reason: user?.wfm_banned_reason })}</Text>
          {endData && (
            <Text>
              {useTranslatePage("wfm.until", { reason: user?.qf_banned_reason })} {endData && <Countdown startDate={new Date()} endDate={endData} />}
            </Text>
          )}
        </Alert>
      )}
      {user?.qf_banned && (
        <Alert color="red" title={useTranslatePage("qf.title")} icon={<FontAwesomeIcon icon={faExclamationTriangle} />} maw={"75%"}>
          <Text>{useTranslatePage("qf.reason", { reason: user?.qf_banned_reason })}</Text>
          {endData && (
            <Text>
              {useTranslatePage("qf.until", { reason: user?.qf_banned_reason })} {endData && <Countdown startDate={new Date()} endDate={endData} />}
            </Text>
          )}
        </Alert>
      )}
    </Center>
  );
}
