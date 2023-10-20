import { Grid, Tabs } from "@mantine/core";
import { TransactionControl } from "../../components/transactionControl";
import { StockItemsPanel, StockRivenPanel } from "./tabs";
import { useEffect, useRef, useState } from "react";
import { useTranslatePage } from "../../hooks";
interface IProps {
  iconName: string;
  wrapperStyle?: string;
  svgProp?: React.SVGProps<SVGSVGElement>;
}
export function useDynamicSvgImport(iconName: string) {
  const importedIconRef = useRef<React.FC<React.SVGProps<SVGElement>>>();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<unknown>();

  useEffect(() => {
    setLoading(true);
    // dynamically import the mentioned svg icon name in props
    const importSvgIcon = async (): Promise<void> => {
      // please make sure all your svg icons are placed in the same directory
      // if we want that part to be configurable then instead of iconName we will send iconPath as prop
      try {
        importedIconRef.current = (
          await import(`../../assets/icons/${iconName}.svg`)
          // await import(`../../assets/icons/${iconName}.svg`)
        ).ReactComponent; // svgr provides ReactComponent for given svg path
      } catch (err) {
        setError(err);
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    importSvgIcon();
  }, [iconName]);

  return { error, loading, SvgIcon: importedIconRef.current };
}
export default function LiveTradingPage() {
  const useTranslate = (key: string, context?: { [key: string]: any }) => useTranslatePage(`live_trading.${key}`, { ...context })
  return (
    <Grid>
      <Grid.Col md={12}>
        <TransactionControl />
        <Tabs defaultValue="items">
          <Tabs.List>
            <Tabs.Tab value="items" >
              {useTranslate('tabs.item.title')}
            </Tabs.Tab>
            <Tabs.Tab value="rivens">
              {useTranslate('tabs.riven.title')}
            </Tabs.Tab>
          </Tabs.List>
          <Tabs.Panel value="items">
            <StockItemsPanel />
          </Tabs.Panel>
          <Tabs.Panel value="rivens">
            <StockRivenPanel />
          </Tabs.Panel>
        </Tabs>
      </Grid.Col>
    </Grid>
  );
}
