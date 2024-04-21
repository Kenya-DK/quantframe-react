import { Container } from "@mantine/core";
import { useAppContext } from "@contexts/index";
import { SettingsForm } from "@components";
import api from "@api/index";
import { useQuery } from "@tanstack/react-query";
import { TradableItemList } from "@components";

interface StockRivenPanelProps {
}
export const StockRivenPanel = ({ }: StockRivenPanelProps) => {
    const { settings } = useAppContext();
    const { data } = useQuery({
        queryKey: ['cache_items'],
        queryFn: () => api.cache.getTradableItems(),
    })
    return (
        <Container>
            {settings && (
                <SettingsForm value={settings} onSubmit={() => {
                    console.log('submit');
                }} />
            )}
            <TradableItemList availableItems={data} />
        </Container>
    );
};